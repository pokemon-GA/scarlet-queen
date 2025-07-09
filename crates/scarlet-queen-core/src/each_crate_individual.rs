use std::rc::Rc;

pub trait EachCrateIndividual<T> {
    fn new(individual: &Rc<T>) -> Self;
}