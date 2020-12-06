extern crate anyhow;
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::sync::Arc;

use clap::{App, Arg};
use crossbeam_queue::SegQueue;
use walkdir::WalkDir;

use crate::read::read_files_into_queue;

mod elb_data;
mod read;
mod write;

fn find_files(input_dir: &str) -> anyhow::Result<Vec<String>> {
    let mut log_files: Vec<String> = Vec::new();
    for entry in WalkDir::new(input_dir) {
        match entry?.path().to_str() {
            None => {}
            Some(pth) => {
                if pth.ends_with(".log.gz") || pth.ends_with(".log") {
                    log_files.push(pth.to_string());
                }
            }
        }
    }
    log_files.sort();
    Ok(log_files)
}

fn process(input_dir: &str, output_file: &str) -> anyhow::Result<()> {
    let log_files = find_files(input_dir)?;
    println!("Found {} log files", log_files.len());
    let record_queue: Arc<SegQueue<Vec<String>>> = Arc::new(SegQueue::new());

    rayon::scope(|s| {
        s.spawn(|_| {
            read_files_into_queue(record_queue.clone(), log_files).unwrap();
            record_queue.push(vec![]);
        });
        s.spawn(|_| {
            write::write_queue_to_file(record_queue.clone(), output_file).unwrap();
        });
    });

    Ok(())
}

fn main() {
    let matches = App::new("elbowgrease")
        .version("1.0")
        .arg(
            Arg::with_name("input")
                .short("i")
                .required(true)
                .value_name("DIR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .required(true)
                .value_name("FILE")
                .takes_value(true),
        )
        .get_matches();
    let input_dir = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    process(input_dir, output_file).unwrap();
}
