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
 * <p>@date:  2023/7/19 16:18
 * </p>
 * <p>@email: zhenlh20@mails.jlu.edu.cn
 * </p>
 * <p>@version: 1.0
 * </p>
 * <p>@description:
 * </p>
 */

pub mod xcsp3_core {
    use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    // #[derive(Clone)]
    #[derive(Clone)]
    pub struct XIntention<'a> {
        scope: Vec<XVarVal>,
        set: &'a XVariableSet,
        tree: ExpressionTree,
    }

    impl<'a> XIntention<'a> {
        pub fn get_expression(&self) -> &ExpressionTree {
            &self.tree
        }

        pub fn from_str_without_scope(
            expression: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            match ExpressionTree::from_string(expression) {
                Ok(tree) => {
                    let scope: Vec<XVarVal> = tree.get(set);
                    Ok(Self::new(scope, set, tree))
                }
                Err(e) => Err(e),
            }
        }

        pub fn new(scope: Vec<XVarVal>, set: &'a XVariableSet, tree: ExpressionTree) -> Self {
            Self { scope, set, tree }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn tree(&self) -> &ExpressionTree {
            &self.tree
        }
    }
}
