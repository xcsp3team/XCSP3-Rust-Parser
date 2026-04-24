use std::path::PathBuf;

use xcsp3_rust::constraints::xconstraint_type::xcsp3_core::XConstraintType;
use xcsp3_rust::objectives::xobjectives_type::xcsp3_core::XObjectivesType;
use xcsp3_rust::variables::xvariable_type::xcsp3_core::XVariableType;
use xcsp3_rust::xcsp_callback::XcspCallback;
use xcsp3_rust::xcsp_runner::XcspRunner;
use xcsp3_rust::xcsp_xml::xcsp_xml_model::xcsp3_xml::XcspXmlModel;

fn fixture_path(name: &str) -> String {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("instances")
        .join(name)
        .display()
        .to_string()
}

fn assert_model_parses_without_internal_errors(name: &str) {
    let model = XcspXmlModel::from_path(&fixture_path(name)).unwrap();
    let variables = model.build_variables();
    let constraints = model.build_constraints(&variables);
    let objectives = model.build_objectives(&variables);

    assert!(
        variables.iter().count() > 0,
        "{name} did not expose any variable"
    );
    assert!(
        constraints.iter().count() > 0,
        "{name} did not expose any constraint"
    );
    assert!(
        variables
            .iter()
            .all(|variable| !matches!(variable, XVariableType::XVariableNone(_))),
        "{name} produced an internal variable parsing error"
    );
    assert!(
        constraints
            .iter()
            .all(|constraint| !matches!(constraint, XConstraintType::XConstraintNone(_))),
        "{name} produced an internal constraint parsing error"
    );
    assert!(
        objectives
            .iter()
            .all(|objective| !matches!(objective, XObjectivesType::XObjectiveNone(_))),
        "{name} produced an internal objective parsing error"
    );
}

#[derive(Default)]
struct CountingCallback {
    array_sections: usize,
    interval_variables: usize,
    enumerated_variables: usize,
}

impl XcspCallback for CountingCallback {
    fn begin_variable_array(&mut self, _name: String) {
        self.array_sections += 1;
    }

    fn on_variable_interval(&mut self, _id: String, _minimum: i32, _maximum: i32) {
        self.interval_variables += 1;
    }

    fn on_variable_values(&mut self, _id: String, _values: &[i32]) {
        self.enumerated_variables += 1;
    }
}

#[test]
fn all_interval_fixture_builds_model_without_internal_errors() {
    assert_model_parses_without_internal_errors("AllInterval-009.xml");
}

#[test]
fn ortholatin_fixture_builds_model_without_internal_errors() {
    assert_model_parses_without_internal_errors("Ortholatin-003.xml");
}

#[test]
fn runner_smoke_test_consumes_the_all_interval_fixture() {
    let mut callback = CountingCallback::default();

    XcspRunner::run(&fixture_path("AllInterval-009.xml"), &mut callback).unwrap();

    assert_eq!(callback.array_sections, 2);
    assert_eq!(callback.interval_variables, 17);
    assert_eq!(callback.enumerated_variables, 0);
}
