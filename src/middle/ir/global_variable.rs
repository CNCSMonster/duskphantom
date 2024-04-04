use super::*;

pub type GlobalPtr = ObjPtr<GlobalVariable>;
impl Display for GlobalPtr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)
    }
}

pub struct GlobalVariable {
    pub name: String,
    pub value_type: ValueType,
    /// True if the global variable is a global variable, false if it is a global constant.
    pub variable_or_constant: bool,
    pub initializer: Vec<Constant>,
    user: Vec<InstPtr>,
}

impl Display for GlobalVariable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)
    }
}

impl GlobalVariable {
    pub fn new(
        name: String,
        value_type: ValueType,
        variable_or_constant: bool,
        initializer: Vec<Constant>,
    ) -> Self {
        Self {
            name,
            value_type,
            variable_or_constant,
            initializer,
            user: Vec::new(),
        }
    }

    pub fn gen_llvm_ir(&self) -> String {
        format!(
            "{} = dso_local {} {} {}\n",
            self,
            if self.variable_or_constant {
                "global"
            } else {
                "constant"
            },
            self.value_type,
            "[".to_owned()
                + &self
                    .initializer
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
                + "]"
        )
    }
}
