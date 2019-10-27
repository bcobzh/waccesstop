#[macro_use]
extern crate clap;

use std::path::Path;
use std::process;



fn main() {
    let yaml = load_yaml!("cli.yml");
    let args = clap::App::from_yaml(yaml).get_matches();

    let file = args.value_of("INPUT").unwrap();
    let myregex = args.value_of("regex").unwrap_or(".");
    let request_lines = args.value_of("lines").unwrap_or("10");

    match Path::new(&file).is_file() {
        false => { println!("file '{}' not found", file);
                   process::exit(1);
        }
        true => {}
    }
    let output_lines = match request_lines.parse::<usize>() {
        Ok(val) => val,
        Err(_) => {println!("!Request output number of line have to be an integer, \n\
                              I use 10 as default \n\
                              ");
                    10}
    };
    
    match myregex {
        "." =>  waccesstop::all_file(&file, output_lines ),
        _ => waccesstop::filter_lines(&file, &myregex, output_lines)
    };
}
