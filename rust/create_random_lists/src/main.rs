use std::fs::File;
use std::io::{self, BufRead,Write};
use std::path::Path;
use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;



fn main() {
    if let Ok(lines) = read_lines("./list.txt") {
        const VALIDATION_SHARE: u32 = 10;
        
        let mut train_file_handler = File::create("./list_train.txt").expect("Couldn't create file for training entries"); 
        let mut val_file_handler = File::create("./list_val.txt").expect("Couldn't create file for validation entries");
        let mut files: Vec<String> = Vec::new();

        for line in lines {
            if let Ok(mut entry) = line {
                entry.push_str("\n");
                files.push(entry);
            }
        }
        let size: u32 = files.len() as u32;
        let split_index: usize = (size / VALIDATION_SHARE) as usize;

        let mut rng = StepRng::new(2, 13);
        let mut irs = Irs::default();

        irs.shuffle(&mut files, &mut rng).expect("Error");

        let (val_data, train_data) = files.split_at(split_index);
        for train_data_line in train_data {
            train_file_handler.write_all(train_data_line.as_bytes()).expect("Error while writing data into list_train.txt");
        }
        for val_data_line in val_data {
            val_file_handler.write_all(val_data_line.as_bytes()).expect("Error while writing data into list_val.txt");
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines())
}