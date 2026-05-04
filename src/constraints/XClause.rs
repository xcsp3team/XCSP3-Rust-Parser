pub mod xcsp3_core {
    use crate::data_structs::xint_val_var::xcsp3_core::XVarVal;
    use crate::errors::xcsp3error::xcsp3_core::Xcsp3Error;
    use crate::utils::utils_functions::xcsp3_utils::list_to_vec_var_val;
    use crate::variables::xdomain::xcsp3_core::XDomainInteger;
    use crate::variables::xvariable_set::xcsp3_core::XVariableSet;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};

    #[derive(Clone)]
    pub struct XClause<'a> {
        positive_literals: Vec<XVarVal>,
        negative_literals: Vec<XVarVal>,
        set: &'a XVariableSet,
    }

    impl<'a> XClause<'a> {
        pub fn from_str(value: &str, set: &'a XVariableSet) -> Result<Self, Xcsp3Error> {
            let mut positive_ones: Vec<String> = Vec::new();
            let mut negative_ones: Vec<String> = Vec::new();

            for token in value.split_whitespace() {
                if let Some(inner) = token.strip_prefix("not(").and_then(|s| s.strip_suffix(")")) {
                    negative_ones.push(inner.to_string());
                } else {
                    positive_ones.push(token.to_string());
                }
            }
            let pos = list_to_vec_var_val(&*positive_ones.join(" "));
            let neg = list_to_vec_var_val(&*negative_ones.join(" "));
            Ok(Self::new(pos?, neg?, set))
        }

        pub fn new(
            positive_literals: Vec<XVarVal>,
            negative_literals: Vec<XVarVal>,
            set: &'a XVariableSet,
        ) -> Self {
            Self {
                positive_literals,
                negative_literals,
                set,
            }
        }

        pub fn positive_literals(&self) -> &Vec<XVarVal> {
            &self.positive_literals
        }

        pub fn negative_literals(&self) -> &Vec<XVarVal> {
            &self.negative_literals
        }

        pub fn set(&self) -> &'a XVariableSet {
            self.set
        }
    }
}
