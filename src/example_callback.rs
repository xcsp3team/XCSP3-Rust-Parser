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
        Self {
            nb_variables: 0,
            nb_constraints: 0,
        }
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

    // -- Cycle de vie --------------------------------------------------------
    fn begin_variables(&mut self) {
        println!("=== Variables ===");
    }
    fn end_variables(&mut self) {
        println!("→ {} variables chargées\n", self.nb_variables);
    }
    fn begin_constraints(&mut self) {
        println!("=== Contraintes ===");
    }
    fn end_constraints(&mut self) {
        println!("→ {} contraintes chargées\n", self.nb_constraints);
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
    fn on_constraint_all_different_v1(&mut self, scope: &[String]) {
        self.nb_constraints += 1;
        println!("  [AllDiff]  {:?}", scope);
    }

    fn on_constraint_all_different_v2(&mut self, scope: &[ExpressionTree]) {
        self.nb_constraints += 1;
        println!("  [AllDiff]  {:?}", scope);
    }

    fn on_constraint_all_different_except(&mut self, scope: &[String], except: &[i32]) {
        self.nb_constraints += 1;
        println!(
            "  [AllDiff Except]  {:?} with except values: {:?}",
            scope, except
        );
    }

    fn on_constraint_all_different_list(&mut self, _lists: &[Vec<String>]) {
        println!("  [AllDiff List]");
        for v in _lists {
            println!("       {:?} ", v);
        }
    }
    fn on_constraint_all_different_matrix(&mut self, _lists: &[Vec<String>]) {
        println!("  [AllDiff Matrix]");
        for v in _lists {
            println!("       {:?} ", v);
        }
    }
    fn on_constraint_all_equal_v1(&mut self, scope: &[String]) {
        self.nb_constraints += 1;
        println!("  [AllEqual]  {:?}", scope);
    }
    fn on_constraint_all_equal_v2(&mut self, scope: &[ExpressionTree]) {
        self.nb_constraints += 1;
        println!("  [AllEqual]  {:?}", scope);
    }

    fn on_constraint_ordered_v1(&mut self, scope: &[String], operator: Operator) {
        println!("  [Ordered]  {:?}, operator {:?}", scope, operator);
    }

    fn on_constraint_ordered_v2(&mut self, scope: &[String], lengths: &[i32], operator: Operator) {
        println!(
            "  [Ordered]  {:?}, length: {:?} operator {:?}",
            scope, lengths, operator
        );
    }

    fn on_constraint_extension(
        &mut self,
        scope: &[String],
        tuples: &Vec<Vec<i32>>,
        is_support: bool,
        has_star: bool,
    ) {
        println!(
            "  [Extension] scope={:?} is_support={} has_star={}",
            scope, is_support, has_star
        );
        println!("           tuples: {:?}", tuples);
        println!("           address = {:X}", tuples.as_ptr() as usize);
    }
    fn on_constraint_unary(&mut self, _scope: &String, values: &[i32], is_support: bool) {
        println!(
            "  [Unary]  {:?} values: {:?} is_support: {}",
            _scope, values, is_support
        );
    }

    fn on_constraint_intention(&mut self, scope: &[String], tree: &ExpressionTree) {
        self.nb_constraints += 1;
        println!("  [Intent]   {} scope={:?}", tree, scope);
    }

    fn on_constraint_instantiation(&mut self, scope: &[String], values: &[i32]) {
        self.nb_constraints += 1;
        println!("  [Instantiation]   {:?} = {:?}", scope, values);
    }

    fn on_constraint_regular(
        &mut self,
        scope: &[String],
        start: String,
        finals: &[String],
        transitions: &[(String, i32, String)],
    ) {
        self.nb_constraints += 1;
        println!(
            "  [Regular]  {:?}. start={}, finals={:?}",
            scope, start, finals
        );
        println!("           transitions: {:?}", transitions);
    }

    fn on_constraint_mdd(&mut self, scope: &[String], transitions: &Vec<(String, i32, String)>) {
        println!("  [MDD]  {:?}", scope);
        println!("           transitions: {:?}", transitions);
    }

    fn on_constraint_sum_v1(&mut self, scope: &[String], operator: Operator, operand: Operand) {
        println!("  [Sum]  {:?} {:?} {:?}", scope, operator, operand);
    }

    fn on_constraint_sum_v2(
        &mut self,
        scope: &[String],
        coeffs: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Sum]  {:?} * {:?} {:?}, {:?}",
            scope, coeffs, operator, operand
        );
    }

    fn on_constraint_sum_v3(
        &mut self,
        scope: &[String],
        coeffs: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Sum]  {:?} * {:?} {:?}, {:?}",
            scope, coeffs, operator, operand
        );
    }

    fn on_constraint_sum_v4(
        &mut self,
        scope: &[ExpressionTree],
        operator: Operator,
        operand: Operand,
    ) {
        println!("  [Sum]  {:?} {:?} {:?}", scope, operator, operand);
    }

    fn on_constraint_sum_v5(
        &mut self,
        scope: &[ExpressionTree],
        coeffs: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Sum]  {:?} * {:?} {:?} {:?}",
            scope, coeffs, operator, operand
        );
    }

    fn on_constraint_maximum_v1(&mut self, scope: &[String], operator: Operator, operand: Operand) {
        println!("  [Maximum]  {:?} {:?}, {:?}", scope, operator, operand);
    }

    fn on_constraint_maximum_v2(
        &mut self,
        scope: &[ExpressionTree],
        operator: Operator,
        operand: Operand,
    ) {
        println!("  [Maximum]  {:?} {:?}, {:?}", scope, operator, operand);
    }

    fn on_constraint_minimum_v1(&mut self, scope: &[String], operator: Operator, operand: Operand) {
        println!("  [Minimum]  {:?} {:?}, {:?}", scope, operator, operand);
    }

    fn on_constraint_minimum_v2(
        &mut self,
        scope: &[ExpressionTree],
        operator: Operator,
        operand: Operand,
    ) {
        println!("  [Minimum]  {:?} {:?}, {:?}", scope, operator, operand);
    }

    fn on_constraint_maximum_arg_v1(
        &mut self,
        scope: &[String],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Maximum Arg]  {:?} rank={} start={} {:?}, {:?}",
            scope, rank, start_index, operator, operand
        );
    }

    fn on_constraint_maximum_arg_v2(
        &mut self,
        scope: &[ExpressionTree],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Maxnimum Arg]  {:?} rank={} start={} {:?}, {:?}",
            scope, rank, start_index, operator, operand
        );
    }

    fn on_constraint_minimum_arg_v1(
        &mut self,
        scope: &[String],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Minimum Arg]  {:?} rank={} start={} {:?}, {:?}",
            scope, rank, start_index, operator, operand
        );
    }

    fn on_constraint_minimum_arg_v2(
        &mut self,
        scope: &[ExpressionTree],
        start_index: i32,
        rank: String,
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Minimum Arg]  {:?} rank={} start={} {:?}, {:?}",
            scope, rank, start_index, operator, operand
        );
    }

    fn on_constraint_count_v1(
        &mut self,
        scope: &[ExpressionTree],
        values: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Count]  {:?} - values: {:?} {:?} {:?}",
            scope, values, operator, operand
        );
    }

    fn on_constraint_count_v2(
        &mut self,
        scope: &[String],
        values: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Count]  {:?} - values: {:?} {:?} {:?}",
            scope, values, operator, operand
        );
    }

    fn on_constraint_count_v4(
        &mut self,
        scope: &[String],
        values: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Count]  {:?} - values: {:?} {:?} {:?}",
            scope, values, operator, operand
        );
    }

    fn on_constraint_count_v3(
        &mut self,
        scope: &[ExpressionTree],
        values: &[String],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [Count]  {:?} - values: {:?} {:?} {:?}",
            scope, values, operator, operand
        );
    }

    fn on_constraint_cumulative_v1(
        &mut self,
        origins: &[String],
        lengths: &[i32],
        heights: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
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
        println!(
            "  [Cumulative V5] {:?} lengths={:?} heights={:?} ends={:? } {:?} {:?}",
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
        println!(
            "  [Cumulative V6] {:?} lengths={:?} heights={:?} ends={:? } {:?} {:?}",
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
        println!(
            "  [Cumulative V7] {:?} lengths={:?} heights={:?} ends={:? } {:?} {:?}",
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
        println!(
            "  [Cumulative V8] {:?} lengths={:?} heights={:?} ends={:? } {:?} {:?}",
            origins, lengths, heights, ends, operator, operand
        );
    }

    fn on_constraint_element_v1(&mut self, scope: &[String], value: i32) {
        println!("  [Element V1] {:?} value={}", scope, value);
    }

    fn on_constraint_nvalues_v1(&mut self, scope: &[String], operator: Operator, operand: Operand) {
        println!("  [NValues]  {:?}  {:?} {:?}", scope, operator, operand);
    }

    fn on_constraint_element_v3(
        &mut self,
        list: &[String],
        start_index: i32,
        index: String,
        value: String,
    ) {
        println!(
            "  [Element V3] {:?} start={} index= {} value={}",
            list, start_index, index, value
        );
    }
    fn on_constraint_element_v4(
        &mut self,
        list: &[String],
        start_index: i32,
        index: String,
        value: i32,
    ) {
        println!(
            "  [Element V4] {:?} start={} index= {} value={}",
            list, start_index, index, value
        );
    }
    fn on_constraint_nvalues_v2(
        &mut self,
        scope: &[String],
        except: &[i32],
        operator: Operator,
        operand: Operand,
    ) {
        println!(
            "  [NValues]  {:?} (exception: {:?}) {:?} {:?}",
            scope, except, operator, operand
        );
    }

    fn on_constraint_nvalues_v3(
        &mut self,
        scope: &[ExpressionTree],
        operator: Operator,
        operand: Operand,
    ) {
        println!("  [NValues]  {:?}  {:?} {:?}", scope, operator, operand);
    }

    fn on_constraint_no_overlap_v1(
        &mut self,
        scope: &[String],
        lengths: &[i32],
        zero_ignored: bool,
    ) {
        println!(
            "  [NoOverlap]  {:?}  lengths={:?} zero {}",
            scope, lengths, zero_ignored
        );
    }

    fn on_constraint_no_overlap_v2(
        &mut self,
        scope: &[String],
        lengths: &[String],
        zero_ignored: bool,
    ) {
        println!(
            "  [NoOverlap]  {:?}  lengths={:?} zero {}",
            scope, lengths, zero_ignored
        );
    }

    fn on_constraint_no_overlap_k_dim_v1(
        &mut self,
        origins: &Vec<Vec<String>>,
        lengths: &Vec<Vec<i32>>,
        zero: bool,
    ) {
        println!(
            "  [NoOverlap]  {:?}  lengths={:?} zero {}",
            origins, lengths, zero
        );
    }

    fn on_constraint_no_overlap_k_dim_v2(
        &mut self,
        origins: &Vec<Vec<String>>,
        lengths: &Vec<Vec<String>>,
        zero: bool,
    ) {
        println!(
            "  [NoOverlap]  {:?}  lengths={:?} zero {}",
            origins, lengths, zero
        );
    }

    fn on_constraint_no_overlap_k_dim_v3(
        &mut self,
        origins: &Vec<Vec<String>>,
        lengths: &Vec<(String, i32)>,
        zero: bool,
    ) {
        println!(
            "  [NoOverlap]  {:?}  lengths={:?} zero {}",
            origins, lengths, zero
        );
    }

    fn on_constraint_cardinality_v1(
        &mut self,
        scope: &[String],
        values: &[i32],
        occurs: &[i32],
        closed: bool,
    ) {
        println!(
            "  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}",
            scope, occurs, values, closed
        );
    }

    fn on_constraint_cardinality_v2(
        &mut self,
        scope: &[String],
        values: &[i32],
        occurs: &[String],
        closed: bool,
    ) {
        println!(
            "  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}",
            scope, occurs, values, closed
        );
    }

    fn on_constraint_cardinality_v3(
        &mut self,
        scope: &[String],
        values: &[i32],
        occurs: &[(i32, i32)],
        closed: bool,
    ) {
        println!(
            "  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}",
            scope, occurs, values, closed
        );
    }

    fn on_constraint_cardinality_v4(
        &mut self,
        scope: &[String],
        values: &[String],
        occurs: &[i32],
        closed: bool,
    ) {
        println!(
            "  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}",
            scope, occurs, values, closed
        );
    }

    fn on_constraint_cardinality_v5(
        &mut self,
        scope: &[String],
        values: &[String],
        occurs: &[String],
        closed: bool,
    ) {
        println!(
            "  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}",
            scope, occurs, values, closed
        );
    }

    fn on_constraint_cardinality_v6(
        &mut self,
        scope: &[String],
        values: &[String],
        occurs: &[(i32, i32)],
        closed: bool,
    ) {
        println!(
            "  [Cardinality]  {:?}  occurs={:?} values={:?} closed {}",
            scope, occurs, values, closed
        );
    }

    fn on_constraint_circuit_v1(&mut self, scope: &Vec<String>) {
        println!("  [Circuit]  {:?}", scope);
    }

    fn on_constraint_circuit_v2(&mut self, scope: &Vec<String>, size: i32) {
        println!("  [Circuit]  {:?} size={}", scope, size);
    }

    fn on_constraint_circuit_v3(&mut self, scope: &Vec<String>, size: String) {
        println!("  [Circuit]  {:?} size={}", scope, size);
    }

    fn on_constraint_precedence_v1(&mut self, scope: &[String], covered: bool) {
        println!("  [Precedence]  {:?} covered={}", scope, covered);
    }

    fn on_constraint_precedence_v2(&mut self, scope: &[String], values: &[i32], covered: bool) {
        println!(
            "  [Precedence]  {:?} covered={} values={:?}",
            scope, covered, values
        );
    }

    fn on_constraint_channel_v1(&mut self, scope: &[String], start_index: i32) {
        println!("  [Channel]  {:?} start_index={}", scope, start_index);
    }

    fn on_constraint_channel_v2(
        &mut self,
        _list1: &[String],
        _start_index1: i32,
        _list2: &[String],
        _start_index2: i32,
    ) {
        println!(
            "  [Channel]\n      l1={:?} start_index1={}\n      l2={:?} start_index2={}",
            _list1, _start_index1, _list2, _start_index2
        );
    }

    fn on_constraint_channel_v3(&mut self, _list: &[String], _start_index: i32, _value: String) {
        println!(
            "  [Channel]  {:?} start_index={}, _value={:?}",
            _list, _start_index, _value
        );
    }

    fn on_constraint_clause(&mut self, _positive: &[String], _negative: &[String]) {
        println!(
            "  [Clause]  pos_lit {:?} neg lit {:?}",
            _positive, _negative
        );
    }

    fn on_constraint_knapsack(
        &mut self,
        _scope: &[String],
        _weights: &[i32],
        _woperator: Operator,
        _woperand: Operand,
        _profits: &[i32],
        _poperator: Operator,
        _poperand: Operand,
    ) {
        println!(
            "  [Knapsack]: {:?} weight: {:?} profits:{:?}",
            _scope, _weights, _profits
        );
    }

    fn on_constraint_lex(&mut self, lists: &Vec<Vec<String>>, operator: Operator) {
        println!("  [Lex] {:?} {:?}", lists, operator);
    }
    fn on_constraint_lex_matrix(&mut self, matrix: &Vec<Vec<String>>, operator: Operator) {
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
}
