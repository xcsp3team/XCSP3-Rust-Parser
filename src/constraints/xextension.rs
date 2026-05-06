/*=============================================================================
* parser for CSP instances represented in XCSP3 Format
*
* Copyright (c) 2023 xcsp.org (contact @ xcsp.org)
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

/*
* <p>@project_name: xcsp3-rust
* </p>
* <p>@author: luhan zhen
* </p>
* <p>@date:  2023/7/14 18:54
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        arg_in_operand, inject_parameters_in_list, inject_parameters_in_operand, max_arg_in_list,
        XConstraintUnfold,
    };
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, tuple_to_vector};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

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

        fn max_args_used(&mut self) -> i32 {
            -1
        }
    }

    impl<'a> XExtension<'a> {
        /// construct the constraint from two strings and a bool
        pub fn from_str(
            list: &str,
            tuple: &str,
            is_support: bool,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let a = match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => match tuple_to_vector(tuple, !tuple.contains('(')) {
                    Ok(tuples) => {
                        let mut has_star = false;
                        Ok(XExtension::new(
                            scope_vec_str,
                            set,
                            tuples,
                            is_support,
                            has_star,
                        ))
                    }
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            };
            a
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
