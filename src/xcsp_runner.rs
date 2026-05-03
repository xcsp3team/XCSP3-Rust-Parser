/*
 * xcsp_runner.rs
 *
 * Moteur principal : charge un fichier XCSP3, parse les variables / contraintes /
 * objectifs, puis dispatch chaque élément vers le callback correspondant.
 *
 * Usage :
 *   XcspRunner::run("mon_fichier.xml", &mut mon_callback)?;
 */
use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintUnfold;
use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
use crate::objectives::xobjectives_type::xcsp3_core::XObjectivesType;
use crate::utils::utils_functions::xcsp3_utils::get_all_variables_between_lower_and_upper;
use crate::utils::utils_functions::{
    is_int_list, is_interval_list, is_var_list, scope_contains_expressions, to_expression_list,
    to_int_list, to_interval_list, to_var_list,
};
use crate::variables::xdomain::xcsp3_core::XDomainInteger;
use crate::variables::xvariable_type::xcsp3_core::XVariableType;
use crate::xcsp_callback::XcspCallback;
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::XcspXmlModel;
use std::error::Error;

pub struct XcspRunner;

impl XcspRunner {
    /// Charge le fichier XML situé à `path` et appelle les méthodes du `callback`
    /// au fur et à mesure du parsing.
    ///
    /// Retourne une erreur si le fichier est introuvable ou mal formé.

    pub fn run<C: XcspCallback>(
        path: &str,
        callback: &mut C,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
        println!("variables: {}", variables);
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
                        if let Ok(vec) = tmp {
                            for (s, domain) in &vec {
                                if XDomainInteger::default().equals(domain)
                                    && av.has_others() == false
                                {
                                } else {
                                    call_var(var_id.clone(), domain, callback);
                                }
                            }
                        }
                    }
                    // get_all_variables_between_lower_and_upper
                    callback.end_variable_array();
                }
                XVariableType::XVariableNone(_) => {}
            }
        }
        callback.end_variables();

        // ── Contraintes ──────────────────────────────────────────────────────
        callback.begin_constraints();
        let mut constraints = model.build_constraints(&variables);
        for c in constraints.iter_mut() {
            match c {
                XConstraintType::XGroup(inner) => {
                    callback.begin_group();
                    for arg in inner.get_args() {
                        println!("{:?}", arg);
                        let mut c = inner.get_template().clone();
                        c.extract_parameters(arg);
                        Self::build_constraint(callback, &mut c)?;
                    }
                    callback.end_group();
                }

                _ => {
                    Self::build_constraint(callback, c)?;
                }
            }
        }
        callback.end_constraints();

        // ── Objectifs ────────────────────────────────────────────────────────
        callback.begin_objectives();
        let objectives = model.build_objectives(&variables);
        for o in objectives.iter() {
            match o {
                XObjectivesType::Minimize(inner) => callback.on_objective_minimize(inner),
                XObjectivesType::Maximize(inner) => callback.on_objective_maximize(inner),
                XObjectivesType::XObjectiveNone(_) => {}
            }
        }
        callback.end_objectives();

        callback.end_instance();

        Ok(())
    }

    fn build_constraint<C: XcspCallback>(
        callback: &mut C,
        c: &mut XConstraintType,
    ) -> Result<(), Box<dyn Error>> {
        match c {
            //---------------------------------------------------------------------------------------------------
            // All Diff constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XAllDifferent(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
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
                let tmp: Vec<_> = inner
                    .lists()
                    .iter()
                    .map(|e| to_var_list(e, inner.set()))
                    .collect();
                callback.on_constraint_all_different_list(&*tmp);
            }

            //---------------------------------------------------------------------------------------------------
            // All Equal constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XAllEqual(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_all_equal_v2(&*scope);
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_all_equal_v1(&*scope);
                }
            }
            XConstraintType::XExtension(inner) => callback.on_constraint_extension(inner),

            //---------------------------------------------------------------------------------------------------
            // Intension constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XIntention(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                callback.on_constraint_intention(&*scope, inner.tree());
            }

            //---------------------------------------------------------------------------------------------------
            // Sum constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XSum(inner) => {
                match inner.coeffs() {
                    None => {
                        if scope_contains_expressions(inner.scope()) {
                            let scope: Vec<ExpressionTree> =
                                to_expression_list(&inner.scope(), &inner.set());
                            callback.on_constraint_sum_v4(
                                &*scope,
                                inner.operator(),
                                inner.operand().clone(),
                            );
                        } else {
                            let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                            callback.on_constraint_sum_v1(
                                &*scope,
                                inner.operator(),
                                inner.operand().clone(),
                            );
                        }
                    }
                    Some(vals) => match vals.first() {
                        Some(XVarVal::IntVal(_)) => {
                            let tmp = to_int_list(vals);
                            if scope_contains_expressions(inner.scope()) {
                                let scope: Vec<ExpressionTree> =
                                    to_expression_list(&inner.scope(), &inner.set());
                                if tmp.len() != scope.len() {
                                    panic!("In constraint sum: scope and coefs must have the same size");
                                }
                                callback.on_constraint_sum_v5(
                                    &*scope,
                                    &*tmp,
                                    inner.operator(),
                                    inner.operand().clone(),
                                );
                            } else {
                                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                                if tmp.len() != scope.len() {
                                    panic!("In constraint sum: scope and coefs must have the same size");
                                }
                                callback.on_constraint_sum_v2(
                                    &*scope,
                                    &*tmp,
                                    inner.operator(),
                                    inner.operand().clone(),
                                );
                            }
                        }
                        Some(XVarVal::IntVar(_)) => {
                            let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                            let coefs: Vec<String> = to_var_list(vals, &inner.set());
                            if coefs.len() != scope.len() {
                                panic!(
                                    "In constraint sum: scope and coefs must have the same size"
                                );
                            }
                            callback.on_constraint_sum_v3(
                                &*scope,
                                &*coefs,
                                inner.operator(),
                                inner.operand().clone(),
                            );
                        }
                        Some(_) => panic!("Unexpected variant in coeffs"),
                        None => panic!("coeffs is empty"),
                    },
                }
            }

            //---------------------------------------------------------------------------------------------------
            // Ordered constraints
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XOrdered(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                match inner.lengths() {
                    Some(val) => {
                        let tmp = to_int_list(val);
                        callback.on_constraint_ordered_v2(&*scope, &*tmp, *inner.operator());
                    }
                    None => {
                        callback.on_constraint_ordered_v1(&*scope, *inner.operator());
                    }
                }
            }

            //---------------------------------------------------------------------------------------------------
            // Regular Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XRegular(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                callback.on_constraint_regular(
                    &*scope,
                    inner.start().parse()?,
                    inner.finals(),
                    inner.transitions(),
                );
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
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_maximum_v2(
                        &*scope,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_maximum_v1(
                        &*scope,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                }
            }
            XConstraintType::XMinimum(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_minimum_v2(
                        &*scope,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    callback.on_constraint_minimum_v1(
                        &*scope,
                        inner.operator(),
                        inner.operand().clone(),
                    );
                }
            }

            XConstraintType::XMinimumArg(inner) => {
                if scope_contains_expressions(inner.scope()) {
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
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
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
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

            XConstraintType::XSlide(inner) => callback.on_constraint_slide(inner),
            //---------------------------------------------------------------------------------------------------
            // Extremum Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XCount(inner) => match inner.values().first() {
                Some(XVarVal::IntVal(_)) => {
                    // Values are integers
                    let values = to_int_list(inner.values());
                    if scope_contains_expressions(inner.scope()) {
                        // scope is expressions
                        let scope: Vec<ExpressionTree> =
                            to_expression_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v1(
                            &*scope,
                            &*values,
                            inner.operator(),
                            inner.operand().clone(),
                        );
                    } else {
                        let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v2(
                            &*scope,
                            &*values,
                            inner.operator(),
                            inner.operand().clone(),
                        );
                    }
                }
                Some(XVarVal::IntVar(_)) => {
                    let values: Vec<String> = to_var_list(inner.values(), &inner.set());
                    if scope_contains_expressions(inner.scope()) {
                        // scope is expressions
                        let scope: Vec<ExpressionTree> =
                            to_expression_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v3(
                            &*scope,
                            &*values,
                            inner.operator(),
                            inner.operand().clone(),
                        );
                    } else {
                        let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                        callback.on_constraint_count_v4(
                            &*scope,
                            &*values,
                            inner.operator(),
                            inner.operand().clone(),
                        );
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
                    let scope: Vec<ExpressionTree> =
                        to_expression_list(&inner.scope(), &inner.set());
                    callback.on_constraint_nvalues_v3(
                        &*scope,
                        inner.operator(),
                        inner.operand().clone(),
                    )
                } else {
                    let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                    match inner.except() {
                        None => callback.on_constraint_nvalues_v1(
                            &*scope,
                            inner.operator(),
                            inner.operand().clone(),
                        ),
                        Some(vals) => {
                            let tmp = to_int_list(vals);
                            callback.on_constraint_nvalues_v2(
                                &*scope,
                                &*tmp,
                                inner.operator(),
                                inner.operand().clone(),
                            )
                        }
                    }
                }
            }
            XConstraintType::XCardinality(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                if is_int_list(inner.values()) && is_int_list(inner.occurs()) {
                    let values = to_int_list(inner.values());
                    let occurs = to_int_list(inner.occurs());
                    callback.on_constraint_cardinality_v1(
                        &*scope,
                        &*values,
                        &*occurs,
                        inner.closed(),
                    );
                }
                if is_int_list(inner.values()) && is_var_list(inner.occurs()) {
                    let values = to_int_list(inner.values());
                    let occurs = to_var_list(inner.occurs(), inner.set());
                    callback.on_constraint_cardinality_v2(
                        &*scope,
                        &*values,
                        &*occurs,
                        inner.closed(),
                    );
                }
                if is_int_list(inner.values()) && is_interval_list(inner.occurs()) {
                    let values = to_int_list(inner.values());
                    let occurs = to_interval_list(inner.occurs());
                    callback.on_constraint_cardinality_v3(
                        &*scope,
                        &*values,
                        &*occurs,
                        inner.closed(),
                    );
                }
                if is_var_list(inner.values()) && is_int_list(inner.occurs()) {
                    let values = to_var_list(inner.values(), inner.set());
                    let occurs = to_int_list(inner.occurs());
                    callback.on_constraint_cardinality_v4(
                        &*scope,
                        &*values,
                        &*occurs,
                        inner.closed(),
                    );
                }
                if is_var_list(inner.values()) && is_var_list(inner.occurs()) {
                    let values = to_var_list(inner.values(), inner.set());
                    let occurs = to_var_list(inner.occurs(), inner.set());
                    callback.on_constraint_cardinality_v5(
                        &*scope,
                        &*values,
                        &*occurs,
                        inner.closed(),
                    );
                }
                if is_var_list(inner.values()) && is_interval_list(inner.occurs()) {
                    let values = to_var_list(inner.values(), inner.set());
                    let occurs = to_interval_list(inner.occurs());
                    callback.on_constraint_cardinality_v6(
                        &*scope,
                        &*values,
                        &*occurs,
                        inner.closed(),
                    );
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
            // Channel Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XChannel(inner) => {
                let scope: Vec<String> = to_var_list(&inner.scope(), &inner.set());
                callback.on_constraint_channel_v1(&*scope, inner.start_index());
            }
            //---------------------------------------------------------------------------------------------------
            // Cumulative Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XCumulative(inner) => match inner.ends() {
                None => {
                    if is_int_list(inner.lengths()) && is_int_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_int_list(inner.lengths());
                        let heights = to_int_list(inner.heights());
                        callback.on_constraint_cumulative_v1(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        );
                    }
                    if is_int_list(inner.lengths()) && is_var_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_int_list(inner.lengths());
                        let heights = to_var_list(inner.heights(), inner.set());
                        callback.on_constraint_cumulative_v2(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        );
                    }
                    if is_var_list(inner.lengths()) && is_int_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_var_list(inner.lengths(), inner.set());
                        let heights = to_int_list(inner.heights());
                        callback.on_constraint_cumulative_v3(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        );
                    }
                    if is_var_list(inner.lengths()) && is_var_list(inner.heights()) {
                        let tmp = to_var_list(inner.scope(), inner.set());
                        let lengths = to_var_list(inner.lengths(), inner.set());
                        let heights = to_var_list(inner.heights(), inner.set());
                        callback.on_constraint_cumulative_v4(
                            &*tmp,
                            &*lengths,
                            &*heights,
                            *inner.operator(),
                            inner.operand().clone(),
                        );
                    }
                }
                Some(ends) => {
                    if is_int_list(inner.lengths())
                        && is_int_list(inner.heights())
                        && is_var_list(ends)
                    {
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
                        );
                    }
                    if is_int_list(inner.lengths())
                        && is_var_list(inner.heights())
                        && is_var_list(ends)
                    {
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
                        );
                    }
                    if is_var_list(inner.lengths())
                        && is_int_list(inner.heights())
                        && is_var_list(ends)
                    {
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
                    }
                    if is_var_list(inner.lengths())
                        && is_var_list(inner.heights())
                        && is_var_list(ends)
                    {
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
                    }
                }
            },
            //---------------------------------------------------------------------------------------------------
            // Element Constraint
            //---------------------------------------------------------------------------------------------------
            XConstraintType::XElement(inner) => match inner.index() {
                None => match inner.value() {
                    XVarVal::IntVal(v) => {
                        let scope = to_var_list(inner.scope(), inner.set());
                        callback.on_constraint_element_v1(&*scope, *v);
                    }

                    _ => {
                        panic!("Unexpected variant in value")
                    }
                },
                Some(index) => {}
            },

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
                    _ => {}
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
                    callback.on_constraint_no_overlap_k_dim_v3(
                        &scope,
                        &special_lengths,
                        inner.zero_ignored(),
                    )
                } else {
                    if inner.is_lengths_int() {
                        let mut intlengths: Vec<Vec<i32>> = Vec::new();
                        for sc in inner.lengths() {
                            intlengths.push(to_int_list(sc));
                        }
                        callback.on_constraint_no_overlap_k_dim_v1(
                            &scope,
                            &intlengths,
                            inner.zero_ignored(),
                        )
                    } else {
                        let mut varlengths: Vec<Vec<String>> = Vec::new();
                        for sc in inner.lengths() {
                            varlengths.push(to_var_list(sc, inner.set()));
                        }
                        callback.on_constraint_no_overlap_k_dim_v2(
                            &scope,
                            &varlengths,
                            inner.zero_ignored(),
                        )
                    }
                }
            }
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
            XConstraintType::XStretch(inner) => callback.on_constraint_stretch(inner),
            XConstraintType::XConstraintNone(_) => {}
            _ => {
                panic!("Unknown constraint");
            }
        }
        Ok(())
    }
}
