/*
 * example_callback.rs
 *
 * Exemple concret d'implémentation du trait XcspCallback.
 * Définit PrintingSolver, un solveur qui affiche et compte chaque élément parsé.
 */
use crate::constraints::xall_different::xcsp3_core::XAllDifferent;
use crate::constraints::xextension::xcsp3_core::XExtension;
use crate::constraints::xintension::xcsp3_core::XIntention;
use crate::constraints::xsum::xcsp3_core::XSum;
use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
use crate::objectives::xobjectives_type::xcsp3_core::XObjective;
use crate::variables::xvariable_type::xcsp3_core::XVariableType;
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
    fn on_constraint_all_different(&mut self, scope: &[String]) {
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

    fn on_constraint_all_equal(&mut self, scope: &[String]) {
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
    fn on_constraint_extension(&mut self, c: &XExtension) {
        self.nb_constraints += 1;
        println!("  [Ext]      {}", c);
    }
    fn on_constraint_intention(&mut self, c: &XIntention) {
        self.nb_constraints += 1;
        println!("  [Intent]   {}", c);
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
    // -- Objectifs -----------------------------------------------------------
    fn on_objective_minimize(&mut self, obj: &XObjective) {
        println!("=== Objectif : Minimiser {} ===", obj);
    }
    fn on_objective_maximize(&mut self, obj: &XObjective) {
        println!("=== Objectif : Maximiser {} ===", obj);
    }
}
