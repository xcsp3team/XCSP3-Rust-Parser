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
* <p>@date:  2023/7/14 22:50
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        inject_parameters_in_list, max_arg_in_list, XConstraintUnfold,
    };
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XAllDifferentList<'a> {
        lists: Vec<Vec<XVarVal>>,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XAllDifferentList<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            for list in self.lists.iter_mut() {
                *list = inject_parameters_in_list(&*list, arg, tmp);
            }
        }
        fn max_args_used(&mut self) -> i32 {
            let mut tmp = -1;
            for list in self.lists.iter() {
                tmp = max(tmp, max_arg_in_list(list));
            }
            tmp
        }
    }

    impl<'a> XAllDifferentList<'a> {
        //pub fn from_str_vec(scope_vec_str: Vec<XVarVal>, set: &'a XVariableSet) -> Self {
        //    XAllDifferentList::new(scope_vec_str, set)
        // }

        pub fn from_str(lists: &[String], set: &'a XVariableSet) -> Result<Self, Xcsp3Error> {
            let mut tmp = Vec::new();
            for list in lists {
                match list_to_vec_var_val(list) {
                    Ok(scope_vec_str) => tmp.push(scope_vec_str),
                    Err(e) => return Err(e),
                }
            }
            Ok(XAllDifferentList::new(tmp, set))
        }
        pub fn new(lists: Vec<Vec<XVarVal>>, set: &'a XVariableSet) -> Self {
            XAllDifferentList { lists, set }
        }

        pub fn lists(&self) -> &Vec<Vec<XVarVal>> {
            &self.lists
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
