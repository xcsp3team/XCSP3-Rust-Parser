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
    use std::fmt::{Display, Formatter};

    use crate::utils::utils_functions::xcsp3_utils::{
        get_all_variables_between_lower_and_upper, size_to_string, sizes_to_double_vec,
        sizes_to_vec,
    };
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;

    #[derive(Clone)]
    pub struct XVariableTree {
        nodes: Vec<XVariableTreeNode>,
        others: XVariableTreeNode,
        pub(crate) id: String,
        pub(crate) sizes: Vec<usize>,
        has_others: bool,
    }

    impl XVariableTree {
        /// return which node the variable belongs to
        pub(crate) fn get_node_by_vec(&self, v: &[usize]) -> &XVariableTreeNode {
            for e in self.nodes.iter() {
                if e.belongs_to_this_node(v) {
                    return e;
                }
            }
            &self.others
        }

        pub fn find_variable(&self, id: &str) -> Vec<(String, &XDomainInteger)> {
            let mut ret: Vec<(String, &XDomainInteger)> = vec![];
            // println!("{}", id);
            match id.find('[') {
                None => panic!("Variable {} unknown", id),
                Some(v) => {
                    let (mut lower, mut upper) = sizes_to_double_vec(&id[v..]);
                    for i in 0..lower.len() {
                        if lower[i] == usize::MAX && upper[i] == usize::MAX {
                            lower[i] = 0;
                            upper[i] = self.sizes[i] - 1;
                        }
                    }
                    let all_variable = get_all_variables_between_lower_and_upper(lower, upper);
                    for size_vec in all_variable.iter() {
                        let node = self.get_node_by_vec(size_vec);
                        ret.push((size_to_string(&id[..v], size_vec), &node.domain));
                    }
                }
            }
            ret
        }

        pub fn new(
            id: &str,
            sizes: &str,
            domain_for: Vec<&String>,
            domain_value: Vec<&String>,
        ) -> Self {
            let (size_vec, _) = sizes_to_vec(sizes);
            let mut has_others = false;
            let mut nodes: Vec<XVariableTreeNode> = Vec::new();
            let mut others_domain = Default::default();
            for i in 0..domain_for.len() {
                let domain = XDomainInteger::from_string(domain_value[i]);
                if domain_for[i].eq("others") {
                    has_others = true;
                    others_domain = domain;
                } else {
                    let for_strs: Vec<&str> = domain_for[i].split_whitespace().collect();
                    for e in for_strs.iter() {
                        let for_str = e.to_string().replace(id, "");
                        let (lower, upper) = sizes_to_double_vec(&for_str);
                        nodes.push(XVariableTreeNode::new(lower, upper, domain.clone()));
                    }
                }
            }
            XVariableTree {
                id: id.to_string(),
                sizes: size_vec,
                others: XVariableTreeNode::new_other(others_domain),
                has_others,
                nodes,
            }
        }

        pub fn sizes(&self) -> &Vec<usize> {
            &self.sizes
        }

        pub fn has_others(&self) -> bool {
            self.has_others
        }
    }

    #[derive(Clone)]
    struct XVariableTreeNode {
        upper: Vec<usize>,
        lower: Vec<usize>,
        domain: XDomainInteger,
        is_other: bool,
    }

    impl XVariableTreeNode {
        pub fn belongs_to_this_node(&self, v: &[usize]) -> bool {
            for (i, v) in v.iter().enumerate() {
                if !(self.lower[i] == usize::MAX && self.upper[i] == usize::MAX
                    || self.lower[i] <= *v && self.upper[i] >= *v)
                {
                    return false;
                }
            }
            true
        }
        pub fn to_string(&self, id: &str) -> String {
            let mut ret = format!("[for = {}", id);
            if self.is_other {
                ret.push_str("[others]..");
            } else {
                for i in 0..self.upper.len() {
                    ret.push('[');
                    if self.lower[i] == self.upper[i] {
                        if self.lower[i] == usize::MAX {
                            ret.push('*');
                        } else {
                            ret.push_str(&self.lower[i].to_string());
                        }
                    } else {
                        ret.push_str(&self.lower[i].to_string());
                        ret.push_str("..");
                        ret.push_str(&self.upper[i].to_string());
                    }

                    ret.push(']');
                }
            }

            ret.push_str("  domain = ");
            ret.push_str(&self.domain.to_string());
            ret
        }

        pub fn new(
            lower: Vec<usize>,
            upper: Vec<usize>,
            domain: XDomainInteger,
        ) -> XVariableTreeNode {
            XVariableTreeNode {
                upper,
                lower,
                domain,
                is_other: false,
            }
        }

        pub fn new_other(domain: XDomainInteger) -> XVariableTreeNode {
            XVariableTreeNode {
                upper: Vec::default(),
                lower: Vec::default(),
                domain,
                is_other: true,
            }
        }
    }

    impl Display for XVariableTree {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret = String::default();
            for e in self.sizes.iter() {
                ret.push('[');
                ret.push_str(e.to_string().as_str());
                ret.push(']');
            }
            ret.push_str("nodes = ");
            for e in self.nodes.iter() {
                ret += &e.to_string(&self.id);
                ret += "]  ";
            }

            write!(f, "XVariableTree:  id = {}, sizes = {}", self.id, ret)
        }
    }

    // impl XVariableTrait for XVariableTree {
    //     fn to_string(&self) -> String {
    //         let mut ret = format!("XVariableTree:  id = {}, sizes = ", self.id);
    //         for e in self.sizes.iter() {
    //             ret.push('[');
    //             ret.push_str(e.to_string().as_str());
    //             ret.push(']');
    //         }
    //         ret.push_str("nodes = ");
    //         for e in self.nodes.iter() {
    //             ret += &e.to_string(&self.id);
    //             ret += "]  ";
    //         }
    //         ret
    //     }
    // }
}
