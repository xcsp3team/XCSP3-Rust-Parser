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
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XGroup<'a> {
        args: Vec<Vec<XVarVal>>,
        template: Box<XConstraintType<'a>>,
    }

    impl<'a> XGroup<'a> {
        pub fn get_args(&self) -> &Vec<Vec<XVarVal>> {
            &self.args
        }

        pub fn get_template(&self) -> &XConstraintType<'a> {
            &self.template
        }

        pub fn from_str(
            cc: XConstraintType<'a>,
            arg_str: &[String],
            set: &'a XVariableSet,
        ) -> Self {
            let mut args: Vec<Vec<XVarVal>> = vec![];
            args.reserve(arg_str.len());
            for a in arg_str.iter() {
                let arg: Vec<XVarVal> = list_to_vec_var_val(a)
                    .iter()
                    .flat_map(|e| match e {
                        XVarVal::IntVar(st) if st.contains('(') => vec![e.clone()],
                        XVarVal::IntVar(st) if st.contains('{') => vec![e.clone()],
                        XVarVal::IntVar(st) => set
                            .construct_scope(&[st])
                            .iter()
                            .map(|(var, _)| XVarVal::IntVar(var.clone()))
                            .collect(),
                        _ => vec![e.clone()],
                    })
                    .collect();
                args.push(arg);
            }
            XGroup::new(args, Box::new(cc))
        }

        pub fn new(args: Vec<Vec<XVarVal>>, template: Box<XConstraintType<'a>>) -> Self {
            Self { args, template }
        }
    }
}
