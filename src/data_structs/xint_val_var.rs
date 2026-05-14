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
    use regex::Regex;
    use std::fmt::{Display, Formatter};
    use std::str::FromStr;

    #[derive(Clone, Debug)]
    pub enum XVarVal {
        // IntVar(Variable<'a>),
        IntVar(String),
        IntVal(i32),
        IntInterval(i32, i32),
        IntArgument(i32),
        IntStart,
        IntNone,
    }

    impl Display for XVarVal {
        fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    XVarVal::IntVar(v) => {
                        v.clone()
                    }
                    XVarVal::IntVal(v) => {
                        v.to_string()
                    }
                    XVarVal::IntInterval(l, r) => {
                        format!("{}..{}", l, r)
                    }
                    XVarVal::IntArgument(e) => {
                        format!("%{}", e)
                    }
                    XVarVal::IntStart => {
                        "%...".to_string()
                    }
                    XVarVal::IntNone => {
                        "".to_string()
                    }
                }
            )
        }
    }

    impl XVarVal {
        pub fn from_string(s: &str) -> Option<Self> {
            if s.contains('[') {
                Some(XVarVal::IntVar(String::from(s)))
            } else if s.is_empty() {
                None
            } else if s.contains('%') {
                if Regex::new(r"%(0|[1-9][0-9]*)").unwrap().is_match(s)
                //%num
                {
                    match i32::from_str(&s[1..]) {
                        Ok(e) => Some(XVarVal::IntArgument(e)),
                        Err(_) => None,
                    }
                } else if Regex::new(r"%([.]*)").unwrap().is_match(s)
                //%...
                {
                    Some(XVarVal::IntStart)
                } else {
                    panic!("invalid parameter in group {}", s);
                }
            } else if let Some((left, right)) = s.split_once("..") {
                let lower: i32 = left.parse().expect("invalid lower bound");
                let upper: i32 = right.parse().expect("invalid upper bound");
                Some(XVarVal::IntInterval(lower, upper))
            } else {
                match i32::from_str(s) {
                    Ok(e) => Some(XVarVal::IntVal(e)),
                    Err(_) => Some(XVarVal::IntVar(String::from(s))),
                }
            }
        }
    }
}
