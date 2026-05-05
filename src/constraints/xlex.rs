pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        arg_in_operand, inject_parameters_in_list, inject_parameters_in_operand, max_arg_in_list,
        XConstraintUnfold,
    };
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{list_to_matrix_ids, list_to_vec_var_val};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XLex<'a> {
        lists: Vec<Vec<XVarVal>>,
        operator: Operator,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XLex<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            for list in self.lists.iter_mut() {
                *list = inject_parameters_in_list(list, arg, tmp);
            }
        }

        fn max_args_used(&mut self) -> i32 {
            let mut tmp = -1;
            for list in self.lists.iter() {
                tmp = max(tmp, max_arg_in_list(list));
            }
            tmp
        }
    }
    impl<'a> XLex<'a> {
        pub fn from_str(
            list_strings: &[String],
            operator: &str,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let operator = match Operator::get_operator_by_str(operator) {
                Some(operator) => operator,
                None => {
                    return Err(Xcsp3Error::get_constraint_list_of_values_error(
                        "parse lex operator error, ",
                    ));
                }
            };

            let mut lists = Vec::with_capacity(list_strings.len());
            for list in list_strings {
                lists.push(list_to_vec_var_val(list)?);
            }

            if lists.len() < 2 {
                return Err(Xcsp3Error::get_constraint_list_of_values_error(
                    "lex requires at least two lists, ",
                ));
            }

            Ok(Self::new(lists, operator, set))
        }

        pub fn new(lists: Vec<Vec<XVarVal>>, operator: Operator, set: &'a XVariableSet) -> Self {
            Self {
                lists,
                operator,
                set,
            }
        }

        pub fn lists(&self) -> &Vec<Vec<XVarVal>> {
            &self.lists
        }

        pub fn operator(&self) -> &Operator {
            &self.operator
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
