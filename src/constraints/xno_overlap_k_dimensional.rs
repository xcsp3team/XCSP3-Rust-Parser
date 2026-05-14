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
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;

    use crate::utils::utils_functions::xcsp3_utils::{list_to_vec_var_val, to_bool_option};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub struct XNoOverlapKDim<'a> {
        scope: Vec<Vec<XVarVal>>,
        lengths: Vec<Vec<XVarVal>>,
        set: &'a XVariableSet,
        zero_ignored: Option<bool>,
    }

    impl<'a> XNoOverlapKDim<'a> {
        pub fn from_str(list: &str, lengths_str: &str, zero_ignored_str: &str, set: &'a XVariableSet) -> Self {
            let scope: Vec<Vec<XVarVal>> = {
                let mut sc = vec![];
                let binding = list.replace(")(", "@").replace(['(', ',', ')'], " ");
                let spilt: Vec<&str> = binding.split("@").collect();
                for e in spilt.iter() {
                    let tmp = list_to_vec_var_val(e);
                    sc.push(tmp);
                }
                sc
            };
            let lengths = {
                let mut le = vec![];
                let binding = lengths_str.replace(")(", "@").replace(['(', ',', ')'], " ");
                let spilt: Vec<&str> = binding.split("@").collect();
                for e in spilt.iter() {
                    let tmp = list_to_vec_var_val(e);
                    le.push(tmp);
                }
                le
            };
            let zero_ignored = to_bool_option(zero_ignored_str);
            Self::new(scope, lengths, set, zero_ignored)
        }
        pub fn new(
            scope: Vec<Vec<XVarVal>>,
            lengths: Vec<Vec<XVarVal>>,
            set: &'a XVariableSet,
            zero_ignored: Option<bool>,
        ) -> Self {
            Self { scope, lengths, set, zero_ignored }
        }

        pub fn scope(&self) -> &Vec<Vec<XVarVal>> {
            &self.scope
        }

        pub fn lengths(&self) -> &Vec<Vec<XVarVal>> {
            &self.lengths
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn zero_ignored(&self) -> bool {
            self.zero_ignored.unwrap_or(true)
        }
        pub fn first_length_is_var_val(&self) -> bool {
            matches!(
            self.lengths.first(),
            Some(first)
                if first.len() == 2
                    && matches!(first[0], XVarVal::IntVar(_))
                    && matches!(first[1], XVarVal::IntVal(_))
            )
        }
        pub fn is_lengths_int(&self) -> bool {
            matches!(self.lengths.first().and_then(|first| first.first()), Some(XVarVal::IntVal(_)))
        }
    }
    impl Display for XNoOverlapKDim<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for vc in self.scope.iter() {
                ret.push('(');
                for (j, e) in vc.iter().enumerate() {
                    ret.push_str(&e.to_string());
                    if j != vc.len() - 1 {
                        ret.push_str(", ");
                    }
                }
                ret.push_str(")");
            }

            ret.push_str("  lengths = ");
            for vc in self.lengths.iter() {
                ret.push('(');
                for (j, e) in vc.iter().enumerate() {
                    ret.push_str(&e.to_string());
                    if j != vc.len() - 1 {
                        ret.push_str(", ");
                    }
                }
                ret.push_str(")");
            }

            if let Some(n) = &self.zero_ignored {
                ret.push_str(&format!(" zeroIgnored = {}, ", n))
            }
            write!(f, "XNoOverlap: origins =  {}, ", ret,)
        }
    }
}
