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
        arg_in_operand, inject_parameters_in_list, inject_parameters_in_operand, max_arg_in_list,
        XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, str_to_condition};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XBinpacking<'a> {
        scope: Vec<XVarVal>,
        sizes: Vec<XVarVal>,
        operator: Option<Operator>,
        operand: Option<Operand>,
        limits: Vec<XVarVal>,
        loads: Vec<XVarVal>,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XBinpacking<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            self.sizes = inject_parameters_in_list(&self.sizes, arg, tmp);
            self.limits = inject_parameters_in_list(&self.limits, arg, tmp);
            self.loads = inject_parameters_in_list(&self.loads, arg, tmp);
            if let Some(o) = &mut self.operand {
                *o = inject_parameters_in_operand(o, arg);
            }
        }

        fn max_args_used(&mut self) -> i32 {
            let mut tmp = max(max_arg_in_list(&*self.sizes), max_arg_in_list(&*self.scope));
            tmp = max(tmp, max_arg_in_list(&*self.limits));
            tmp = max(tmp, max_arg_in_list(&*self.loads));
            match self.operand.clone() {
                Some(v) => max(tmp, arg_in_operand(&v)),
                None => tmp,
            }
        }
    }
    impl<'a> XBinpacking<'a> {
        pub fn from_str(
            list: &str,
            sizes: &str,
            condition: &str,
            limits: &str,
            loads: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = list_to_vec_var_val(list)?;
            let sizes = list_to_vec_var_val(sizes)?;
            let (operator, operand) = if condition.is_empty() {
                (None, None)
            } else {
                match str_to_condition(&condition) {
                    Ok((op, val)) => (Some(op), Some(val)),
                    Err(_) => panic!("condition in binpacking is wrong: {}", condition),
                }
            };
            let limits = list_to_vec_var_val(limits)?;
            let loads = list_to_vec_var_val(loads)?;

            if operator.is_none() && limits.len() == 0 && loads.len() == 0 {
                return Err(Xcsp3Error::get_constraint_sum_error(
                    "binPacking requires a condition, limits, or loads, ",
                ));
            }

            Ok(Self::new(
                scope, sizes, operator, operand, limits, loads, set,
            ))
        }

        pub fn new(
            scope: Vec<XVarVal>,
            sizes: Vec<XVarVal>,
            operator: Option<Operator>,
            operand: Option<Operand>,
            limits: Vec<XVarVal>,
            loads: Vec<XVarVal>,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                scope,
                sizes,
                operator,
                operand,
                limits,
                loads,
                set,
            }
        }

        pub fn sizes(&self) -> &Vec<XVarVal> {
            &self.sizes
        }

        pub fn operator(&self) -> &Option<Operator> {
            &self.operator
        }

        pub fn operand(&self) -> &Option<Operand> {
            &self.operand
        }

        pub fn limits(&self) -> &Vec<XVarVal> {
            &self.limits
        }

        pub fn loads(&self) -> &Vec<XVarVal> {
            &self.loads
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }
    }
}
