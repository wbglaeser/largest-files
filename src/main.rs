use structopt::StructOpt;
use largest_files::{parse_dir, Cli, FileList};

fn main() {
   
    // setup arguments and file tracker
    let args = Cli::from_args();
    
    // initialse container for files
    let mut file_list = FileList(vec![]);

    // recursively scan directories
    parse_dir(args.path, &mut file_list, &args.exclude, args.faulty_files);

    // print latest file
    println!("{}", file_list);

}
