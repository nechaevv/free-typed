use core::marker::PhantomData;

// Abstract algebra marker trait
pub trait Algebra {
    type Input;
    type Output;
}

// Composable algebraic structure embedding other algebras
pub trait Free: Algebra {
    type Compose<O: Free<Input = Self::Output>>: Free<Input=Self::Input, Output=O::Output>;
}

// Free algebra chain element embedding single operation and referencing rest of the chain
pub struct Suspend<A: Algebra, N: Free<Input = A::Output>> {
    _a: PhantomData<A>,
    _n: PhantomData<N>
}

impl <A: Algebra, N: Free<Input = A::Output>> Algebra for Suspend<A, N> {
    type Input = A::Input;
    type Output = N::Output;
}

impl<A: Algebra, N: Free<Input = A::Output>> Free for Suspend<A, N> {
    type Compose<O: Free<Input = Self::Output>> = Suspend<A, N::Compose<O>>;
}

// Algebra chain end element
pub struct Return<T> {
    _t: PhantomData<T>
}
impl <T> Algebra for Return<T> {
    type Input = T;
    type Output = T;
}

impl <T> Free for Return<T> {
    type Compose<O: Free<Input = T>> = O;
}

pub type LiftF<A: Algebra> = Suspend<A, Return<A::Output>>;

pub trait IdInterpreter: Algebra {
    fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output;
}

impl <T> IdInterpreter for Return<T> {
    fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output {
        i
    }
}