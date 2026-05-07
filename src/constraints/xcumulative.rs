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
    use crate::utils::utils_functions::xcsp3_utils::{
        list_to_vec_var_val, str_to_condition, to_i32_option,
    };
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XCumulative<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        lengths: Vec<XVarVal>,
        heights: Vec<XVarVal>,
        ends: Option<Vec<XVarVal>>,
        machines: Option<Vec<XVarVal>>,
        operator: Operator,
        operand: Operand,
        star_index: Option<i32>,
    }

    impl XConstraintUnfold for XCumulative<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            self.lengths = inject_parameters_in_list(&self.lengths, arg, tmp);
            self.heights = inject_parameters_in_list(&self.heights, arg, tmp);
            if let Some(vals) = &mut self.ends {
                *vals = inject_parameters_in_list(vals, arg, tmp);
            }
            self.operand = inject_parameters_in_operand(&self.operand, arg)
        }

        fn max_args_used(&mut self) -> i32 {
            let mut tmp = max(arg_in_operand(&self.operand), max_arg_in_list(&*self.scope));
            tmp = max(tmp, max_arg_in_list(&*self.heights));
            tmp = max(tmp, max_arg_in_list(&*self.heights));
            match self.ends.as_deref() {
                Some(v) => max(tmp, max_arg_in_list(v)),
                None => tmp,
            }
        }
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
        ) -> Self {
            let scope = list_to_vec_var_val(origins_str);
            let lengths = list_to_vec_var_val(lengths_str);
            let heights = list_to_vec_var_val(heights_str);
            let binding = condition_str.replace(['(', ')', ','], " ");
            let (ope, rand) = str_to_condition(condition_str);
            let ends = if ends_str.is_empty() {
                None
            } else {
                Option::from(list_to_vec_var_val(ends_str))
            };

            let machines = if machines_str.is_empty() {
                None
            } else {
                Option::from(list_to_vec_var_val(machines_str))
            };
            let start_index = to_i32_option(start_index_str);
            Self::new(
                scope,
                set,
                lengths,
                heights,
                ends,
                machines,
                ope,
                rand,
                start_index,
            )
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

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
