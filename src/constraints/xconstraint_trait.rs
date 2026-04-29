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
* <p>@date:  2023/7/14 18:54
* </p>
* <p>@email: zhenlh20@mails.jlu.edu.cn
* </p>
* <p>@version: 1.0
* </p>
 * <p>@description: 1.0
* </p>
 */

pub mod xcsp3_core {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;

    pub fn inject_parameters_in_list(list: &[XVarVal], arg: &[XVarVal]) -> Vec<XVarVal> {
        let mut unfolded_scope = Vec::new();

        for value in list.iter() {
            match value {
                XVarVal::IntArgument(index) => {
                    let index = *index as usize;
                    if let Some(argument) = arg.get(index) {
                        unfolded_scope.push(argument.clone());
                    } else {
                        panic!("Invalid argument index %{} in allDifferent", index);
                    }
                }
                XVarVal::IntStart => {
                    unfolded_scope.extend_from_slice(arg);
                }
                _ => {
                    unfolded_scope.push(value.clone());
                }
            }
        }
        unfolded_scope
    }

    pub fn inject_parameters_in_operand(operand: &Operand, arg: &[XVarVal]) -> Operand {
        match operand {
            Operand::IntArgument(index) => match arg.get(*index as usize) {
                Some(XVarVal::IntVal(val)) => Operand::Integer(*val),
                Some(XVarVal::IntVar(var)) => Operand::Variable(var.clone()),
                Some(XVarVal::IntInterval(a, b)) => Operand::Interval(*a, *b),
                _ => operand.clone(),
            },
            _ => operand.clone(),
        }
    }

    pub trait XConstraintUnfold {
        fn extract_parameters(&mut self, arg: &[XVarVal]);
    }
}
