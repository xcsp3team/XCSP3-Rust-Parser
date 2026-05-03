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
* <p>@date:  2023/7/23 00:59
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
        inject_parameters_in_list, inject_parameters_in_var_val, max_arg_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XElement<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        value: XVarVal,
        index: Option<XVarVal>,
        start_index: Option<i32>,
    }

    impl XConstraintUnfold for XElement<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            self.value = inject_parameters_in_var_val(self.value.clone(), arg);
        }
        fn max_args_used(&mut self) -> i32 {
            let mut s = Vec::new();
            s.push(self.value.clone());
            max(max_arg_in_list(&*s), max_arg_in_list(&*self.scope))
        }
    }

    impl<'a> XElement<'a> {
        pub fn from_str(
            list: &str,
            value_str: &str,
            index_str: &str,
            start_index_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            // println!("{start_index_str}");
            match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => {
                    let value = match XVarVal::from_string(value_str) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse element constraint value error, ",
                            ));
                        }
                        Some(v) => v,
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

                    Ok(XElement::new(scope_vec_str, set, value, index, start_index))
                }

                Err(e) => Err(e),
            }
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            value: XVarVal,
            index: Option<XVarVal>,
            start_index: Option<i32>,
        ) -> Self {
            Self {
                scope,
                set,
                value,
                index,
                start_index,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn value(&self) -> &XVarVal {
            &self.value
        }

        pub fn index(&self) -> &Option<XVarVal> {
            &self.index
        }

        pub fn start_index(&self) -> Option<i32> {
            self.start_index
        }
    }
}
