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
use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
use crate::constraints::xall_different_except::xcsp3_core::XAllDifferentExcept;
use crate::constraints::xall_equal::xcsp3_core::XAllEqual;
use crate::constraints::xcardinality::xcsp3_core::XCardinality;
use crate::constraints::xchannel::xcsp3_core::XChannel;
use crate::constraints::xcount::xcsp3_core::XCount;
use crate::constraints::xcumulative::xcsp3_core::XCumulative;
use crate::constraints::xelement::xcsp3_core::XElement;
use crate::constraints::xextension::xcsp3_core::XExtension;
use crate::constraints::xgroup::xcsp3_core::XGroup;
use crate::constraints::xinstantiation::xcsp3_core::XInstantiation;
use crate::constraints::xintension::xcsp3_core::XIntention;
use crate::constraints::xmax_min::xcsp3_core::XMaxMin;
use crate::constraints::xmdd::xcsp3_core::XMdd;
use crate::constraints::xn_values::xcsp3_core::XNValues;
use crate::constraints::xno_overlap::xcsp3_core::XNoOverlap;
use crate::constraints::xno_overlap_k_dimensional::xcsp3_core::XNoOverlapKDim;
use crate::constraints::xordered::xcsp3_core::XOrdered;
use crate::constraints::xregular::xcsp3_core::XRegular;
use crate::constraints::xslide::xcsp3_core::XSlide;
use crate::constraints::xstretch::xcsp3_core::XStretch;
use crate::constraints::xsum::xcsp3_core::XSum;
use crate::data_structs::expression_tree::xcsp3_utils::ExpressionTree;
use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
use crate::objectives::xobjectives_set::xcsp3_core::XObjective;
use crate::variables::xvariable_type::xcsp3_core::XVariableType;
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
    fn on_variable_interval(&mut self, id: String, minimum: i32, maximum: i32) {
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
    fn on_variable_values(&mut self, id: String, values: &[i32]) {
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
    fn begin_variable_array(&mut self, name: String) {}

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
    /// <channel> ... </channel>
    fn on_constraint_channel(&mut self, _c: &XChannel) {}

    /// <cumulative> ... </cumulative>
    fn on_constraint_cumulative(&mut self, _c: &XCumulative) {}

    /// <noOverlap> ... </noOverlap>  (2D)
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

    /// <stretch> ... </stretch>
    fn on_constraint_stretch(&mut self, _c: &XStretch) {}

    // -------------------------------------------------------------------------
    // Objectifs
    // -------------------------------------------------------------------------

    /// <minimize> ... </minimize>
    fn on_objective_minimize(&mut self, _obj: &XObjective) {}

    /// <maximize> ... </maximize>
    fn on_objective_maximize(&mut self, _obj: &XObjective) {}
}
