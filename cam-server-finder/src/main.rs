use std::{fs, time::SystemTime};

fn main() {
    let videos_path = "./videos";

    println!("Listing from {}", videos_path);

    let paths = fs::read_dir(videos_path).unwrap();

    let time = SystemTime::now();
    // TODO: Figure out how to parse time from files read, construct date time, compare with current date time to see if X seconds have passed

    for path in paths {
        // Get video filename without extension
        let video_file = String::from(
            path.unwrap().path().file_stem().unwrap().to_str().unwrap()
        );

        println!("Processing video {}", video_file);

        // Obtain time portion of filename
        let time_split: Vec<&str> = video_file.split("_").collect();
        println!("{}", time_split[1].to_string());
    }
}
