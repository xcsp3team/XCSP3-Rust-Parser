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
 * <p>@date:  2023/7/19 12:20
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::fmt::{Display, Formatter};

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XGroup<'a> {
        args: Vec<Vec<XVarVal>>,
        map: Vec<Vec<(String, &'a XDomainInteger)>>,
        set: &'a XVariableSet,
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
        ) -> Result<Self, Xcsp3Error> {
            // let tt = TimeInterval::new();
            let mut args: Vec<Vec<XVarVal>> = vec![];
            args.reserve(arg_str.len());
            for a in arg_str.iter() {
                // args.push(list_to_vec_var(a));
                match list_to_vec_var_val(a) {
                    Ok(scope_vec_str) => {
                        args.push(scope_vec_str);
                    }
                    Err(e) => return Err(e),
                }
            }
            // println!("this group cost {:?}", tt.get());
            Ok(XGroup::new(args, set, Box::new(cc)))
        }

        pub fn new(
            args: Vec<Vec<XVarVal>>,
            set: &'a XVariableSet,
            template: Box<XConstraintType<'a>>,
        ) -> Self {
            Self {
                args,
                map: vec![],
                set,
                template,
            }
        }
    }
}
