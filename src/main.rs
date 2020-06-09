use structopt::StructOpt;
use largest_files::{parse_dir, Cli, FileList, ProgressTracker};

fn main() {
   
    // setup arguments and file tracker
    let args = Cli::from_args();
    
    // initialse container for files
    let mut file_list = FileList(vec![]);
    let mut tracker = ProgressTracker::initialise();

    // recursively scan directories
    parse_dir(args.path, &mut file_list, &args.exclude, args.faulty_files, &mut tracker);

    // print latest file
    println!("{}", file_list);

}
