pub mod xcsp3_core {
    use crate::constraints::xconstraint_trait::xcsp3_core::{
        arg_in_operand, inject_parameters_in_list, inject_parameters_in_operand, max_arg_in_list,
        XConstraintUnfold,
    };
    use crate::constraints::xsum::xcsp3_core::XSum;
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::data_structs::xrelational_operand::xcsp3_core::Operand;
    use crate::data_structs::xrelational_operator::xcsp3_core::Operator;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::{extract_operator, list_to_vec_var_val};
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::cmp::max;

    #[derive(Clone)]
    pub struct XKnapsack<'a> {
        scope: Vec<XVarVal>,
        weights: Vec<XVarVal>,
        profits: Vec<XVarVal>,
        weight_operator: Operator,
        weight_operand: Operand,
        profit_operator: Operator,
        profit_operand: Operand,
        set: &'a XVariableSet,
    }

    impl XConstraintUnfold for XKnapsack<'_> {
        fn extract_parameters(&mut self, arg: &[XVarVal]) {
            let tmp = self.max_args_used();
            self.scope = inject_parameters_in_list(&self.scope, arg, tmp);
            self.weights = inject_parameters_in_list(&self.weights, arg, tmp);
            self.profits = inject_parameters_in_list(&self.profits, arg, tmp);
            self.profit_operand = inject_parameters_in_operand(&self.profit_operand, arg);
            self.weight_operand = inject_parameters_in_operand(&self.weight_operand, arg);
        }

        fn max_args_used(&mut self) -> i32 {
            let mut tmp = max(
                arg_in_operand(&self.profit_operand),
                arg_in_operand(&self.weight_operand),
            );
            tmp = max(tmp, max_arg_in_list(&*self.scope));

            tmp = max(tmp, max_arg_in_list(&*self.profits));
            max(tmp, max_arg_in_list(&*self.weights))
        }
    }
    impl<'a> XKnapsack<'a> {
        pub fn from_str(
            list: &str,
            weights: &str,
            profits: &str,
            conditions: &Box<[String]>,
            set: &'a XVariableSet,
        ) -> Result<Self, Xcsp3Error> {
            let scope = list_to_vec_var_val(list)?;
            let weights = list_to_vec_var_val(weights)?;
            let profits = list_to_vec_var_val(profits)?;
            let (weight_operator, weight_operand) = match extract_operator(&*conditions[0]) {
                Ok(value) => value,
                Err(value) => panic!("Error on condition: {}", conditions[0]),
            };
            let (profit_operator, profit_operand) = match extract_operator(&*conditions[1]) {
                Ok(value) => value,
                Err(value) => panic!("Error on condition: {}", conditions[1]),
            };
            Ok(Self::new(
                scope,
                weights,
                profits,
                weight_operator,
                weight_operand,
                profit_operator,
                profit_operand,
                set,
            ))
        }

        #[allow(clippy::too_many_arguments)]
        pub fn new(
            scope: Vec<XVarVal>,
            weights: Vec<XVarVal>,
            profits: Vec<XVarVal>,
            weight_operator: Operator,
            weight_operand: Operand,
            profit_operator: Operator,
            profit_operand: Operand,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                scope,
                weights,
                profits,
                weight_operator,
                weight_operand,
                profit_operator,
                profit_operand,
                set,
            }
        }

        pub fn scope(&self) -> &Vec<XVarVal> {
            &self.scope
        }

        pub fn weights(&self) -> &Vec<XVarVal> {
            &self.weights
        }

        pub fn profits(&self) -> &Vec<XVarVal> {
            &self.profits
        }

        pub fn weight_operator(&self) -> Operator {
            self.weight_operator
        }

        pub fn weight_operand(&self) -> &Operand {
            &self.weight_operand
        }

        pub fn profit_operator(&self) -> Operator {
            self.profit_operator
        }

        pub fn profit_operand(&self) -> &Operand {
            &self.profit_operand
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
