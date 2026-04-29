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
* <p>@date:  2023/7/15 15:13
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        inject_parameters_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{list_to_transitions, list_to_vec_var_val};
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XRegular<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        start: String,
        r#final: Vec<String>,
        transitions: Vec<(String, i32, String)>,
    }

    impl XConstraintUnfold for XRegular<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            self.scope = inject_parameters_in_list(&*self.scope, arg, -1);
        }

        fn max_args_used(&mut self) -> i32 {
            -1
        }
    }

    impl<'a> XRegular<'a> {
        pub fn from_str(
            list: &str,
            transitions_str: &str,
            start_str: &str,
            final_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => {
                    let mut finals: Vec<String> = vec![];
                    let t_final: Vec<&str> = final_str.split_whitespace().collect();
                    for s in t_final.iter() {
                        finals.push(s.to_string());
                    }
                    match list_to_transitions(transitions_str) {
                        Ok(transitions) => Ok(XRegular::new(
                            scope_vec_str,
                            set,
                            start_str.to_string(),
                            finals,
                            transitions,
                        )),
                        Err(e) => Err(e),
                    }
                }
                Err(e) => Err(e),
            }
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            start: String,
            r#final: Vec<String>,
            transitions: Vec<(String, i32, String)>,
        ) -> Self {
            XRegular {
                scope,
                set,
                start,
                r#final,
                transitions,
            }
        }

        pub fn start(&self) -> &str {
            &self.start
        }

        pub fn finals(&self) -> &Vec<String> {
            &self.r#final
        }

        pub fn transitions(&self) -> &Vec<(String, i32, String)> {
            &self.transitions
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
