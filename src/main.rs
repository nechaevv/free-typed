use std::cell::RefCell;
use std::error::Error;
use std::io::Result;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::async_free::AsyncInterpreter;
use crate::free::{Algebra, Free, LiftF};

mod free;
mod async_free;

struct TcpConnect;
type TcpConnectF = LiftF<TcpConnect>;
impl Algebra for TcpConnect {
    type Input = ();
    type Output = Result<RefCell<TcpStream>>;
}
impl AsyncInterpreter for TcpConnect {
    async fn apply(_: <Self as Algebra>::Input) -> <Self as Algebra>::Output {
        println!("Hello, world!");
        let stream = TcpStream::connect("127.0.0.1:6142").await?;
        Ok(RefCell::new(stream))
    }
}
struct WriteHello;
type WriteHelloF = LiftF<WriteHello>;
impl Algebra for WriteHello {
    type Input  = Result<RefCell<TcpStream>>;
    type Output = Result<RefCell<TcpStream>>;
}
impl AsyncInterpreter for WriteHello {
    async fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output {
        let stream = i?;
        stream.borrow_mut().write_all(b"Hello").await?;
        Ok(stream)
    }
}

struct WriteWorld;
type WriteWorldF = LiftF<WriteWorld>;
impl Algebra for WriteWorld {
    type Input = Result<RefCell<TcpStream>>;
    type Output = Result<RefCell<TcpStream>>;
}

impl AsyncInterpreter for WriteWorld {
    async fn apply(i: <Self as Algebra>::Input) -> <Self as Algebra>::Output {
        let stream = i?;
        stream.borrow_mut().write_all(b"World").await?;
        Ok(stream)
    }
}

type Prog = <<TcpConnectF as Free>::Compose<WriteHelloF> as Free>::Compose<WriteWorldF>;

#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let result = Prog::apply(()).await?;

    Ok(())
}
