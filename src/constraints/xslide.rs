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
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;

    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, to_bool_option, to_i32_option};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XSlide<'a> {
        args: Vec<XVarVal>,
        template: Box<XConstraintType<'a>>,
        circular: Option<bool>,
        offset: Option<i32>,
        collect: Option<i32>,
        set: &'a XVariableSet,
    }

    impl<'a> XSlide<'a> {
        pub fn from_str(
            cc: XConstraintType<'a>,
            arg_str: &str,
            offset_str: &str,
            circular_str: &str,
            collect_str: &str,
            set: &'a XVariableSet,
        ) -> Self {
            let scope_vec_str = list_to_vec_var_val(arg_str)
                .iter()
                .flat_map(|e| match e {
                    XVarVal::IntVar(st) if st.contains('(') => vec![e.clone()],
                    XVarVal::IntVar(st) => {
                        set.construct_scope(&[st]).iter().map(|(var, _)| XVarVal::IntVar(var.clone())).collect()
                    }
                    _ => vec![e.clone()],
                })
                .collect();
            let offset = to_i32_option(offset_str);
            let collect = to_i32_option(collect_str);
            let circular = to_bool_option(circular_str);
            Self::new(scope_vec_str, Box::new(cc), circular, offset, collect, set)
        }

        pub fn new(
            args: Vec<XVarVal>,
            template: Box<XConstraintType<'a>>,
            circular: Option<bool>,
            offset: Option<i32>,
            collect: Option<i32>,
            set: &'a XVariableSet,
        ) -> Self {
            XSlide { args, template, circular, offset, collect, set }
        }

        pub fn args(&self) -> &Vec<XVarVal> {
            &self.args
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn template(&self) -> &Box<XConstraintType<'a>> {
            &self.template
        }

        pub fn circular(&self) -> bool {
            self.circular.unwrap_or(false)
        }

        pub fn offset(&self) -> i32 {
            self.offset.unwrap_or(1)
        }
        pub fn collect(&self) -> i32 {
            self.collect.unwrap_or(1)
        }
    }
}
