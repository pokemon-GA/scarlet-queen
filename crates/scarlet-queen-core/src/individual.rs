use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Individual<T> {
    id: RefCell<usize>,
    value: T,
}

impl<T> Individual<T> {
    pub fn new(id: usize, value: T) -> Individual<T> {
        Individual {
            id: RefCell::new(id),
            value,
        }
    }

    pub fn get_id(&self) -> usize {
        *self.id.borrow()
    }

    pub fn set_id(&self, id: usize) {
        *self.id.borrow_mut() = id;
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

pub trait EachCrateIndividual<T> {
    fn new(individual: &Rc<Individual<T>>) -> Self;
    fn get_id(&self) -> usize;
    fn get_value(&self) -> &T;
}
