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
        inject_parameters_in_list, inject_parameters_in_var_val, max_arg_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XChannel<'a> {
        list1: Vec<XVarVal>,
        start_index1: i32,
        list2: Vec<XVarVal>,
        start_index2: i32,
        value: Option<XVarVal>,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XChannel<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.list1 = inject_parameters_in_list(&*self.list1, arg, tmp);
            self.list2 = inject_parameters_in_list(&*self.list2, arg, tmp);
            if let Some(value) = &mut self.value {
                self.value = Option::from(inject_parameters_in_var_val(value.clone(), arg));
            }
        }
        fn max_args_used(&mut self) -> i32 {
            let mut tmp = max(max_arg_in_list(&*self.list1), max_arg_in_list(&*self.list2));
            if let Some(value) = &mut self.value {
                let mut s = Vec::new();
                s.push(value.clone());
                tmp = max(max_arg_in_list(&*s), tmp);
            }
            tmp
        }
    }
    impl<'a> XChannel<'a> {
        pub fn from_str(
            list1: &str,
            start_index_str1: &str,
            list2: &str,
            start_index_str2: &str,
            value_str: &str,
            set: &'a XVariableSet,
        ) -> Self {
            let sc1 = list_to_vec_var_val(list1);
            let sc2 = list_to_vec_var_val(list2);
            let start1 = start_index_str1.parse().unwrap();
            let start2 = start_index_str2.parse().unwrap();
            let v = XVarVal::from_string(value_str);
            Self::new(sc1, start1, sc2, start2, v, set)
        }

        pub fn new(
            list1: Vec<XVarVal>,
            start_index1: i32,
            list2: Vec<XVarVal>,
            start_index2: i32,
            value: Option<XVarVal>,
            set: &'a XVariableSet,
        ) -> Self {
            XChannel {
                list1,
                start_index1,
                list2,
                start_index2,
                value,
                set,
            }
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn start_index1(&self) -> i32 {
            self.start_index1
        }
        pub fn start_index2(&self) -> i32 {
            self.start_index2
        }

        pub fn list1(&self) -> &Vec<XVarVal> {
            &self.list1
        }

        pub fn list2(&self) -> &Vec<XVarVal> {
            &self.list2
        }

        pub fn value(&self) -> &Option<XVarVal> {
            &self.value
        }
    }
}
