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
    use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
    use crate::constraints::xall_different_except::xcsp3_core::XAllDifferentExcept;
    use crate::constraints::xall_different_list::xcsp3_core::XAllDifferentList;
    use crate::constraints::xall_different_matrix::xcsp3_core::XAllDifferentMatrix;
    use crate::constraints::xall_equal::xcsp3_core::XAllEqual;
    use crate::constraints::xbinpacking::xcsp3_core::XBinpacking;
    use crate::constraints::xcardinality::xcsp3_core::XCardinality;
    use crate::constraints::xchannel::xcsp3_core::XChannel;
    use crate::constraints::xcircuit::xcsp3_core::XCircuit;
    use crate::constraints::xclause::xcsp3_core::XClause;
    use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
    use crate::constraints::xcount::xcsp3_core::XCount;
    use crate::constraints::xcumulative::xcsp3_core::XCumulative;
    use crate::constraints::xelement::xcsp3_core::XElement;
    use crate::constraints::xelement_matrix::xcsp3_core::XElementMatrix;
    use crate::constraints::xextension::xcsp3_core::XExtension;
    use crate::constraints::xgroup::xcsp3_core::XGroup;
    use crate::constraints::xinstantiation::xcsp3_core::XInstantiation;
    use crate::constraints::xintension::xcsp3_core::XIntention;
    use crate::constraints::xknapsack::xcsp3_core::XKnapsack;
    use crate::constraints::xlex::xcsp3_core::XLex;
    use crate::constraints::xlex_matrix::xcsp3_core::XLexMatrix;
    use crate::constraints::xmax_min::xcsp3_core::XMaxMin;
    use crate::constraints::xmax_min_arg::xcsp3_core::XMaxMinArg;
    use crate::constraints::xmdd::xcsp3_core::XMdd;
    use crate::constraints::xn_values::xcsp3_core::XNValues;
    use crate::constraints::xno_overlap::xcsp3_core::XNoOverlap;
    use crate::constraints::xno_overlap_k_dimensional::xcsp3_core::XNoOverlapKDim;
    use crate::constraints::xordered::xcsp3_core::XOrdered;
    use crate::constraints::xprecedence::xcsp3_core::XPrecedence;
    use crate::constraints::xregular::xcsp3_core::XRegular;
    use crate::constraints::xslide::xcsp3_core::XSlide;
    use crate::constraints::xstretch::xcsp3_core::XStretch;
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::slice::{Iter, IterMut};

    /**
    the XConstraintSet is a container that stores all constraints.
     */
    pub struct XConstraintSet<'a> {
        constraints: Vec<XConstraintType<'a>>,
        set: &'a XVariableSet,
    }

    impl<'a> XConstraintSet<'a> {
        pub fn new(set: &'a XVariableSet) -> XConstraintSet<'a> {
            XConstraintSet {
                constraints: vec![],
                set,
            }
        }

        pub fn build_no_overlap_k_dim(
            &mut self,
            list: &str,
            lengths_str: &str,
            zero_ignored_str: &str,
        ) {
            let c = XNoOverlapKDim::from_str(list, lengths_str, zero_ignored_str, self.set);
            self.constraints.push(XConstraintType::XNoOverlapKDim(c));
        }

        pub fn build_no_overlap(&mut self, list: &str, lengths_str: &str, zero_ignored_str: &str) {
            let c = XNoOverlap::from_str(list, lengths_str, zero_ignored_str, self.set);
            self.constraints.push(XConstraintType::XNoOverlap(c));
        }
        pub fn build_bin_packing(
            &mut self,
            list: &str,
            sizes: &str,
            condition: &str,
            limits: &str,
            loads: &str,
        ) {
            let c = XBinpacking::from_str(list, sizes, condition, limits, loads, self.set);
            self.constraints.push(XConstraintType::XBinpacking(c));
        }
        pub fn build_cumulative(
            &mut self,
            origins_str: &str,
            lengths_str: &str,
            heights_str: &str,
            condition_str: &str,
            ends_str: &str,
            machines_str: &str,
            start_index_str: &str,
        ) {
            let c = XCumulative::from_str(
                origins_str,
                lengths_str,
                heights_str,
                condition_str,
                ends_str,
                machines_str,
                start_index_str,
                self.set,
            );
            self.constraints.push(XConstraintType::XCumulative(c));
        }

        pub fn build_lex(&mut self, lists: &[String], operator: &str) {
            let c = XLex::from_str(lists, operator, self.set);
            self.constraints.push(XConstraintType::XLex(c));
        }
        pub fn build_lex_matrix(&mut self, matrix: &str, operator: &str) {
            let c = XLexMatrix::from_str(matrix, operator, self.set);
            self.constraints.push(XConstraintType::XLexMatrix(c));
        }
        pub fn build_channel(
            &mut self,
            list1: &str,
            start_index1: &str,
            list2: &str,
            start_index2: &str,
            value_str: &str,
        ) {
            let c = XChannel::from_str(
                list1,
                start_index1,
                list2,
                start_index2,
                value_str,
                self.set,
            );
            self.constraints.push(XConstraintType::XChannel(c));
        }
        pub fn build_cardinality(
            &mut self,
            list: &str,
            values_str: &str,
            occurs_str: &str,
            closed_str: &str,
        ) {
            let c = XCardinality::from_str(list, values_str, occurs_str, closed_str, self.set);
            self.constraints.push(XConstraintType::XCardinality(c));
        }

        pub fn build_stretch(
            &mut self,
            list: &str,
            value_str: &str,
            widths_str: &str,
            patterns_str: &str,
        ) {
            let c = XStretch::from_str(list, value_str, widths_str, patterns_str, self.set);
            self.constraints.push(XConstraintType::XStretch(c));
        }
        pub fn build_element(
            &mut self,
            vars: &str,
            values_str: &str,
            index_str: &str,
            start_index_str: &str,
            condition: &str,
        ) {
            let c = XElement::from_str(
                vars,
                values_str,
                index_str,
                start_index_str,
                condition,
                self.set,
            );
            self.constraints.push(XConstraintType::XElement(c));
        }

        pub fn build_element_matrix(
            &mut self,
            matrix: &str,
            values_str: &str,
            index_str: &str,
            row_index_str: &str,
            col_index_str: &str,
            condition: &str,
        ) {
            let c = XElementMatrix::from_str(
                matrix,
                values_str,
                index_str,
                row_index_str,
                col_index_str,
                condition,
                self.set,
            );
            self.constraints.push(XConstraintType::XElementMatrix(c));
        }
        pub fn build_slide(
            &mut self,
            cc: XConstraintType<'a>,
            vars: &str,
            offset_str: &str,
            circular_str: &str,
            collect_str: &str,
        ) {
            let c = XSlide::from_str(cc, vars, offset_str, circular_str, collect_str, self.set);
            self.constraints.push(XConstraintType::XSlide(c));
        }
        pub fn build_group(&mut self, cc: XConstraintType<'a>, args: &[String]) {
            let c = XGroup::from_str(cc, args, self.set);
            self.constraints.push(XConstraintType::XGroup(c));
        }

        /// this function is designed for XGroup, parse the template for XGroup
        pub fn get_last_constraint(&mut self) -> Option<XConstraintType<'a>> {
            self.constraints.pop()
        }

        pub fn iter(&self) -> Iter<'_, XConstraintType<'_>> {
            self.constraints.iter()
        }

        pub fn iter_mut(&mut self) -> IterMut<'_, XConstraintType<'a>> {
            self.constraints.iter_mut()
        }

        pub fn build_minimum(&mut self, vars: &str, condition: &str) {
            let c = XMaxMin::from_str(vars, condition, true, self.set);
            self.constraints.push(XConstraintType::XMinimum(c));
        }
        pub fn build_maximum(&mut self, vars: &str, condition: &str) {
            let c = XMaxMin::from_str(vars, condition, false, self.set);
            self.constraints.push(XConstraintType::XMaximum(c));
        }
        pub fn build_maximum_arg(
            &mut self,
            vars: &str,
            rank: &str,
            start_index: i32,
            condition: &str,
        ) {
            let c = XMaxMinArg::from_str(vars, condition, rank, start_index, true, self.set);
            self.constraints.push(XConstraintType::XMaximumArg(c));
        }

        pub fn build_minimum_arg(
            &mut self,
            vars: &str,
            rank: &str,
            start_index: i32,
            condition: &str,
        ) {
            let c = XMaxMinArg::from_str(vars, condition, rank, start_index, false, self.set);
            self.constraints.push(XConstraintType::XMinimumArg(c));
        }

        pub fn build_count(&mut self, vars: &str, condition: &str, coeffs: &str) {
            let c = XCount::from_str(vars, condition, coeffs, self.set);
            self.constraints.push(XConstraintType::XCount(c));
        }

        pub fn build_n_values(&mut self, vars: &str, condition: &str, coeffs: &str) {
            let c = XNValues::from_str(vars, condition, coeffs, self.set);
            self.constraints.push(XConstraintType::XNValues(c));
        }

        pub fn build_sum(&mut self, vars: &str, condition: &str, coeffs: &str) {
            let c = XSum::from_str(vars, condition, coeffs, self.set);
            self.constraints.push(XConstraintType::XSum(c));
        }

        pub fn build_intention(&mut self, function: &str) {
            let c = XIntention::create(function, self.set);
            self.constraints.push(XConstraintType::XIntention(c));
        }

        pub fn build_regular(
            &mut self,
            list: &str,
            transitions_str: &str,
            start_str: &str,
            final_str: &str,
        ) {
            let c = XRegular::from_str(list, transitions_str, start_str, final_str, self.set);
            self.constraints.push(XConstraintType::XRegular(c));
        }

        pub fn build_mdd(&mut self, list: &str, transitions_str: &str) {
            let c = XMdd::from_str(list, transitions_str, self.set);
            self.constraints.push(XConstraintType::XMdd(c));
        }

        pub fn build_ordered(&mut self, list: &str, lengths_str: &str, operator: &str) {
            if lengths_str.is_empty() {
                let c = XOrdered::from_str_without_lengths(list, operator, self.set);
                self.constraints.push(XConstraintType::XOrdered(c));
            } else {
                let c = XOrdered::from_str(list, lengths_str, operator, self.set);
                self.constraints.push(XConstraintType::XOrdered(c));
            }
        }

        pub fn build_instantiation(&mut self, list: &str, values: &str) {
            let c = XInstantiation::from_str(list, values, self.set);
            self.constraints.push(XConstraintType::XInstantiation(c));
        }

        pub fn build_extension(&mut self, list: &str, tuple: &str, is_support: bool) {
            let c = XExtension::from_str(list, tuple, is_support, self.set);
            self.constraints.push(XConstraintType::XExtension(c));
        }

        pub fn build_all_equal(&mut self, list: &str) {
            let c = XAllEqual::from_str(list, self.set);
            self.constraints.push(XConstraintType::XAllEqual(c));
        }

        pub fn build_all_different(&mut self, list: &str) {
            let c = XAllDifferent::from_str(list, self.set);
            self.constraints.push(XConstraintType::XAllDifferent(c));
        }

        pub fn build_all_different_except(&mut self, list: &str, except: &str) {
            let c = XAllDifferentExcept::from_str(list, except, self.set);
            self.constraints
                .push(XConstraintType::XAllDifferentExcept(c));
        }
        pub fn build_all_different_list(&mut self, lists: &[String]) {
            let c = XAllDifferentList::from_str(lists, self.set);
            self.constraints.push(XConstraintType::XAllDifferentList(c));
        }
        pub fn build_all_different_matrix(&mut self, list: &str) {
            let c = XAllDifferentMatrix::from_str(list, self.set);
            self.constraints
                .push(XConstraintType::XAllDifferentMatrix(c));
        }

        pub fn build_circuit(&mut self, list: &str, size: &str) {
            let c = XCircuit::from_str(list, size, self.set);
            self.constraints.push(XConstraintType::XCircuit(c));
        }
        pub fn build_clause(&mut self, value: &str) {
            let c = XClause::from_str(value, self.set);
            self.constraints.push(XConstraintType::XClause(c));
        }
        pub fn build_knapsack(
            &mut self,
            list: &str,
            weights: &str,
            profits: &str,
            conditions: &Box<[String]>,
        ) {
            let c = XKnapsack::from_str(list, weights, profits, conditions, self.set);
            self.constraints.push(XConstraintType::XKnapsack(c));
        }
        pub fn build_precedence(&mut self, list: &str, values: &str, covered: bool) {
            let c = XPrecedence::from_str(list, values, Option::from(covered), self.set);
            self.constraints.push(XConstraintType::XPrecedence(c));
        }
    }
}
