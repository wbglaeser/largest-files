use std::io::Write;
use std::io;
use std::fs;
use std::time::{SystemTime, Duration};
use chrono::{Local, DateTime};
use std::path::PathBuf;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::fmt;
use structopt::StructOpt;
use std::error::Error;
use csv::Writer;
use serde::Serialize;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    #[structopt(short = "e", default_value = " ")]
    pub exclude: String,

    #[structopt(short = "f")]
    pub faulty_files: bool,

    #[structopt(short = "s")]
    pub store_list: bool,
}

#[derive(Serialize)]
struct Row<'a> {
    file_size: f64,
    file_name: &'a str,
    full_path: &'a str,
    created_at: &'a str,
    modified_at: &'a str,
}

pub fn store_results(file_list: &FileList) -> Result<(), Box<dyn Error>> {
    
    // initialise writer
    let mut wtr = Writer::from_path("foo.csv")?;

    for file in file_list.0.iter() {
       
        let created_at: DateTime<Local> = DateTime::from(file.created_at);
        let modified_at: DateTime<Local> = DateTime::from(file.modified_at);

        wtr.serialize(Row{
           file_size: file.file_size,
           file_name: file.file_name.to_str().unwrap(),
           full_path: file.full_path.to_str().unwrap(),
           created_at: &created_at.to_rfc2822(),
           modified_at: &modified_at.to_rfc2822(),
        })?;
    }
    wtr.flush()?;
    //let _data = String::from_utf8(wtr.into_inner()?)?;
    Ok(())
}

pub struct ProgressTracker {
    current_time: SystemTime,
    idx: usize,
    symbols: Vec<String>,
}

impl ProgressTracker {
    pub fn initialise() -> Self {
        
        print!("\x1B[2J\x1B[1;1H");
        
        Self {
            current_time: SystemTime::now(),
            idx: 0,
            symbols: vec![String::from("-"), String::from("\\"), String::from("|"),String::from("/"), String::from("-"), String::from("\\"), String::from("|"), String::from("/")],
        }
    }

    pub fn progress(&mut self) {
        let now = SystemTime::now();
        let duration = Duration::from_millis(150);

        if now.duration_since(self.current_time).unwrap() > duration {
            
            let current_symbol = self.idx % 8;
            
            print!("\rScanning directories {} ", self.symbols.get(current_symbol).unwrap());
            let _drop = io::stdout().flush();

            self.idx = self.idx + 1;
            self.current_time = now;
        }
    }
}

pub struct FileList ( pub Vec<FileEntry> );

impl FileList {
    pub fn update(&mut self, new_file: FileEntry) {

        let mut idx_ = self.0.len();

        // all subsequent entries
        if self.0.len() != 0 {
            for ( idx, entry ) in self.0.iter().enumerate() {
                if new_file.file_size >= entry.file_size {
                    idx_ = idx;
                    break;
                }    
            }
        }

        // insert
        self.0.insert(idx_, new_file);
        
        // remove if too long
        if self.0.len() > 100_000 {
            let _drop = self.0.pop();
        }

    }
}

impl fmt::Display for FileList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}{}{}{}{}\n", self.0.get(0).unwrap(), self.0.get(1).unwrap(), self.0.get(2).unwrap(), self.0.get(3).unwrap(), self.0.get(5).unwrap())
    }
}

pub struct FileEntry {
    pub modified_at: SystemTime,
    pub created_at: SystemTime,
    pub is_directory: bool,
    pub full_path: PathBuf,
    pub file_name: OsString,
    pub file_size: f64,
}

impl FileEntry {
    pub fn parse(dir_entry: &DirEntry) -> Self {
        
        // full file path
        let full_path = dir_entry.path();

        // file name
        let file_name = dir_entry.file_name();
        
        let meta_data = fs::metadata(dir_entry.path()).unwrap();

        // creation date
        let created_at = meta_data.created().unwrap();

        // modification date
        let modified_at = meta_data.modified().unwrap();

        // is directory
        let is_directory = meta_data.is_dir();
        
        // file size
        let divisor: f64 = 1_048_576.0;
        let file_size = meta_data.len() as f64/divisor;

        Self {
            modified_at: modified_at,
            created_at: created_at,
            is_directory: is_directory,
            full_path: full_path,
            file_name: file_name,
            file_size: file_size,
        }
    }
}

impl fmt::Display for FileEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let modified_date: DateTime<Local> = DateTime::from(self.modified_at);
        let created_date: DateTime<Local> = DateTime::from(self.created_at);
        write!(f, "\nSize: {:.4}MB\nFilename: {:?}\nFull path: {:?}\nModified: {:?}\nCreated: {:?}\n", self.file_size, self.file_name, self.full_path, modified_date, created_date)
    }
}

pub fn parse_dir(dir_path: PathBuf, mut file_list: &mut FileList, exclude: &String, faulty: bool, tracker: &mut ProgressTracker ) {
   
    if let Ok(dir_list) = fs::read_dir(dir_path) {
        
        for p in dir_list {
         
            tracker.progress();
            
            if let Ok(path) = p {

                if path.file_name().into_string().unwrap().contains(exclude) {
                    continue;
                }

                if let Ok(metadata) = fs::metadata(&path.path()) {

                    // if file is dir we want to jump in a little deeper
                    if metadata.is_dir() == true {
                        parse_dir(path.path(), &mut file_list, exclude, faulty, tracker);
                    }
        
                    // if file is dir we can skip here
                    if metadata.is_dir() == true {continue;}
                    
                    // add new file to file list
                    let new_file = FileEntry::parse(&path);
                    file_list.update(new_file);
                
                } 
                else { 
                    if faulty {
                        println!("Faulty file: {:?}", path.path()); 
                    }
                }
            }
        }
    }
}
