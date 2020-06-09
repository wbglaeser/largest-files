use structopt::StructOpt;
use largest_files::*;

fn main() {
   
    // setup arguments and file tracker
    let args = Cli::from_args();
    
    // initialse container for files
    let mut file_list = FileList(vec![]);
    let mut tracker = ProgressTracker::initialise();

    // recursively scan directories
    parse_dir(args.path, &mut file_list, &args.exclude, args.faulty_files, &mut tracker);

    // store results to csv
    let _drop = store_results(&file_list);

    // print latest file
    println!("{}", file_list);

}
