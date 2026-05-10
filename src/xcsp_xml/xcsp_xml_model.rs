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

/**
the SYNTAX of xcsp3 is as follows:
```xml
<instance format="XCSP<sup>3</sup>" type="frameworkType">
  <variables>
    ( <var .../>
    | <array .../>
    )+
  </variables>
  <constraints>
    ( <constraint .../>
    | <metaConstraint .../>
    | <group .../>
    | <block .../>
    )*
  </constraints>
  [<objectives  [ combination="combinationType" ]>
    ( <minimize .../>
    | <maximize .../>
    )+
  </objectives>]
  [<annotations .../>]
</instance>
```
 */
pub mod xcsp3_xml {
    use crate::constraints::xconstraint_set::xcsp3_core::XConstraintSet;
    use crate::objectives::xobjectives_set::xcsp3_core::XObjectivesSet;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use crate::xcsp_xml::constraint::xcsp3_xml::Constraint;
    use crate::xcsp_xml::constraint_type::xcsp3_xml::ConstraintType;
    use crate::xcsp_xml::objective::xcsp3_xml::Objective;
    use crate::xcsp_xml::variable::xcsp3_xml::Variable;
    use crate::xcsp_xml::variable_type::xcsp3_xml::VariableType;
    use quick_xml::de::from_str;
    use quick_xml::DeError;
    use serde::Deserialize;
    use std::fs;
    use std::time::Instant;

    #[derive(Deserialize, Debug)]
    pub enum InstanceType {
        #[serde(rename = "CSP")]
        Csp,
        #[serde(rename = "COP")]
        Cop,
    }

    /// the instance of XCSP3
    #[derive(Deserialize)]
    pub struct XcspXmlModel {
        #[serde(rename = "@format")]
        format: String,
        #[serde(rename = "@type")]
        r#type: InstanceType,
        variables: Option<Variable>,
        constraints: Constraint,
        objectives: Option<Objective>,
    }

    impl XcspXmlModel {
        pub fn build_objectives<'a>(&'a self, set: &'a XVariableSet) -> XObjectivesSet<'a> {
            let mut object = XObjectivesSet::new(set);
            // println!("{:?}", self.objectives);
            if let Some(oo) = &self.objectives {
                for e in oo.maximize.iter() {
                    object.build_maximize(&e.list, &e.coeffs, &e.expression, &e.r#type.to_string())
                }
                for e in oo.minimize.iter() {
                    object.build_minimize(&e.list, &e.coeffs, &e.expression, &e.r#type.to_string())
                }
            }

            object
        }

        /// read the instance from the xml file
        pub fn from_path(path: &str) -> Result<XcspXmlModel, DeError> {
            let _now = Instant::now();
            if !path.ends_with(".xml") {
                return Err(DeError::UnexpectedEof);
            }
            let xml = fs::read_to_string(path).unwrap();

            // println!("read the instance named {} by {:?}.", path, _now.elapsed());
            from_str(&xml)
        }

        /// read the instance from string
        pub fn from_string(string: &str) -> Result<XcspXmlModel, DeError> {
            let now = Instant::now();
            let r = from_str(string);
            println!(
                "read the instance by {} microseconds",
                now.elapsed().as_micros()
            );
            r
        }

        /// get the format of the instance: "XCSP3"
        pub fn get_format(&self) -> String {
            self.format.clone()
        }

        /// get the type of the instance:  COP or CSP
        pub fn get_instance_type(&self) -> &InstanceType {
            &self.r#type
        }

        /// build the variables
        pub fn build_variables(&self) -> XVariableSet {
            let mut variables: XVariableSet = XVariableSet::new();

            for var_type in self.variables.as_ref().unwrap().variables.iter() {
                match var_type {
                    VariableType::Var(var_string) => {
                        if var_string.r#as.is_empty() {
                            variables.build_variable_int(
                                &var_string.id,
                                &var_string.value,
                                &var_string.r#type,
                            );
                        } else {
                            variables.build_variable_int_as(&var_string.id, &var_string.r#as);
                        }
                    }
                    VariableType::Array(var_array_str) => {
                        // println!("var_array {:?}", var_array)
                        if var_array_str.domains.is_empty() {
                            variables.build_variable_array(
                                &var_array_str.id,
                                &var_array_str.size,
                                &var_array_str.value,
                            );
                        } else {
                            let mut domain_for: Vec<&String> = vec![];
                            let mut domain_value: Vec<&String> = vec![];
                            for e in var_array_str.domains.iter() {
                                domain_value.push(&e.value);
                                domain_for.push(&e.r#for);
                            }
                            variables.build_variable_tree(
                                &var_array_str.id,
                                &var_array_str.size,
                                domain_for,
                                domain_value,
                            );
                        }
                    }
                }
            }
            variables
        }

        fn parse_constraint(con_type: &ConstraintType, set: &mut XConstraintSet) {
            match con_type {
                ConstraintType::Group(group) => {
                    // println!("{:?}", group.constraints);
                    XcspXmlModel::parse_constraint(&group.constraints, set);

                    match set.get_last_constraint() {
                        None => {}
                        Some(cc) => {
                            // println!("{}",cc.to_string())
                            set.build_group(cc, &group.args);
                        }
                    }
                }
                ConstraintType::Block(block) => {
                    for e in block.constraints.iter() {
                        // println!("{:?}", e);
                        XcspXmlModel::parse_constraint(e, set);
                    }
                }
                ConstraintType::AllDifferent {
                    vars,
                    list,
                    except,
                    matrix,
                } => {
                    if !vars.is_empty() {
                        set.build_all_different(vars)
                    } else if matrix.is_empty() {
                        if !except.is_empty() {
                            for e in list.iter() {
                                set.build_all_different_except(e, except);
                            }
                        } else {
                            let tmp: Vec<_> = list.iter().map(|e| e.clone()).collect();
                            set.build_all_different_list(&*tmp);
                        }
                    } else {
                        set.build_all_different_matrix(matrix);
                    }
                }
                ConstraintType::AllEqual { vars, list } => {
                    if !vars.is_empty() {
                        set.build_all_equal(vars);
                    } else {
                        for e in list.iter() {
                            set.build_all_equal(e);
                        }
                    }
                }
                ConstraintType::Circuit { vars, list, size } => {
                    if !vars.is_empty() {
                        set.build_circuit(vars, size);
                    } else {
                        for e in list.iter() {
                            set.build_circuit(e, size);
                        }
                    }
                }
                ConstraintType::Precedence { vars, list, values } => {
                    if !vars.is_empty() {
                        set.build_precedence(vars, &*values.value, false);
                    } else {
                        let mut tmp = false;
                        if values.covered.is_empty() == false {
                            if values.covered == "true" {
                                tmp = true;
                            }
                        }
                        for e in list.iter() {
                            set.build_precedence(e, &*values.value, tmp);
                        }
                    }
                }

                ConstraintType::Ordered {
                    vars,
                    operator,
                    case,
                    lengths,
                    list,
                } => {
                    if list.is_empty() {
                        let op: String = match case.as_str() {
                            "increasing" => String::from("ge"),
                            "strictly_increasing" => String::from("gt"),
                            "decreasing" => String::from("le"),
                            "strictly_decreasing" => String::from("lt"),
                            _ => panic!("case undefined in ordered constraint: {}", case), // default case
                        };
                        set.build_ordered(vars, lengths, &*op)
                    } else {
                        set.build_ordered(list, lengths, operator)
                    }
                }

                ConstraintType::Intension { value, function } => {
                    if !value.is_empty() {
                        set.build_intention(value);
                    } else if !function.is_empty() {
                        set.build_intention(function);
                    }
                }
                ConstraintType::Extension {
                    vars,
                    supports,
                    conflicts,
                } => {
                    if supports.is_empty() {
                        set.build_extension(vars, conflicts, false)
                    } else if conflicts.is_empty() {
                        set.build_extension(vars, supports, true)
                    } else {
                        eprintln!("can't build extension, conflicts or supports must be non empty.")
                    }
                }
                ConstraintType::Regular {
                    vars,
                    transitions,
                    start,
                    r#final,
                } => set.build_regular(vars, transitions, start, r#final),
                ConstraintType::Mdd { vars, transitions } => set.build_mdd(vars, transitions),
                ConstraintType::Sum {
                    vars,
                    condition,
                    coeffs,
                } => set.build_sum(vars, condition, coeffs),
                ConstraintType::Count {
                    vars,
                    values,
                    condition,
                } => set.build_count(vars, condition, values),

                ConstraintType::NValues {
                    vars,
                    except,
                    condition,
                } => set.build_n_values(vars, condition, except),
                ConstraintType::Cardinality {
                    list,
                    values,
                    occurs,
                } => set.build_cardinality(list, &values.vars, occurs, &values.closed),
                ConstraintType::Minimum { list, condition } => set.build_minimum(list, condition),
                ConstraintType::Maximum { list, condition } => set.build_maximum(list, condition),
                ConstraintType::MinimumArg {
                    list,
                    rank,
                    condition,
                } => {
                    set.build_minimum_arg(
                        &*list.value,
                        if rank.is_empty() { "any" } else { rank },
                        if list.start_index.is_empty() {
                            0
                        } else {
                            list.start_index.parse().unwrap()
                        },
                        condition,
                    );
                }
                ConstraintType::MaximumArg {
                    list,
                    rank,
                    condition,
                } => {
                    set.build_maximum_arg(
                        &*list.value,
                        if rank.is_empty() { "any" } else { rank },
                        if list.start_index.is_empty() {
                            0
                        } else {
                            list.start_index.parse().unwrap()
                        },
                        condition,
                    );
                }

                ConstraintType::Element {
                    vars,
                    value,
                    index,
                    condition,
                    matrix,
                } => {
                    if matrix.value.is_empty() {
                        set.build_element(&vars.value, value, index, &vars.start_index, condition)
                    } else {
                        set.build_element_matrix(
                            &*matrix.value,
                            value,
                            index,
                            &*matrix.row_index,
                            &*matrix.col_index,
                            condition,
                        )
                    }
                }

                ConstraintType::Stretch {
                    vars,
                    values,
                    widths,
                    patterns,
                } => set.build_stretch(vars, values, widths, patterns),
                ConstraintType::NoOverlap {
                    origins,
                    lengths,
                    zero_ignored,
                } => {
                    if origins.contains(',') && origins.contains('(') {
                        set.build_no_overlap_k_dim(origins, lengths, zero_ignored);
                    } else {
                        set.build_no_overlap(origins, lengths, zero_ignored);
                    }
                }
                ConstraintType::BinPacking {
                    list,
                    sizes,
                    condition,
                    limits,
                    loads,
                } => set.build_bin_packing(list, sizes, condition, limits, loads),
                ConstraintType::Cumulative {
                    origins,
                    lengths,
                    heights,
                    condition,
                    ends,
                    machines,
                } => set.build_cumulative(
                    origins,
                    lengths,
                    heights,
                    &condition.value,
                    ends,
                    machines,
                    &condition.start_index,
                ),
                ConstraintType::Instantiation { vars, values } => {
                    // println!("{}{:?}", vars, values);
                    set.build_instantiation(vars, values);
                }
                ConstraintType::Slide {
                    circular,
                    list,
                    constraints,
                } => {
                    // println!("{circular} {:?},{:?}", list, constraints);
                    XcspXmlModel::parse_constraint(constraints, set);
                    match set.get_last_constraint() {
                        None => {}
                        Some(cc) => {
                            set.build_slide(cc, &list.vars, &list.collect, circular);
                        }
                    }
                }
                ConstraintType::Channel {
                    lists,
                    with_value,
                    simplified_list,
                } => {
                    if simplified_list.is_empty() == false {
                        set.build_channel(simplified_list, "0", "", "0", "");
                        return;
                    }
                    if with_value.is_empty() {
                        let st1 = if lists[0].start_index.is_empty() {
                            "0"
                        } else {
                            &*lists[0].start_index
                        };
                        if lists.len() == 1 {
                            set.build_channel(&lists[0].value, st1, "", "0", with_value);
                        } else {
                            let st2 = if lists[1].start_index.is_empty() {
                                "0"
                            } else {
                                &*lists[1].start_index
                            };
                            set.build_channel(
                                &lists[0].value,
                                st1,
                                &lists[1].value,
                                st2,
                                with_value,
                            );
                        }
                    } else {
                        let st1 = if lists[0].start_index.is_empty() {
                            "0"
                        } else {
                            &*lists[0].start_index
                        };
                        set.build_channel(&lists[0].value, st1, "", "0", with_value);
                    }
                }
                ConstraintType::Clause { vars } => set.build_clause(vars),

                ConstraintType::Lex {
                    lists,
                    matrix,
                    operator,
                } => {
                    if matrix.is_empty() == false {
                        set.build_lex_matrix(matrix, operator)
                    } else {
                        set.build_lex(lists, operator)
                    }
                }

                ConstraintType::Knapsack {
                    list,
                    weights,
                    profits,
                    condition,
                } => set.build_knapsack(list, weights, profits, condition),
                // ConstraintType::AllDistant { .. } => {}
                // ConstraintType::Precedence { .. } => {}
                // ConstraintType::Balance { .. } => {}
                // ConstraintType::Spread { .. } => {}
                // ConstraintType::Deviation { .. } => {}
                // ConstraintType::BinPacking { .. } => {}
                // ConstraintType::Lex { .. } => {}
                // ConstraintType::Clause { .. } => {}
                // _ => {}
                ConstraintType::ConstraintNone => {}
            }
        }

        pub fn build_constraints<'a>(&'a self, set: &'a XVariableSet) -> XConstraintSet<'a> {
            let mut constraint: XConstraintSet = XConstraintSet::new(set);
            for con_type in self.constraints.constraints.iter() {
                XcspXmlModel::parse_constraint(con_type, &mut constraint);
            }
            constraint
        }
    }
}
