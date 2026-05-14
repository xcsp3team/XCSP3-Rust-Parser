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
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XClause<'a> {
        positive_literals: Vec<XVarVal>,
        negative_literals: Vec<XVarVal>,
        set: &'a XVariableSet,
    }

    impl<'a> XClause<'a> {
        pub fn from_str(value: &str, set: &'a XVariableSet) -> Self {
            let mut positive_ones: Vec<String> = Vec::new();
            let mut negative_ones: Vec<String> = Vec::new();

            for token in value.split_whitespace() {
                if let Some(inner) = token.strip_prefix("not(").and_then(|s| s.strip_suffix(")")) {
                    negative_ones.push(inner.to_string());
                } else {
                    positive_ones.push(token.to_string());
                }
            }
            let pos = list_to_vec_var_val(&*positive_ones.join(" "));
            let neg = list_to_vec_var_val(&*negative_ones.join(" "));
            Self::new(pos, neg, set)
        }

        pub fn new(positive_literals: Vec<XVarVal>, negative_literals: Vec<XVarVal>, set: &'a XVariableSet) -> Self {
            Self { positive_literals, negative_literals, set }
        }

        pub fn positive_literals(&self) -> &Vec<XVarVal> {
            &self.positive_literals
        }

        pub fn negative_literals(&self) -> &Vec<XVarVal> {
            &self.negative_literals
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
