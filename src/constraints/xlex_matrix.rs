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
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_matrix_ids;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use crate::variables::xvariable_type::xcsp3_core::XVariableType::{
        XVariableArray, XVariableTree,
    };

    #[derive(Clone)]
    pub struct XLexMatrix<'a> {
        matrix: Vec<Vec<XVarVal>>,
        operator: Operator,
        set: &'a XVariableSet,
    }

    impl<'a> XLexMatrix<'a> {
        pub fn from_str(
            list: &str,
            operator: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let operator = match Operator::get_operator_by_str(operator) {
                Some(operator) => operator,
                None => {
                    return Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse lex operator error, ",
                    ));
                }
            };

            if list.contains("[][]") {
                let name = list.split('[').next().unwrap_or(list);
                let vartype = set.find_variable(name)?;
                let size = match vartype {
                    XVariableArray(v) => v.sizes[0],
                    XVariableTree(v) => v.sizes[0],
                    _ => 0,
                };
                let mut matrix: Vec<Vec<XVarVal>> = Vec::with_capacity(size);
                for i in 0..size {
                    matrix.push(vec![]);
                    for j in 0..size {
                        matrix[i].push(XVarVal::IntVar(format!("{}[{}][{}]", name, i, j)));
                    }
                }
                Ok(Self::new(matrix, operator, set))
            } else {
                let matrix: Vec<Vec<XVarVal>> = list_to_matrix_ids(list)
                    .iter()
                    .map(|line| line.iter().map(|e| XVarVal::IntVar(e.clone())).collect())
                    .collect();
                Ok(Self::new(matrix, operator, set))
            }
        }

        pub fn new(lists: Vec<Vec<XVarVal>>, operator: Operator, set: &'a XVariableSet) -> Self {
            Self {
                matrix: lists,
                operator,
                set,
            }
        }

        pub fn matrix(&self) -> &Vec<Vec<XVarVal>> {
            &self.matrix
        }

        pub fn operator(&self) -> &Operator {
            &self.operator
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
