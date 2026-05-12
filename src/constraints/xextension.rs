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
    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, tuple_to_vector};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XExtension<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        ///if the  value in tuples is i32::MAX, then it is the star
        tuples: Vec<Vec<i32>>,
        is_support: bool,
        has_star: bool,
    }

    impl XConstraintUnfold for XExtension<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            self.scope = inject_parameters_in_list(&self.scope, arg, -1);
        }

        fn max_args_used(&self) -> i32 {
            max_arg_in_list(&*self.scope)
        }
    }

    impl<'a> XExtension<'a> {
        pub fn from_str(list: &str, tuple: &str, is_support: bool, set: &'a XVariableSet) -> Self {
            let scope_vec_str = list_to_vec_var_val(list);
            let tuples = tuple_to_vector(tuple, !tuple.contains('('));
            let has_star = tuples.iter().flatten().any(|&x| x == i32::MAX);
            XExtension::new(scope_vec_str, set, tuples, is_support, has_star)
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            tuples: Vec<Vec<i32>>,
            is_support: bool,
            has_star: bool,
        ) -> Self {
            XExtension {
                scope,
                set,
                tuples,
                is_support,
                has_star,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn tuples(&self) -> &Vec<Vec<i32>> {
            &self.tuples
        }

        pub fn is_support(&self) -> bool {
            self.is_support
        }

        pub fn has_star(&self) -> bool {
            self.has_star
        }
    }
}
