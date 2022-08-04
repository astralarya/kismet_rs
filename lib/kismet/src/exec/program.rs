use super::Value;

pub struct Program<I, O, S> {
    pub sym: S,
    pub code: Function<I, O, S>,
}

pub type Function<I, O, S> = dyn Fn(I, S) -> O;

pub trait Exec {
    type Input;
    type Output;

    fn exec(&self, i: Self::Input) -> Self::Output;
}

impl<I, O, S> Exec for Program<I, O, S>
where
    S: Clone,
{
    type Input = I;
    type Output = O;

    fn exec(&self, args: Self::Input) -> Self::Output {
        (self.code)(args, self.sym.clone())
    }
}
