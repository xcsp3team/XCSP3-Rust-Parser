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
    use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintUnfold;
    use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XIntention<'a> {
        expression: String,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XIntention<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            for index in 0..arg.len() {
                self.expression = self
                    .expression
                    .replace(&format!("%{}", index), arg[index].to_string().as_str());
            }
        }
        fn max_args_used(&mut self) -> i32 {
            -1
        }
    }
    impl<'a> XIntention<'a> {
        pub fn create(expression: &str, set: &'a XVariableSet) -> Result<Self, Xcsp3Error> {
            Ok(Self::new(expression.to_string(), set))
        }
        pub fn to_tree(&self) -> ExpressionTree {
            match ExpressionTree::from_string(&*self.expression) {
                Ok(tree) => tree,
                Err(e) => panic!("{:?}", e),
            }
        }
        pub fn new(expression: String, set: &'a XVariableSet) -> Self {
            Self { expression, set }
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
