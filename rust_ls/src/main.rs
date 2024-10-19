use clap::Parser;
use file_lib::files::list_files;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    recurse: Option<u32>,

    #[arg(required = true)]
    path: String,
}

fn main() {
    let args = Args::parse();

    println!("My custom file lister");
    //let args: Vec<String> = env::args().collect();

    let mut path = args.path;
    if path == "" {
        path = ".".to_string();
    }
    //println!("Showing path for {}",&args.path);

    println!("{:<5}{:<20} {:<20}", "Permission", "Size", "Name"); // some printing for clear information
    list_files("".to_owned(), &path, args.recurse.unwrap_or(0)).unwrap();
}
