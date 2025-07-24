use crate::error::CoreError;

pub trait GroupTrait<T>: Clone {
    fn new(data: Vec<T>) -> Self;
    fn one_loop(&mut self) -> Result<(), CoreError>;
    fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T>
        where 
            T: 'a;
}