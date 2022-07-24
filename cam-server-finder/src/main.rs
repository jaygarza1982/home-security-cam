use std::{fs, num::ParseIntError};
use chrono::{DateTime, NaiveDateTime, Utc, Duration};

#[allow(dead_code)]

fn get_file_age(filename: &str) -> Result<i64, ParseIntError> {
    // Obtain file timestamp
    let file_split: Vec<&str> = filename.split("file").collect();
    let time_str: &str = file_split[0];
    let timestamp: i64 = time_str.parse::<i64>()?;

    // Convert to datetime
    let naive: NaiveDateTime = NaiveDateTime::from_timestamp(timestamp, 0);
    let file_date: DateTime<Utc> = DateTime::from_utc(naive, Utc);
    
    // Find difference between now and file creation
    let now: DateTime<Utc> = Utc::now();
    let difference: Duration = now - file_date;
    let seconds: i64 = difference.num_seconds();

    return Ok(seconds);
}

fn process_file(filename: &str) {
    println!("Processing video because was older than 10 secs {}", filename);
}

fn main() {
    let videos_path = "./videos";

    println!("Listing from {}", videos_path);

    // TODO: Hashset of files seen and only process ones that have not been seen
    // TODO: Infinite loop
    let paths = fs::read_dir(videos_path).unwrap();

    for path in paths {
        // Get video filename without extension
        let video_file = String::from(
            path.unwrap().path().file_stem().unwrap().to_str().unwrap()
        );

        // TODO: handle error
        let age_in_secs: i64 = get_file_age(&video_file).unwrap();

        if age_in_secs > 10 {
            process_file(&video_file);
        }
    }
}
