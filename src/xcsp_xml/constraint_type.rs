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
    use crate::xcsp_xml::constraint::xcsp3_xml::{
        ListWithClosed, ListWithCovered, ListWithStartIndex, Matrix,
    };
    use crate::xcsp_xml::constraint_block::xcsp3_xml::ConstraintBlock;
    use crate::xcsp_xml::constraint_group::xcsp3_xml::ConstraintGroup;
    use crate::xcsp_xml::constraint_slide::xcsp3_xml::ConstraintSlide;
    use serde::Deserialize;

    #[derive(Deserialize, Debug, Default)]
    pub enum ConstraintType {
        #[serde(rename = "group")]
        Group(ConstraintGroup),
        #[serde(rename = "block")]
        Block(ConstraintBlock),
        #[serde(rename = "allDifferent")]
        AllDifferent {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Box<[String]>,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "matrix", default)]
            matrix: String,
        },
        #[serde(rename = "allEqual")]
        AllEqual {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Vec<String>,
        },
        #[serde(rename = "circuit")]
        Circuit {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Box<[String]>,
            #[serde(rename = "size", default)]
            size: String,
        },
        #[serde(rename = "ordered")]
        Ordered {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "@case", default)]
            case: String,
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "operator", default)]
            operator: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
        },
        #[serde(rename = "intension")]
        Intension {
            #[serde(rename = "$value", default)]
            value: String,
            #[serde(rename = "function", default)]
            function: String,
        },
        #[serde(rename = "extension")]
        Extension {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "supports", default)]
            supports: String,
            #[serde(rename = "conflicts", default)]
            conflicts: String,
        },
        #[serde(rename = "regular")]
        Regular {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
            #[serde(rename = "start", default)]
            start: String,
            #[serde(rename = "final", default)]
            r#final: String,
        },
        #[serde(rename = "mdd")]
        Mdd {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "transitions", default)]
            transitions: String,
        },
        #[serde(rename = "sum")]
        Sum {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "coeffs", default)]
            coeffs: String,
        },
        #[serde(rename = "count")]
        Count {
            // #[serde(rename = "@id", default)]
            // id: String,
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "nValues")]
        NValues {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "except", default)]
            except: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "cardinality")]
        Cardinality {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "values", default)]
            values: ListWithClosed,
            #[serde(rename = "occurs", default)]
            occurs: String,
        },
        #[serde(rename = "minimum")]
        Minimum {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },

        #[serde(rename = "minimumArg")]
        MinimumArg {
            #[serde(rename = "list", default)]
            list: ListWithStartIndex,
            #[serde(rename = "@rank", default)]
            rank: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "maximum")]
        Maximum {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "maximumArg")]
        MaximumArg {
            #[serde(rename = "list", default)]
            list: ListWithStartIndex,
            #[serde(rename = "@rank", default)]
            rank: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[serde(rename = "element")]
        Element {
            #[serde(rename = "list", default)]
            vars: ListWithStartIndex,
            #[serde(rename = "value", default)]
            value: String,
            #[serde(rename = "index", default)]
            index: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "matrix", default)]
            matrix: Matrix,
        },
        #[serde(rename = "stretch")]
        Stretch {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
            #[serde(rename = "widths", default)]
            widths: String,
            #[serde(rename = "patterns", default)]
            patterns: String,
        },
        #[serde(rename = "noOverlap")]
        NoOverlap {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
            #[serde(rename = "@zeroIgnored", default)]
            zero_ignored: String,
        },
        #[serde(rename = "cumulative")]
        Cumulative {
            #[serde(rename = "origins", default)]
            origins: String,
            #[serde(rename = "lengths", default)]
            lengths: String,
            #[serde(rename = "heights", default)]
            heights: String,
            #[serde(rename = "condition", default)]
            condition: ListWithStartIndex,
            #[serde(rename = "ends", default)]
            ends: String,
            #[serde(rename = "machines", default)]
            machines: String,
        },
        #[serde(rename = "instantiation")]
        Instantiation {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "values", default)]
            values: String,
        },
        #[serde(rename = "slide")]
        Slide(ConstraintSlide),
        #[serde(rename = "channel")]
        Channel {
            #[serde(rename = "list", default)]
            lists: Box<[ListWithStartIndex]>,
            #[serde(rename = "value", default)]
            with_value: String,
            #[serde(rename = "$value", default)]
            simplified_list: String,
        },
        #[serde(rename = "precedence")]
        Precedence {
            #[serde(rename = "$value", default)]
            vars: String,
            #[serde(rename = "list", default)]
            list: Box<[String]>,
            #[serde(rename = "values", default)]
            values: Box<ListWithCovered>,
        },
        #[serde(rename = "knapsack")]
        Knapsack {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "weights", default)]
            weights: String,
            #[serde(rename = "condition", default)]
            condition: Box<[String]>,
            #[serde(rename = "profits", default)]
            profits: String,
        },

        #[serde(rename = "binPacking")]
        BinPacking {
            #[serde(rename = "list", default)]
            list: String,
            #[serde(rename = "sizes", default)]
            sizes: String,
            #[serde(rename = "condition", default)]
            condition: String,
            #[serde(rename = "limits", default)]
            limits: String,
            #[serde(rename = "loads", default)]
            loads: String,
        },

        #[serde(rename = "clause")]
        Clause {
            #[serde(rename = "$value", default)]
            vars: String,
        },

        #[serde(rename = "lex")]
        Lex {
            #[serde(rename = "list", default)]
            lists: Vec<String>,
            #[serde(rename = "matrix", default)]
            matrix: String,
            #[serde(rename = "operator", default)]
            operator: String,
        },

        #[serde(rename = "flow")]
        Flow {
            #[serde(rename = "list", default)]
            vars: String,
            #[serde(rename = "balance", default)]
            balance: String,
            #[serde(rename = "weights", default)]
            weights: String,
            #[serde(rename = "arcs", default)]
            arcs: String,
            #[serde(rename = "condition", default)]
            condition: String,
        },
        #[default]
        ConstraintNone,
    }
}
