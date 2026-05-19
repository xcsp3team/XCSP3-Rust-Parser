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
use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
use crate::objectives::xobjective_element::xcsp3_core::XElementOperator;
use crate::xcsp_callback::XcspCallback;
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::InstanceType;
// ---------------------------------------------------------------------------
// PrintingSolver : affiche et compte chaque élément du fichier XCSP3
// ---------------------------------------------------------------------------

pub struct PrintingSolver {
    pub nb_variables: usize,
    pub nb_constraints: usize,
}

impl PrintingSolver {
    pub fn new() -> Self {
        Self { nb_variables: 0, nb_constraints: 0 }
    }
}

impl XcspCallback for PrintingSolver {
    // -- Cycle de vie --------------------------------------------------------
    fn begin_instance(&mut self, _type: &InstanceType) {
        println!("Start to load an instance of type {:?}", _type);
    }
    fn end_instance(&mut self) {
        println!("Done...");
    }

    fn begin_variables(&mut self) {
        println!("=== Variables ===");
    }
    fn end_variables(&mut self) {
        println!("Number of variables: {}\n", self.nb_variables);
    }
    fn begin_constraints(&mut self) {
        println!("=== Constraints ===");
    }
    fn end_constraints(&mut self) {
        println!("Number of constraints: {} \n", self.nb_constraints);
    }

    fn begin_group(&mut self) {
        println!("  [Group] begin");
    }
    fn end_group(&mut self) {
        println!("  [Group] end");
    }

    fn begin_slide(&mut self) {
        println!("  [Slide] begin");
    }
    fn end_slide(&mut self) {
        println!("  [Slide] end");
    }

    fn begin_objectives(&mut self) {
        println!("=== Objectives ===");
    }
    fn end_objectives(&mut self) {
        println!("=== Objectives done ===");
    }

    // -- Variables -----------------------------------------------------------
    fn on_variable_interval(&mut self, id: String, minimum: i32, maximum: i32) {
        self.nb_variables += 1;
        println!("  Interval Var     {}: {}..{}", id, minimum, maximum);
    }

    fn on_variable_values(&mut self, id: String, values: &[i32]) {
        self.nb_variables += 1;
        print!("  Values Variable   {}: ", id);
        for v in values {
            print!("{} ", v)
        }
        println!()
    }

    fn begin_variable_array(&mut self, name: String) {
        println!("An array of variables named {}", name);
    }

    fn end_variable_array(&mut self) {
        println!("Array of variables done");
    }

    // -- Contraintes ---------------------------------------------------------
    fn on_constraint_all_different_v1(&mut self, list: &[String]) {
        self.nb_constraints += 1;
        println!("  [AllDiff]  {:?}", list);
    }

    fn on_constraint_all_different_v2(&mut self, list: &[ExpressionTree]) {
        self.nb_constraints += 1;
        println!("  [AllDiff]  {:?}", list);
    }

    fn on_constraint_all_different_except(&mut self, list: &[String], except: &[i32]) {
        self.nb_constraints += 1;
        println!("  [AllDiff Except]  {:?} with except values: {:?}", list, except);
    }

    fn on_constraint_all_different_list(&mut self, _lists: &[Vec<String>]) {
        self.nb_constraints += 1;
        println!("  [AllDiff List]");
        for v in _lists {
            println!("       {:?} ", v);
        }
    }
    fn on_constraint_all_different_matrix(&mut self, _lists: &[Vec<String>]) {
        self.nb_constraints += 1;
        println!("  [AllDiff Matrix]");
        for v in _lists {
            println!("       {:?} ", v);
        }
    }
    fn on_constraint_all_equal_v1(&mut self, list: &[String]) {
        self.nb_constraints += 1;
        println!("  [AllEqual]  {:?}", list);
    }
    fn on_constraint_all_equal_v2(&mut self, list: &[ExpressionTree]) {
        self.nb_constraints += 1;
        println!("  [AllEqual]  {:?}", list);
    }

    fn on_constraint_ordered_v1(&mut self, list: &[String], operator: Operator) {
        self.nb_constraints += 1;
        println!("  [Ordered V1]  {:?}, operator {:?}", list, operator);
    }

    fn on_constraint_ordered_v2(&mut self, list: &[String], lengths: &[i32], operator: Operator) {
        self.nb_constraints += 1;
        println!("  [Ordered V2]  {:?}, length: {:?} operator {:?}", list, lengths, operator);
    }

    fn on_constraint_ordered_v3(&mut self, list: &[String], lengths: &[String], operator: Operator) {
        self.nb_constraints += 1;
        println!("  [Ordered V3]  {:?}, length: {:?} operator {:?}", list, lengths, operator);
    }
    fn on_constraint_extension(&mut self, list: &[String], tuples: &Vec<Vec<i32>>, is_support: bool, has_star: bool) {
        self.nb_constraints += 1;
        println!("  [Extension] list={:?} is_support={} has_star={}", list, is_support, has_star);
        if tuples.len() > 10 {
            println!("           too many tuples. nb tuples: {}", tuples.len());
        } else {
            println!("           tuples: {:?}", tuples);
        }
        println!("           address = {:X}", tuples.as_ptr() as usize);
    }
    fn on_constraint_unary(&mut self, _list: &String, values: &[i32], is_support: bool) {
        self.nb_constraints += 1;
        println!("  [Unary]  {:?} values: {:?} is_support: {}", _list, values, is_support);
    }

    fn on_constraint_intention(&mut self, list: &[String], tree: &ExpressionTree) {
        self.nb_constraints += 1;
        println!("  [Intent]   {} list={:?}", tree, list);
    }

    fn on_constraint_instantiation(&mut self, list: &[String], values: &[i32]) {
        self.nb_constraints += 1;
        println!("  [Instantiation]   {:?} = {:?}", list, values);
    }

    fn on_constraint_regular(
        &mut self,
        list: &[String],
        start: String,
        finals: &[String],
        transitions: &[(String, i32, String)],
    ) {
        self.nb_constraints += 1;
        println!("  [Regular]  {:?}. start={}, finals={:?}", list, start, finals);
        println!("           transitions: {:?}", transitions);
    }

    fn on_constraint_mdd(&mut self, list: &[String], transitions: &Vec<(String, i32, String)>) {
        self.nb_constraints += 1;
        println!("  [MDD]  {:?}", list);
        println!("           transitions: {:?}", transitions);
    }

    fn on_constraint_sum_v1(&mut self, list: &[String], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Sum]  {:?} {:?} {:?}", list, operator, operand);
    }

    fn on_constraint_sum_v2(&mut self, list: &[String], coeffs: &[i32], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Sum]  {:?} * {:?} {:?}, {:?}", list, coeffs, operator, operand);
    }

    fn on_constraint_sum_v3(&mut self, list: &[String], coeffs: &[String], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Sum]  {:?} * {:?} {:?}, {:?}", list, coeffs, operator, operand);
    }

    fn on_constraint_sum_v4(&mut self, list: &[ExpressionTree], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Sum]  {:?} {:?} {:?}", list, operator, operand);
    }

    fn on_constraint_sum_v5(&mut self, list: &[ExpressionTree], coeffs: &[i32], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Sum]  {:?} * {:?} {:?} {:?}", list, coeffs, operator, operand);
    }

    fn on_constraint_maximum_v1(&mut self, list: &[String], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Maximum]  {:?} {:?}, {:?}", list, operator, operand);
    }

    fn on_constraint_maximum_v2(&mut self, list: &[ExpressionTree], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Maximum]  {:?} {:?}, {:?}", list, operator, operand);
    }

    fn on_constraint_minimum_v1(&mut self, list: &[String], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Minimum]  {:?} {:?}, {:?}", list, operator, operand);
    }

    fn on_constraint_minimum_v2(&mut self, list: &[ExpressionTree], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Minimum]  {:?} {:?}, {:?}", list, operator, operand);
    }

    fn on_constraint_maximum_arg_v1(
        &mut self,
        list: &[String],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Maximum Arg]  {:?} rank={} start={} {:?}, {:?}", list, rank, start_index, operator, operand);
    }

    fn on_constraint_maximum_arg_v2(
        &mut self,
        list: &[ExpressionTree],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Maxnimum Arg]  {:?} rank={} start={} {:?}, {:?}", list, rank, start_index, operator, operand);
    }

    fn on_constraint_minimum_arg_v1(
        &mut self,
        list: &[String],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Minimum Arg]  {:?} rank={} start={} {:?}, {:?}", list, rank, start_index, operator, operand);
    }

    fn on_constraint_minimum_arg_v2(
        &mut self,
        list: &[ExpressionTree],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Minimum Arg]  {:?} rank={} start={} {:?}, {:?}", list, rank, start_index, operator, operand);
    }

    fn on_constraint_count_v1(
        &mut self,
        list: &[ExpressionTree],
        values: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Count]  {:?} - values: {:?} {:?} {:?}", list, values, operator, operand);
    }

    fn on_constraint_count_v2(&mut self, list: &[String], values: &[i32], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Count]  {:?} - values: {:?} {:?} {:?}", list, values, operator, operand);
    }

    fn on_constraint_count_v4(&mut self, list: &[String], values: &[String], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [Count]  {:?} - values: {:?} {:?} {:?}", list, values, operator, operand);
    }

    fn on_constraint_count_v3(
        &mut self,
        list: &[ExpressionTree],
        values: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Count]  {:?} - values: {:?} {:?} {:?}", list, values, operator, operand);
    }

    fn on_constraint_cumulative_v1(
        &mut self,
        origins: &[String],
        lengths: &[i32],
        heights: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V1] {:?} lengths={:?} heights={:?} {:?} {:?}",
            origins, lengths, heights, operator, operand
        );
    }

    fn on_constraint_cumulative_v2(
        &mut self,
        origins: &[String],
        lengths: &[i32],
        heights: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V2] {:?} lengths={:?} heights={:?} {:?} {:?}",
            origins, lengths, heights, operator, operand
        );
    }

    fn on_constraint_cumulative_v3(
        &mut self,
        origins: &[String],
        lengths: &[String],
        heights: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V3] {:?} lengths={:?} heights={:?} {:?} {:?}",
            origins, lengths, heights, operator, operand
        );
    }

    fn on_constraint_cumulative_v4(
        &mut self,
        origins: &[String],
        lengths: &[String],
        heights: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V4] {:?} lengths={:?} heights={:?} {:?} {:?}",
            origins, lengths, heights, operator, operand
        );
    }

    fn on_constraint_cumulative_v5(
        &mut self,
        origins: &[String],
        lengths: &[i32],
        heights: &[i32],
        ends: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V5] {:?} lengths={:?} heights={:?} ends={:?} {:?} {:?}",
            origins, lengths, heights, ends, operator, operand
        );
    }

    fn on_constraint_cumulative_v6(
        &mut self,
        origins: &[String],
        lengths: &[i32],
        heights: &[String],
        ends: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V6] {:?} lengths={:?} heights={:?} ends={:?} {:?} {:?}",
            origins, lengths, heights, ends, operator, operand
        );
    }

    fn on_constraint_cumulative_v7(
        &mut self,
        origins: &[String],
        lengths: &[String],
        heights: &[i32],
        ends: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V7] {:?} lengths={:?} heights={:?} ends={:?} {:?} {:?}",
            origins, lengths, heights, ends, operator, operand
        );
    }

    fn on_constraint_cumulative_v8(
        &mut self,
        origins: &[String],
        lengths: &[String],
        heights: &[String],
        ends: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Cumulative V8] {:?} lengths={:?} heights={:?} ends={:?} {:?} {:?}",
            origins, lengths, heights, ends, operator, operand
        );
    }

    fn on_constraint_nvalues_v1(&mut self, list: &[String], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [NValues]  {:?}  {:?} {:?}", list, operator, operand);
    }

    fn on_constraint_nvalues_v2(&mut self, list: &[String], except: &[i32], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [NValues]  {:?} (exception: {:?}) {:?} {:?}", list, except, operator, operand);
    }

    fn on_constraint_nvalues_v3(&mut self, list: &[ExpressionTree], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [NValues]  {:?}  {:?} {:?}", list, operator, operand);
    }

    fn on_constraint_element_v1(&mut self, list: &[String], value: i32) {
        self.nb_constraints += 1;
        println!("  [Element V1] {:?} value={}", list, value);
    }

    fn on_constraint_element_v2(&mut self, list: &[String], value: String) {
        self.nb_constraints += 1;
        println!("  [Element V2] {:?} value={}", list, value);
    }

    fn on_constraint_element_v3(&mut self, list: &[String], start_index: i32, index: String, value: String) {
        self.nb_constraints += 1;
        println!("  [Element V3] {:?} start={} index= {} value={}", list, start_index, index, value);
    }
    fn on_constraint_element_v4(&mut self, list: &[String], start_index: i32, index: String, value: i32) {
        self.nb_constraints += 1;
        println!("  [Element V4] {:?} start={} index= {} value={}", list, start_index, index, value);
    }

    fn on_constraint_element_v5(
        &mut self,
        list: &[String],
        start_index: i32,
        index: String,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element V5] {:?} start={} index= {}  operator={:?}, operand={:?}",
            list, start_index, index, operator, operand
        );
    }

    fn on_constraint_element_v6(&mut self, list: &[i32], start_index: i32, index: String, value: String) {
        self.nb_constraints += 1;
        println!("  [Element V6] {:?} start={} index= {} value={}", list, start_index, index, value);
    }
    fn on_constraint_element_v7(&mut self, list: &[i32], start_index: i32, index: String, value: i32) {
        self.nb_constraints += 1;
        println!("  [Element V7] {:?} start={} index= {} value={}", list, start_index, index, value);
    }

    fn on_constraint_element_v8(
        &mut self,
        list: &[i32],
        start_index: i32,
        index: String,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element V8] {:?} start={} index= {}  operator={:?}, operand={:?}",
            list, start_index, index, operator, operand
        );
    }

    fn on_constraint_element_matrix_v1(
        &mut self,
        matrix: &Vec<Vec<String>>,
        row_index: String,
        col_index: String,
        start_row_index: i32,
        start_col_index: i32,
        value: i32,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element Matrix V1] {:?} row={} col={} startRow={} startCol={} value={}",
            matrix, row_index, col_index, start_row_index, start_col_index, value
        );
    }

    fn on_constraint_element_matrix_v2(
        &mut self,
        matrix: &Vec<Vec<String>>,
        row_index: String,
        col_index: String,
        start_row_index: i32,
        start_col_index: i32,
        value: String,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element Matrix V2] {:?} row={} col={} startRow={} startCol={} value={}",
            matrix, row_index, col_index, start_row_index, start_col_index, value
        );
    }

    fn on_constraint_element_matrix_v3(
        &mut self,
        matrix: &Vec<Vec<String>>,
        row_index: String,
        col_index: String,
        start_row_index: i32,
        start_col_index: i32,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element Matrix V3] {:?} row={} col={} startRow={} startCol={} {:?} {:?}",
            matrix, row_index, col_index, start_row_index, start_col_index, operator, operand
        );
    }

    fn on_constraint_element_matrix_v4(
        &mut self,
        matrix: &Vec<Vec<i32>>,
        row_index: String,
        col_index: String,
        start_row_index: i32,
        start_col_index: i32,
        operator: Operator,
        operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element Matrix V4] {:?} row={} col={} startRow={} startCol={} {:?} {:?}",
            matrix, row_index, col_index, start_row_index, start_col_index, operator, operand
        );
    }

    fn on_constraint_element_matrix_v5(
        &mut self,
        matrix: &Vec<Vec<i32>>,
        row_index: String,
        col_index: String,
        start_row_index: i32,
        start_col_index: i32,
        value: i32,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element Matrix V5] {:?} row={} col={} startRow={} startCol={} value={}",
            matrix, row_index, col_index, start_row_index, start_col_index, value
        );
    }

    fn on_constraint_element_matrix_v6(
        &mut self,
        matrix: &Vec<Vec<i32>>,
        row_index: String,
        col_index: String,
        start_row_index: i32,
        start_col_index: i32,
        value: String,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Element Matrix V6] {:?} row={} col={} startRow={} startCol={} value={}",
            matrix, row_index, col_index, start_row_index, start_col_index, value
        );
    }

    fn on_constraint_no_overlap_v1(&mut self, list: &[String], lengths: &[i32], zero_ignored: bool) {
        self.nb_constraints += 1;
        println!("  [NoOverlap]  {:?}  lengths={:?} zero {}", list, lengths, zero_ignored);
    }

    fn on_constraint_no_overlap_v2(&mut self, list: &[String], lengths: &[String], zero_ignored: bool) {
        self.nb_constraints += 1;
        println!("  [NoOverlap]  {:?}  lengths={:?} zero {}", list, lengths, zero_ignored);
    }

    fn on_constraint_no_overlap_k_dim_v1(&mut self, origins: &Vec<Vec<String>>, lengths: &Vec<Vec<i32>>, zero: bool) {
        self.nb_constraints += 1;
        println!("  [NoOverlap]  {:?}  lengths={:?} zero {}", origins, lengths, zero);
    }

    fn on_constraint_no_overlap_k_dim_v2(
        &mut self,
        origins: &Vec<Vec<String>>,
        lengths: &Vec<Vec<String>>,
        zero: bool,
    ) {
        self.nb_constraints += 1;
        println!("  [NoOverlap]  {:?}  lengths={:?} zero {}", origins, lengths, zero);
    }

    fn on_constraint_no_overlap_k_dim_v3(
        &mut self,
        origins: &Vec<Vec<String>>,
        lengths: &Vec<(String, i32)>,
        zero: bool,
    ) {
        self.nb_constraints += 1;
        println!("  [NoOverlap]  {:?}  lengths={:?} zero {}", origins, lengths, zero);
    }

    fn on_constraint_cardinality_v1(&mut self, list: &[String], values: &[i32], occurs: &[i32], closed: bool) {
        self.nb_constraints += 1;
        println!("  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}", list, occurs, values, closed);
    }

    fn on_constraint_cardinality_v2(&mut self, list: &[String], values: &[i32], occurs: &[String], closed: bool) {
        self.nb_constraints += 1;
        println!("  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}", list, occurs, values, closed);
    }

    fn on_constraint_cardinality_v3(&mut self, list: &[String], values: &[i32], occurs: &[(i32, i32)], closed: bool) {
        self.nb_constraints += 1;
        println!("  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}", list, occurs, values, closed);
    }

    fn on_constraint_cardinality_v4(&mut self, list: &[String], values: &[String], occurs: &[i32], closed: bool) {
        self.nb_constraints += 1;
        println!("  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}", list, occurs, values, closed);
    }

    fn on_constraint_cardinality_v5(&mut self, list: &[String], values: &[String], occurs: &[String], closed: bool) {
        self.nb_constraints += 1;
        println!("  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}", list, occurs, values, closed);
    }

    fn on_constraint_cardinality_v6(
        &mut self,
        list: &[String],
        values: &[String],
        occurs: &[(i32, i32)],
        closed: bool,
    ) {
        self.nb_constraints += 1;
        println!("  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}", list, occurs, values, closed);
    }

    fn on_constraint_circuit_v1(&mut self, list: &Vec<String>) {
        self.nb_constraints += 1;
        println!("  [Circuit V1]  {:?}", list);
    }

    fn on_constraint_circuit_v2(&mut self, list: &Vec<String>, size: i32) {
        self.nb_constraints += 1;
        println!("  [Circuit V2]  {:?} size={}", list, size);
    }

    fn on_constraint_circuit_v3(&mut self, list: &Vec<String>, size: String) {
        self.nb_constraints += 1;
        println!("  [Circuit V3]  {:?} size={}", list, size);
    }

    fn on_constraint_precedence_v1(&mut self, list: &[String], covered: bool) {
        self.nb_constraints += 1;
        println!("  [Precedence V2]  {:?} covered={}", list, covered);
    }

    fn on_constraint_precedence_v2(&mut self, list: &[String], values: &[i32], covered: bool) {
        self.nb_constraints += 1;
        println!("  [Precedence V2]  {:?} covered={} values={:?}", list, covered, values);
    }

    fn on_constraint_channel_v1(&mut self, list: &[String], start_index: i32) {
        self.nb_constraints += 1;
        println!("  [Channel V1]  {:?} start_index={}", list, start_index);
    }

    fn on_constraint_channel_v2(
        &mut self,
        _list1: &[String],
        _start_index1: i32,
        _list2: &[String],
        _start_index2: i32,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Channel V2]\n      l1={:?} start_index1={}\n      l2={:?} start_index2={}",
            _list1, _start_index1, _list2, _start_index2
        );
    }

    fn on_constraint_channel_v3(&mut self, _list: &[String], _start_index: i32, _value: String) {
        self.nb_constraints += 1;
        println!("  [Channel V3]  {:?} start_index={}, _value={:?}", _list, _start_index, _value);
    }

    fn on_constraint_stretch_v1(&mut self, list: &[String], values: &[(i32, i32)], widths: &[i32]) {
        self.nb_constraints += 1;
        println!("  [Stretch V1]  {:?} values={:?} widths={:?}", list, values, widths);
    }

    fn on_constraint_stretch_v2(&mut self, list: &[String], values: &[(i32, i32)], widths: &[i32]) {
        self.nb_constraints += 1;
        println!("  [Stretch V2]  {:?} values={:?} widths={:?}", list, values, widths);
    }

    fn on_constraint_clause(&mut self, _positive: &[String], _negative: &[String]) {
        self.nb_constraints += 1;
        println!("  [Clause]  pos_lit {:?} neg lit {:?}", _positive, _negative);
    }

    fn on_constraint_flow(
        &mut self,
        _list: &[String],
        _balance: &[i32],
        _weights: &[i32],
        _arcs: &[(i32, i32)],
        _operator: Operator,
        _operand: Operand,
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Flow] list={:?} balance={:?} weights={:?} arcs={:?} operator={:?} operand={:?}",
            _list, _balance, _weights, _arcs, _operator, _operand
        );
    }

    fn on_constraint_knapsack(
        &mut self,
        _list: &[String],
        _weights: &[i32],
        _woperator: Operator,
        _woperand: Operand,
        _profits: &[i32],
        _poperator: Operator,
        _poperand: Operand,
    ) {
        self.nb_constraints += 1;
        println!("  [Knapsack]: {:?} weight: {:?} profits:{:?}", _list, _weights, _profits);
    }

    fn on_constraint_bin_packing_v1(&mut self, list: &[String], sizes: &[i32], operator: Operator, operand: Operand) {
        self.nb_constraints += 1;
        println!("  [BinPacking V1]  {:?} sizes={:?} {:?} {:?}", list, sizes, operator, operand);
    }

    fn on_constraint_bin_packing_v2(&mut self, list: &[String], sizes: &[i32], limits: &[i32]) {
        self.nb_constraints += 1;
        println!("  [BinPacking V2]  {:?} sizes={:?} limits={:?}", list, sizes, limits);
    }

    fn on_constraint_bin_packing_v3(&mut self, list: &[String], sizes: &[i32], limits: &[String]) {
        self.nb_constraints += 1;
        println!("  [BinPacking V3]  {:?} sizes={:?} limits={:?}", list, sizes, limits);
    }

    fn on_constraint_bin_packing_v4(&mut self, list: &[String], sizes: &[i32], loads: &[i32]) {
        self.nb_constraints += 1;
        println!("  [BinPacking V4]  {:?} sizes={:?} loads={:?}", list, sizes, loads);
    }

    fn on_constraint_bin_packing_v5(&mut self, list: &[String], sizes: &[i32], loads: &[String]) {
        self.nb_constraints += 1;
        println!("  [BinPacking V5]  {:?} sizes={:?} loads={:?}", list, sizes, loads);
    }

    fn on_constraint_lex(&mut self, lists: &Vec<Vec<String>>, operator: Operator) {
        self.nb_constraints += 1;
        println!("  [Lex] {:?} {:?}", lists, operator);
    }
    fn on_constraint_lex_matrix(&mut self, matrix: &Vec<Vec<String>>, operator: Operator) {
        self.nb_constraints += 1;
        println!("  [Lex Matrix] {:?} {:?}", matrix, operator);
    }

    // -- Objectifs -----------------------------------------------------------
    fn on_minimize_var(&mut self, var: String) {
        println!("Objectives: Minimize {:?}", var);
    }
    fn on_maximize_var(&mut self, var: String) {
        println!("Objectives: Maximize {:?}", var);
    }

    fn on_minimize_expression(&mut self, expr: &ExpressionTree) {
        println!("Objectives: Minimize {:?}", expr);
    }

    fn on_maximize_expression(&mut self, expr: &ExpressionTree) {
        println!("Objectives: Maximize {:?}", expr);
    }

    fn on_minimize_v1(&mut self, _type: XElementOperator, list: &[String], coefs: &[i32]) {
        println!("  [Minimize V1] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_maximize_v1(&mut self, _type: XElementOperator, list: &[String], coefs: &[i32]) {
        println!("  [Maximize V1] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_minimize_v2(&mut self, _type: XElementOperator, list: &[String], coefs: &[String]) {
        println!("  [Minimize V2] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_maximize_v2(&mut self, _type: XElementOperator, list: &[String], coefs: &[String]) {
        println!("  [Maximize V2] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_minimize_v3(&mut self, _type: XElementOperator, list: &[ExpressionTree], coefs: &[i32]) {
        println!("  [Minimize V3] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_maximize_v3(&mut self, _type: XElementOperator, list: &[ExpressionTree], coefs: &[i32]) {
        println!("  [Maximize V3] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_minimize_v4(&mut self, _type: XElementOperator, list: &[ExpressionTree], coefs: &[String]) {
        println!("  [Minimize V4] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_maximize_v4(&mut self, _type: XElementOperator, list: &[ExpressionTree], coefs: &[String]) {
        println!("  [Maximize V4] type={:?} {:?} coefs={:?}", _type, list, coefs);
    }

    fn on_minimize_v5(&mut self, _type: XElementOperator, list: &[String]) {
        println!("  [Minimize V5] type={:?} {:?}", _type, list);
    }

    fn on_maximize_v5(&mut self, _type: XElementOperator, list: &[String]) {
        println!("  [Maximize V5] type={:?} {:?}", _type, list);
    }

    fn on_minimize_v6(&mut self, _type: XElementOperator, list: &[ExpressionTree]) {
        println!("  [Minimize V6] type={:?} {:?}", _type, list);
    }

    fn on_maximize_v6(&mut self, _type: XElementOperator, list: &[ExpressionTree]) {
        println!("  [Maximize V6] type={:?} {:?}", _type, list);
    }
}
