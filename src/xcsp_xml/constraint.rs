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

pub mod xcsp3_xml {
    use crate::xcsp_xml::constraint_type::xcsp3_xml::ConstraintType;
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct Constraint {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub constraints: Vec<ConstraintType>,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct ListWithOffset {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub vars: String,
        #[serde(rename = "@collect", default)]
        pub collect: String,
        #[serde(rename = "@offset", default)]
        pub offset: String,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct ListWithClosed {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub vars: String,
        #[serde(rename = "@closed", default)]
        pub closed: String,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct ListWithStartIndex {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "@startIndex", default)]
        pub start_index: String,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct ListWithCovered {
        // #[serde(rename = "constraint", default)]
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "@covered", default)]
        pub covered: String,
    }

    #[derive(Deserialize, Debug, Default)]
    pub struct Matrix {
        #[serde(rename = "$value", default)]
        pub value: String,
        #[serde(rename = "@rowIndex", default)]
        pub row_index: String,
        #[serde(rename = "@colIndex", default)]
        pub col_index: String,
    }
}
