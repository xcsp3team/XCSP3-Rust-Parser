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
pub mod xcsp3_utils {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;

    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;

    #[derive(Debug, Clone)]
    pub enum Operator {
        Add,
        Neg,
        Abs,
        Sub,
        Mul,
        Div,
        Mod,
        Sqr,
        Pow,
        Min,
        Max,
        Dist,
        Lt,
        Le,
        Ge,
        Gt,
        Ne,
        Eq,
        And,
        Not,
        Or,
        Xor,
        Iff,
        Imp,
        If,
        Set,
        In,
    }

    impl Operator {
        pub fn get_operator_by_str(op: &str) -> Option<Self> {
            match op {
                "add" => Some(Operator::Add),
                "neg" => Some(Operator::Neg),
                "abs" => Some(Operator::Abs),
                "sub" => Some(Operator::Sub),
                "mul" => Some(Operator::Mul),
                "div" => Some(Operator::Div),
                "mod" => Some(Operator::Mod),
                "sqr" => Some(Operator::Sqr),
                "pow" => Some(Operator::Pow),
                "min" => Some(Operator::Min),
                "max" => Some(Operator::Max),
                "dist" => Some(Operator::Dist),
                "lt" => Some(Operator::Lt),
                "le" => Some(Operator::Le),
                "ge" => Some(Operator::Ge),
                "gt" => Some(Operator::Gt),
                "ne" => Some(Operator::Ne),
                "eq" => Some(Operator::Eq),
                "and" => Some(Operator::And),
                "not" => Some(Operator::Not),
                "or" => Some(Operator::Or),
                "xor" => Some(Operator::Xor),
                "iff" => Some(Operator::Iff),
                "imp" => Some(Operator::Imp),
                "if" => Some(Operator::If),
                "set" => Some(Operator::Set),
                "in" => Some(Operator::In),
                _ => None,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum TreeNode {
        RightBracket,
        LeftBracket,
        Constant(i32),
        Argument(i32),
        Variable(String),
        Operator(Operator, Vec<TreeNode>),
    }

    impl Display for TreeNode {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    TreeNode::Constant(i) => i.to_string(),
                    TreeNode::RightBracket => ")".to_string(),
                    TreeNode::LeftBracket => "(".to_string(),
                    TreeNode::Variable(v) => v.to_string(),
                    TreeNode::Argument(a) => format!("%{}", a),
                    TreeNode::Operator(o, _) => {
                        format!("{:?}", o)
                    }
                }
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct ExpressionTree {
        root: TreeNode,
        // expression: String,
    }

    impl Display for ExpressionTree {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::new();
            for e in self.first_order_iter() {
                ret += &*e.to_string();
                match e {
                    TreeNode::Variable(_) => ret += ",",
                    TreeNode::Argument(_) => ret += ",",
                    TreeNode::Constant(_) => ret += ",",
                    _ => {}
                }
            }
            write!(f, "{}", ret)
        }
    }
    impl ExpressionTree {
        pub fn get_scope(&self) -> Vec<String> {
            let mut scope = vec![];
            for e in self.first_order_iter() {
                if let TreeNode::Variable(v) = e {
                    scope.push(v.clone());
                }
            }
            scope
        }
        pub fn get(&self, set: &XVariableSet) -> Vec<XVarVal> {
            let mut scope: Vec<XVarVal> = vec![];
            for e in self.first_order_iter() {
                if let TreeNode::Variable(v) = e {
                    let r = set.find_variable(v);
                    scope.push(XVarVal::IntVar(v.to_string()))
                }
            }
            scope
        }

        pub fn from_string(expression: &str) -> Self {
            ExpressionTree {
                root: ExpressionTree::parse(expression),
            }
        }

        fn operator(exp: &str, stack: &mut Vec<TreeNode>) {
            let expression: String = exp.chars().rev().collect();
            // expression = expression.replace("(", "").replace(")", "");
            match Operator::get_operator_by_str(&expression) {
                None => {
                    if expression.contains('%') {
                        match i32::from_str(&expression[1..]) {
                            Ok(n) => stack.push(TreeNode::Argument(n)),
                            Err(_) => panic!("parse the expression error {}", expression),
                        }
                    } else {
                        match i32::from_str(&expression[..]) {
                            Ok(n) => stack.push(TreeNode::Constant(n)),
                            Err(_) => stack.push(TreeNode::Variable(expression)),
                        }
                    }
                }
                Some(ope) => {
                    let mut nodes = vec![];
                    loop {
                        let top = stack.pop();
                        match top {
                            None => {
                                panic!("parse the expression error {}", expression);
                            }
                            Some(n) => match n {
                                TreeNode::RightBracket => {
                                    stack.push(TreeNode::Operator(ope, nodes));
                                    break;
                                }
                                _ => {
                                    nodes.push(n);
                                }
                            },
                        }
                    }
                }
            }
        }

        fn parse(expression: &str) -> TreeNode {
            let mut stack: Vec<TreeNode> = vec![];
            let exp: String = expression.chars().filter(|c| !c.is_whitespace()).collect();

            let rev_exp: String = exp.chars().rev().collect();
            // println!("{rev_exp}");
            let mut i = 0;
            let mut last = 0;
            while i < rev_exp.len() {
                if &rev_exp[i..i + 1] == ")" {
                    stack.push(TreeNode::RightBracket);
                    last = i;
                } else if &rev_exp[i..i + 1] == "," || &rev_exp[i..i + 1] == "(" {
                    ExpressionTree::operator(&rev_exp[last + 1..i], &mut stack);
                    last = i;
                } else if i == rev_exp.len() - 1 {
                    ExpressionTree::operator(&rev_exp[last + 1..i + 1], &mut stack);
                }
                i += 1
            }
            stack.pop().unwrap()
        }
        pub fn first_order_iter(&self) -> ExpressionFirstOrderIter<'_> {
            ExpressionFirstOrderIter {
                stack: vec![&self.root],
            }
        }
        // pub fn last_order_iter(&self) -> ExpressionLastOrderIter {
        //     ExpressionLastOrderIter { stack: vec![&self.root] }
        // }

        pub fn is_variable(&self) -> bool {
            matches!(self.root, TreeNode::Variable(_))
        }

        pub fn as_variable(&self) -> Option<&String> {
            match &self.root {
                TreeNode::Variable(name) => Some(name),
                _ => None,
            }
        }
    }

    pub struct ExpressionFirstOrderIter<'a> {
        stack: Vec<&'a TreeNode>,
    }

    impl<'a> Iterator for ExpressionFirstOrderIter<'a> {
        type Item = &'a TreeNode;
        fn next(&mut self) -> Option<Self::Item> {
            let top = match self.stack.pop() {
                None => {
                    return None;
                }
                Some(t) => t,
            };
            if let TreeNode::Operator(_, vec) = top {
                self.stack.push(&TreeNode::RightBracket);
                (0..vec.len()).rev().for_each(|i| {
                    self.stack.push(&vec[i]);
                });
                self.stack.push(&TreeNode::LeftBracket);
            };

            Some(top)
        }
    }

    // pub struct ExpressionLastOrderIter<'a> {
    //     stack: Vec<&'a TreeNode>,
    // }
    //
    // impl<'a> Iterator for ExpressionLastOrderIter<'a> {
    //     type Item = &'a TreeNode;
    //     fn next(&mut self) -> Option<Self::Item> {
    //         loop
    //         {
    //             match self.stack.last() {
    //                 None => { return None; }
    //                 Some(top) => {
    //                     match top
    //                     {
    //                         TreeNode::Operator(_, vec) => {
    //                             (0..vec.len()).rev().for_each(|i| {
    //                                 self.stack.push(&vec[i]);
    //                             })
    //                         }
    //                         _ => {break}
    //                     }
    //                 }
    //             }
    //         }
    //         match self.stack.pop()
    //         {
    //             None => { None }
    //             Some(t) => { Some(t) }
    //         }
    //     }
    // }
}
