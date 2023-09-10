use std::thread;
use std::time::{Duration, Instant};
use std::{error::Error, io, process};


#[derive(Debug,serde:Deserialize)]
struct Video {
    video_id: String,
    trending_date: Date,
    title: String,
    channel_title: String,
    category_id: i32,
    publish_time: Date,
    tags: String,
    views: i32,
    likes: i32,
    dislikes: i32,
    comment_count: i32,
    thumbnail_link: String,
    comments_disabled: bool,
    ratings_disabled: bool,
    video_error_or_removed: bool,
    description: String
}

fn main() {

    let data = get_data();
    let data = [7, 3, 2, 16, 24, 4, 11, 9];

    rayon::ThreadPoolBuilder::new().build_global();

    let start = Instant::now();
    println!("{:?}", start.elapsed());

    println!("{:?}", merged);
}


fn get_data() -> Result<Vec<Video>, Box<dyn Error>> {
    let mut data : Vec<Video> = vec![];
    let mut rdr = csv::Reader::from_reader("../data/USvideos.csv");
    for result in rdr.desirialize() {
        let video: Video = result?;
        data.push(video);
    }
    return data;
}
