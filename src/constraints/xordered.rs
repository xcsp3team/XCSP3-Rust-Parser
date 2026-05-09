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

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XOrdered<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        lengths: Option<Vec<XVarVal>>,
        operator: Operator,
    }

    impl XConstraintUnfold for XOrdered<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            println!("tmp: {}", tmp);
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            if let Some(vals) = &mut self.lengths {
                *vals = inject_parameters_in_list(vals, arg, tmp);
            }
        }
        fn max_args_used(&mut self) -> i32 {
            let tmp = max_arg_in_list(&*self.scope);
            match self.lengths.as_deref() {
                Some(v) => max(tmp, max_arg_in_list(v)),
                None => tmp,
            }
        }
    }
    impl<'a> XOrdered<'a> {
        pub fn from_str(
            list: &str,
            lengths_str: &str,
            operator: &str,
            set: &'a XVariableSet,
        ) -> Self {
            let scope = list_to_vec_var_val(list);
            let length_vec_str = list_to_vec_var_val(lengths_str);
            let ope = Operator::get_operator_by_str(operator);
            XOrdered::new(scope, set, Some(length_vec_str), ope)
        }

        pub fn from_str_without_lengths(list: &str, operator: &str, set: &'a XVariableSet) -> Self {
            let scope = list_to_vec_var_val(list);
            let ope = Operator::get_operator_by_str(operator);
            XOrdered::new(scope, set, None, ope)
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            lengths: Option<Vec<XVarVal>>,
            operator: Operator,
        ) -> Self {
            XOrdered {
                scope,
                set,
                lengths,
                operator,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn lengths(&self) -> &Option<Vec<XVarVal>> {
            &self.lengths
        }

        pub fn operator(&self) -> &Operator {
            &self.operator
        }
    }
}
