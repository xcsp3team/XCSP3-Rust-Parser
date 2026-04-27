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
    pub struct XCumulative<'a> {
        scope: Vec<XVarVal>,
        map: HashMap<String, &'a XDomainInteger>,
        set: &'a XVariableSet,
        lengths: Vec<XVarVal>,
        heights: Vec<XVarVal>,
        ends: Option<Vec<XVarVal>>,
        machines: Option<Vec<XVarVal>>,
        // condition
        operator: Operator,
        operand: Operand,
        star_index: Option<i32>,
    }

    impl<'a> XCumulative<'a> {
        pub fn from_str(
            origins_str: &str,
            lengths_str: &str,
            heights_str: &str,
            condition_str: &str,
            ends_str: &str,
            machines_str: &str,
            start_index_str: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = match list_to_vec_var_val(origins_str) {
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
            let heights = match list_to_vec_var_val(heights_str) {
                Ok(n) => n,
                Err(e) => {
                    return Err(e);
                }
            };
            let binding = condition_str.replace(['(', ')', ','], " ");
            let spilt: Vec<&str> = binding.split_whitespace().collect();
            let ope: Operator = match Operator::get_operator_by_str(spilt[0]) {
                None => {
                    return Err(Xcsp3Error::get_constraint_cumulative_error(
                        "parse cumulative constraint error, ",
                    ))
                }
                Some(o) => o,
            };
            let rand: Operand = match Operand::get_operand_by_str(&spilt[1..], &ope) {
                None => {
                    return Err(Xcsp3Error::get_constraint_cumulative_error(
                        "parse cumulative constraint error, ",
                    ))
                }
                Some(r) => r,
            };
            let ends = if ends_str.is_empty() {
                None
            } else {
                match list_to_vec_var_val(ends_str) {
                    Ok(n) => Some(n),
                    Err(e) => {
                        return Err(e);
                    }
                }
            };

            let machines = if machines_str.is_empty() {
                None
            } else {
                match list_to_vec_var_val(machines_str) {
                    Ok(n) => Some(n),
                    Err(e) => {
                        return Err(e);
                    }
                }
            };
            let start_index = if !start_index_str.is_empty() {
                match start_index_str.parse::<i32>() {
                    Ok(n) => Some(n),
                    Err(_) => {
                        return Err(Xcsp3Error::get_constraint_cumulative_error(
                            "parse cumulative constraint start_index error, ",
                        ));
                    }
                }
            } else {
                None
            };

            Ok(Self::new(
                scope,
                set,
                lengths,
                heights,
                ends,
                machines,
                ope,
                rand,
                start_index,
            ))
        }

        pub fn new(
            scope: Vec<XVarVal>,
            set: &'a XVariableSet,
            lengths: Vec<XVarVal>,
            heights: Vec<XVarVal>,
            ends: Option<Vec<XVarVal>>,
            machines: Option<Vec<XVarVal>>,
            operator: Operator,
            operand: Operand,
            star_index: Option<i32>,
        ) -> Self {
            Self {
                scope,
                map: Default::default(),
                set,
                lengths,
                heights,
                ends,
                machines,
                operator,
                operand,
                star_index,
            }
        }

        pub fn lengths(&self) -> &Vec<XVarVal> {
            &self.lengths
        }
        pub fn heights(&self) -> &Vec<XVarVal> {
            &self.heights
        }
        pub fn machines(&self) -> &Option<Vec<XVarVal>> {
            &self.machines
        }
        pub fn ends(&self) -> &Option<Vec<XVarVal>> {
            &self.ends
        }
        pub fn operator(&self) -> &Operator {
            &self.operator
        }
        pub fn operand(&self) -> &Operand {
            &self.operand
        }
        pub fn star_index(&self) -> Option<i32> {
            self.star_index
        }
    }
}
