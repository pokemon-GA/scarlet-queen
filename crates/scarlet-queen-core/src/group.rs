use std::{fmt::Debug, io::Write};

pub trait InitializerTrait<T, const N: usize> {
    fn initializer() -> [T; N];
}

pub trait GroupTrait<T, const N: usize, const R: usize>
where
    T: Clone,
    Self: Sized,
{
    type Err: Debug;

    fn new(data: [T; N]) -> Self;

    fn one_loop(&mut self) -> Result<(), Self::Err>;

    fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    fn init<I>() -> Self
    where
        I: InitializerTrait<T, N>,
    {
        Self::new(I::initializer())
    }

    fn one_loop_out<W>(&mut self, _: W) -> Result<(), Self::Err>
    where
        W: Write,
    {
        <Self as GroupTrait<T, N, R>>::one_loop(self)
    }

    fn clone_values(&self) -> Vec<T> {
        self.iter().cloned().collect::<Vec<T>>()
    }
}
