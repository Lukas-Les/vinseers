mod inputs;

use std::env;

use inputs::parse::parse_args;
use inputs::config::Config;


fn main() {
    let args: Vec<String> = env::args().collect();
    let cfg = parse_args(args).expect("Failed configuration!");

    dbg!(cfg);
}
