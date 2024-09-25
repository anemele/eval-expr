use std::env;

mod compute;
mod eval;
mod token;
mod transform;
use compute::compute;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{} <expr>...", args[0]);
    } else {
        for s in &args[1..] {
            if let Ok(v) = compute(&s) {
                println!("{s} = {v}");
            } else {
                println!("invalid expression: {s}")
            }
        }
    }
    Ok(())
}
