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
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::InstanceType;

pub trait XcspCallback {
    /**
     * Start to parse a XCSP instance.
     * Related to tag <instance type="CSP/COP">
     * See http://xcsp.org/specifications/skeleton
     * @param type COP or CSP
     */
    fn begin_instance(&mut self, _type: &InstanceType) {}

    /**
     * End of parsing
     * Related to tag </instance>
     * See http://xcsp.org/specifications/skeleton
     */
    fn end_instance(&mut self) {}

    /**
     * Start to parse variables
     * Related to tag <variables>
     * See http://xcsp.org/specifications/skeleton
     */
    fn begin_variables(&mut self) {}

    /**
     * The end of parsing variables
     * Related to tag </variables>
     * See http://xcsp.org/specifications/skeleton
     */
    fn end_variables(&mut self) {}

    /**
     * Start to parse constraints
     * Related to tag <constraints>
     * See http://xcsp.org/specifications/skeleton
     */
    fn begin_constraints(&mut self) {}

    /**
     * The end of parsing constraints
     * Related to tag </constraints>
     * See http://xcsp.org/specifications/skeleton
     */
    fn end_constraints(&mut self) {}

    /**
     * Start to parse a group of constraints
     * Related to tag <group>
     * See http://xcsp.org/specifications/groups
     */

    fn begin_group(&mut self) {}

    /**
     * The end to parse a group of constraints
     * Related to tag <group>
     * See http://xcsp.org/specifications/groups
     */

    fn end_group(&mut self) {}

    /**
     * Start to parse a slide of constraints
     * Related to tag <group>
     * See http://xcsp.org/specifications/groups
     */

    fn begin_slide(&mut self) {}

    /**
     * The end to parse a slide of constraints
     * Related to tag <group>
     * See http://xcsp.org/specifications/groups
     */

    fn end_slide(&mut self) {}

    /**
     * Start to parse objectives
     * Related to tag <objectives>
     * See http://xcsp.org/specifications/objectives
     */

    fn begin_objectives(&mut self) {}

    /**
     * The end of parsing objectives
     * Related to tag </objectives>
     * See http://xcsp.org/specifications/objectives
     */
    fn end_objectives(&mut self) {}

    // -------------------------------------------------------------------------
    // Variables
    // -------------------------------------------------------------------------

    /**
     * The callback function related to an integer variable with a range domain
     * See http://xcsp.org/specifications/integers
     *
     * Example: &lt;var id="bar"> 0..6 </var>
     *
     * @param id the id (name) of the group
     * @param minimum the minimum value in the range
     * @param maximum the maxnimum value in the range
     */
    fn on_variable_interval(&mut self, _id: String, _minimum: i32, _maximum: i32) {
        panic!("You must implement callbacks for variables");
    }

    /**
     * The callback function related to an integer variable with a domain consisting in a sequence of integers
     * See http://xcsp.org/specifications/integers
     *
     * Example <var id="bar"> 1 3 5 10 </var>
     *
     * @param id the id (name) of the group
     * @param values the set of values in the domain
     */
    fn on_variable_values(&mut self, _id: String, _values: &[i32]) {
        panic!("You must implement callbacks for variables");
    }
    /**
     * Start to parse an array of variables
     * Related to tag <array>
     * See http://xcsp.org/specifications/arrays
     * Note that for each variable in the array a call is done to one of the functions #buildVariableInteger
     *
     * @param id the id (name) of the array variable
     */
    fn begin_variable_array(&mut self, _name: String) {}

    /**
     * End of parsing an array of variables
     * Related to tag </array>
     * See http://xcsp.org/specifications/arrays
     */
    fn end_variable_array(&mut self) {}

    // -------------------------------------------------------------------------
    // Contraintes
    // -------------------------------------------------------------------------

    /**
     * The callback function related to an constraint in extension
     * See http://xcsp.org/specifications/extension
     *
     * Example:
     * <extension>
     *   <list> y1 y2 y3 y4 </list>
     *   <conflicts> (1,2,3,4)(3,1,3,4) </conflicts>
     * </extension>
     *
     * @param list the list of the constraint
     * @param tuples the set of tuples in the constraint
     * @param support  support or conflicts?
     * @param hasStar is the tuples contain star values?
     */
    fn on_constraint_extension(
        &mut self,
        _list: &[String],
        _tuples: &Vec<Vec<i32>>,
        _is_support: bool,
        _has_star: bool,
    ) {
        println!("c Extension not yet implemented");
        println!("WARNING: tuples are not checked wrt domains");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_unary(&mut self, _list: &String, _values: &[i32], _is_support: bool) {
        println!("c Unary not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a alldifferent constraint.
     * See http://xcsp.org/specifications/alldifferent
     *
     * Example:
     * &lt;allDifferent>
     *   x1 x2 x3 x4 x5
     * &lt;/allDifferent>
     *
     * @param list the list of the constraint
     */
    fn on_constraint_all_different_v1(&mut self, _list: &[String]) {
        println!("c Alldifferent Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a alldifferent constraint with expressions
     * See http://xcsp.org/specifications/alldifferent
     *
     * Example:
     * &lt;allDifferent>
     *   add(q[0],0) add(q[1],1) add(q[2],2) add(q[3],3) add(q[4],4) add(q[5],5) add(q[6],6) add(q[7],7)
     * &lt;/allDifferent>
     *
     * @param list the trees of the constraint
     */
    fn on_constraint_all_different_v2(&mut self, _list: &[ExpressionTree]) {
        println!("c Alldifferent Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a alldifferent with some excepted values constraint
     * See http://xcsp.org/specifications/alldifferent
     *
     * Example:
     * &lt;allDifferent>
     *   x1 x2 x3 x4 x5
     *   &lt;except>0</except>
     * &lt;/allDifferent>
     *
     * @param list the list of the constraint
     * @param except the set of excepted values
     */
    fn on_constraint_all_different_except(&mut self, _list: &[String], _except: &[i32]) {
        println!("c Alldifferent except not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a alldifferent  list constraint
     * See http://xcsp.org/specifications/alldifferent
     *
     * Example:
     * <allDifferent id="c1">
     *    <list> x1 x2 x3 x4 </list>
     *    <list> y1 y2 y3 y4 </list>
     * </allDifferent>
     *
     * @param lists the set of lists (not the list, a variable may appear at different place!)
     */
    fn on_constraint_all_different_list(&mut self, _lists: &[Vec<String>]) {
        println!("c Alldifferent lists not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a alldifferent  matrix constraint
     * See http://xcsp.org/specifications/alldifferent
     *
     * Example:
     * <allDifferent id="c1">
     *    <matrix>
     *     (x1,x2,x3,x4,x5)
     *     (y1,y2,y3,y4,y5)
     *     (z1,z2,z3,z4,z5)
     *    </matrix>
     * </allDifferent>
     *
     * @param matrix the matrix (not the list, a variable may appear at different place!)
     */
    fn on_constraint_all_different_matrix(&mut self, _lists: &[Vec<String>]) {
        println!("c Alldifferent matrix not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a allequal constraint
     * See http://xcsp.org/specifications/allEqual
     *
     * Example:
     * &lt;allEqual>
     *  x1 x2 x3 x4 x5
     * &lt;/allEqual>
     *
     * @param list the list of the constraint
     *
     */
    fn on_constraint_all_equal_v1(&mut self, _list: &[String]) {
        println!("c AllEqual Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a allEqual constraint with expressions
     * See http://xcsp.org/specifications/allEqual
     *
     * Example:
     * &lt;allEqual>
     *   add(q[0],0) add(q[1],1) add(q[2],2) add(q[3],3) add(q[4],4) add(q[5],5) add(q[6],6) add(q[7],7)
     * &lt;/allEqual>
     *
     * @param list the trees of the constraint
     */
    fn on_constraint_all_equal_v2(&mut self, _list: &[ExpressionTree]) {
        println!("c AllEqual Variant 2 with expressions in list not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a constraint in intension
     * See http://xcsp.org/specifications/intension
     * Example:
     * &lt;intension> eq(add(x,y),z) &lt;/intension>
     *
     * @param id the id (name) of the constraint
     * @param tree the canonized form related to the tree
     */
    fn on_constraint_intention(&mut self, _list: &[String], _tree: &ExpressionTree) {
        println!("c Intension not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a sum constraint with all coefs are equal to one
     * See http://xcsp.org/specifications/sum
     *
     * Example:
     * &lt;sum>
     *   &lt;list> x1 x2 x3 &lt;/list>
     *   &lt;condition> (gt,y) &lt;/condition>
     * &lt;/sum>
     *
     * @param list the list of the constraint
     * @param operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v1(&mut self, _list: &[String], _operator: Operator, _operand: Operand) {
        println!("c Sum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a sum constraint with all coefs are equal to one
     * See http://xcsp.org/specifications/sum
     *
     * Example:
     * &lt;sum>
     *   &lt;list> x1 x2 x3 &lt;/list>
     *   &lt;coeffs>1 3 5 &lt;coeffs>
     *   &lt;condition> (gt,y) &lt;/condition>
     * &lt;/sum>
     *
     * @param list the list of the constraint
     * @param coeefs the coefficient of the sum (int)
     * @param Operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v2(&mut self, _list: &[String], _coeffs: &[i32], _operator: Operator, _operand: Operand) {
        println!("c Sum Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a sum constraint with all coefs are equal to one
     * See http://xcsp.org/specifications/sum
     *
     * Example:
     * &lt;sum>
     *   &lt;list> x1 x2 x3 &lt;/list>
     *   &lt;coeffs>w z t &lt;coeffs>
     *   &lt;condition> (gt,y) &lt;/condition>
     * &lt;/sum>
     *
     * @param list the list of the constraint
     * @param coeefs the coefficient of the sum (variables)
     * @param tperator the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v3(&mut self, _list: &[String], _coeffs: &[String], _operator: Operator, _operand: Operand) {
        println!("c Sum Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a sum constraint with expressions in list and weights
     *
     * Example:
     * &lt;sum>
     *   &lt;list>or(eq(x[5],0),eq(x[7],0)) or(eq(x[1],0),eq(x[2],0),eq(x[8],0)) or(eq(x[0],0),eq(x[3],0),eq(x[4],0),eq(x[6],0),eq(x[9],0))</list>
     *   &lt;coeff>1 2 3 &lt;coeff>
     *   &lt;condition> (gt,y) </condition>
     * &lt;/sum>
     *
     * @param _list the different trees
     * @param coeefs the coefficient of the sum (variables)
     * @param _operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v5(
        &mut self,
        _list: &[ExpressionTree],
        _coeffs: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Sum Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a sum constraint with expressions in list
     *
     * Example:
     * &lt;sum>
     *   &lt;list>or(eq(x[5],0),eq(x[7],0)) or(eq(x[1],0),eq(x[2],0),eq(x[8],0)) or(eq(x[0],0),eq(x[3],0),eq(x[4],0),eq(x[6],0),eq(x[9],0))</list>
     *   &lt;condition> (gt,y) </condition>
     * &lt;/sum>
     *
     * @param _list the different trees
     * @param Operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v4(&mut self, _list: &[ExpressionTree], _operator: Operator, _operand: Operand) {
        println!("c Sum Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an ordered constraint
     * See http://xcsp.org/specifications/ordered
     *
     * Ordered is Le, Lt, Ge, Gt...
     *
     * Example:
     * &lt;ordered>
     *   &lt;list> x1 x2 x3 x4 </list>
     *   &lt;operator> lt </operator>
     * &lt;/ordered>
     *
     * @param list the list of the constraint
     * @param operator the order Lt, Le...
     */
    fn on_constraint_ordered_v1(&mut self, _list: &[String], _operator: Operator) {
        println!("c Ordered Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to an ordered constraint with lenths int
     * See http://xcsp.org/specifications/ordered
     *
     * Ordered is Lt, Le, Gt, Ge...
     *
     * Example:
     * &lt;ordered>
     *   &lt;list> x1 x2 x3 x4 </list>
     *   <&lt;operator> lt </operator>
     * &lt;/ordered>
     *
     * @param _list the list of the constraint
     * @param _lengths the lengths

    * @param order the order Lt, Le...
     */
    fn on_constraint_ordered_v2(&mut self, _list: &[String], _lengths: &[i32], _operator: Operator) {
        println!("c Ordered Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an ordered constraint with lengths var
     * See http://xcsp.org/specifications/ordered
     *
     * Ordered is Lt, Le, Gt, Ge...
     *
     * Example:
     * &lt;ordered>
     *   &lt;list> x1 x2 x3 x4 </list>
     *   <&lt;operator> lt </operator>
     * &lt;/ordered>
     *
     * @param _list the list of the constraint
     * @param _lengths the lengths

    * @param order the order Lt, Le...
    */
    fn on_constraint_ordered_v3(&mut self, _list: &[String], _lengths: &[String], _operator: Operator) {
        println!("c Ordered Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a regular constraint.
     * See http://xcsp.org/specifications/regular
     * Example:
     * &lt;regular>
     *  &lt;list> x1 x2 x3 x4 x5 x6 x7 </list>
     *  &lt;transitions>
     *    (a,0,a)(a,1,b)(b,1,c)(c,0,d)(d,0,d)(d,1,e)(e,0,e)
     *  &lt;/transitions>
     *  &lt;start> a </start>
     * </&lt;regular>
     * XTransition is an object with 3 fields: from (string), val(int) and to(string)
     * Then, in the first transition of the example from=a, to=a and val=0
     *
     * @param _list the list of the constraint
     * @param _start the starting node
     * @param _final the set of final nodes
     * @param _transitions the set of transitions
     */
    fn on_constraint_regular(
        &mut self,
        _list: &[String],
        _start: String,
        _finals: &[String],
        _transitions: &[(String, i32, String)],
    ) {
        println!("c Regular not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a MDD constraint.
     * See http://xcsp.org/specifications/mdd
     *
     * Example:
     * &lt;mdd>
     *   &lt;list> x1 x2 x3 </list>
     *   &lt;transitions>
     *     (r,0,n1)(r,1,n2)(r,2,n3)
     *     (n1,2,n4)(n2,2,n4)(n3,0,n5)
     *     (n4,0,t)(n5,0,t)
     *   &lt;/transitions>
     * &lt;/mdd>
     *
     * @param list the list of the constraint
     * @param transitions the set of transitions
     */
    fn on_constraint_mdd(&mut self, _list: &[String], _transitions: &Vec<(String, i32, String)>) {
        println!("c MDD not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an instantiation  constraint
     * See http://xcsp.org/specifications/instantiation
     *
     * Example:
     * &lt;instantiation>
     *   &lt;list> x y z </list>
     *   &lt;values> 12 4 30 </values>
     * </instantiation>
     *
     * @param _list the list of the constraint
     * @param _values the value for each variable
     */
    fn on_constraint_instantiation(&mut self, _list: &[String], _values: &[i32]) {
        println!("c Instantiation not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a maximum constraint
     * See http://xcsp.org/specifications/maximum
     *
     * Example:
     * &lt;maximum>
     *    &lt;list> x1 x2 x3 x4 &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/maximum>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_v1(&mut self, _list: &[String], _operator: Operator, _operand: Operand) {
        println!("c Maximum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a maximum constraint with expressions in list
     * See http://xcsp.org/specifications/maximum
     *
     * Example:
     * &lt;maximum>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/maximum>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_v2(&mut self, _list: &[ExpressionTree], _operator: Operator, _operand: Operand) {
        println!("c Maximum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a maximum constraint
     * See http://xcsp.org/specifications/maximum
     *
     * Example:
     * &lt;minimum>
     *    &lt;list> x1 x2 x3 x4 &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimum>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_v1(&mut self, _list: &[String], _operator: Operator, _operand: Operand) {
        println!("c Minimum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minimum constraint with expressions in list
     * See http://xcsp.org/specifications/minimum
     *
     * Example:
     * &lt;minimumArg>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimumArg>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_arg_v2(
        &mut self,
        _list: &[ExpressionTree],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Minimum Arg Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minim arg constraint
     * See http://xcsp.org/specifications/maximum
     *
     * Example:
     * &lt;minimumArg>
     *    &lt;list> x1 x2 x3 x4 &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimumArg>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_arg_v1(
        &mut self,
        _list: &[String],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Minimum Arg Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minimum constraint with expressions in list
     * See http://xcsp.org/specifications/minimum
     *
     * Example:
     * &lt;minimum>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimum>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_v2(&mut self, _list: &[ExpressionTree], _operator: Operator, _operand: Operand) {
        println!("c Minimum Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minimum constraint with expressions in list
     * See http://xcsp.org/specifications/minimum
     *
     * Example:
     * &lt;minimumArg>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimumArg>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_arg_v2(
        &mut self,
        _list: &[ExpressionTree],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Maximum Arg Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minim arg constraint
     * See http://xcsp.org/specifications/maximum
     *
     * Example:
     * &lt;minimumArg>
     *    &lt;list> x1 x2 x3 x4 &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimumArg>
     *
     * @param list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_arg_v1(
        &mut self,
        _list: &[String],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Maximum Arg Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a count constraint with expressions
     * See http://xcsp.org/specifications/count
     * Example:
     * &lt;count id="c1">
     *     &lt;list> eq(x,1) ne(z,2) </list>
     *     &lt;values> 2 </values>
     *     &lt;condition> (ne,k1) </condition>
     * &lt;/count>
     *
     * @param _list the expression
     * @param values the set of integer values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)     */
    fn on_constraint_count_v1(
        &mut self,
        _list: &[ExpressionTree],
        _values: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Count Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a count constraint
     * See http://xcsp.org/specifications/count
     * Example:
     * &lt;count id="c1">
     *     &lt;list> y[] </list>
     *     &lt;values> 1 2 </values>
     *     &lt;condition> (ne,k1) </condition>
     * &lt;/count>
     *
     * @param _list the expression
     * @param values the set of integer values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)     */
    fn on_constraint_count_v2(&mut self, _list: &[String], _values: &[i32], _operator: Operator, _operand: Operand) {
        println!("c Count Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a count constraint with expressions
     * See http://xcsp.org/specifications/count
     * Example:
     * &lt;count id="c1">
     *     &lt;list> eq(x,1) ne(z,2) </list>
     *     &lt;values> 2 </values>
     *     &lt;condition> (ne,k1) </condition>
     * &lt;/count>
     *
     * @param _list the expression
     * @param values the set of variables values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)     */
    fn on_constraint_count_v3(
        &mut self,
        _list: &[ExpressionTree],
        _values: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Count Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a count constraint
     * See http://xcsp.org/specifications/count
     * Example:
     * &lt;count id="c1">
     *     &lt;list> y[] </list>
     *     &lt;values> x </values>
     *     &lt;condition> (ne,k1) </condition>
     * &lt;/count>
     *
     * @param _list the expression
     * @param values the set of variables values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_count_v4(&mut self, _list: &[String], _values: &[String], _operator: Operator, _operand: Operand) {
        println!("c Count Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a nValues constraint
     * See http://xcsp.org/specifications/nValues
     * Example:
     * &lt;nValues id="c3">
     *   &lt;list> z1 z2 z3 &lt;/list>
     *    &lt;condition> (eq,2) &lt;/condition>
     * &lt;/nValues>
     *
     * @param _list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_nvalues_v1(&mut self, _list: &[String], _operator: Operator, _operand: Operand) {
        println!("c NValues Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a nValues constraint with exceptions
     * See http://xcsp.org/specifications/nValues
     * Example:
     * &lt;nValues id="c3">
     *   &lt;list> z1 z2 z3 &lt;/list>
     *   &lt;except> 0 &lt;\except>
     *    &lt;condition> (eq,2) &lt;/condition>
     * &lt;/nValues>
     *
     * @param _list the list of the constraint
     * @param _except the set of exceptions
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_nvalues_v2(&mut self, _list: &[String], _except: &[i32], _operator: Operator, _operand: Operand) {
        println!("c NValues Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a nValues constraint with expressions
     * See http://xcsp.org/specifications/nValues
     * Example:
     * &lt;nValues id="c3">
     *   &lt;list> eq(z1,5) ne(z2,4) &lt;/list>
     *    &lt;condition> (eq,2) &lt;/condition>
     * &lt;/nValues>
     *
     * @param _list the list of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_nvalues_v3(&mut self, _list: &[ExpressionTree], _operator: Operator, _operand: Operand) {
        println!("c NValues Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cardinality constraint with int values and int occurs
     * See http://xcsp.org/specifications/cardinality
     *
     * Example:
     * <cardinality>
     *   <list> x1 x2 x3 x4 </list>
     *   <values> 2 5 10 </values>
     *   <occurs> 1 2 3 </occurs>
     * </cardinality>
     *
     * @param _list the list of the constraint
     * @param _values the set of values (here int)
     * @param _occurs the number of occurrences (here int)
     * @param _closed is the constraint is closed
     */
    fn on_constraint_cardinality_v1(&mut self, _list: &[String], _values: &[i32], _occurs: &[i32], _closed: bool) {
        println!("c Cardinality Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cardinality constraint with int values and variable occurs
     * See http://xcsp.org/specifications/cardinality
     *
     * Example:
     * <cardinality>
     *   <list> x1 x2 x3 x4 </list>
     *   <values> 0 1 2 3 </values>
     *   <occurs> z0 z1 z2 z3 </occurs>
     * </cardinality>
     *
     * @param _list the list of the constraint
     * @param _values the set of values (here int)
     * @param _occurs the number of occurrences (here variables)
     * @param _closed is the constraint is closed
     */
    fn on_constraint_cardinality_v2(&mut self, _list: &[String], _values: &[i32], _occurs: &[String], _closed: bool) {
        println!("c Cardinality Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cardinality constraint with int values and interval occurs
     * See http://xcsp.org/specifications/cardinality
     *
     * Example:
     * <cardinality>
     *   <list> x1 x2 x3 x4 </list>
     *   <values> 2 5 10 </values>
     *   <occurs> 0..1 1..3 2..3 </occurs>
     * </cardinality>
     *
     *
     * @param _list the list of the constraint
     * @param _values the set of values (here int)
     * @param _occurs the number of occurrences (here interval)
     * @param _closed is the constraint is closed
     */
    fn on_constraint_cardinality_v3(
        &mut self,
        _list: &[String],
        _values: &[i32],
        _occurs: &[(i32, i32)],
        _closed: bool,
    ) {
        println!("c Cardinality Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cardinality constraint with variable values and int occurs
     * See http://xcsp.org/specifications/cardinality
     *
     * Example:
     * <cardinality>
     *   <list> x1 x2 x3 x4 </list>
     *   <values> z1 z2 z3 </values>
     *   <occurs> 1 2 3 </occurs>
     * </cardinality>
     *
     * @param _list the list of the constraint (not the list...)
     * @param _values the set of values (here variable)
     * @param _occurs the number of occurences (here int)
     * @param closed is the constraint is closed
     */
    fn on_constraint_cardinality_v4(&mut self, _list: &[String], _values: &[String], _occurs: &[i32], _closed: bool) {
        println!("c Cardinality Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a cardinality constraint with variable values and variable occurs
     * See http://xcsp.org/specifications/cardinality
     *
     * Example:
     * <cardinality>
     *   <list> x1 x2 x3 x4 </list>
     *   <values> z1 z2 z3 </values>
     *   <occurs> y1 y2 y3 </occurs>
     * </cardinality>
     *
     * @param list the list of the constraint (not the list)
     * @param values the set of values (here variables)
     * @param occurs the number of occurences (here variables)
     * @param closed is the constraint is closed
     */
    fn on_constraint_cardinality_v5(
        &mut self,
        _list: &[String],
        _values: &[String],
        _occurs: &[String],
        _closed: bool,
    ) {
        println!("c Cardinality Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cardinality constraint with variable values and interval occurs
     * See http://xcsp.org/specifications/cardinality
     *
     * Example:
     * <cardinality>
     *   <list> x1 x2 x3 x4 </list>
     *   <values> z1 z2 z3 </values>
     *   <occurs> 1..2 3..5 2..4 </occurs>
     * </cardinality>
     *
     * @param list the list of the constraint (not the list)
     * @param values the set of values (here variables)
     * @param occurs the number of occurences (here intervals)
     * @param closed is the constraint is closed
     */

    fn on_constraint_cardinality_v6(
        &mut self,
        _list: &[String],
        _values: &[String],
        _occurs: &[(i32, i32)],
        _closed: bool,
    ) {
        println!("c Cardinality Variant 6 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origins, int lengths and int heights
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> 1 2 3 4 </lengths>
     *     <heights> 3 4 5 6 </heights>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here ints)
     * @param heights the vector of heights (here ints)
     * @param _operator the operator,
     * @param _operand: the operand (int, var...),
     */
    fn on_constraint_cumulative_v1(
        &mut self,
        _origins: &[String],
        _lengths: &[i32],
        _heights: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, int lengths and variable heights
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> 1 2 3 4 </lengths>
     *     <heights> h1 h2 h3 h4 </heights>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here ints)
     * @param heights the vector of heights (here variables)
     * @param xc the condition (see XCondition)
     */
    fn on_constraint_cumulative_v2(
        &mut self,
        _origins: &[String],
        _lengths: &[i32],
        _heights: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, variable lengths and int heights
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> l1 l2 l3 l4 </lengths>
     *     <heights> 1 2 3 4 </heights>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here variables)
     * @param heights the vector of heights (here ints)
     * @param xc the condition (see XCondition)
     */

    fn on_constraint_cumulative_v3(
        &mut self,
        _origins: &[String],
        _lengths: &[String],
        _heights: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, variable lengths and variable heights
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> l1 l2 l3 l4 </lengths>
     *     <heights> h1 h2 h3 h4 </heights>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param id the id (name) of the constraint
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here variables)
     * @param heights the vector of heights (here variables)
     * @param xc the condition (see XCondition)
     */
    fn on_constraint_cumulative_v4(
        &mut self,
        _origins: &[String],
        _lengths: &[String],
        _heights: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, int lengths and int heights and variable ends
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> 1 2 3 4 </lengths>
     *     <heights> 1 2 3 4 </heights>
     *     <end> e1 e2 e3 e4 </end>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param id the id (name) of the constraint
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here ints)
     * @param heights the vector of heights (here ints)
     * @param ends the vector of ends (here variables)
     * @param xc the condition (see XCondition)
     */
    fn on_constraint_cumulative_v5(
        &mut self,
        _origins: &[String],
        _lengths: &[i32],
        _heights: &[i32],
        _ends: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, int lengths and variable heights and variable ends
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> 1 2 3 4 </lengths>
     *     <heights> h1 h2 h3 h4 </heights>
     *     <end> e1 e2 e3 e4 </end>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param id the id (name) of the constraint
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here ints)
     * @param heights the vector of heights (here variables)
     * @param ends the vector of ends (here variables)
     * @param xc the condition (see XCondition)
     */

    fn on_constraint_cumulative_v6(
        &mut self,
        _origins: &[String],
        _lengths: &[i32],
        _heights: &[String],
        _ends: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 6 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, variable lengths and int heights and variable ends
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> l1 l2 l3 l4 </lengths>
     *     <heights> 1 2 3 4 </heights>
     *     <end> e1 e2 e3 e4 </end>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here variables)
     * @param heights the vector of heights (here ints)
     * @param ends the vector of ends (here variables)
     * @param xc the condition (see XCondition)
     */

    fn on_constraint_cumulative_v7(
        &mut self,
        _origins: &[String],
        _lengths: &[String],
        _heights: &[i32],
        _ends: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 7 not yet implemented");
    }

    /**
     * The callback function related to a cumulative constraint with variable origin, variable lengths and variable heights and variable ends
     * See http://xcsp.org/specifications/cumulative
     *
     * Example:
     * <cumulative>
     *     <origins> s1 s2 s3 s4 </origins>
     *     <lengths> l1 l2 l3 l4 </lengths>
     *     <heights> h1 h2 h3 h4 </heights>
     *     <end> e1 e2 e3 e4 </end>
     *     <condition> (le,4) </condition>
     * </cumulative>
     *
     * @param id the id (name) of the constraint
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here variables)
     * @param heights the vector of heights (here variables)
     * @param ends the vector of ends (here variables)
     * @param xc the condition (see XCondition)
     */
    fn on_constraint_cumulative_v8(
        &mut self,
        _origins: &[String],
        _lengths: &[String],
        _heights: &[String],
        _ends: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Cumulative Variant 8 not yet implemented");
    }

    /**
     * The callback function related to a element constraint with int value
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> y[] </list>
     *    <value> 2 </value>
     * </element>
     *
     * @param list the list of the constraint
     * @param value the value (here an int)
     */
    fn on_constraint_element_v1(&mut self, _list: &[String], _value: i32) {
        println!("c Element Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a element constraint with var value
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> y[] </list>
     *    <value> v </value>
     * </element>
     *
     * @param list the list of the constraint
     * @param value the value (here a variable)
     */
    fn on_constraint_element_v2(&mut self, _list: &[String], _value: String) {
        println!("c Element Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element constraint with index and variable value
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> y[]  </list>
     *    <index> x </index>
     *    <value> z </value>
     * </element>
     *
     * @param list the list of vars
     * @param value the value (here an integer)
     * @param start_index the start index
     * @param index the index (here a variable)
     */
    fn on_constraint_element_v3(&mut self, _list: &[String], _start_index: i32, _index: String, _value: String) {
        println!("c Element Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element constraint with index and int value
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> y[]  </list>
     *    <index> x </index>
     *    <value> 3 </value>
     * </element>
     *
     * @param list the list of vars
     * @param value the value (here an integer)
     * @param start_index the start index
     * @param index the index (here a int)
     */
    fn on_constraint_element_v4(&mut self, _list: &[String], _start_index: i32, _index: String, _value: i32) {
        println!("c Element Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element constraint with index and condition
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> y[]  </list>
     *    <index> x </index>
     *    <condition> (le,3) </condition>
     * </element>
     * </element>
     *
     * @param list the list of vars
     * @param start_index the start index
     * @param index the index (here a int)
     * @param _operator the operator,
     * @param _operand: the operand (int, var...),
     */
    fn on_constraint_element_v5(
        &mut self,
        _list: &[String],
        _start_index: i32,
        _index: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Element Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element with int in list constraint with index and variable value
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> 1 2 4 5   </list>
     *    <index> x </index>
     *    <value> z </value>
     * </element>
     *
     * @param list the list of int
     * @param value the value (here an integer)
     * @param start_index the start index
     * @param index the index (here a variable)
     */
    fn on_constraint_element_v6(&mut self, _list: &[i32], _start_index: i32, _index: String, _value: String) {
        println!("c Element Variant 6 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element constraint list of int with index and int value
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> 1 2 3 6   </list>
     *    <index> x </index>
     *    <value> 3 </value>
     * </element>
     *
     * @param list the list of int
     * @param value the value (here an integer)
     * @param start_index the start index
     * @param index the index (here a int)
     */
    fn on_constraint_element_v7(&mut self, _list: &[i32], _start_index: i32, _index: String, _value: i32) {
        println!("c Element Variant 7 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element with int in list constraint with index and condition
     * See http://xcsp.org/specifications/element
     *
     * Example:
     * <element>
     *    <list> 1 2 5O 230  </list>
     *    <index> x </index>
     *    <condition> (le,3) </condition>
     * </element>
     * </element>
     *
     * @param list the list of int
     * @param start_index the start index
     * @param index the index (here a int)
     * @param _operator the operator,
     * @param _operand: the operand (int, var...),
     */
    fn on_constraint_element_v8(
        &mut self,
        _list: &[i32],
        _start_index: i32,
        _index: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Element Variant 8 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a element constraint matrix with index and int value
     * See http://xcsp.org/specifications/element
     *
     * Example:  matrix[t][v] = z
     * <element>
     *    <matrix> (x1 x2 x3)(y1 y2 y3) </matrix>
     *    <index> t v </index>
     *    <value> 2 </value>
     * </element>
     *
     * @param matrix the matrix (of variables)
     * @param rowIndex the row index
     * @param colIndex the col index
     * @param startRowIndex the start index for rows
     * @param startColIndex the start index for cols
     * @param value the value (here a variable)
     */
    fn on_constraint_element_matrix_v1(
        &mut self,
        _matrix: &Vec<Vec<String>>,
        _row_index: String,
        _col_index: String,
        _start_row_index: i32,
        _start_col_index: i32,
        _value: i32,
    ) {
        println!("c Element Matrix Variant 1 not yet implemented");
    }

    /**
     * The callback function related to a element constraint matrix with index and variable value
     * See http://xcsp.org/specifications/element
     *
     * Example:  matrix[t][v] = z
     * <element>
     *    <matrix> (x1 x2 x3)(y1 y2 y3) </matrix>
     *    <index> t v </index>
     *    <value> z </value>
     * </element>
     *
     * @param matrix the matrix (of variables)
     * @param rowIndex the row index
     * @param colIndex the col index
     * @param startRowIndex the start index for rows
     * @param startColIndex the start index for cols
     * @param value the value (here a variable)
     */
    fn on_constraint_element_matrix_v2(
        &mut self,
        _matrix: &Vec<Vec<String>>,
        _row_index: String,
        _col_index: String,
        _start_row_index: i32,
        _start_col_index: i32,
        _value: String,
    ) {
        println!("c Element Matrix Variant 2 not yet implemented");
    }
    /**
     * The callback function related to a element constraint matrix with index and variable value
     * See http://xcsp.org/specifications/element
     *
     * Example:  matrix[t][v] = z
     * <element>
     *    <matrix> (x1 x2 x3)(y1 y2 y3) </matrix>
     *    <index> t v </index>
     *    <value> z </value>
     * </element>
     *
     * @param matrix the matrix (of variables)
     * @param rowIndex the row index
     * @param colIndex the col index
     * @param startRowIndex the start index for rows
     * @param operand
     * @param operator
     */
    fn on_constraint_element_matrix_v3(
        &mut self,
        _matrix: &Vec<Vec<String>>,
        _row_index: String,
        _col_index: String,
        _start_row_index: i32,
        _start_col_index: i32,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Element Matrix Variant 3 not yet implemented");
    }

    /**
     * The callback function related to a element constraint matrix of int with index and condition
     * See http://xcsp.org/specifications/element
     *
     * Example:  matrix[t][v] = z
     * <element>
     *    <matrix> (1 2 3 )(4 5 6) </matrix>
     *    <index> t v </index>
     *    <condition> (eq,3) </condition>
     * </element>
     *
     * @param matrix the matrix (of variables)
     * @param rowIndex the row index
     * @param colIndex the col index
     * @param startRowIndex the start index for rows
     * @param operand
     * @param operator
     */
    fn on_constraint_element_matrix_v4(
        &mut self,
        _matrix: &Vec<Vec<i32>>,
        _row_index: String,
        _col_index: String,
        _start_row_index: i32,
        _start_col_index: i32,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Element Matrix Variant 4 not yet implemented");
    }

    /**
     * The callback function related to a element constraint matrix of int with index and int value
     * See http://xcsp.org/specifications/element
     *
     * Example:  matrix[t][v] = z
     * <element>
     *    <matrix> (1 2 3 )(4 5 6) </matrix>
     *    <index> t v </index>
     *    <value> 3 </value>
     * </element>
     *
     * @param matrix the matrix (of variables)
     * @param rowIndex the row index
     * @param colIndex the col index
     * @param startRowIndex the start index for rows
     * @param operand
     * @param operator
     */
    fn on_constraint_element_matrix_v5(
        &mut self,
        _matrix: &Vec<Vec<i32>>,
        _row_index: String,
        _col_index: String,
        _start_row_index: i32,
        _start_col_index: i32,
        _value: i32,
    ) {
        println!("c Element Matrix Variant 5 not yet implemented");
    }

    /**
     * The callback function related to a element constraint matrix of int with index and var value
     * See http://xcsp.org/specifications/element
     *
     * Example:  matrix[t][v] = z
     * <element>
     *    <matrix> (1 2 3 )(4 5 6) </matrix>
     *    <index> t v </index>
     *    <value> z </value>
     * </element>
     *
     * @param matrix the matrix (of variables)
     * @param rowIndex the row index
     * @param colIndex the col index
     * @param startRowIndex the start index for rows
     * @param operand
     * @param operator
     */
    fn on_constraint_element_matrix_v6(
        &mut self,
        _matrix: &Vec<Vec<i32>>,
        _row_index: String,
        _col_index: String,
        _start_row_index: i32,
        _start_col_index: i32,
        _value: String,
    ) {
        println!("c Element Matrix Variant 5 not yet implemented");
    }
    /**
     * The callback function related to a channel constraint
     * See http://xcsp.org/specifications/channel
     *
     * Example:
     * <channel>
     *    <list> z1 z2 z3 z4 z5 </list>
     * </channel>
     *
     * @param _list the list of the constraint
     */
    fn on_constraint_channel_v1(&mut self, _list: &[String], _start_index: i32) {
        println!("c Channel Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a channel constraint
     * See http://xcsp.org/specifications/channel
     *
     * Example:
     * <channel>
     *     <list> x1 x2 x3 x4 </list>
     *     <list> y1 y2 y3 y4 </list>
     * </channel>
     *
     * The size of the array {@code list1} must be less than or equal to the size of {@code list2}.
     *
     * If list1.size() == list2.size() then list1[i] = j <=> list2[j] = i
     * If list1.size() <  list2.size() then list1[i] = j  => list2[j] = i
     *
     * @param id the id (name) of the constraint
     * @param list1 the first list
     * @param startIndex1 the starting index for list1
     * @param list2 the second list
     * @param startIndex2 the starting index for list2
     *
     */
    fn on_constraint_channel_v2(
        &mut self,
        _list1: &[String],
        _start_index1: i32,
        _list2: &[String],
        _start_index2: i32,
    ) {
        println!("c Channel Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a channel constraint with a value
     * See http://xcsp.org/specifications/channel
     *
     * Example:
     * <channel>
     * <list> z1 z2 z3 z4 z5 </list>
     * <value> v </value>
     * </channel>
     *
     * @param id the id (name) of the constraint
     * @param list the list of the constraint not necessary the list)
     * @param startIndex the starting index for list
     * @param value the vaule
     */
    fn on_constraint_channel_v3(&mut self, _list: &[String], _start_index: i32, _value: String) {
        println!("c Channel Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    fn on_constraint_no_overlap_v1(&mut self, _list: &[String], _lengths: &[i32], _zero_ignored: bool) {
        println!("c No Overlap Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_no_overlap_v2(&mut self, _list: &[String], _lengths: &[String], _zero_ignored: bool) {
        println!("c No Overlap Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a no overlap constraint with k dimensional variable origins and  int  lenghts
     * See http://xcsp.org/specifications/noOverlap
     *
     * Example:
     * &lt;noOverlap>
     *    &lt;origins> (x1,y1,z1)(x2,y2,z2)(x3,y3,z3)(x4,y4,z4) </origins>
     *    &lt;lengths> (2,4,1)(4,2,3)(5,1,2)(3,3,2) </lengths>
     * &lt;/noOverlap>
     *
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here vector of int)
     * @param zeroIgnored are zero ignored?
     */
    fn on_constraint_no_overlap_k_dim_v1(
        &mut self,
        _origins: &Vec<Vec<String>>,
        _lengths: &Vec<Vec<i32>>,
        _zero: bool,
    ) {
        println!("c No Overlap K Dim Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a no overlap constraint with k dimensional variable origins and  var  lenghts
     * See http://xcsp.org/specifications/noOverlap
     *
     * Example:
     * &lt;noOverlap>
     *    &lt;origins> (x1,y1,z1)(x2,y2,z2)(x3,y3,z3)(x4,y4,z4) </origins>
     *    &lt;lengths> (2,4,1)(4,2,3)(5,1,2)(3,3,2) </lengths>
     * &lt;/noOverlap>
     *
     * @param origins the vector of origins
     * @param lengths the vector of lenghts (here vector of Variables)
     * @param zeroIgnored are zero ignored?
     */
    fn on_constraint_no_overlap_k_dim_v2(
        &mut self,
        _origins: &Vec<Vec<String>>,
        _lengths: &Vec<Vec<String>>,
        _zero: bool,
    ) {
        println!("c No Overlap K Dim Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a no overlap constraint with variable origins and 2 dimensional mixed var/int lenghts
     * See http://xcsp.org/specifications/noOverlap
     *
     * Example:
     * <noOverlap>
     *    <origins> (x1,y1)(x2,y2)(x3,y3)(x4,y4) </origins>
     *    <lengths> (z1,3)(z2,2)(z3,5)(z4,1) </lengths>
     * </noOverlap>
     *
     * @param origins the vector of origins
     * @param _lengths the vector of var lengths
     * @param zeroIgnored are zero ignored?
     */
    fn on_constraint_no_overlap_k_dim_v3(
        &mut self,
        _origins: &Vec<Vec<String>>,
        _lengths: &Vec<(String, i32)>,
        _zero: bool,
    ) {
        println!("c No Overlap K Dim Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to circuit constraint
     * See http://xcsp.org/specifications/circuit
     *
     * Example:
     * <circuit>
     *   <list> x y z </list>
     * </circuit>
     *
     * @param _list the list of the constraint
     */

    fn on_constraint_circuit_v1(&mut self, _list: &Vec<String>) {
        println!("c Circuit Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to circuit constraint with defined int size
     * See http://xcsp.org/specifications/circuit
     *
     * Example:
     * <circuit>
     *   <list> x y z </list>
     *   <size> 4 </size>
     * </circuit>
     *
     * @param _list the list of the constraint
     * @param startIndex the start index for the list
     * @param size the size of the circuit (here an int)
     */
    fn on_constraint_circuit_v2(&mut self, _list: &Vec<String>, _size: i32) {
        println!("c Circuit Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to circuit constraint with defined int size
     * See http://xcsp.org/specifications/circuit
     *
     * Example:
     * <circuit>
     *   <list> x y z </list>
     *   <size> 4 </size>
     * </circuit>
     *
     * @param _list the list of the constraint
     * @param startIndex the start index for the list
     * @param size the size of the circuit (here an int)
     */
    fn on_constraint_circuit_v3(&mut self, _list: &Vec<String>, _ssize: String) {
        println!("c Circuit Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to precedence  constraint with defined variable size
     *
     * Example:
     * <precedence class="symmetry-breaking">
     * <list> x[][] </list>
     * </precedence>
     *
     * @param  the list of variables (not necessary the list)
     */
    fn on_constraint_precedence_v1(&mut self, _list: &[String], _covered: bool) {
        println!("c Precedence Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to precedence constraint with defined variable size
     *
     * Example:
     * <precedence class="symmetry-breaking">
     * <list> x[][] </list>
     * <values> 1 2 </values>
     * </precedence>
     *
     * @param list the list of variables (not necessary the list)
     * @param values the different vaules
     */
    fn on_constraint_precedence_v2(&mut self, _list: &[String], _values: &[i32], _covered: bool) {
        println!("c Precedence Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a strectch constraint with values and widths
     * See http://xcsp.org/specifications/stretch
     *
     * Example:
     * <stretch>
     *   <list> x1 x2 x3 x4 x5 x6 x7 </list>
     *   <values> 1 2 3 0 </values>
     *   <widths> 1..3 1..3 2..3 2..4 </widths>
     * </stretch>
     *
     * @param list the list of the constraint
     * @param values thelist of values
     * @param widths the list of intervals for widths
     */
    fn on_constraint_stretch_v1(&mut self, _list: &[String], _values: &[(i32, i32)], _widths: &[i32]) {
        println!("c Stretch Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a strectch constraint with values, widths and patterns
     * See http://xcsp.org/specifications/stretch
     *
     * @param id the id (name) of the constraint
     * @param list the list of the constraint
     * @param values thelist of values
     * @param widths the list of intervals for widths
     * @param patterns
     *
     */
    fn on_constraint_stretch_v2(&mut self, _list: &[String], _values: &[(i32, i32)], _widths: &[i32]) {
        println!("c Stretch Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a clause  constraint
     *
     * Example:
     * <clause>
     *   <list> x not(y) z </list>
     * </clause>
     *
     * @param positive the positive variables in the clause
     * @param negative the negative variables in the clause
     */

    fn on_constraint_clause(&mut self, _positive: &[String], _negative: &[String]) {
        println!("c Clause not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to knapsack  constraint
     *
     * @param list the list of variables (not necessary the list)
     * @param weights
     * @param _woperator: Operator,
     * @param _woperand: Operand,
     * @param profits
     * @param _poperator: Operator,
     * @param _poperand: Operand,
     */
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
        println!("c Knapsack  not yet implemented");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_bin_packing_v1(
        &mut self,
        _list: &[String],
        _sizes: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Bin Packing Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_bin_packing_v2(&mut self, _list: &[String], _sizes: &[i32], _limits: &[i32]) {
        println!("c Bin Packing Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_bin_packing_v3(&mut self, _list: &[String], _sizes: &[i32], _limits: &[String]) {
        println!("c Bin Packing Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    fn on_constraint_bin_packing_v4(&mut self, _list: &[String], _sizes: &[i32], _loads: &[i32]) {
        println!("c Bin Packing Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_bin_packing_v5(&mut self, _list: &[String], _sizes: &[i32], _loads: &[String]) {
        println!("c Bin Packing Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an ordered list constraint (this is a lex constraint)
     * See http://xcsp.org/specifications/ordered
     *
     *
     * Example:
     * <ordered>
     *   <list> x1 x2 x3 x4 </list>
     *   <list> y1 y2 y3 y4 </list>
     *   <operator> lt </operator>
     * </ordered>
     *
     * @param lists the set of lists (not the list, a variable may appear at different place!)
     * @param order the order LT, LE...
     */
    fn on_constraint_lex(&mut self, _lists: &Vec<Vec<String>>, _operator: Operator) {
        println!("c Lex not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an ordered matrix constraint (this is a lex constraint)
     * See http://xcsp.org/specifications/ordered
     *
     *
     * Example:
     * <ordered>
     *   <matrix>
     *   x[][]
     *   </matrix>
     *   <operator> lt </operator>
     * </ordered>
     *
     * @param lists the set of lists (not the list, a variable may appear at different place!)
     * @param order the order LT, LE...
     */
    fn on_constraint_lex_matrix(&mut self, _matrix: &Vec<Vec<String>>, _operator: Operator) {
        println!("c Lex matrix not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to flow  constraint
     *
     * @param list the list of variables (not necessary the list)
     * @param balance the different values
     * @param weights the different values
     * @param arcs the different values
     * @param operator
     * @param operand
     */
    fn on_constraint_flow(
        &mut self,
        _list: &[String],
        _balance: &[i32],
        _weights: &[i32],
        _arcs: &[(i32, i32)],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Flow not yet implemented");
        panic!("s UNSUPPORTED");
    }
    // -------------------------------------------------------------------------
    // Objectifs
    // -------------------------------------------------------------------------

    /**
     * The callback function related to an objective maximize a variable
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * &lt;objectives>
     *    &lt;maximize> x &lt;/maximize>
     * &lt;/objectives>
     *
     * @param var the variable
     */
    fn on_maximize_var(&mut self, _var: String) {
        println!("c Objective Maximize Var not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective minimize a variable
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * &lt;objectives>
     *    &lt;minimize> x &lt;/minimize>
     * &lt;/objectives>
     *
     * @param var the variable
     */
    fn on_minimize_var(&mut self, _var: String) {
        println!("c Objective Minimize Var not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to an objective maximize an expression
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * &lt;objectives>
     *    &lt;maximize> x &lt;/maximize>
     * &lt;/objectives>
     *
     * @param var the variable
     */
    fn on_maximize_expression(&mut self, _expr: &ExpressionTree) {
        println!("c Objective Maximize expression not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective minimize an expression
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * &lt;objectives>
     *    &lt;maximize> x &lt;/maximize>
     * &lt;/objectives>
     *
     * @param var the variable
     */
    fn on_minimize_expression(&mut self, _expr: &ExpressionTree) {
        println!("c Objective Minimize expression not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective minimize a sum/product with int coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> x1 x2 x3 x4 x5 </list>
     *     <coeffs> 2 4 1 4 8 </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v1(&mut self, _type: XElementOperator, _list: &[String], _coefs: &[i32]) {
        println!("c Objective Minimize Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective maximize a sum/product with int coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> x1 x2 x3 x4 x5 </list>
     *     <coeffs> 2 4 1 4 8 </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v1(&mut self, _type: XElementOperator, _list: &[String], _coefs: &[i32]) {
        println!("c Objective Maximize Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to an objective minimize a sum/product with vars coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> x1 x2 x3 x4 x5 </list>
     *     <coeffs> y[] </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v2(&mut self, _type: XElementOperator, _list: &[String], _coefs: &[String]) {
        println!("c Objective Minimize Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective maximize a sum/product with var coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> x1 x2 x3 x4 x5 </list>
     *     <coeffs> y[]</coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v2(&mut self, _type: XElementOperator, _list: &[String], _coefs: &[String]) {
        println!("c Objective Maximize Variant 2 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective minimize a sum/product with expression and int coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> eq(x[0],0) eq(x[1],0)</list>
     *     <coeffs> 1 2  </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v3(&mut self, _type: XElementOperator, _list: &[ExpressionTree], _coefs: &[i32]) {
        println!("c Objective Minimize Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective maximize a sum/product with expressions and int coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> eq(x[0],0) eq(x[1],0)</list>
     *     <coeffs> 1 2  </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v3(&mut self, _type: XElementOperator, _list: &[ExpressionTree], _coefs: &[i32]) {
        println!("c Objective Maximize Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to an objective minimize a sum/product with expression and var coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> eq(x[0],0) eq(x[1],0)</list>
     *     <coeffs> y[]  </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v4(&mut self, _type: XElementOperator, _list: &[ExpressionTree], _coefs: &[String]) {
        println!("c Objective Minimize Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective maximize a sum/product with expressions and int coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> eq(x[0],0) eq(x[1],0)</list>
     *     <coeffs> y[]  </coeffs>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v4(&mut self, _type: XElementOperator, _list: &[ExpressionTree], _coefs: &[String]) {
        println!("c Objective Maximize Variant 4 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective minimize a sum/product without coefs
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> x1 x2 x3 x4 x5 </list>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     */

    fn on_minimize_v5(&mut self, _type: XElementOperator, _list: &[String]) {
        println!("c Objective Minimize Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective maximize a sum/product
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> x1 x2 x3 x4 x5 </list>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     */

    fn on_maximize_v5(&mut self, _type: XElementOperator, _list: &[String]) {
        println!("c Objective Maximize Variant 5 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective minimize a sum/product with expression
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> eq(x[0],0) eq(x[1],0)</list>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     */

    fn on_minimize_v6(&mut self, _type: XElementOperator, _list: &[ExpressionTree]) {
        println!("c Objective Minimize Variant 6 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to an objective maximize a sum/product with expressions
     * See http://xcsp.org/specifications/objectives
     *
     * Example:
     * <objectives>
     *   <minimize type="sum">
     *     <list> eq(x[0],0) eq(x[1],0)</list>
     *   </minimize>
     * <objectives>
     *
     * @param type SUM, PRODUCT...
     * @param list the list
     */

    fn on_maximize_v6(&mut self, _type: XElementOperator, _list: &[ExpressionTree]) {
        println!("c Objective Maximize Variant 6 not yet implemented");
        panic!("s UNSUPPORTED");
    }
}
