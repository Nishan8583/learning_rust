use std::env;
use file_lib::files::list_files;

fn main() {
    println!("My custom file lister");
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide path to list");
        return;
    }

    list_files(args.get(1).unwrap()).unwrap();
}
