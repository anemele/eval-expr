use std::env;

mod compute;
mod eval;
mod frac;
mod token;
mod transform;

use compute::compute;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        anyhow::bail!("requires at least one argument");
    }
    for s in &args[1..] {
        match compute(&s) {
            Ok(v) => println!("{s} = {v}"),
            Err(e) => println!("Error: {e}"),
        }
    }
    Ok(())
}
