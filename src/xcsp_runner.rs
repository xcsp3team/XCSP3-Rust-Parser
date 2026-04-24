/*
 * xcsp_runner.rs
 *
 * Moteur principal : charge un fichier XCSP3, parse les variables / contraintes /
 * objectifs, puis dispatch chaque élément vers le callback correspondant.
 *
 * Usage :
 *   XcspRunner::run("mon_fichier.xml", &mut mon_callback)?;
 */
use crate::constraints::xconstraint_trait::xcsp3_core::XConstraintTrait;
use crate::constraints::xconstraint_type::xcsp3_core::XConstraintType;
use crate::objectives::xobjectives_type::xcsp3_core::XObjectivesType;
use crate::utils::utils_functions::xcsp3_utils::get_all_variables_between_lower_and_upper;
use crate::variables::xdomain::xcsp3_core::XDomainInteger;
use crate::variables::xvariable_type::xcsp3_core::XVariableType;
use crate::xcsp_callback::XcspCallback;
use crate::xcsp_xml::xcsp_xml_model::xcsp3_xml::XcspXmlModel;

pub struct XcspRunner;

impl XcspRunner {
    /// Charge le fichier XML situé à `path` et appelle les méthodes du `callback`
    /// au fur et à mesure du parsing.
    ///
    /// Retourne une erreur si le fichier est introuvable ou mal formé.


    pub fn run<C: XcspCallback>(
        path: &str,
        callback: &mut C,
    ) -> Result<(), Box<dyn std::error::Error>> {
        pub fn call_var(id: String, domain: &XDomainInteger, callback: &mut dyn XcspCallback) {
            if(domain.is_interval()) {
                callback.on_variable_interval(id, domain.minimum(), domain.maximum());
            }
            else {
                let mut result = vec![];
                for v in domain.iter() {
                    result.push(v)
                }
                callback.on_variable_values(id, &result)
            }
        }



        let model = XcspXmlModel::from_path(path)?;

        callback.begin_instance(model.get_instance_type());
        // ── Variables ────────────────────────────────────────────────────────
        callback.begin_variables();
        let variables = model.build_variables();
        for v in variables.iter() {
            match v {
                XVariableType::XVariableInt(var) =>  {
                    call_var(v.get_id(), &var.domain, callback);
                },
                XVariableType::XVariableArray(av) => {
                    callback.begin_variable_array(v.get_id());
                    for var_id in av.variables.iter() {
                        call_var(var_id.clone(), &av.domain, callback);
                    }

                    callback.end_variable_array();
                },
                XVariableType::XVariableTree(_) => callback.on_variable_tree(v),
                XVariableType::XVariableNone(_) => {}
            }
        }
        callback.end_variables();

        // ── Contraintes ──────────────────────────────────────────────────────
        callback.begin_constraints();
        let mut constraints = model.build_constraints(&variables);
        for  c in constraints.iter_mut() {
            match c {
                XConstraintType::XAllDifferent( inner) => {
                    let scope: Vec<String> = inner.get_scope().iter().map(|(s, _)| s.to_string()).collect();
                    callback.on_constraint_all_different(&*scope);
                }
                XConstraintType::XAllDifferentExcept(inner) => {
                    let scope: Vec<String> = inner.get_scope().iter().map(|(s, _)| s.to_string()).collect();
                    callback.on_constraint_all_different_except(&*scope, &*inner.get_except());
                }
                XConstraintType::XAllEqual(inner) => callback.on_constraint_all_equal(inner),
                XConstraintType::XExtension(inner) => callback.on_constraint_extension(inner),
                XConstraintType::XIntention(inner) => callback.on_constraint_intention(inner),
                XConstraintType::XSum(inner) => callback.on_constraint_sum(inner),
                XConstraintType::XOrdered(inner) => callback.on_constraint_ordered(inner),
                XConstraintType::XRegular(inner) => callback.on_constraint_regular(inner),
                XConstraintType::XMdd(inner) => callback.on_constraint_mdd(inner),
                XConstraintType::XInstantiation(inner) => {
                    callback.on_constraint_instantiation(inner)
                }
                XConstraintType::XGroup(inner) => callback.on_constraint_group(inner),
                XConstraintType::XMaximum(inner) => callback.on_constraint_maximum(inner),
                XConstraintType::XMinimum(inner) => callback.on_constraint_minimum(inner),
                XConstraintType::XElement(inner) => callback.on_constraint_element(inner),
                XConstraintType::XSlide(inner) => callback.on_constraint_slide(inner),
                XConstraintType::XCount(inner) => callback.on_constraint_count(inner),
                XConstraintType::XNValues(inner) => callback.on_constraint_n_values(inner),
                XConstraintType::XCardinality(inner) => callback.on_constraint_cardinality(inner),
                XConstraintType::XChannel(inner) => callback.on_constraint_channel(inner),
                XConstraintType::XCumulative(inner) => callback.on_constraint_cumulative(inner),
                XConstraintType::XNoOverlap(inner) => callback.on_constraint_no_overlap(inner),
                XConstraintType::XNoOverlapKDim(inner) => {
                    callback.on_constraint_no_overlap_k_dim(inner)
                }
                XConstraintType::XStretch(inner) => callback.on_constraint_stretch(inner),
                XConstraintType::XConstraintNone(_) => {}
            }
        }
        callback.end_constraints();

        // ── Objectifs ────────────────────────────────────────────────────────
        callback.begin_objectives();
        let objectives = model.build_objectives(&variables);
        for o in objectives.iter() {
            match o {
                XObjectivesType::Minimize(inner) => callback.on_objective_minimize(inner),
                XObjectivesType::Maximize(inner) => callback.on_objective_maximize(inner),
                XObjectivesType::XObjectiveNone(_) => {}
            }
        }
        callback.end_objectives();

        callback.end_instance();

        Ok(())
    }
}
