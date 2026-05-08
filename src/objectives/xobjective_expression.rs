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
    use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;

    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    #[derive(Clone)]
    pub struct XObjectiveExpression<'a> {
        expression: ExpressionTree,
        scope: Vec<XVarVal>,
        is_maximize: bool,
        set: &'a XVariableSet,
    }

    impl<'a> XObjectiveExpression<'a> {
        pub fn get_scope_string(&self) -> &Vec<XVarVal> {
            &self.scope
        }
        pub fn get_expression(&self) -> &ExpressionTree {
            &self.expression
        }

        pub fn from_expr(expr: &str, is_maximize: bool, set: &'a XVariableSet) -> Self {
            let tree = ExpressionTree::from_string(expr);
            let scope: Vec<XVarVal> = tree.get(set);
            Self::new(tree, scope, is_maximize, set)
        }

        pub fn new(
            expression: ExpressionTree,
            scope: Vec<XVarVal>,
            is_maximize: bool,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                expression,
                scope,
                is_maximize,
                set,
            }
        }

        pub fn expression(&self) -> &ExpressionTree {
            &self.expression
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn is_maximize(&self) -> bool {
            self.is_maximize
        }
    }
}
