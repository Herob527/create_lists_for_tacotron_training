/*
This program divides file list.txt into list_train.txt and list_val.txt.

Before inserting values, it sorts by audio length to ensure the longest files are in validation set
*/

use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

// Structure containing audio data needed further for sorting
struct AudioFileData {
    audio_path: PathBuf,
    text: String,
    audio_length: f64,
}

fn main() {
    if let Ok(lines) = read_lines("./list.txt") {
        const VALIDATION_SHARE: u32 = 10;

        let mut train_file_handler =
            File::create("./list_train.txt").expect("Couldn't create file for training entries");
        let mut val_file_handler =
            File::create("./list_val.txt").expect("Couldn't create file for validation entries");
        let mut files: Vec<AudioFileData> = Vec::new();

        for line in lines {
            if let Ok(entry) = line {
                let split_text: Vec<&str> = entry.split("|").collect();
                let mut audio_path = PathBuf::new();
                audio_path.push(split_text[0]);
                let wav_transcript = split_text[1];
                let wav_length = calculate_wav_length(&audio_path);

                let combined_data = AudioFileData {
                    audio_path,
                    text: wav_transcript.to_string(),
                    audio_length: wav_length,
                };

                files.push(combined_data);
            }
        }

        files.sort_by(|prev, cur| prev.audio_length.partial_cmp(&cur.audio_length).unwrap());
        files.reverse();

        let size: u32 = files.len() as u32;
        let split_index: usize = (size / VALIDATION_SHARE) as usize;

        let (val_data, train_data) = files.split_at(split_index);
        for train_data_line in train_data {
            let (wav, text) = (&train_data_line.audio_path, &train_data_line.text);
            let complete_line = format!("{}|{}\n", wav.as_path().display(), text);
            train_file_handler.write_all(complete_line.as_bytes()).expect("Error while writing data into list_train.txt");            
        }
        for val_data_line in val_data {
            let (wav, text) = (&val_data_line.audio_path, &val_data_line.text);
            let complete_line = format!("{}|{}\n", wav.as_path().display(), text);
            val_file_handler.write_all(complete_line.as_bytes()).expect("Error while writing data into list_val.txt");            
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    return Ok(io::BufReader::new(file).lines());
}

fn calculate_wav_length(wav: &PathBuf) -> f64 {
    let mut input_file: File = File::open(wav).unwrap();
    let file_size: f64 = input_file.metadata().unwrap().len() as f64;

    let (header, _data) = match wav::read(&mut input_file) {
        Ok(f) => f,
        Err(_err) => return 0.0,
    };
    let bytes_per_second: f64 = header.bytes_per_second as f64;
    let seconds: f64 = file_size / bytes_per_second;
    return seconds;
}
