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
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XCardinality<'a> {
        scope: Vec<XVarVal>,
        values: Vec<XVarVal>,
        occurs: Vec<XVarVal>,
        set: &'a XVariableSet,
        closed: Option<bool>,
    }

    impl XConstraintUnfold for XCardinality<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&*self.scope, arg, tmp);
            self.values = inject_parameters_in_list(&*self.values, arg, tmp);
            self.occurs = inject_parameters_in_list(&*self.occurs, arg, tmp);
        }

        fn max_args_used(&mut self) -> i32 {
            max(
                max(
                    max_arg_in_list(&*self.values),
                    max_arg_in_list(&*self.scope),
                ),
                max_arg_in_list(&*self.occurs),
            )
        }
    }

    impl<'a> XCardinality<'a> {
        pub fn from_str(
            list: &str,
            values_str: &str,
            occurs_str: &str,
            closed_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = match list_to_vec_var_val(list) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let value = match list_to_vec_var_val(values_str) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let occurs = match list_to_vec_var_val(occurs_str) {
                Ok(s) => s,
                Err(e) => return Err(e),
            };
            let closed = if !closed_str.is_empty() {
                match closed_str.parse::<bool>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_cardinality_error(
                            "parse cardinality  constraint closed error, ",
                        ));
                    }
                }
            } else {
                None
            };
            Ok(Self::new(scope, value, occurs, set, closed))
        }

        pub fn new(
            scope: Vec<XVarVal>,
            values: Vec<XVarVal>,
            occurs: Vec<XVarVal>,
            set: &'a XVariableSet,
            closed: Option<bool>,
        ) -> Self {
            Self {
                scope,
                values,
                occurs,
                set,
                closed,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn values(&self) -> &Vec<XVarVal> {
            &self.values
        }

        pub fn occurs(&self) -> &Vec<XVarVal> {
            &self.occurs
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn closed(&self) -> bool {
            self.closed.unwrap_or(true)
        }
    }
}
