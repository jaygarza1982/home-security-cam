use std::{fs};
use chrono::{DateTime, ParseResult, FixedOffset, NaiveDateTime, Utc};

#[allow(dead_code)]

fn date_from_filename(filename: &str) {
    println!("Processing video {}", filename);

    // Obtain file timestamp
    let fileSplit: Vec<&str> = filename.split("file").collect();
    let timeStr: &str = fileSplit[0];
    let timestamp = timeStr.parse::<i64>().unwrap();

    let naive = NaiveDateTime::from_timestamp(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    // TODO: construct current date and check if file was created N seconds after current date

    println!("datetime {}", datetime);
}

fn main() {
    let videos_path = "./videos";

    println!("Listing from {}", videos_path);

    let paths = fs::read_dir(videos_path).unwrap();

    for path in paths {
        // Get video filename without extension
        let video_file = String::from(
            path.unwrap().path().file_stem().unwrap().to_str().unwrap()
        );

       date_from_filename(&video_file)
    }
}
