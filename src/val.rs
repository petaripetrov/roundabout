use std::{
    cell::{Ref, RefCell},
    collections::HashSet,
    fmt,
    hash::Hash,
    ops::Deref,
    rc::Rc,
};

pub type PropFn = fn(value: &Ref<Data>);

#[derive(Clone)]
pub struct Data {
    pub data: f64,
    pub grad: f64,
    pub prev: Vec<Val>,
    pub prop: Option<PropFn>,
    pub op: Option<String>,
    pub label: Option<String>,
}

// TODO rewrite so this makes more sense
// During the backpropagation process we need to borrow various Val objects
// that may or may not be owned by something else. Because of this, we need to use
// Rc<RefCell<Data> or a different wrapper class that allows multiple borrows
// This could also be done via an Arena, but would make it difficult to allow for basic
// tensor operations as we would need a GLOBAL arena to allocate Vals. However, we can
// put an arena on a Model struct and force allocation through that, preventing the possibility
// of Rc leaking memory (which is possible in some cyclic dependencies).
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Val(Rc<RefCell<Data>>);

impl Val {
    pub fn new(data: Data) -> Val {
        Val(Rc::new(RefCell::new(data)))
    }

    pub fn data(&self) -> f64 {
        self.0.borrow().data
    }

    pub fn relu(self) -> Val {
        let data = self.data().max(0.0);

        let prop_fn: PropFn = |value| {
            let mut prev = value.prev[0].borrow_mut();
            let factor = if value.data > 0.0 { 1.0 } else { 0.0 };

            prev.grad += factor * value.grad;
        };

        Val::new(Data {
            data,
            grad: 0.0,
            prev: vec![self.clone()],
            prop: Some(prop_fn),
            op: None,
            label: None,
        })
    }

    pub fn tanh(self) -> Val {
        let x = self.data();
        let new_data = ((2.0 * x).exp_m1()) / ((2.0 * x).exp() + 1.0);

        let prop_fn: PropFn = |value| {
            let mut prev = value.prev[0].borrow_mut();

            prev.grad += (1.0 - value.data.powf(2.0)) * value.grad;
        };

        Val::new(Data {
            data: new_data,
            grad: 0.0,
            prev: vec![self.clone()],
            prop: Some(prop_fn),
            op: None,
            label: None,
        })
    }

    pub fn backward(&self) {
        // https://github.com/danielway/micrograd-rs/blob/7765a1cc5ace5aa3a205125a7dcfc2e679e1c615/src/value.rs#L83
        // Copied from ^
        // TODO refactor into iterator and then iterate over everything, calling the functions
        let mut visited = HashSet::<Val>::new();
        let mut topo = Vec::<Val>::new();

        self.borrow_mut().grad = 1.0;
        // self._backward(&mut visited, self);
        self.build_top(&mut visited, self, &mut topo);

        for el in topo.iter().rev() {
            let borrowed = el.borrow();

            if let Some(prop_fn) = borrowed.prop {
                prop_fn(&borrowed);
            }
        }
        // for el in topo.iter().rev() {
        //     println!("{:?}", el.borrow());
        // }
    }

    fn _backward(&self, visited: &mut HashSet<Val>, value: &Val) {
        if !visited.contains(&value) {
            visited.insert(value.clone());

            let borrowed = value.borrow();
            if let Some(prop_fn) = borrowed.prop {
                // TODO look into this
                // What if, instead of holding prop_fns we register the operation type
                // match on it, and execute a function on Val/Engine object
                // so instead of prop_fn, if we have Add, we match against an enum
                // and execute prop_add(self, children) which updates everything?
                prop_fn(&borrowed);
            }

            for child in &value.borrow().prev {
                self._backward(visited, child);
            }
        }
    }

    fn build_top(&self, visited: &mut HashSet<Val>, value: &Val, topo: &mut Vec<Val>) {
        if !visited.contains(&value) {
            visited.insert(value.clone());

            for child in &value.borrow().prev {
                self.build_top(visited, child, topo);
            }

            topo.push(value.clone());
        }
    }

    pub fn pow(&self, exp: Val) -> Val {
        let res = self.borrow().data.powf(exp.borrow().data);

        let prop_fn: PropFn = |value| {
            let power = value.prev[1].borrow().data;
            let mut base = value.prev[0].borrow_mut();

            base.grad += power * (base.data.powf(power - 1.0)) * value.grad;
        };

        Val::new(Data {
            data: res,
            grad: 0.0,
            prev: vec![self.clone(), exp.clone()],
            prop: Some(prop_fn),
            op: Some("^".to_string()),
            label: None,
        })
    }
}

// Type traits
impl fmt::Debug for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Data")
            .field("data", &self.data)
            .field("grad", &self.grad)
            .finish()
    }
}

impl Deref for Val {
    type Target = Rc<RefCell<Data>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Into<f64>> From<T> for Val {
    fn from(value: T) -> Self {
        let data = Data {
            data: value.into(),
            grad: 0.0,
            prev: vec![],
            prop: None,
            op: None,
            label: None,
        };

        Val::new(data)
    }
}

impl PartialEq<f64> for Val {
    fn eq(&self, other: &f64) -> bool {
        self.0.borrow().data == *other
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        // TODO expand this to check more things
        self.data == other.data
    }
}

impl PartialEq<f64> for Data {
    fn eq(&self, other: &f64) -> bool {
        self.data == *other
    }
}

impl Eq for Data {}

impl Hash for Data {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.data.to_bits().hash(state);
        self.grad.to_bits().hash(state);
        self.prev.hash(state);
        self.prop.hash(state);
        self.op.hash(state);
        self.label.hash(state);
    }
}

impl Hash for Val {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.borrow().hash(state)
    }
}
