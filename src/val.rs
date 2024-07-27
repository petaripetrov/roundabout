// trait Precision {}

// impl Precision for f64 {}
// impl Precision for f32 {}
// impl Precision for f16 {} // not stable

// type PropFn = fn(value: Val);

pub enum ArgType {
    Val(Val),
    Numeric(f64),
}

impl From<Val> for ArgType {
    fn from(value: Val) -> Self {
        ArgType::Val(value)
    }
}

impl From<f64> for ArgType {
    fn from(value: f64) -> Self {
        ArgType::Numeric(value)
    }
}

impl From<i64> for ArgType {
    fn from(value: i64) -> Self {
        ArgType::Numeric(value as f64)
    }
}

#[derive(Debug, Clone)] // TODO look itno implementing debug manually
pub struct Val {
    pub data: f64,
    pub grad: f64,
    pub prev: Vec<Val>, // prop: PropFn,
    // prev: Vec<Value>,
    op: Option<String>,
}

impl Val {
    pub fn new(data: f64) -> Val {
        Val {
            data,
            grad: Default::default(),
            prev: Default::default(),
            op: None
        }
    }

    pub fn with_children(self, children: Vec<Val>) -> Val {
        Val {
            prev: children,
            ..self
        }
    }

    pub fn with_op(self, op: &str) -> Val {
        Val {
            op: Some(op.to_string()),
            ..self
        }
    }

    pub fn pow(&self, exp: ArgType) -> Val {
        match exp {
            ArgType::Numeric(num) => self.data.powf(num).into(),
            ArgType::Val(val) => self.data.powf(val.data).into(),
        }
    }

    pub fn relu(self) -> Self {
        let data = if self.data < 0.0 { 0.0 } else { self.data };

        Self::new(data)
    }

    pub fn backward(self) {
        todo!()
    }
}

impl From<f64> for Val {
    fn from(data: f64) -> Self {
        Val {
            data,
            grad: Default::default(),
            prev: Default::default(),
            op: None
        }
    }
}

// TODO fininsh all of the operations
// TODO move all of the operators into their own files for readibility
