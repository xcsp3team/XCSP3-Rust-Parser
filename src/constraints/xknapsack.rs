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
    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, str_to_condition};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XKnapsack<'a> {
        scope: Vec<XVarVal>,
        weights: Vec<XVarVal>,
        profits: Vec<XVarVal>,
        weight_operator: Operator,
        weight_operand: Operand,
        profit_operator: Operator,
        profit_operand: Operand,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XKnapsack<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            self.weights = inject_parameters_in_list(&self.weights, arg, tmp);
            self.profits = inject_parameters_in_list(&self.profits, arg, tmp);
            self.profit_operand = inject_parameters_in_operand(&self.profit_operand, arg);
            self.weight_operand = inject_parameters_in_operand(&self.weight_operand, arg);
        }

        fn max_args_used(&mut self) -> i32 {
            let mut tmp = max(
                arg_in_operand(&self.profit_operand),
                arg_in_operand(&self.weight_operand),
            );
            tmp = max(tmp, max_arg_in_list(&*self.scope));

            tmp = max(tmp, max_arg_in_list(&*self.profits));
            max(tmp, max_arg_in_list(&*self.weights))
        }
    }
    impl<'a> XKnapsack<'a> {
        pub fn from_str(
            list: &str,
            weights: &str,
            profits: &str,
            conditions: &Box<[String]>,
            set: &'a XVariableSet,
        ) -> Self {
            let scope = list_to_vec_var_val(list);
            let weights = list_to_vec_var_val(weights);
            let profits = list_to_vec_var_val(profits);
            let (weight_operator, weight_operand) = str_to_condition(&*conditions[0]);
            let (profit_operator, profit_operand) = str_to_condition(&*conditions[1]);
            Self::new(
                scope,
                weights,
                profits,
                weight_operator,
                weight_operand,
                profit_operator,
                profit_operand,
                set,
            )
        }

        #[allow(clippy::too_many_arguments)]
        pub fn new(
            scope: Vec<XVarVal>,
            weights: Vec<XVarVal>,
            profits: Vec<XVarVal>,
            weight_operator: Operator,
            weight_operand: Operand,
            profit_operator: Operator,
            profit_operand: Operand,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                scope,
                weights,
                profits,
                weight_operator,
                weight_operand,
                profit_operator,
                profit_operand,
                set,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn weights(&self) -> &Vec<XVarVal> {
            &self.weights
        }

        pub fn profits(&self) -> &Vec<XVarVal> {
            &self.profits
        }

        pub fn weight_operator(&self) -> Operator {
            self.weight_operator
        }

        pub fn weight_operand(&self) -> &Operand {
            &self.weight_operand
        }

        pub fn profit_operator(&self) -> Operator {
            self.profit_operator
        }

        pub fn profit_operand(&self) -> &Operand {
            &self.profit_operand
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
