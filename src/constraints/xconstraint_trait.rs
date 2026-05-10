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
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use std::collections::HashSet;
    use std::str::FromStr;

    pub fn inject_parameters_in_var_val(value: XVarVal, arg: &[XVarVal]) -> XVarVal {
        match value {
            XVarVal::IntArgument(index) => {
                let index = index as usize;
                if let Some(argument) = arg.get(index) {
                    argument.clone()
                } else {
                    panic!("Invalid argument index %{}", index);
                }
            }
            _ => value.clone(),
        }
    }

    pub fn inject_parameters_in_list(
        list: &[XVarVal],
        arg: &[XVarVal],
        start: i32,
    ) -> Vec<XVarVal> {
        let mut unfolded_scope = Vec::new();
        let all_params = &arg[(start + 1) as usize..];
        for value in list.iter() {
            match value {
                XVarVal::IntArgument(index) => {
                    let index = *index as usize;
                    if let Some(argument) = arg.get(index) {
                        unfolded_scope.push(argument.clone());
                    } else {
                        panic!("Invalid argument index %{}", index);
                    }
                }
                XVarVal::IntStart => {
                    unfolded_scope.extend_from_slice(all_params);
                }
                _ => {
                    unfolded_scope.push(value.clone());
                }
            }
        }
        unfolded_scope
    }
    /*
        let mut ret: HashSet<i32> = HashSet::new();
        for l in s.iter() {
    match i32::from_str(l) {
    Ok(n) => {
    ret.insert(n);
    }
    Err(_) => {
    return None;
    }
    }
    }
        Some(Operand::SetInteger(ret))
    */
    pub fn inject_parameters_in_operand(operand: &Operand, arg: &[XVarVal]) -> Operand {
        match operand {
            Operand::IntArgument(index) => match arg.get(*index as usize) {
                Some(XVarVal::IntVal(val)) => Operand::Integer(*val),
                Some(XVarVal::IntVar(interval)) if interval.starts_with("{") => {
                    let mut tmp = interval.replace(['{', '}', ','], " ");
                    let split: Vec<&str> = tmp.split_whitespace().collect();
                    let mut ret: HashSet<i32> = HashSet::new();
                    for l in split.iter() {
                        match i32::from_str(l) {
                            Ok(n) => {
                                ret.insert(n);
                            }
                            Err(_) => {
                                panic!("parse interval error: {:?}", interval);
                            }
                        }
                    }
                    Operand::SetInteger(ret)
                }
                Some(XVarVal::IntVar(var)) => Operand::Variable(var.clone()),
                Some(XVarVal::IntInterval(a, b)) => Operand::Interval(*a, *b),
                _ => operand.clone(),
            },
            _ => operand.clone(),
        }
    }

    pub fn max_arg_in_list(list: &[XVarVal]) -> i32 {
        let mut max = -1;
        for value in list.iter() {
            match value {
                XVarVal::IntArgument(index) => {
                    if *index as i32 > max {
                        max = *index as i32;
                    }
                }
                _ => (),
            }
        }
        max
    }

    pub fn arg_in_var(value: &XVarVal) -> i32 {
        match value {
            XVarVal::IntArgument(index) => *index as i32,
            _ => -1,
        }
    }
    pub fn arg_in_operand(operand: &Operand) -> i32 {
        match operand {
            Operand::IntArgument(index) => *index as i32,
            _ => -1,
        }
    }

    pub trait XConstraintUnfold {
        fn extract_parameters(&mut self, arg: &[XVarVal]);
        fn max_args_used(&mut self) -> i32;
    }
}
