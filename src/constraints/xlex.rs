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
        inject_parameters_in_list, max_arg_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XLex<'a> {
        lists: Vec<Vec<XVarVal>>,
        operator: Operator,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XLex<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            for list in self.lists.iter_mut() {
                *list = inject_parameters_in_list(list, arg, tmp);
            }
        }

        fn max_args_used(&mut self) -> i32 {
            let mut tmp = -1;
            for list in self.lists.iter() {
                tmp = max(tmp, max_arg_in_list(list));
            }
            tmp
        }
    }
    impl<'a> XLex<'a> {
        pub fn from_str(list_strings: &[String], operator: &str, set: &'a XVariableSet) -> Self {
            let operator = Operator::get_operator_by_str(operator);
            let mut lists = Vec::with_capacity(list_strings.len());
            for list in list_strings {
                lists.push(list_to_vec_var_val(list));
            }

            if lists.len() < 2 {
                panic!("lex requires at least two lists, ");
            }

            Self::new(lists, operator, set)
        }

        pub fn new(lists: Vec<Vec<XVarVal>>, operator: Operator, set: &'a XVariableSet) -> Self {
            Self {
                lists,
                operator,
                set,
            }
        }

        pub fn lists(&self) -> &Vec<Vec<XVarVal>> {
            &self.lists
        }

        pub fn operator(&self) -> &Operator {
            &self.operator
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
