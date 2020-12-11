use std::{env, process};
use minigrep::Cfg;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let cfg = Cfg::new(&args).unwrap_or_else(|err|{
        eprintln!("Could not parse arguments: {}", err);
        process::exit(1);
    });
    if let Err(e) = minigrep::run(cfg){
        eprintln!("App err : {}", e);
        process::exit(1);
    };
}
