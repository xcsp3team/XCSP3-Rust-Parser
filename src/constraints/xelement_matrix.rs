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
        inject_parameters_in_operand, inject_parameters_in_var_val, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;

    use crate::utils::utils_functions::xcsp3_utils::{
        list_to_vec_var_val, str_to_condition_option, to_i32_option, to_matrix,
    };
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XElementMatrix<'a> {
        matrix: Vec<Vec<XVarVal>>,
        set: &'a XVariableSet,
        value: Option<XVarVal>,
        row_index: XVarVal,
        col_index: XVarVal,
        start_row_index: Option<i32>,
        start_col_index: Option<i32>,
        operator: Option<Operator>,
        operand: Option<Operand>,
    }

    impl XConstraintUnfold for XElementMatrix<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            self.row_index = inject_parameters_in_var_val(self.row_index.clone(), arg);
            self.col_index = inject_parameters_in_var_val(self.col_index.clone(), arg);
            if let Some(value) = &mut self.value {
                *value = inject_parameters_in_var_val(value.clone(), arg);
            }
            if let Some(o) = &mut self.operand {
                *o = inject_parameters_in_operand(o, arg);
            }
        }
        fn max_args_used(&self) -> i32 {
            -1
        }
    }

    impl<'a> XElementMatrix<'a> {
        pub fn from_str(
            list: &str,
            value_str: &str,
            index_str: &str,
            start_row_index_str: &str,
            start_col_index_str: &str,
            condition: &str,
            set: &'a XVariableSet,
        ) -> Self {
            let value = XVarVal::from_string(value_str);
            let (row_index, col_index) = if index_str.is_empty() {
                panic!("index in element matrix constraint must be specified");
            } else {
                let tmp = list_to_vec_var_val(index_str);
                if tmp.len() != 2 {
                    panic!("index in element matrix constraint must be of length 2");
                }
                (tmp[0].clone(), tmp[1].clone())
            };

            let start_row_index = to_i32_option(start_row_index_str);
            let start_col_index = to_i32_option(start_col_index_str);
            let (operator, operand) = str_to_condition_option(condition);

            if list.contains("%") {
                panic!("Group and parameters in the matrix of an element is not yet implemented");
            }
            let matrix = to_matrix(list, set);
            Self::new(
                matrix,
                set,
                value,
                row_index,
                col_index,
                start_row_index,
                start_col_index,
                operator,
                operand,
            )
        }

        pub fn new(
            matrix: Vec<Vec<XVarVal>>,
            set: &'a XVariableSet,
            value: Option<XVarVal>,
            row_index: XVarVal,
            col_index: XVarVal,
            start_row_index: Option<i32>,
            start_col_index: Option<i32>,
            operator: Option<Operator>,
            operand: Option<Operand>,
        ) -> Self {
            Self {
                matrix,
                set,
                value,
                row_index,
                col_index,
                start_row_index,
                start_col_index,
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

        pub fn operator(&self) -> &Option<Operator> {
            &self.operator
        }

        pub fn operand(&self) -> &Option<Operand> {
            &self.operand
        }

        pub fn start_row_index(&self) -> i32 {
            self.start_row_index.unwrap_or(0)
        }

        pub fn start_col_index(&self) -> i32 {
            self.start_col_index.unwrap_or(0)
        }

        pub fn row_index(&self) -> &XVarVal {
            &self.row_index
        }

        pub fn col_index(&self) -> &XVarVal {
            &self.col_index
        }
    }
}
