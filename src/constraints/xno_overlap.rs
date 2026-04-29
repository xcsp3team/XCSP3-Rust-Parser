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
 * <p>@date:  2023/7/31 12:42
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */
pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        inject_parameters_in_list, inject_parameters_in_operand, XConstraintUnfold,
    };
    use crate::constraints::xn_values::xcsp3_core::XNValues;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XNoOverlap<'a> {
        scope: Vec<XVarVal>,
        lengths: Vec<XVarVal>,
        set: &'a XVariableSet,
        zero_ignored: Option<bool>,
    }

    impl XConstraintUnfold for XNoOverlap<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            self.scope = inject_parameters_in_list(&self.scope, arg);
            self.lengths = inject_parameters_in_list(&self.lengths, arg);
        }
    }
    impl<'a> XNoOverlap<'a> {
        pub fn from_str(
            list: &str,
            lengths_str: &str,
            zero_ignored_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = match list_to_vec_var_val(list) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let lengths = match list_to_vec_var_val(lengths_str) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let zero_ignored = if !zero_ignored_str.is_empty() {
                match zero_ignored_str.parse::<bool>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_no_overlap_error(
                            "parse XNoOverlap constraint zero_ignored error, ",
                        ));
                    }
                }
            } else {
                None
            };
            Ok(Self::new(scope, lengths, set, zero_ignored))
        }
        pub fn new(
            scope: Vec<XVarVal>,
            lengths: Vec<XVarVal>,
            set: &'a XVariableSet,
            zero_ignored: Option<bool>,
        ) -> Self {
            Self {
                scope,
                lengths,
                set,
                zero_ignored,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn lengths(&self) -> &Vec<XVarVal> {
            &self.lengths
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn zero_ignored(&self) -> bool {
            self.zero_ignored.unwrap_or(true)
        }
    }
}
