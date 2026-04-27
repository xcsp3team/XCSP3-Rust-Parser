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
* <p>@date:  2023/7/15 14:56
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XOrdered<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        lengths: Option<Vec<XVarVal>>,
        operator: Operator,
    }

    impl<'a> XOrdered<'a> {
        pub fn from_str(
            list: &str,
            lengths_str: &str,
            operator: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => match list_to_vec_var_val(lengths_str) {
                    Ok(length_vec_str) => match Operator::get_operator_by_str(operator) {
                        None => Err(Xcsp3Error::get_constraint_list_of_values_error(
                            "parse the list of values error. ",
                        )),
                        Some(ope) => {
                            Ok(XOrdered::new(scope_vec_str, set, Some(length_vec_str), ope))
                        }
                    },
                    Err(e) => Err(e),
                },
                Err(e) => Err(e),
            }
        }

        pub fn from_str_without_lengths(
            list: &str,
            operator: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => match Operator::get_operator_by_str(operator) {
                    None => Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse the list of values error. ",
                    )),
                    Some(ope) => Ok(XOrdered::new(scope_vec_str, set, None, ope)),
                },
                Err(e) => Err(e),
            }
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
