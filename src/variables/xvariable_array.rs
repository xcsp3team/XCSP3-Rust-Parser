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
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{
        get_all_variables_between_lower_and_upper, size_to_string, sizes_to_double_vec,
        sizes_to_vec,
    };
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use std::fmt::{Display, Formatter};

    // use crate::variables::xvariable_trait::xcsp3_core::XVariableTrait;

    #[derive(Clone)]
    pub struct XVariableArray {
        pub(crate) id: String,
        pub(crate) sizes: Vec<usize>,
        pub domain: XDomainInteger,
        pub variables: Vec<String>,
    }

    impl Display for XVariableArray {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let mut ret: String = String::default();
            // ret.push_str(self.id.as_str());
            // ret.push_str("  size = ");
            for e in self.sizes.iter() {
                ret.push('[');
                ret.push_str(e.to_string().as_str());
                ret.push(']');
            }
            // ret.push_str("");
            // ret.push_str(self.domain.to_string().as_str());
            write!(
                f,
                "XVariableArray: id = {},   size =  {} domain = {}",
                self.id,
                ret,
                self.domain.to_string().as_str()
            )
        }
    }

    // impl XVariableTrait for XVariableArray {
    //     fn to_string(&self) -> String {
    //         let mut ret: String = String::from("XVariableArray: id = ");
    //         ret.push_str(self.id.as_str());
    //         ret.push_str("  size = ");
    //         for e in self.sizes.iter() {
    //             ret.push('[');
    //             ret.push_str(e.to_string().as_str());
    //             ret.push(']');
    //         }
    //         ret.push_str(" domain = ");
    //         ret.push_str(self.domain.to_string().as_str());
    //         ret
    //     }
    // }

    impl XVariableArray {
        pub fn find_variable(
            &self,
            id: &str,
        ) -> Result<Vec<(String, &XDomainInteger)>, Xcsp3Error> {
            let mut ret: Vec<(String, &XDomainInteger)> = vec![];
            match id.find('[') {
                None => {
                    return Err(Xcsp3Error::get_variable_size_invalid_error(
                        "find_variable in XVariableArray error",
                    ));
                }
                Some(v) => match sizes_to_double_vec(&id[v..]) {
                    Ok((mut lower, mut upper)) => {
                        for i in 0..lower.len() {
                            if lower[i] == usize::MAX && upper[i] == usize::MAX {
                                lower[i] = 0;
                                upper[i] = self.sizes[i] - 1;
                            }
                            if lower[i] > upper[i] || upper[i] >= self.sizes[i] {
                                return Err(Xcsp3Error::get_variable_size_invalid_error(
                                    "find_variable in XVariableArray error",
                                ));
                            }
                        }
                        let all_variable = get_all_variables_between_lower_and_upper(lower, upper);
                        for size_vec in all_variable.iter() {
                            ret.push((size_to_string(&id[..v], size_vec), &self.domain));
                        }
                    }
                    Err(e) => return Err(e),
                },
            }
            Ok(ret)
        }

        pub fn associated_variables(id: &str, sizes: &[usize]) -> Vec<String> {
            let lower = vec![0; sizes.len()];
            let upper: Vec<usize> = sizes.iter().map(|size| size - 1).collect();
            let all = get_all_variables_between_lower_and_upper(lower, upper);
            let mut result = vec![];
            for current in all {
                result.push(size_to_string(id, &*current));
            }
            result
        }
        pub fn new(id: &str, sizes: &str, domain: XDomainInteger) -> Result<Self, Xcsp3Error> {
            let result = sizes_to_vec(sizes);
            match result {
                Ok((size_vec, _)) => Ok(XVariableArray {
                    id: id.to_string(),
                    sizes: size_vec.clone(),
                    domain,
                    variables: Self::associated_variables(id, &size_vec),
                }),
                Err(e) => Err(e),
            }
        }
    }
}
