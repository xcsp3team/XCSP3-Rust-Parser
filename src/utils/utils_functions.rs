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
use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
use crate::variables::xvariable_set::xcsp3_core::XVariableSet;

pub mod xcsp3_utils {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use crate::variables::xvariable_type::xcsp3_core::XVariableType::{XVariableArray, XVariableTree};
    // use std::str::FromStr;

    pub fn str_to_interval(interval: &str) -> (i32, i32) {
        let interval: Vec<&str> = interval.split("..").collect();
        if interval.len() == 2 {
            let left = interval[0].parse::<i32>();
            let right = interval[1].parse::<i32>();
            match left {
                Ok(l) => match right {
                    Ok(r) => (l, r),
                    Err(_) => panic!("parse interval error{:?}", interval),
                },
                Err(_) => panic!("parse interval error{:?}", interval),
            }
        } else {
            panic!("parse interval error{:?}", interval);
        }
    }

    pub fn str_to_condition(condition: &str) -> (Operator, Operand) {
        let tmp = condition.replace(['(', ')', ',', '{', '}'], " ");
        let split: Vec<&str> = tmp.split_whitespace().collect();
        let ope = Operator::get_operator_by_str(split[0]);
        let rand: Operand = match Operand::get_operand_by_str(&split[1..], &ope) {
            None => panic!("parse condition  error, {}", condition),
            Some(r) => r,
        };
        (ope, rand)
    }

    pub fn str_to_condition_option(condition: &str) -> (Option<Operator>, Option<Operand>) {
        if condition.is_empty() {
            (None, None)
        } else {
            let tmp = str_to_condition(&condition);
            (Some(tmp.0), Some(tmp.1))
        }
    }

    pub fn to_bool_option(string: &str) -> Option<bool> {
        if !string.trim().is_empty() {
            match string.trim().parse::<bool>() {
                Ok(n) => Some(n),
                Err(_) => panic!("parse bool error {}", string),
            }
        } else {
            None
        }
    }

    pub fn to_i32_option(string: &str) -> Option<i32> {
        if !string.trim().is_empty() {
            match string.trim().parse::<i32>() {
                Ok(n) => Some(n),
                Err(_) => panic!("parse i32 error {}", string),
            }
        } else {
            None
        }
    }
    pub fn to_matrix(list: &str, set: &XVariableSet) -> Vec<Vec<XVarVal>> {
        if list.contains("[][]") {
            let name = list.split('[').next().unwrap_or(list);
            let var = set.find_variable(name);
            let row_size = match var {
                XVariableArray(v) => v.sizes[0],
                XVariableTree(v) => v.sizes[0],
                _ => todo!("In progress)"),
            };
            let col_size = match var {
                XVariableArray(v) => v.sizes[1],
                XVariableTree(v) => v.sizes[1],
                _ => todo!("In progress)"),
            };

            let mut matrix: Vec<Vec<XVarVal>> = Vec::with_capacity(row_size);
            for i in 0..row_size {
                matrix.push(vec![]);
                for j in 0..col_size {
                    matrix[i].push(XVarVal::IntVar(format!("{}[{}][{}]", name, i, j)));
                }
            }
            matrix
        } else {
            let tmp = list_to_matrix_ids(list);
            if tmp[0][0].parse::<i32>().is_ok() {
                let matrix: Vec<Vec<XVarVal>> = list_to_matrix_ids(list)
                    .iter()
                    .map(|line| line.iter().map(|e| XVarVal::IntVal(e.parse::<i32>().expect("argl"))).collect())
                    .collect();
                matrix
            } else {
                let matrix: Vec<Vec<XVarVal>> = list_to_matrix_ids(list)
                    .iter()
                    .map(|line| line.iter().map(|e| XVarVal::IntVar(e.clone())).collect())
                    .collect();
                matrix
            }
        }
    }

    pub fn list_to_vec_var_val(list: &str) -> Vec<XVarVal> {
        let mut ret: Vec<XVarVal> = vec![];
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            if e.trim().starts_with(|c: char| c.is_ascii_digit() || c == '-') && e.contains("x") {
                // Deal with compressed coefficient
                if let Some((value_str, count_str)) = e.trim().split_once('x') {
                    let count: usize = count_str.parse().expect("invalid count");
                    let value: i32 = value_str.parse().expect("invalid value");
                    ret.extend(std::iter::repeat(XVarVal::IntVal(value)).take(count));
                }
            } else {
                match XVarVal::from_string(e) {
                    None => panic!("parsing  list membererror {}", e),
                    Some(vv) => {
                        ret.push(vv);
                    }
                }
            }
        }
        ret
    }

    // pub fn list_to_vec_var(list: &str) -> Vec<XVarVal> {
    //     let mut ret: Vec<XVarVal> = vec![];
    //     let lists: Vec<&str> = list.split_whitespace().collect();
    //     for e in lists.into_iter() {
    //         ret.push(XVarVal::IntVar(e.to_owned()));
    //
    //     }
    //     ret
    // }

    pub fn size_to_string(id: &str, size: &[usize]) -> String {
        let mut ret = id.to_string();

        for e in size.iter() {
            ret.push_str(&format!("[{}]", e))
        }
        ret
    }

    ///([2,3,4],[2,4,8]) -> [[2,3,4],[2,3,5],[2,3,6],[2,3,7],[2,3,8],[2,4,4],[2,4,5],[2,4,6],[2,4,7],[2,4,8]]
    pub fn get_all_variables_between_lower_and_upper(lower: Vec<usize>, upper: Vec<usize>) -> Vec<Vec<usize>> {
        let mut tmp: Vec<Vec<usize>> = vec![];
        for i in lower[0]..=upper[0] {
            tmp.push(vec![i]);
        }

        for deep in 1..lower.len() {
            let mut ret: Vec<Vec<usize>> = vec![];
            for e in tmp.iter() {
                // outer: existing combinations
                for i in lower[deep]..=upper[deep] {
                    // inner: new dimension
                    let mut ee = e.clone();
                    ee.push(i);
                    ret.push(ee);
                }
            }
            tmp = ret;
        }
        tmp
    }

    /// return the nth of the [] in id(str)
    /// eg x[2][5][] -> 2,  y[] -> 0, z[3][] ->1, zzz[4][][4] ->1
    pub fn get_nth_square_of_name(id: &str) -> usize {
        match id.find("[]") {
            None => 0,
            Some(v) => {
                let mut cnt: usize = 0;
                for (_, c) in id[..v].chars().enumerate() {
                    if c == '[' {
                        cnt += 1
                    }
                }
                cnt
            }
        }
    }

    /// return the list of scopes,
    /// eg str"x[1] x[3] x[5]" - > vec[x[1], x[3], x[5]]
    pub fn list_to_scope_ids(list: &str) -> Vec<String> {
        let mut ret: Vec<String> = Vec::new();
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            ret.push(e.to_string());
        }
        ret
    }

    /// return the transitions,
    /// eg  "(a,0,a)(a,1,b)(b,1,c)(c,0,d)(d,0,d)(d,1,e)(e,0,e)" -> vec[ (a,0,a),(a,1,b),(b,1,c),(c,0,d),(d,0,d),(d,1,e),(e,0,e)]
    pub fn list_to_transitions(list: &str) -> Vec<(String, i32, String)> {
        let mut ret: Vec<(String, i32, String)> = Vec::new();
        let chars = list.chars();
        if let Some(left) = list.find('(') {
            if let Some(right) = list.find(')') {
                ret.reserve(list.len() / (right - left));
                // n = (right - left - 1) / 2;
            }
        }
        let mut last = usize::MAX;
        let mut last_comma1 = usize::MAX;
        let mut last_comma2 = usize::MAX;
        for (i, x) in chars.enumerate() {
            if x == '(' {
                last = i;
            } else if x == ')' {
                // println!("{}",&tuple_str[last+1..i]);
                match list[last_comma1 + 1..last_comma2].parse::<i32>() {
                    Ok(num) => {
                        ret.push((list[last + 1..last_comma1].to_string(), num, list[last_comma2 + 1..i].to_string()))
                    }
                    Err(_) => panic!("parse the transitions error: {} ", list),
                }
                last_comma1 = usize::MAX;
            } else if x == ',' {
                if last_comma1 == usize::MAX {
                    last_comma1 = i;
                } else {
                    last_comma2 = i;
                }
            }
        }
        ret
    }

    /// return the matrix,
    /// eg str"(x1,x2,x3,x4,x5)(y1,y2,y3,y4,y5)(z1,z2,z3,z4,z5)" - > vec[[x1,x2,x3,x4,x5][y1,y2,y3,y4,y5][z1,z2,z3,z4,z5]]
    pub fn list_to_matrix_ids(list: &str) -> Vec<Vec<String>> {
        let mut ret: Vec<Vec<String>> = Vec::new();
        let list = list.to_string().replace(')', "@").replace('\n', "").replace('(', " ");
        let lists: Vec<&str> = list.split('@').collect();
        for e in lists.iter() {
            if !e.is_empty() {
                let ss = e.replace(',', " ");
                ret.push(list_to_scope_ids(&ss));
            }
        }
        ret
    }

    /// return the list of values,
    /// eg str"1 3 5 76" -> vec[1,3,5,76],
    pub fn list_to_values(list: &str) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let lists: Vec<&str> = list.split_whitespace().collect();
        for l in lists.iter() {
            match l.parse::<i32>() {
                Ok(n) => ret.push(n),
                Err(_) => panic!("parsing  list of int error: {} ", list),
            }
        }
        ret
    }

    /// return the list of values,
    /// eg str"(1, 3, 5, 76)" -> vec[1,3,5,76],
    pub fn list_with_bracket_comma_to_values(list: &str) -> Vec<XVarVal> {
        let mut ret: Vec<XVarVal> = Vec::new();
        let list = list.to_string().replace(['(', ')', ','], " ");
        let lists: Vec<&str> = list.split_whitespace().collect();
        for e in lists.iter() {
            match e.parse::<i32>() {
                Ok(n) => ret.push(XVarVal::IntVal(n)),
                Err(_) => panic!("parsing list error: {} ", list),
            }
        }
        ret
    }

    ///return the tuples by given string,
    /// eg (0,0,1)(0,1,0)(1,0,0)(1,1,1) -> [[0,0,1],[0,1,0],[1,0,0],[1,1,1]]
    pub fn tuple_to_vector(tuple_str: &str, is_unary: bool) -> Vec<Vec<i32>> {
        // let ti = TimeInterval::new();
        let mut ret: Vec<Vec<i32>> = Vec::new();
        if is_unary {
            let tuples: Vec<&str> = tuple_str.split_whitespace().collect();
            for tuple in tuples.iter() {
                if tuple.contains("..") {
                    let interval: Vec<&str> = tuple.split("..").collect();
                    if interval.len() == 2 {
                        let left = interval[0].parse::<i32>();
                        let right = interval[1].parse::<i32>();
                        match left {
                            Ok(l) => match right {
                                Ok(r) => {
                                    if l <= r {
                                        for i in l..r + 1 {
                                            ret.push(vec![i])
                                        }
                                    } else {
                                        panic!("parsing tuples error: {} ", tuple_str);
                                    }
                                }
                                Err(_) => panic!("parsing tuples error: {} ", tuple_str),
                            },
                            Err(_) => panic!("parsing tuples error: {} ", tuple_str),
                        }
                    }
                } else {
                    match tuple.parse::<i32>() {
                        Ok(v) => ret.push(vec![v]),
                        Err(_) => panic!("parsing tuples error: {} ", tuple_str),
                    }
                }
            }
        } else {
            let chars = tuple_str.chars();
            let mut last = 0;
            let mut tt: Vec<i32> = vec![];
            let mut n: usize = 0;
            if let Some(left) = tuple_str.find('(') {
                if let Some(right) = tuple_str.find(')') {
                    ret.reserve(tuple_str.len() / (right - left));
                    n = (right - left - 1) / 2;
                }
            }
            for (i, x) in chars.enumerate() {
                if x == '(' {
                    tt.clear();
                    tt.reserve(n);
                    last = i;
                } else if x == ')' {
                    // println!("{}",&tuple_str[last+1..i]);
                    match tuple_str[last + 1..i].parse::<i32>() {
                        Ok(num) => {
                            tt.push(num);
                        }
                        Err(_) => {
                            if &tuple_str[last + 1..i] == "*" {
                                tt.push(i32::MAX);
                            } else {
                                panic!("parsing tuples error: {} ", tuple_str);
                            }
                        }
                    }
                    ret.push(tt.clone())
                } else if x == ',' {
                    // println!("{}",&tuple_str[last+1..i]);
                    match tuple_str[last + 1..i].parse::<i32>() {
                        Ok(num) => {
                            tt.push(num);
                        }
                        Err(_) => {
                            if &tuple_str[last + 1..i] == "*" {
                                tt.push(i32::MAX);
                            } else {
                                panic!("parsing tuples error: {} ", tuple_str);
                            }
                        }
                    }
                    last = i;
                }
            }
        }
        // println!("parse Extension {:?}",ti.get());
        ret
    }

    /// transform the string size to vector sizes
    /// eg:  [2][3..4][4..8] -> ([2,3,4],[2,4,8])
    pub fn sizes_to_double_vec(sizes: &str) -> (Vec<usize>, Vec<usize>) {
        let mut lower: Vec<usize> = vec![];
        let mut upper: Vec<usize> = vec![];
        let sizes = sizes.replace("[]", "[*]").replace(['[', ']'], " ");
        let nums: Vec<&str> = sizes.split_whitespace().collect();
        for n in nums.iter() {
            if n.find('*').is_some() {
                lower.push(usize::MAX);
                upper.push(usize::MAX);
            } else if n.contains("..") {
                let interval: Vec<&str> = n.split("..").collect();
                if interval.len() == 2 {
                    let low = interval[0].parse::<usize>();
                    let up = interval[1].parse::<usize>();

                    match low {
                        Ok(l) => match up {
                            Ok(u) => {
                                lower.push(l);
                                upper.push(u);
                            }
                            Err(_) => panic!("parsing sizes error: {} ", sizes),
                        },
                        Err(_) => panic!("parsing sizes error: {} ", sizes),
                    }
                }
            } else {
                match n.parse::<usize>() {
                    Ok(v) => {
                        lower.push(v);
                        upper.push(v);
                    }
                    Err(_) => panic!("parsing sizes error: {} ", sizes),
                };
            }
        }
        (lower, upper)
    }

    /// transform the string size to vector sizes
    /// eg:  [2][3][4] -> ([2,3,4], 24)
    pub fn sizes_to_vec(sizes: &str) -> (Vec<usize>, usize) {
        let mut ret: Vec<usize> = vec![];
        let mut sz: usize = 1;
        let mut sizes = sizes.replace('[', " ");
        sizes = sizes.replace(']', " ");
        let nums: Vec<&str> = sizes.split_whitespace().collect();
        for n in nums.iter() {
            match n.parse::<usize>() {
                Ok(v) => {
                    ret.push(v);
                    sz *= v;
                }
                Err(_) => panic!("parse the size of variable error {}", sizes),
            };
        }
        (ret, sz)
    }
}

pub fn to_int_list(the_list: &[XVarVal]) -> Vec<i32> {
    let mut tmp = vec![];
    for v in the_list {
        match v {
            XVarVal::IntVal(value) => tmp.push(*value),
            XVarVal::IntInterval(v1, v2) => {
                for i in *v1..*v2 {
                    tmp.push(i);
                }
            }
            _ => panic!("Only integers are allowed in this list"),
        }
    }
    tmp
}

pub fn to_var_list(the_list: &[XVarVal], set: &XVariableSet) -> Vec<String> {
    the_list
        .iter()
        .filter_map(|e| match e {
            XVarVal::IntVar(s) => {
                let tmp = set.construct_scope(&[&s]);
                Some(tmp)
            }
            _ => {
                panic!("Only vars in this list are allowed: {}", e)
            }
        })
        .flatten()
        .map(|(vs, _vv)| vs)
        .collect()
}

pub fn to_interval_list(the_list: &[XVarVal]) -> Vec<(i32, i32)> {
    let mut tmp = vec![];
    for v in the_list {
        match v {
            XVarVal::IntInterval(v1, v2) => {
                tmp.push((*v1, *v2));
            }
            _ => panic!("Only intervals are allowed in this list"),
        }
    }
    tmp
}

pub fn to_expression_list(the_list: &[XVarVal], _set: &XVariableSet) -> Vec<ExpressionTree> {
    let mut trees = vec![];

    for v in the_list {
        match v {
            XVarVal::IntVar(expr) => {
                let tree = ExpressionTree::from_string(expr);
                trees.push(tree);
            }
            _ => panic!("Only IntVar expressions are allowed in this list"),
        }
    }
    trees
}
pub fn scope_contains_expressions(scope: &[XVarVal]) -> bool {
    scope.iter().any(|s| s.to_string().contains('('))
}

pub fn is_int_list(scope: &[XVarVal]) -> bool {
    match scope.first() {
        Some(XVarVal::IntVal(_)) => true,
        _ => false,
    }
}

pub fn is_var_list(scope: &[XVarVal]) -> bool {
    match scope.first() {
        Some(XVarVal::IntVar(_)) => true,
        _ => false,
    }
}

pub fn is_interval_list(scope: &[XVarVal]) -> bool {
    match scope.first() {
        Some(XVarVal::IntInterval(_, ..)) => true,
        _ => false,
    }
}
