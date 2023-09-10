use std::{fs, collections::HashMap};
use std::time::Instant;
use std::error::Error;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};


#[derive(Debug)]
struct Video {
    channel_title: String,
    views: i64,
}

fn main() {
    let start : Instant = Instant::now();
    let dirs : Vec<String> = fs::read_dir("./archive").unwrap().map(|res| {
        res.unwrap().path().to_str().unwrap().to_string()
    }).collect(); 
    
    let result = dirs.par_iter().flat_map(|dir| {
        get_data(dir.to_string())
    }).map(|data| {
        let mut map : HashMap<String, i64> = HashMap::new();
        for value in data {
            map = get_count_from_data(value, map);
        }
        map
    }).reduce(|| HashMap::new(), |mut acc, counts| {
        for (key, value) in counts {
            let channel = acc.entry(key).or_insert_with(|| value);
            *channel += value;
        }
        acc
    }); 

    // let result = dirs.iter().flat_map(|dir| {
    //     get_data(dir.to_string())
    // }).map(|data| {
    //     let mut map : HashMap<String, i64> = HashMap::new();
    //     for value in data {
    //         map = get_count_from_data(value, map);
    //     }
    //     map
    // }).fold(HashMap::new() , |mut acc, counts| {
    //     for (key, value) in counts {
    //         let channel = acc.entry(key).or_insert_with(|| value);
    //         *channel += value;
    //     }
    //     acc
    // }); 
    println!("{:?}", start.elapsed());

    //println!("{:?}", result);
}

fn get_count_from_data(data : Video, mut map : HashMap<String, i64>) -> HashMap<String, i64> {
    let channel = map.entry(data.channel_title).or_insert_with(|| 0);
    *channel += data.views;
    return map;
}


fn get_data(dir : String) -> Result<Vec<Video>, Box<dyn Error>> {
    let mut data : Vec<Video> = vec![];
    let mut rdr = csv::Reader::from_path(dir)?;
    for result in rdr.records() {
        let res = result?;
        let video: Video = Video { channel_title: String::from(&res[3]), views: res[7].parse::<i64>().unwrap() };
        data.push(video);
    }
    return Ok(data);
}
