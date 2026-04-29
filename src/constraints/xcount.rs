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
 * <p>@date:  2023/7/29 16:47
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
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub struct XCount<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        operator: Operator,
        operand: Operand,
        values: Vec<XVarVal>,
    }

    impl XConstraintUnfold for XCount<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            self.scope = inject_parameters_in_list(&self.scope, arg);
            self.values = inject_parameters_in_list(&self.values, arg);
            self.operand = inject_parameters_in_operand(&self.operand, arg)
        }
    }

    impl<'a> XCount<'a> {
        pub fn from_str(
            list: &str,
            condition: &str,
            value_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match list_to_vec_var_val(list) {
                Ok(scope_vec_str) => {
                    let values = match list_to_vec_var_val(value_str) {
                        Ok(coe_vec) => coe_vec,
                        Err(e) => return Err(e),
                    };

                    let condition = condition.replace(['(', ')', ','], " ");
                    let spilt: Vec<&str> = condition.split_whitespace().collect();

                    let ope: Operator = match Operator::get_operator_by_str(spilt[0]) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse count constraint Operator error, ",
                            ));
                        }
                        Some(o) => o,
                    };

                    let rand: Operand = match Operand::get_operand_by_str(&spilt[1..], &ope) {
                        None => {
                            return Err(Xcsp3Error::get_constraint_sum_error(
                                "parse count constraint Operand error, ",
                            ));
                        }
                        Some(r) => r,
                    };
                    Ok(Self::new(scope_vec_str, set, ope, rand, values))
                }
                Err(e) => Err(e),
            }
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            operator: Operator,
            operand: Operand,
            values: Vec<XVarVal>,
        ) -> Self {
            Self {
                scope,
                set,
                operator,
                operand,
                values,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn operator(&self) -> Operator {
            self.operator
        }

        pub fn operand(&self) -> &Operand {
            &self.operand
        }

        pub fn values(&self) -> &Vec<XVarVal> {
            &self.values
        }
    }
}
