/*=============================================================================
* RUST parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2026 xcsp.org (contact @ xcsp.org)
*
* Based on the original Rust parser proposed in https://github.com/luhanzhen/xcsp3-rust
* by Luhan Zhen (zhenlh20@mails.jlu.edu.cn)
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in
* all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
* THE SOFTWARE.
*=============================================================================
*/
use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintUnfold;
use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
use crate::objectives::xobjectives_set::xcsp3_core::XObjective;
use crate::objectives::xobjectives_set::xcsp3_core::XObjective::{XObjectiveElement, XObjectiveExpression};
use crate::utils::utils_functions::xcsp3_utils::get_all_variables_between_lower_and_upper;
use crate::utils::utils_functions::{
    is_int_list, is_interval_list, is_var_list, scope_contains_expressions, to_expression_list, to_int_list,
    to_interval_list, to_var_list,
};
use crate::variables::xdomain::xcsp3_core::XDomainInteger;
use crate::variables::xvariable_type::xcsp3_core::XVariableType;
use crate::xcsp_callback::XcspCallback;
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::{InstanceType, XcspXmlModel};
use std::error::Error;

pub struct XcspRunner;

impl XcspRunner {
    /// Charge le fichier XML situé à `path` et appelle les méthodes du `callback`
    /// au fur et à mesure du parsing.
    ///
    /// Retourne une erreur si le fichier est introuvable ou mal formé.

    pub fn run<C: XcspCallback>(path: &str, callback: &mut C) -> Result<(), Box<dyn std::error::Error>> {
        pub fn call_var(id: String, domain: &XDomainInteger, callback: &mut dyn XcspCallback) {
            if domain.is_interval() {
                callback.on_variable_interval(id, domain.minimum(), domain.maximum());
            } else {
                let mut result = vec![];
                for v in domain.iter() {
                    result.push(v)
                }
                callback.on_variable_values(id, &result)
            }
        }

        let model = XcspXmlModel::from_path(path)?;

        callback.begin_instance(model.get_instance_type());
        // ── Variables ────────────────────────────────────────────────────────
        callback.begin_variables();
        let variables = model.build_variables();
        for v in variables.iter() {
            match v {
                XVariableType::XVariableInt(var) => {
                    call_var(v.get_id(), &var.domain, callback);
                }
                XVariableType::XVariableArray(av) => {
                    callback.begin_variable_array(v.get_id());
                    for var_id in av.variables.iter() {
                        call_var(var_id.clone(), &av.domain, callback);
                    }

                    callback.end_variable_array();
                }
                XVariableType::XVariableTree(av) => {
                    callback.begin_variable_array(v.get_id());
                    let lower = vec![0; av.sizes().len()];
                    let upper: Vec<usize> = av.sizes().iter().map(|size| size - 1).collect();
                    let all = get_all_variables_between_lower_and_upper(lower, upper);
                    for sz in all.iter() {
                        let brackets: String = sz.iter().map(|n| format!("[{}]", n)).collect();
                        let var_id = format!("{}{}", av.id, brackets);
                        let tmp = av.find_variable(&*brackets);
                        for (_s, domain) in &tmp {
                            if XDomainInteger::default().equals(domain) && av.has_others() == false {
                            } else {
                                call_var(var_id.clone(), domain, callback);
                            }
                        }
                    }
                    // get_all_variables_between_lower_and_upper
                    callback.end_variable_array();
                }
            }
        }
        callback.end_variables();

        // ── Contraintes ──────────────────────────────────────────────────────
        callback.begin_constraints();
        let mut constraints = model.build_constraints(&variables);
        for c in constraints.iter_mut() {
            match c {
                // --------------- GROUP -------------------------
                XConstraintType::XGroup(inner) => {
                    callback.begin_group();
                    for arg in inner.get_args() {
                        let mut c = inner.get_template().clone();
                        c.extract_parameters(arg);
                        Self::build_constraint(callback, &mut c)?;
                    }
                    callback.end_group();
                }
                // --------------- Slide -------------------------
                XConstraintType::XSlide(inner) => {
                    callback.begin_slide();
                    let len = inner.args().len();
                    let arguments = inner.args();
                    let mut tmp = inner.template().max_args_used() + 1;
                    if tmp == 0 {
                        tmp = inner.collect();
                    }
                    let nb: usize = tmp as usize;
                    let mut i = 0;
                    while i + nb <= len || (inner.circular() && i < len) {
                        let arg: Vec<XVarVal> = (0..nb).map(|j| arguments[(i + j) % len].clone()).collect();
                        let mut c = inner.template().clone();
                        c.extract_parameters(&*arg);
                        Self::build_constraint(callback, &mut c)?;
                        i += inner.offset() as usize;
                    }
                    callback.end_slide();
                }

                _ => {
                    Self::build_constraint(callback, c)?;
                }
            }
        }
        callback.end_constraints();

        // ── Objectifs ────────────────────────────────────────────────────────
        if model.get_instance_type() == InstanceType::Cop {
            callback.begin_objectives();

            let objectives = model.build_objectives(&variables);
            for objective in objectives.objectives().iter() {
                Self::build_objective(callback, objective);
            }
            callback.end_objectives();
        }
        callback.end_instance();

        Ok(())
    }

    fn build_constraint<C: XcspCallback>(callback: &mut C, c: &mut XConstraintType) -> Result<(), Box<dyn Error>> {
        match c {
            //---------------------------------------------------------------------------------------------------
            // All Diff constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XAllDifferent(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_all_different_v2(&*scope);
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_all_different_v1(&*scope);
                }
            }
            XConstraintType::XAllDifferentExcept(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                callback.on_constraint_all_different_except(&*scope, &*inner.except());
            }

            XConstraintType::XAllDifferentList(inner) => {
                let tmp: Vec<_> = inner.lists().iter().map(|e| to_var_list(e, inner.set())).collect();
                callback.on_constraint_all_different_list(&*tmp);
            }
            XConstraintType::XAllDifferentMatrix(inner) => {
                let tmp: Vec<_> = inner.matrix().iter().map(|e| to_var_list(e, inner.set())).collect();
                callback.on_constraint_all_different_matrix(&*tmp);
            }

            //---------------------------------------------------------------------------------------------------
            // All Equal constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XAllEqual(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_all_equal_v2(&*scope);
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_all_equal_v1(&*scope);
                }
            }
            XConstraintType::XExtension(inner) => {
                let scope = to_var_list(&inner.scope(), &inner.set());
                if scope.len() == 1 {
                    let tmp: Vec<_> = inner.tuples().iter().flatten().copied().collect();
                    callback.on_constraint_unary(&scope[0], &*tmp, inner.is_support());
                } else {
                    callback.on_constraint_extension(&*scope, inner.tuples(), inner.is_support(), inner.has_star());
                }
            }

            //---------------------------------------------------------------------------------------------------
            // Intension constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XIntention(inner) => {
                let tree = inner.to_tree();
                let scope = tree.get_scope();
                callback.on_constraint_intention(&*scope, &tree);
            }

            //---------------------------------------------------------------------------------------------------
            // Sum constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XSum(inner) => match inner.coeffs() {
                None => {
                    if scope_contains_expressions(inner.scope()) {
                        let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                        callback.on_constraint_sum_v4(&*scope, inner.operator(), inner.operand().clone());
                    } else {
                        let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                        callback.on_constraint_sum_v1(&*scope, inner.operator(), inner.operand().clone());
                    }
                }
                Some(vals) => match vals.first() {
                    Some(XVarVal::IntVal(_)) => {
                        let tmp = to_int_list(vals);
                        if scope_contains_expressions(inner.scope()) {
                            let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                            if tmp.len() != scope.len() {
                                panic!("In constraint sum: scope and coefs must have the same size");
                            }
                            callback.on_constraint_sum_v5(&*scope, &*tmp, inner.operator(), inner.operand().clone());
                        } else {
                            let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                            if tmp.len() != scope.len() {
                                panic!("In constraint sum: scope and coefs must have the same size");
                            }
                            callback.on_constraint_sum_v2(&*scope, &*tmp, inner.operator(), inner.operand().clone());
                        }
                    }
                    Some(XVarVal::IntVar(_)) => {
                        let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                        let coefs: Vec<String> = to_var_list(vals, &inner.set());
                        if coefs.len() != scope.len() {
                            panic!("In constraint sum: scope and coefs must have the same size");
                        }
                        callback.on_constraint_sum_v3(&*scope, &*coefs, inner.operator(), inner.operand().clone());
                    }
                    Some(_) => panic!("Unexpected variant in coeffs"),
                    None => panic!("coeffs is empty"),
                },
            },

            //---------------------------------------------------------------------------------------------------
            // Ordered constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XOrdered(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                match inner.lengths() {
                    Some(val) => {
                        if is_int_list(val) {
                            let tmp = to_int_list(val);
                            callback.on_constraint_ordered_v2(&*scope, &*tmp, *inner.operator());
                        } else {
                            let tmp: Vec<String> = to_var_list(val, &inner.set());
                            callback.on_constraint_ordered_v3(&*scope, &*tmp, *inner.operator());
                        }
                    }
                    None => {
                        callback.on_constraint_ordered_v1(&*scope, *inner.operator());
                    }
                }
            }
            //---------------------------------------------------------------------------------------------------
            // LEx constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XLex(inner) => {
                let mut lists = Vec::with_capacity(inner.lists().len());
                for list in inner.lists().iter() {
                    lists.push(to_var_list(list, inner.set()));
                }
                callback.on_constraint_lex(&lists, *inner.operator());
            }
            XConstraintType::XLexMatrix(inner) => {
                let mut lists = Vec::with_capacity(inner.matrix().len());
                for list in inner.matrix().iter() {
                    lists.push(to_var_list(list, inner.set()));
                }
                callback.on_constraint_lex_matrix(&lists, *inner.operator());
            }
            //---------------------------------------------------------------------------------------------------
            // Regular Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XRegular(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                callback.on_constraint_regular(&*scope, inner.start().parse()?, inner.finals(), inner.transitions());
            }

            //---------------------------------------------------------------------------------------------------
            // MDD Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XMdd(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                callback.on_constraint_mdd(&*scope, inner.transitions());
            }

            //---------------------------------------------------------------------------------------------------
            // Instantiation constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XInstantiation(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                if scope.len() != inner.values().len() {
                    panic!("In instantiation constraint: list and values must have same size");
                }
                let values = to_int_list(inner.values());
                callback.on_constraint_instantiation(&*scope, &*values)
            }
            //---------------------------------------------------------------------------------------------------
            // Extremum Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XMaximum(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_maximum_v2(&*scope, inner.operator(), inner.operand().clone());
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_maximum_v1(&*scope, inner.operator(), inner.operand().clone());
                }
            }
            XConstraintType::XMinimum(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_minimum_v2(&*scope, inner.operator(), inner.operand().clone());
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_minimum_v1(&*scope, inner.operator(), inner.operand().clone());
                }
            }

            XConstraintType::XMinimumArg(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_minimum_arg_v2(
                        &*scope,
                        inner.start_index(),
                        inner.rank().parse()?,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_minimum_arg_v1(
                        &*scope,
                        inner.start_index(),
                        inner.rank().parse()?,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                }
            }

            XConstraintType::XMaximumArg(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_maximum_arg_v2(
                        &*scope,
                        inner.start_index(),
                        inner.rank().parse()?,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_maximum_arg_v1(
                        &*scope,
                        inner.start_index(),
                        inner.rank().parse()?,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                }
            }

            //---------------------------------------------------------------------------------------------------
            // Extremum Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XCount(inner) => match inner.values().first() {
                Some(XVarVal::IntVal(_)) => {
                    // Values are integers
                    let values = to_int_list(inner.values());
                    if scope_contains_expressions(inner.scope()) {
                        // scope is expressions
                        let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v1(&*scope, &*values, inner.operator(), inner.operand().clone());
                    } else {
                        let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v2(&*scope, &*values, inner.operator(), inner.operand().clone());
                    }
                }
                Some(XVarVal::IntVar(_)) => {
                    let values: Vec<String> = to_var_list(inner.values(), &inner.set());
                    if scope_contains_expressions(inner.scope()) {
                        // scope is expressions
                        let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v3(&*scope, &*values, inner.operator(), inner.operand().clone());
                    } else {
                        let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v4(&*scope, &*values, inner.operator(), inner.operand().clone());
                    }
                }
                Some(_) => panic!("Unexpected variant in coeffs"),
                None => panic!("coeffs is empty"),
            },

            //---------------------------------------------------------------------------------------------------
            // NVALUES Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XNValues(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> = to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_nvalues_v3(&*scope, inner.operator(), inner.operand().clone())
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    match inner.except() {
                        None => callback.on_constraint_nvalues_v1(&*scope, inner.operator(), inner.operand().clone()),
                        Some(vals) => {
                            let tmp = to_int_list(vals);
                            callback.on_constraint_nvalues_v2(&*scope, &*tmp, inner.operator(), inner.operand().clone())
                        }
                    }
                }
            }
            XConstraintType::XCardinality(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                if is_int_list(inner.values()) && is_int_list(inner.occurs()) {
                    let values = to_int_list(inner.values());
                    let occurs = to_int_list(inner.occurs());
                    callback.on_constraint_cardinality_v1(&*scope, &*values, &*occurs, inner.closed())
                } else if is_int_list(inner.values()) && is_var_list(inner.occurs()) {
                    let values = to_int_list(inner.values());
                    let occurs = to_var_list(inner.occurs(), inner.set());
                    callback.on_constraint_cardinality_v2(&*scope, &*values, &*occurs, inner.closed())
                } else if is_int_list(inner.values()) && is_interval_list(inner.occurs()) {
                    let values = to_int_list(inner.values());
                    let occurs = to_interval_list(inner.occurs());
                    callback.on_constraint_cardinality_v3(&*scope, &*values, &*occurs, inner.closed())
                } else if is_var_list(inner.values()) && is_int_list(inner.occurs()) {
                    let values = to_var_list(inner.values(), inner.set());
                    let occurs = to_int_list(inner.occurs());
                    callback.on_constraint_cardinality_v4(&*scope, &*values, &*occurs, inner.closed())
                } else if is_var_list(inner.values()) && is_var_list(inner.occurs()) {
                    let values = to_var_list(inner.values(), inner.set());
                    let occurs = to_var_list(inner.occurs(), inner.set());
                    callback.on_constraint_cardinality_v5(&*scope, &*values, &*occurs, inner.closed())
                } else if is_var_list(inner.values()) && is_interval_list(inner.occurs()) {
                    let values = to_var_list(inner.values(), inner.set());
                    let occurs = to_interval_list(inner.occurs());
                    callback.on_constraint_cardinality_v6(&*scope, &*values, &*occurs, inner.closed())
                } else {
                    panic!("Unexpected variant for cardinality constraint");
                }
            }
            //---------------------------------------------------------------------------------------------------
            // Precedence Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XPrecedence(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());

                match inner.values() {
                    None => {
                        callback.on_constraint_precedence_v1(&*scope, inner.covered());
                    }
                    Some(vals) => {
                        let values = to_int_list(vals);
                        callback.on_constraint_precedence_v2(&*scope, &*values, inner.covered());
                    }
                }
            }
            //---------------------------------------------------------------------------------------------------
            // Clause Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XClause(inner) => {
                let pos = to_var_list(inner.positive_literals(), inner.set());
                let neg = to_var_list(inner.negative_literals(), inner.set());
                callback.on_constraint_clause(&*pos, &*neg);
            }
            //---------------------------------------------------------------------------------------------------
            // Flow Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XFlow(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                let balances = to_int_list(inner.balances());
                let weights = to_int_list(inner.weights());

                callback.on_constraint_flow(
                    &*scope,
                    &*balances,
                    &*weights,
                    inner.arcs(),
                    inner.operator(),
                    inner.operand().clone(),
                );
            }
            //---------------------------------------------------------------------------------------------------
            // Knapsack Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XKnapsack(inner) => {
                let scope = to_var_list(&inner.scope(), &inner.set());
                let profits = to_int_list(&inner.profits());
                let weights = to_int_list(&inner.weights());
                callback.on_constraint_knapsack(
                    &*scope,
                    &*weights,
                    inner.weight_operator(),
                    inner.weight_operand().clone(),
                    &*profits,
                    inner.profit_operator(),
                    inner.profit_operand().clone(),
                );
            }
            //---------------------------------------------------------------------------------------------------
            // BinPacking Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XBinpacking(inner) => {
                let scope = to_var_list(&inner.scope(), &inner.set());
                let sizes = to_int_list(&inner.sizes());
                match inner.operand() {
                    None => {
                        if inner.limits().len() > 0 && is_int_list(inner.limits()) {
                            let limits = to_int_list(inner.limits());
                            callback.on_constraint_bin_packing_v2(&*scope, &*sizes, &*limits);
                        }
                        if inner.limits().len() > 0 && is_var_list(inner.limits()) {
                            let limits = to_var_list(inner.limits(), inner.set());
                            callback.on_constraint_bin_packing_v3(&*scope, &*sizes, &*limits);
                        }
                        if inner.loads().len() > 0 && is_int_list(inner.loads()) {
                            let loads = to_int_list(inner.loads());
                            callback.on_constraint_bin_packing_v4(&*scope, &*sizes, &*loads);
                        }
                        if inner.loads().len() > 0 && is_var_list(inner.loads()) {
                            let loads = to_var_list(inner.loads(), inner.set());
                            callback.on_constraint_bin_packing_v5(&*scope, &*sizes, &*loads);
                        }
                    }
                    Some(operand) => match inner.operator() {
                        None => {}
                        Some(o) => {
                            callback.on_constraint_bin_packing_v1(&*scope, &*sizes, *o, operand.clone());
                        }
                    },
                }
            }

            //---------------------------------------------------------------------------------------------------
            // Channel Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XChannel(inner) => {
                let list1: Vec<String> = to_var_list(&inner.list1(), &inner.set());
                match inner.value() {
                    None => {
                        if inner.list2().len() == 0 {
                            callback.on_constraint_channel_v1(&*list1, inner.start_index1())
                        } else {
                            let list2 = to_var_list(&inner.list2(), &inner.set());
                            callback.on_constraint_channel_v2(
                                &*list1,
                                inner.start_index1(),
                                &*list2,
                                inner.start_index2(),
                            );
                        }
                    }
                    Some(value) => {
                        match value {
                            XVarVal::IntVar(v) => {
                                callback.on_constraint_channel_v3(&*list1, inner.start_index1(), v.clone())
                            }
                            _ => panic!("Expected value in channel to be var"),
                        };
                    }
                }
            }
            //---------------------------------------------------------------------------------------------------
            // Cumulative Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XCumulative(inner) => match inner.ends() {
                None => {
                    if is_int_list(inner.lengths()) && is_int_list(inner.heights()) {
                        println!("{:?}", inner.scope());
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_int_list(inner.lengths());
                        let heights = to_int_list(inner.heights());
                        callback.on_constraint_cumulative_v1(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else if is_int_list(inner.lengths()) && is_var_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_int_list(inner.lengths());
                        let heights = to_var_list(inner.heights(), inner.set());
                        callback.on_constraint_cumulative_v2(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else if is_var_list(inner.lengths()) && is_int_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_var_list(inner.lengths(), inner.set());
                        let heights = to_int_list(inner.heights());
                        callback.on_constraint_cumulative_v3(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else if is_var_list(inner.lengths()) && is_var_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_var_list(inner.lengths(), inner.set());
                        let heights = to_var_list(inner.heights(), inner.set());
                        callback.on_constraint_cumulative_v4(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else {
                        panic!("Unexpected variant for cumulative constraint");
                    }
                }
                Some(ends) => {
                    if is_int_list(inner.lengths()) && is_int_list(inner.heights()) && is_var_list(ends) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_int_list(inner.lengths());
                        let heights = to_int_list(inner.heights());
                        let to_ends = to_var_list(ends, inner.set());
                        callback.on_constraint_cumulative_v5(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            &*to_ends,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else if is_int_list(inner.lengths()) && is_var_list(inner.heights()) && is_var_list(ends) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_int_list(inner.lengths());
                        let heights = to_var_list(inner.heights(), inner.set());
                        let to_ends = to_var_list(ends, inner.set());
                        callback.on_constraint_cumulative_v6(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            &*to_ends,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else if is_var_list(inner.lengths()) && is_int_list(inner.heights()) && is_var_list(ends) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_var_list(inner.lengths(), inner.set());
                        let heights = to_int_list(inner.heights());
                        let to_ends = to_var_list(ends, inner.set());
                        callback.on_constraint_cumulative_v7(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            &*to_ends,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else if is_var_list(inner.lengths()) && is_var_list(inner.heights()) && is_var_list(ends) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_var_list(inner.lengths(), inner.set());
                        let heights = to_var_list(inner.heights(), inner.set());
                        let to_ends = to_var_list(ends, inner.set());
                        callback.on_constraint_cumulative_v8(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            &*to_ends,
                            *inner.operator(),
                            inner.operand().clone(),
                        )
                    } else {
                        panic!("Unexpected variant for cumulative constraint");
                    }
                }
            },
            //---------------------------------------------------------------------------------------------------
            // Element Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XElement(inner) => {
                if is_var_list(inner.scope()) {
                    let scope = to_var_list(inner.scope(), inner.set());
                    if let Some(index) = inner.index() {
                        if let Some(value) = inner.value() {
                            match value {
                                XVarVal::IntVal(v) => callback.on_constraint_element_v4(
                                    &*scope,
                                    inner.start_index(),
                                    index.to_string(),
                                    *v,
                                ),
                                XVarVal::IntVar(v) => callback.on_constraint_element_v3(
                                    &*scope,
                                    inner.start_index(),
                                    index.to_string(),
                                    v.clone(),
                                ),
                                _ => panic!("Unexpected value for element constraint"),
                            }
                        } else {
                            let (operand, operator) = inner
                                .operand()
                                .clone()
                                .zip(*inner.operator())
                                .expect("Missing condition for element constraint");

                            callback.on_constraint_element_v5(
                                &*scope,
                                inner.start_index(),
                                index.to_string(),
                                operator,
                                operand.clone(),
                            );
                        }
                    } else {
                        let Some(value) = inner.value() else {
                            panic!("Wanted a value or an index in element constraint")
                        };
                        match value {
                            XVarVal::IntVal(v) => callback.on_constraint_element_v1(&*scope, *v),
                            XVarVal::IntVar(v) => callback.on_constraint_element_v2(&*scope, v.clone()),
                            _ => panic!("Unexpected value for element constraint"),
                        }
                    }
                } else if is_int_list(inner.scope()) {
                    let scope = to_int_list(inner.scope());
                    let Some(index) = inner.index() else { panic!("Wanted an index element constraint with int list") };
                    match inner.value() {
                        None => {
                            let (operand, operator) = inner
                                .operand()
                                .clone()
                                .zip(*inner.operator())
                                .expect("Missing condition for element constraint");
                            callback.on_constraint_element_v8(
                                &*scope,
                                inner.start_index(),
                                index.to_string(),
                                operator,
                                operand.clone(),
                            )
                        }
                        Some(value) => match value {
                            XVarVal::IntVal(v) => {
                                callback.on_constraint_element_v7(&*scope, inner.start_index(), index.to_string(), *v)
                            }
                            XVarVal::IntVar(v) => callback.on_constraint_element_v6(
                                &*scope,
                                inner.start_index(),
                                index.to_string(),
                                v.clone(),
                            ),
                            _ => panic!("Unexpected value for element constraint"),
                        },
                    }
                }
            }
            //---------------------------------------------------------------------------------------------------
            // Element Matrix Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XElementMatrix(inner) => {
                if is_var_list(&*inner.matrix()[0]) {
                    let matrix: Vec<_> = inner.matrix().iter().map(|e| to_var_list(e, inner.set())).collect();
                    if let Some(value) = inner.value() {
                        match value {
                            XVarVal::IntVal(v) => callback.on_constraint_element_matrix_v1(
                                &matrix,
                                inner.row_index().to_string(),
                                inner.col_index().to_string(),
                                inner.start_row_index(),
                                inner.start_col_index(),
                                *v,
                            ),
                            XVarVal::IntVar(v) => callback.on_constraint_element_matrix_v2(
                                &matrix,
                                inner.row_index().to_string(),
                                inner.col_index().to_string(),
                                inner.start_row_index(),
                                inner.start_col_index(),
                                v.clone(),
                            ),
                            _ => panic!("Unexpected value for element constraint"),
                        }
                    } else {
                        let (operand, operator) = inner
                            .operand()
                            .clone()
                            .zip(*inner.operator())
                            .expect("Missing condition for element constraint");
                        callback.on_constraint_element_matrix_v3(
                            &matrix,
                            inner.row_index().to_string(),
                            inner.col_index().to_string(),
                            inner.start_row_index(),
                            inner.start_col_index(),
                            operator,
                            operand,
                        )
                    }
                }
                if is_int_list(&*inner.matrix()[0]) {
                    let matrix: Vec<_> = inner.matrix().iter().map(|e| to_int_list(e)).collect();

                    match inner.value() {
                        None => {
                            let (operand, operator) = inner
                                .operand()
                                .clone()
                                .zip(*inner.operator())
                                .expect("Missing condition for element constraint");
                            callback.on_constraint_element_matrix_v4(
                                &matrix,
                                inner.row_index().to_string(),
                                inner.col_index().to_string(),
                                inner.start_row_index(),
                                inner.start_col_index(),
                                operator,
                                operand,
                            )
                        }
                        Some(value) => match value {
                            XVarVal::IntVal(v) => callback.on_constraint_element_matrix_v5(
                                &matrix,
                                inner.row_index().to_string(),
                                inner.col_index().to_string(),
                                inner.start_row_index(),
                                inner.start_col_index(),
                                *v,
                            ),
                            XVarVal::IntVar(v) => callback.on_constraint_element_matrix_v6(
                                &matrix,
                                inner.row_index().to_string(),
                                inner.col_index().to_string(),
                                inner.start_row_index(),
                                inner.start_col_index(),
                                v.clone(),
                            ),
                            _ => panic!("Unexpected value for element constraint"),
                        },
                    }
                }
            }
            //---------------------------------------------------------------------------------------------------
            // NoOverlap Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XNoOverlap(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                match inner.lengths().first() {
                    Some(XVarVal::IntVal(_)) => {
                        let tmp = to_int_list(inner.lengths());
                        callback.on_constraint_no_overlap_v1(&*scope, &*tmp, inner.zero_ignored())
                    }
                    Some(XVarVal::IntVar(_)) => {
                        let tmp = to_var_list(&inner.lengths(), inner.set());
                        callback.on_constraint_no_overlap_v2(&*scope, &*tmp, inner.zero_ignored())
                    }
                    _ => {
                        panic!("Unexpected variant for nooverlap constraint");
                    }
                }
            }
            XConstraintType::XNoOverlapKDim(inner) => {
                let mut scope: Vec<Vec<String>> = Vec::new();
                for sc in inner.scope() {
                    scope.push(to_var_list(sc, inner.set()));
                }

                if inner.first_length_is_var_val() {
                    let special_lengths: Vec<_> = inner
                        .lengths()
                        .iter()
                        .map(|length| match length.as_slice() {
                            [XVarVal::IntVar(var), XVarVal::IntVal(value)] => (var.clone(), *value),
                            _ => panic!("Expected each length to be [IntVar, IntVal]"),
                        })
                        .collect();
                    callback.on_constraint_no_overlap_k_dim_v3(&scope, &special_lengths, inner.zero_ignored())
                } else {
                    if inner.is_lengths_int() {
                        let mut intlengths: Vec<Vec<i32>> = Vec::new();
                        for sc in inner.lengths() {
                            intlengths.push(to_int_list(sc));
                        }
                        callback.on_constraint_no_overlap_k_dim_v1(&scope, &intlengths, inner.zero_ignored())
                    } else {
                        let mut varlengths: Vec<Vec<String>> = Vec::new();
                        for sc in inner.lengths() {
                            varlengths.push(to_var_list(sc, inner.set()));
                        }
                        callback.on_constraint_no_overlap_k_dim_v2(&scope, &varlengths, inner.zero_ignored())
                    }
                }
            }
            //---------------------------------------------------------------------------------------------------
            // Element Circuit
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XCircuit(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());

                match inner.size() {
                    None => {
                        callback.on_constraint_circuit_v1(&scope);
                    }
                    Some(size) => match size {
                        XVarVal::IntVar(v) => callback.on_constraint_circuit_v3(&scope, v.clone()),
                        XVarVal::IntVal(v) => callback.on_constraint_circuit_v2(&scope, *v),
                        _ => panic!("Expected size to be var or integer"),
                    },
                }
            }
            //---------------------------------------------------------------------------------------------------
            // Streteh Constraint
            //---------------------------------------------------------------------------------------------------
            //XConstraintType::XStretch(inner) => callback.on_constraint_stretch(inner),
            _ => {
                panic!("Unknown constraint");
            }
        }
        Ok(())
    }

    fn build_objective<C: XcspCallback>(callback: &mut C, objective: &XObjective) {
        match objective {
            XObjectiveExpression(o) => match o.expression().as_variable() {
                None => {
                    if o.is_maximize() {
                        callback.on_maximize_expression(o.expression());
                    } else {
                        callback.on_minimize_expression(o.expression());
                    }
                }
                Some(v) => {
                    if o.is_maximize() {
                        callback.on_maximize_var(v.clone());
                    } else {
                        callback.on_minimize_var(v.clone());
                    }
                }
            },
            XObjectiveElement(o) => {
                if o.coeffs().is_empty() == false && scope_contains_expressions(o.scope()) && is_var_list(o.coeffs()) {
                    let sc = to_expression_list(o.scope(), o.set());
                    let co = to_var_list(o.coeffs(), o.set());
                    if o.is_maximize() {
                        callback.on_maximize_v4(o.operator().clone(), &*sc, &*co);
                    } else {
                        callback.on_minimize_v4(o.operator().clone(), &*sc, &*co);
                    }
                    return;
                }
                if o.coeffs().is_empty() == false && scope_contains_expressions(o.scope()) && is_int_list(o.coeffs()) {
                    let sc = to_expression_list(o.scope(), o.set());
                    let co = to_int_list(o.coeffs());
                    if o.is_maximize() {
                        callback.on_maximize_v3(o.operator().clone(), &*sc, &*co);
                    } else {
                        callback.on_minimize_v3(o.operator().clone(), &*sc, &*co);
                    }
                    return;
                }
                if o.coeffs().is_empty() == false && is_var_list(o.scope()) && is_int_list(o.coeffs()) {
                    let sc = to_var_list(o.scope(), o.set());
                    let co = to_int_list(o.coeffs());
                    if o.is_maximize() {
                        callback.on_maximize_v1(o.operator().clone(), &*sc, &*co);
                    } else {
                        callback.on_minimize_v1(o.operator().clone(), &*sc, &*co);
                    }
                    return;
                }
                if o.coeffs().is_empty() == false && is_var_list(o.scope()) && is_var_list(o.coeffs()) {
                    let sc = to_var_list(o.scope(), o.set());
                    let co = to_var_list(o.coeffs(), o.set());
                    if o.is_maximize() {
                        callback.on_maximize_v2(o.operator().clone(), &*sc, &*co);
                    } else {
                        callback.on_minimize_v2(o.operator().clone(), &*sc, &*co);
                    }
                    return;
                }

                // ------------ WIHTOUT COEFS

                if o.coeffs().is_empty() && scope_contains_expressions(o.scope()) {
                    let sc = to_expression_list(o.scope(), o.set());
                    if o.is_maximize() {
                        callback.on_maximize_v6(o.operator().clone(), &*sc);
                    } else {
                        callback.on_minimize_v6(o.operator().clone(), &*sc);
                    }
                    return;
                }
                if o.coeffs().is_empty() && is_var_list(o.scope()) {
                    let sc = to_var_list(o.scope(), o.set());
                    if o.is_maximize() {
                        callback.on_maximize_v5(o.operator().clone(), &*sc);
                    } else {
                        callback.on_minimize_v5(o.operator().clone(), &*sc);
                    }
                    return;
                }
                panic!("Unexpected case of objective");
            }
        }
    }
}
