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
    use crate::objectives::xobjective_element::xcsp3_core::XObjectiveElement;
    use crate::objectives::xobjective_expression::xcsp3_core::XObjectiveExpression;
    pub(crate) use crate::objectives::xobjectives_type::xcsp3_core::XObjective;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

    pub struct XObjectivesSet<'a> {
        objectives: Vec<XObjective<'a>>,
        set: &'a XVariableSet,
    }

    impl<'a> XObjectivesSet<'a> {
        pub fn build_maximize(
            &mut self,
            list: &str,
            coeffs: &str,
            expression: &str,
            type_str: &str,
        ) {
            // println!("list {} coeffs {} expression {} type_str {}", list, coeffs, expression, type_str);
            if type_str.is_empty() {
                match XObjectiveExpression::from_expr(expression, true, self.set) {
                    Ok(xoe) => {
                        self.objectives.push(XObjective::XObjectiveExpression(xoe));
                    }
                    Err(_e) => panic!("Objectives not recognized"),
                }
            } else {
                match XObjectiveElement::from_str(
                    if !list.is_empty() { list } else { expression },
                    coeffs,
                    type_str,
                    true,
                    self.set,
                ) {
                    Ok(ele) => self.objectives.push(XObjective::XObjectiveElement(ele)),
                    Err(_e) => panic!("Objectives not recognized"),
                }
            }
        }

        pub fn build_minimize(
            &mut self,
            list: &str,
            coeffs: &str,
            expression: &str,
            type_str: &str,
        ) {
            // println!("list {} coeffs {} expression {} type_str {}", list, coeffs, expression, type_str);
            if type_str.is_empty() {
                match XObjectiveExpression::from_expr(expression, false, self.set) {
                    Ok(xoe) => {
                        self.objectives.push(XObjective::XObjectiveExpression(xoe));
                    }
                    Err(_e) => panic!("Objectives not recognized"),
                }
            } else {
                match XObjectiveElement::from_str(
                    if !list.is_empty() { list } else { expression },
                    coeffs,
                    type_str,
                    false,
                    self.set,
                ) {
                    Ok(ele) => self.objectives.push(XObjective::XObjectiveElement(ele)),
                    Err(_e) => panic!("Objectives not recognized"),
                }
            }
        }

        pub fn new(set: &'a XVariableSet) -> Self {
            Self {
                objectives: vec![],
                set,
            }
        }

        pub fn objectives(&self) -> &Vec<XObjective<'a>> {
            &self.objectives
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
