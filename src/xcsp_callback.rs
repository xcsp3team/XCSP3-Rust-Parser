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
use crate::objectives::xobjectives_set::xcsp3_core::XObjective;
use crate::variables::xvariable_type::xcsp3_core::XVariableType;
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::InstanceType;

pub trait XcspCallback {
    // -------------------------------------------------------------------------
    // Hooks de cycle de vie — appelés avant/après chaque section du fichier XML
    // -------------------------------------------------------------------------

    fn begin_instance(&mut self, _type : &InstanceType) {}

    fn end_instance(&mut self) {}

    /// Appelé juste avant de traiter la section <variables>
    fn begin_variables(&mut self) {}

    /// Appelé juste après avoir traité toutes les variables
    fn end_variables(&mut self) {}

    /// Appelé juste avant de traiter la section <constraints>
    fn begin_constraints(&mut self) {}

    /// Appelé juste après avoir traité toutes les contraintes
    fn end_constraints(&mut self) {}

    /// Appelé juste avant de traiter la section <objectives>
    fn begin_objectives(&mut self) {}

    /// Appelé juste après avoir traité tous les objectifs
    fn end_objectives(&mut self) {}

    // -------------------------------------------------------------------------
    // Variables
    // -------------------------------------------------------------------------

    /// Variable entière simple : <var id="x"> 1..10 </var>
    fn on_variable_interval(&mut self, id: String, minimum: i32, maximum: i32) {
        panic!("You must implement callbacks for variables");
    }

    /// Tableau de variables : <array id="x[]" size="5"> 0..4 </array>
    fn on_variable_values(&mut self, id: String, values: &[i32]) {
        panic!("You must implement callbacks for variables");
    }
    fn begin_variable_array(&mut self, name: String) {}

    fn end_variable_array(&mut self) {}

    /// Variable arbre (domaines complexes)
    fn on_variable_tree(&mut self, _var: &XVariableType) {}

    // -------------------------------------------------------------------------
    // Contraintes
    // -------------------------------------------------------------------------

    /// <allDifferent> x y z </allDifferent>
    fn on_constraint_all_different(&mut self, scope: &[String]) {}

    /// <allDifferent> x y z <except> 0 </except> </allDifferent>
    fn on_constraint_all_different_except(&mut self, scope: &[String], except: &[i32]) {}

    /// <allEqual> x y z </allEqual>
    fn on_constraint_all_equal(&mut self, _c: &XAllEqual) {}

    /// <extension> ... </extension>  (table de tuples autorisés/interdits)
    fn on_constraint_extension(&mut self, _c: &XExtension) {}

    /// <intension> eq(x, add(y,1)) </intension>
    fn on_constraint_intention(&mut self, _c: &XIntention) {}

    /// <sum> ... <condition> (ge, 10) </condition> </sum>
    fn on_constraint_sum(&mut self, _c: &XSum) {}

    /// <ordered> x y z <operator> lt </operator> </ordered>
    fn on_constraint_ordered(&mut self, _c: &XOrdered) {}

    /// <regular> ... </regular>  (automate fini déterministe)
    fn on_constraint_regular(&mut self, _c: &XRegular) {}

    /// <mdd> ... </mdd>  (diagramme de décision multi-valué)
    fn on_constraint_mdd(&mut self, _c: &XMdd) {}

    /// <instantiation> ... </instantiation>
    fn on_constraint_instantiation(&mut self, _c: &XInstantiation) {}

    /// <group> ... </group>  (groupe de contraintes)
    fn on_constraint_group(&mut self, _c: &XGroup) {}

    /// <maximum> ... <condition> (ge, 5) </condition> </maximum>
    fn on_constraint_maximum(&mut self, _c: &XMaxMin) {}

    /// <minimum> ... <condition> (le, 3) </condition> </minimum>
    fn on_constraint_minimum(&mut self, _c: &XMaxMin) {}

    /// <element> ... </element>
    fn on_constraint_element(&mut self, _c: &XElement) {}

    /// <slide> ... </slide>
    fn on_constraint_slide(&mut self, _c: &XSlide) {}

    /// <count> ... </count>
    fn on_constraint_count(&mut self, _c: &XCount) {}

    /// <nValues> ... </nValues>
    fn on_constraint_n_values(&mut self, _c: &XNValues) {}

    /// <cardinality> ... </cardinality>
    fn on_constraint_cardinality(&mut self, _c: &XCardinality) {}

    /// <channel> ... </channel>
    fn on_constraint_channel(&mut self, _c: &XChannel) {}

    /// <cumulative> ... </cumulative>
    fn on_constraint_cumulative(&mut self, _c: &XCumulative) {}

    /// <noOverlap> ... </noOverlap>  (2D)
    fn on_constraint_no_overlap(&mut self, _c: &XNoOverlap) {}

    /// <noOverlap> ... </noOverlap>  (k dimensions)
    fn on_constraint_no_overlap_k_dim(&mut self, _c: &XNoOverlapKDim) {}

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
