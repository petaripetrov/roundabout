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
    pub op: Option<String>,
    pub label: Option<String>,
}

impl Val {
    pub fn new(data: f64) -> Val {
        Val {
            data,
            grad: Default::default(),
            prev: Default::default(),
            op: None,
            label: None,
        }
    }

    // pub fn with_children(self, children: Vec<Val>) -> Val {
    //     Val {
    //         prev: children,
    //         ..self
    //     }
    // }

    // pub fn with_op(self, op: &str) -> Val {
    //     Val {
    //         op: Some(op.to_string()),
    //         ..self
    //     }
    // }

    pub fn with_label(self, label: &str) -> Val {
        Self {
            label: Some(label.to_owned()),
            ..self
        }
    }

    pub fn pow(&self, exp: ArgType) -> Val {
        match exp {
            ArgType::Numeric(num) => Val {
                data: self.data.powf(num),
                label: Some(format!("pow({:?})", self.label)),
                ..self.clone()
            },
            ArgType::Val(val) => Val {
                data: self.data.powf(val.data),
                label: Some(format!("pow({:?})", self.label)),
                ..self.clone()
            },
        }
    }

    pub fn relu(self) -> Self {
        let data = if self.data < 0.0 { 0.0 } else { self.data };

        Self {
            data,
            label: Some(format!("relu({:?})", self.label)),
            prev: vec![self.clone()],
            ..self
        }
    }

    pub fn tanh(self) -> Self {
        let x = self.data;

        Self {
            data: ((2.0 * x).exp_m1())/((2.0 * x).exp() + 1.0),
            grad: 0.0,
            prev: vec![self.clone()],
            op: Some(format!("tanh({:?})", self.label)),
            label: None,
        }
    }

    pub fn backward(self) {
        todo!()
    }

    pub fn print(self) {
        println!("{:.4}", self.data);

        for child in self.prev {
            child.print();
        }
    }
}

impl From<f64> for Val {
    fn from(data: f64) -> Self {
        Val {
            data,
            grad: Default::default(),
            prev: Default::default(),
            op: None,
            label: Some(format!("{}", data)),
        }
    }
}

// TODO fininsh all of the operations
// TODO move all of the operators into their own files for readibility
