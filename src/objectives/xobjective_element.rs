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
    use std::fmt::{Display, Formatter};

    #[derive(Clone, Debug)]
    pub enum XElementOperator {
        Sum,
        Product,
        Minimum,
        Maximum,
        NValues,
        Lex,
    }

    impl XElementOperator {
        pub fn get_objectives_operator_by_str(op: &str) -> Self {
            match op {
                "sum" => Self::Sum,
                "product" => Self::Product,
                "minimum" => Self::Minimum,
                "maximum" => Self::Maximum,
                "nValues" => Self::NValues,
                "lex" => Self::Lex,
                _ => panic!("objective operator {} is not supported", op),
            }
        }
    }

    #[derive(Clone)]
    pub struct XObjectiveElement<'a> {
        operator: XElementOperator,
        scope: Vec<XVarVal>,
        coeffs: Vec<XVarVal>,
        set: &'a XVariableSet,
        is_maximize: bool,
    }

    impl<'a> XObjectiveElement<'a> {
        pub fn get_scope_string(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn new(
            operator: XElementOperator,
            scope: Vec<XVarVal>,
            coeffs: Vec<XVarVal>,
            is_maximize: bool,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                operator,
                scope,
                coeffs,
                is_maximize,
                set,
            }
        }

        pub fn from_str(
            list_str: &str,
            coeffs_str: &str,
            ope_str: &str,
            is_maximize: bool,
            set: &'a XVariableSet,
        ) -> Self {
            let scope = list_to_vec_var_val(list_str);
            let coeffs = list_to_vec_var_val(coeffs_str);
            let v = XElementOperator::get_objectives_operator_by_str(ope_str);
            Self::new(v, scope, coeffs, is_maximize, set)
        }

        pub fn operator(&self) -> &XElementOperator {
            &self.operator
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn coeffs(&self) -> &Vec<XVarVal> {
            &self.coeffs
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }

        pub fn is_maximize(&self) -> bool {
            self.is_maximize
        }
    }

    impl Display for XObjectiveElement<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret1 = String::default();
            for e in self.scope.iter() {
                ret1.push('(');
                ret1.push_str(&e.to_string());
                ret1.push_str("), ")
            }
            let mut ret2 = String::default();
            for e in self.coeffs.iter() {
                ret2.push('(');
                ret2.push_str(&e.to_string());
                ret2.push_str("), ")
            }
            write!(
                f,
                "operator = {:?}, list =  {}, coeffs = {}",
                self.operator, ret1, ret2
            )
        }
    }
}
