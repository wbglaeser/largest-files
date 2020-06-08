use std::fs;
use std::time::SystemTime;
use chrono::{Local, DateTime};
use std::path::PathBuf;
use std::ffi::OsString;
use std::fs::DirEntry;
use std::fmt;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(parse(from_os_str))]
    pub path: std::path::PathBuf,

    #[structopt(short = "e", default_value = " ")]
    pub exclude: String,

    #[structopt(short = "f")]
    pub faulty_files: bool,
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
        if self.0.len() > 100 {
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

pub fn parse_dir(dir_path: PathBuf, mut file_list: &mut FileList, exclude: &String, faulty: bool) {
    
    if let Ok(dir_list) = fs::read_dir(dir_path) {
        
        for p in dir_list {
        
            if let Ok(path) = p {

                if path.file_name().into_string().unwrap().contains(exclude) {
                    continue;
                }

                if let Ok(metadata) = fs::metadata(&path.path()) {

                    // if file is dir we want to jump in a little deeper
                    if metadata.is_dir() == true {
                        parse_dir(path.path(), &mut file_list, exclude, faulty);
                    }
        
                    // if file is dir we can skip here
                    if metadata.is_dir() == true {continue;}
                    
                    // add new file to file list
                    let new_file = FileEntry::parse(&path);
                    file_list.update(new_file);
                
                } else { 
                    if faulty {
                        println!("Faulty file: {:?}", path.path()); 
                    }
                }
            } else { println!("Faulty file"); }
        }
    } else { println!("Faulty file"); }
}
