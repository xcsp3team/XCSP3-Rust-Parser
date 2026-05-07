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

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        arg_in_operand, arg_in_var, inject_parameters_in_list, inject_parameters_in_operand,
        inject_parameters_in_var_val, max_arg_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{
        extract_operator, list_to_matrix_ids, list_to_vec_var_val,
    };
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use crate::variables::xvariable_type::xcsp3_core::XVariableType::{
        XVariableArray, XVariableTree,
    };
    use std::cmp::max;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XElementMatrix<'a> {
        matrix: Vec<Vec<XVarVal>>,
        set: &'a XVariableSet,
        value: Option<XVarVal>,
        index: Option<XVarVal>,
        start_index: Option<i32>,
        operator: Option<Operator>,
        operand: Option<Operand>,
    }

    /*impl XConstraintUnfold for XElementM<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            if let Some(index) = &mut self.index {
                *index = inject_parameters_in_var_val(index.clone(), arg);
            }
            if let Some(value) = &mut self.value {
                *value = inject_parameters_in_var_val(value.clone(), arg);
            }
            if let Some(o) = &mut self.operand {
                *o = inject_parameters_in_operand(o, arg);
            }
        }
        fn max_args_used(&mut self) -> i32 {
            let tmp = max_arg_in_list(&*self.scope);
            let tmp = match self.operand.clone() {
                Some(v) => max(tmp, arg_in_operand(&v)),
                None => tmp,
            };
            let tmp = match self.index() {
                Some(v) => max(tmp, arg_in_var(&v)),
                None => tmp,
            };
            match self.index() {
                Some(v) => max(tmp, arg_in_var(&v)),
                None => tmp,
            }
        }
    }*/

    impl<'a> XElementMatrix<'a> {
        pub fn from_str(
            list: &str,
            value_str: &str,
            index_str: &str,
            start_index_str: &str,
            condition: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let value = match XVarVal::from_string(value_str) {
                None => None,
                Some(v) => Some(v),
            };
            let index = if index_str.is_empty() {
                None
            } else {
                match XVarVal::from_string(index_str) {
                    None => {
                        return Err(Xcsp3Error::get_constraint_sum_error(
                            "parse element constraint index error, ",
                        ));
                    }
                    Some(i) => Some(i),
                }
            };
            let start_index = if !start_index_str.is_empty() {
                match start_index_str.parse::<i32>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_sum_error(
                            "parse element constraint start_index error, ",
                        ));
                    }
                }
            } else {
                None
            };
            let (operator, operand) = if condition.is_empty() {
                (None, None)
            } else {
                match extract_operator(&condition) {
                    Ok((op, val)) => (Some(op), Some(val)),
                    Err(_) => panic!("condition in binpacking is wrong: {}", condition),
                }
            };
            if list.contains("[][]") {
                let name = list.split('[').next().unwrap_or(list);
                let vartype = set.find_variable(name)?;
                let size = match vartype {
                    XVariableArray(v) => v.sizes[0],
                    XVariableTree(v) => v.sizes[0],
                    _ => 0,
                };
                let mut matrix: Vec<Vec<XVarVal>> = Vec::with_capacity(size);
                for i in 0..size {
                    matrix.push(vec![]);
                    for j in 0..size {
                        matrix[i].push(XVarVal::IntVar(format!("{}[{}][{}]", name, i, j)));
                    }
                }
                Ok(Self::new(
                    matrix,
                    set,
                    value,
                    index,
                    start_index,
                    operator,
                    operand,
                ))
            } else {
                let matrix: Vec<Vec<XVarVal>> = list_to_matrix_ids(list)
                    .iter()
                    .map(|line| line.iter().map(|e| XVarVal::IntVar(e.clone())).collect())
                    .collect();
                Ok(Self::new(
                    matrix,
                    set,
                    value,
                    index,
                    start_index,
                    operator,
                    operand,
                ))
            }
        }

        pub fn new(
            matrix: Vec<Vec<XVarVal>>,
            set: &'a XVariableSet,
            value: Option<XVarVal>,
            index: Option<XVarVal>,
            start_index: Option<i32>,
            operator: Option<Operator>,
            operand: Option<Operand>,
        ) -> Self {
            Self {
                matrix,
                set,
                value,
                index,
                start_index,
                operator,
                operand,
            }
        }

        pub fn matrix(&self) -> &Vec<Vec<XVarVal>> {
            &self.matrix
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn value(&self) -> &Option<XVarVal> {
            &self.value
        }

        pub fn index(&self) -> &Option<XVarVal> {
            &self.index
        }

        pub fn start_index(&self) -> i32 {
            self.start_index.unwrap_or(0)
        }

        pub fn operator(&self) -> &Option<Operator> {
            &self.operator
        }

        pub fn operand(&self) -> &Option<Operand> {
            &self.operand
        }
    }
}
