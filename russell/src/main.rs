use std::io::{self, Read};

use russell_engine::Engine;

fn main() -> anyhow::Result<()> {
    // read input
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let mut buf = String::default();

    lock.read_to_string(&mut buf)?;

    // eval
    let engine = Engine::default();

    println!("{}", engine.eval_str(buf)?);

    Ok(())
}
