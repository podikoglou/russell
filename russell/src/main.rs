use std::{
    collections::HashMap,
    env::args,
    io::{self, Read},
};

use anyhow::bail;
use russell_engine::Engine;

fn main() -> anyhow::Result<()> {
    // read input
    let stdin = io::stdin();
    let mut lock = stdin.lock();

    let mut buf = String::default();

    lock.read_to_string(&mut buf)?;

    // read assignments from cli args (todo: don't do that...)
    let mut assignments: HashMap<char, bool> = HashMap::default();

    for arg in args().skip(1) {
        let split: Vec<&str> = arg.split("=").collect();

        let first = split.first().expect("bad kv pair!");
        let last = split.last().expect("bad kv pair!");

        let key = if first.len() != 1 {
            bail!("variable names must be 1 character long!");
        } else {
            first.chars().last().unwrap()
        };

        let value = match last {
            &"true" => true,
            &"false" => false,

            _ => bail!("invalid value (must be true or false)"),
        };

        assignments.insert(key, value);
    }

    // eval
    let engine = Engine::default();

    println!("{}", engine.eval_str(buf, &assignments)?);

    Ok(())
}
