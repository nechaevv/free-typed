use crate::free::{Algebra, Free, IdInterpreter, Suspend};
pub trait AsyncInterpreter: Algebra {
    async fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output;
}

// Lift every pure interpreter into Async
impl <I: IdInterpreter> AsyncInterpreter for I {
    async fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output {
        I::apply(i)
    }
}

impl <A: AsyncInterpreter,
      N: Free<Input = A::Output> + AsyncInterpreter> AsyncInterpreter for Suspend<A, N> {
    async fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output {
        let a = A::apply(i).await;
        let n = N::apply(a).await;
        n
    }
}
