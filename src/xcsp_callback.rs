/*
 * xcsp_callback.rs
 *
 * Définissez votre propre solveur en implémentant ce trait.
 * Toutes les méthodes ont une implémentation vide par défaut :
 * vous n'implémentez que ce dont vous avez besoin.
 *
 * Exemple d'utilisation :
 *   struct MonSolveur { ... }
 *   impl XcspCallback for MonSolveur {
 *       fn on_variable_int(&mut self, var: &XVariableInt) { ... }
 *       fn on_constraint_all_different(&mut self, c: &XAllDifferent) { ... }
 *   }
 *   XcspRunner::run("mon_fichier.xml", &mut MonSolveur { ... }).unwrap();
 */
use crate::constraints::xelement::xcsp3_core::XElement;
use crate::constraints::xextension::xcsp3_core::XExtension;
use crate::constraints::xslide::xcsp3_core::XSlide;
use crate::constraints::xstretch::xcsp3_core::XStretch;
use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
use crate::objectives::xobjective_element::xcsp3_core::XElementOperator;
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::InstanceType;

pub trait XcspCallback {
    // -------------------------------------------------------------------------
    // Hooks de cycle de vie — appelés avant/après chaque section du fichier XML
    // -------------------------------------------------------------------------

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
     * The callback function related to a alldifferent constraint.
     * See http://xcsp.org/specifications/alldifferent
     *
     * Example:
     * &lt;allDifferent>
     *   x1 x2 x3 x4 x5
     * &lt;/allDifferent>
     *
     * @param scope the scope of the constraint
     */
    fn on_constraint_all_different_v1(&mut self, _scope: &[String]) {
        println!("c Alldifferent not yet implemented");
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
     * @param scope the trees of the constraint
     */
    fn on_constraint_all_different_v2(&mut self, _scope: &[ExpressionTree]) {
        println!("c Alldifferent with expressions in list not yet implemented");
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
     * @param scope the scope of the constraint
     * @param except the set of excepted values
     */
    fn on_constraint_all_different_except(&mut self, _scope: &[String], _except: &[i32]) {
        println!("c Alldifferent not yet implemented");
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
     * @param lists the set of lists (not the scope, a variable may appear at different place!)
     */
    fn on_constraint_all_different_list(&mut self, _lists: &[Vec<String>]) {
        println!("c Alldifferent lists not yet implemented");
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
     * @param list the scope of the constraint
     *
     */
    fn on_constraint_all_equal_v1(&mut self, _scope: &[String]) {
        println!("c Alldifferent not yet implemented");
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
    fn on_constraint_all_equal_v2(&mut self, _scope: &[ExpressionTree]) {
        println!("c Alldifferent with expressions in scope not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /// <extension> ... </extension>  (table de tuples autorisés/interdits)
    fn on_constraint_extension(&mut self, _c: &XExtension) {}

    /**
     * The callback function related to a constraint in intension
     * See http://xcsp.org/specifications/intension
     * Example:
     * &lt;intension> eq(add(x,y),z) &lt;/intension>
     *
     * @param id the id (name) of the constraint
     * @param tree the canonized form related to the tree
     */
    fn on_constraint_intention(&mut self, _scope: &[String], _tree: &ExpressionTree) {
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
     * @param scope the scope of the constraint
     * @param operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v1(&mut self, _scope: &[String], _operator: Operator, _operand: Operand) {
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
     * @param scope the scope of the constraint
     * @param coeefs the coefficient of the sum (int)
     * @param Operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v2(
        &mut self,
        _scope: &[String],
        _coeffs: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param scope the scope of the constraint
     * @param coeefs the coefficient of the sum (variables)
     * @param tperator the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v3(
        &mut self,
        _scope: &[String],
        _coeffs: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param _scope the different trees
     * @param coeefs the coefficient of the sum (variables)
     * @param _operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v5(
        &mut self,
        _scope: &[ExpressionTree],
        _coeffs: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Sum Variant 5s not yet implemented");
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
     * @param _scope the different trees
     * @param Operaor the condition (Le, Gt...)
     * @param operand the operant (Var, val, ...)
     */
    fn on_constraint_sum_v4(
        &mut self,
        _scope: &[ExpressionTree],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param scope the scope of the constraint
     * @param operator the order Lt, Le...
     */
    fn on_constraint_ordered_v1(&mut self, _scope: &[String], _operator: Operator) {
        println!("c Ordered Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to an ordered constraint
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
     * @param _scope the scope of the constraint
     * @param _lengths the lengths

    * @param order the order Lt, Le...
     */
    fn on_constraint_ordered_v2(
        &mut self,
        _scope: &[String],
        _lengths: &[i32],
        _operator: Operator,
    ) {
        println!("c Ordered Variant 2 not yet implemented");
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
     * @param _scope the scope of the constraint
     * @param _start the starting node
     * @param _final the set of final nodes
     * @param _transitions the set of transitions
     */
    fn on_constraint_regular(
        &mut self,
        _scope: &[String],
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
     * @param scope the scope of the constraint
     * @param transitions the set of transitions
     */
    fn on_constraint_mdd(&mut self, _scope: &[String], _transitions: &Vec<(String, i32, String)>) {
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
     * @param _scope the scope of the constraint
     * @param _values the value for each variable
     */
    fn on_constraint_instantiation(&mut self, _scope: &[String], _values: &[i32]) {}

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
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_v1(
        &mut self,
        _scope: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Maximum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a maximum constraint with expressions in scope
     * See http://xcsp.org/specifications/maximum
     *
     * Example:
     * &lt;maximum>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/maximum>
     *
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_v2(
        &mut self,
        _scope: &[ExpressionTree],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_v1(
        &mut self,
        _scope: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Minimum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minimum constraint with expressions in scope
     * See http://xcsp.org/specifications/minimum
     *
     * Example:
     * &lt;minimumArg>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimumArg>
     *
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_arg_v2(
        &mut self,
        _scope: &[ExpressionTree],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Minimum Arg Variant 1 not yet implemented");
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
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_arg_v1(
        &mut self,
        _scope: &[String],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Minimum Arg Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minimum constraint with expressions in scope
     * See http://xcsp.org/specifications/minimum
     *
     * Example:
     * &lt;minimum>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimum>
     *
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_minimum_v2(
        &mut self,
        _scope: &[ExpressionTree],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Minimum Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /**
     * The callback function related to a minimum constraint with expressions in scope
     * See http://xcsp.org/specifications/minimum
     *
     * Example:
     * &lt;minimumArg>
     *    &lt;list> eq(x1,3) add(x2,2) le(x3,0) div(x4,4) &lt;/list>
     *    &lt;condition> (ge,2) &lt;/condition>
     * &lt;/minimumArg>
     *
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_arg_v2(
        &mut self,
        _scope: &[ExpressionTree],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Maximum Arg Variant 1 not yet implemented");
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
     * @param scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */

    fn on_constraint_maximum_arg_v1(
        &mut self,
        _scope: &[String],
        _start_index: i32,
        _rank: String,
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Maximum Arg Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    /// <element> ... </element>
    fn on_constraint_element(&mut self, _c: &XElement) {}

    /// <slide> ... </slide>
    fn on_constraint_slide(&mut self, _c: &XSlide) {}

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
     * @param _scope the expression
     * @param values the set of integer values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)     */
    fn on_constraint_count_v1(
        &mut self,
        _scope: &[ExpressionTree],
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
     * @param _scope the expression
     * @param values the set of integer values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)     */
    fn on_constraint_count_v2(
        &mut self,
        _scope: &[String],
        _values: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param _scope the expression
     * @param values the set of variables values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)     */
    fn on_constraint_count_v3(
        &mut self,
        _scope: &[ExpressionTree],
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
     * @param _scope the expression
     * @param values the set of variables values
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_count_v4(
        &mut self,
        _scope: &[String],
        _values: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
        println!("c Count Variant 2 not yet implemented");
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
     * @param _scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_nvalues_v1(
        &mut self,
        _scope: &[String],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param _scope the scope of the constraint
     * @param _except the set of exceptions
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_nvalues_v2(
        &mut self,
        _scope: &[String],
        _except: &[i32],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param _scope the scope of the constraint
     * @param operator the operator (Le,...)
     * @param operand the operand (int, var...)
     */
    fn on_constraint_nvalues_v3(
        &mut self,
        _scope: &[ExpressionTree],
        _operator: Operator,
        _operand: Operand,
    ) {
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
     * @param _scope the scope of the constraint
     * @param _values the set of values (here int)
     * @param _occurs the number of occurrences (here int)
     * @param _closed is the constraint is closed
     */
    fn on_constraint_cardinality_v1(
        &mut self,
        _scope: &[String],
        _values: &[i32],
        _occurs: &[i32],
        _closed: bool,
    ) {
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
     * @param _scope the scope of the constraint
     * @param _values the set of values (here int)
     * @param _occurs the number of occurrences (here variables)
     * @param _closed is the constraint is closed
     */
    fn on_constraint_cardinality_v2(
        &mut self,
        _scope: &[String],
        _values: &[i32],
        _occurs: &[String],
        _closed: bool,
    ) {
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
     * @param _scope the scope of the constraint
     * @param _values the set of values (here int)
     * @param _occurs the number of occurrences (here interval)
     * @param _closed is the constraint is closed
     */
    fn on_constraint_cardinality_v3(
        &mut self,
        _scope: &[String],
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
     * @param _scope the list of the constraint (not the scope...)
     * @param _values the set of values (here variable)
     * @param _occurs the number of occurences (here int)
     * @param closed is the constraint is closed
     */
    fn on_constraint_cardinality_v4(
        &mut self,
        _scope: &[String],
        _values: &[String],
        _occurs: &[i32],
        _closed: bool,
    ) {
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
     * @param scope the list of the constraint (not the scope)
     * @param values the set of values (here variables)
     * @param occurs the number of occurences (here variables)
     * @param closed is the constraint is closed
     */
    fn on_constraint_cardinality_v5(
        &mut self,
        _scope: &[String],
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
     * @param scope the list of the constraint (not the scope)
     * @param values the set of values (here variables)
     * @param occurs the number of occurences (here intervals)
     * @param closed is the constraint is closed
     */

    fn on_constraint_cardinality_v6(
        &mut self,
        _scope: &[String],
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
     * @param xc the condition (see XCondition)
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
     * @param scope the scope of the constraint
     * @param value the value (here an int)
     */
    fn on_constraint_element_v1(&mut self, scope: &[String], value: i32) {
        println!("c Element Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
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
     * @param _scope the scope of the constraint
     */
    fn on_constraint_channel_v1(&mut self, _scope: &[String], _start_index: i32) {
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
     * @param list the list of the constraint not necessary the scope)
     * @param startIndex the starting index for list
     * @param value the vaule
     */
    fn on_constraint_channel_v3(&mut self, _list: &[String], _start_index: i32, _value: String) {
        println!("c Channel Variant 3 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    fn on_constraint_no_overlap_v1(
        &mut self,
        _scope: &[String],
        _lengths: &[i32],
        _zero_ignored: bool,
    ) {
        println!("c No Overlap Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }

    fn on_constraint_no_overlap_v2(
        &mut self,
        _scope: &[String],
        _lengths: &[String],
        _zero_ignored: bool,
    ) {
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
     * @param _scope the scope of the constraint
     */

    fn on_constraint_circuit_v1(&mut self, _scope: &Vec<String>) {
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
     * @param _scope the scope of the constraint
     * @param startIndex the start index for the list
     * @param size the size of the circuit (here an int)
     */
    fn on_constraint_circuit_v2(&mut self, _scope: &Vec<String>, _size: i32) {
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
     * @param _scope the scope of the constraint
     * @param startIndex the start index for the list
     * @param size the size of the circuit (here an int)
     */
    fn on_constraint_circuit_v3(&mut self, _scope: &Vec<String>, _ssize: String) {
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
     * @param  the list of variables (not necessary the scope)
     */
    fn on_constraint_precedence_v1(&mut self, _scope: &[String], _covered: bool) {
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
     * @param list the list of variables (not necessary the scope)
     * @param values the different vaules
     */
    fn on_constraint_precedence_v2(&mut self, _scope: &[String], _values: &[i32], _covered: bool) {
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
     * @param list the scope of the constraint
     * @param values thelist of values
     * @param widths the list of intervals for widths
     */
    fn on_constraint_stretch_v1(
        &mut self,
        _scope: &[String],
        _values: &[(i32, i32)],
        _widths: &[i32],
    ) {
        println!("c Stretch Variant 1 not yet implemented");
        panic!("s UNSUPPORTED");
    }
    /**
     * The callback function related to a strectch constraint with values, widths and patterns
     * See http://xcsp.org/specifications/stretch
     *
     * @param id the id (name) of the constraint
     * @param list the scope of the constraint
     * @param values thelist of values
     * @param widths the list of intervals for widths
     * @param patterns
     *
     */
    fn on_constraint_stretch_v2(
        &mut self,
        _scope: &[String],
        _values: &[(i32, i32)],
        _widths: &[i32],
    ) {
        println!("c Stretch Variant 1 not yet implemented");
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
     * @param list the list of variables (not necessary the scope)
     * @param weights
     * @param _woperator: Operator,
     * @param _woperand: Operand,
     * @param profits
     * @param _poperator: Operator,
     * @param _poperand: Operand,
     */
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
        println!("c Knapsack  not yet implemented");
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
        println!("c Objective Minimiez Var not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v1(&mut self, _type: XElementOperator, _scope: &[String], _coefs: &[i32]) {
        println!("c Objective Minimize v1 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v1(&mut self, _type: XElementOperator, _scope: &[String], _coefs: &[i32]) {
        println!("c Objective Maximize v1 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v2(&mut self, _type: XElementOperator, _scope: &[String], _coefs: &[String]) {
        println!("c Objective Minimize v2 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v2(&mut self, _type: XElementOperator, _scope: &[String], _coefs: &[String]) {
        println!("c Objective Maximize v2 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v3(
        &mut self,
        _type: XElementOperator,
        _scope: &[ExpressionTree],
        _coefs: &[i32],
    ) {
        println!("c Objective Minimize v3 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v3(
        &mut self,
        _type: XElementOperator,
        _scope: &[ExpressionTree],
        _coefs: &[i32],
    ) {
        println!("c Objective Maximize v3 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_minimize_v4(
        &mut self,
        _type: XElementOperator,
        _scope: &[ExpressionTree],
        _coefs: &[String],
    ) {
        println!("c Objective Minimize v4 not yet implemented");
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
     * @param list the scope
     * @param coefs the vector of coefficients
     */

    fn on_maximize_v4(
        &mut self,
        _type: XElementOperator,
        _scope: &[ExpressionTree],
        _coefs: &[String],
    ) {
        println!("c Objective Maximize v4 not yet implemented");
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
     * @param list the scope
     */

    fn on_minimize_v5(&mut self, _type: XElementOperator, _scope: &[String]) {
        println!("c Objective Minimize v5 not yet implemented");
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
     * @param list the scope
     */

    fn on_maximize_v5(&mut self, _type: XElementOperator, _scope: &[String]) {
        println!("c Objective Maximize v5 not yet implemented");
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
     * @param list the scope
     */

    fn on_minimize_v6(&mut self, _type: XElementOperator, _scope: &[ExpressionTree]) {
        println!("c Objective Minimize v4 not yet implemented");
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
     * @param list the scope
     */

    fn on_maximize_v6(&mut self, _type: XElementOperator, _scope: &[ExpressionTree]) {
        println!("c Objective Maximize v6 not yet implemented");
        panic!("s UNSUPPORTED");
    }
}
