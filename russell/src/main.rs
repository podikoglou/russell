use std::{
    collections::{HashMap, HashSet},
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

    let expr = engine.parse(buf)?;
    let variables = engine.collect_variables(&expr);

    // check if contradiction / tautology
    if assignments.len() == 0 {
        dbg!(engine.check_tautology(expr)?);

        return Ok(());
    }

    // if we simply don't have enough assignments as we have variables
    if assignments.len() != variables.len() {
        let unassigned_vars = variables
            .iter()
            .filter(|symbol| !assignments.contains_key(*symbol))
            .collect::<HashSet<_>>()
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        bail!(
            "not enough assignments! ({} are unassigned)",
            unassigned_vars
        );
    }

    // we have enough assignments, and we can simply evaluate the expression
    println!("{}", engine.eval(expr, &assignments)?);

    Ok(())
}
