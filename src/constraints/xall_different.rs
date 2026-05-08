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
        inject_parameters_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XAllDifferent<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XAllDifferent<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&*self.scope, arg, tmp);
        }
        fn max_args_used(&mut self) -> i32 {
            -1
        }
    }

    impl<'a> XAllDifferent<'a> {
        pub fn from_str_vec(scope_vec_str: Vec<XVarVal>, set: &'a XVariableSet) -> Self {
            XAllDifferent::new(scope_vec_str, set)
        }

        pub fn from_str(list: &str, set: &'a XVariableSet) -> Self {
            let scope = list_to_vec_var_val(list);
            XAllDifferent::new(scope, set)
        }
        pub fn new(scope: Vec<XVarVal>, set: &'a XVariableSet) -> Self {
            XAllDifferent { scope, set }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
