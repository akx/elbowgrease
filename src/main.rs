extern crate anyhow;
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::sync::Arc;

use clap::{App, Arg};
use crossbeam_queue::SegQueue;
use walkdir::WalkDir;

use crate::elb_data::FIELD_NAMES;
use crate::read::read_files_into_queue;

mod elb_data;
mod read;
mod write;

struct Config<'a> {
    pub input_dir: &'a str,
    pub output_file: &'a str,
    pub fields: Vec<&'a str>,
    pub use_tab: bool,
}

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

fn process(config: &Config) -> anyhow::Result<()> {
    let log_files = find_files(config.input_dir)?;
    println!("Found {} log files", log_files.len());
    let record_queue: Arc<SegQueue<Vec<String>>> = Arc::new(SegQueue::new());

    rayon::scope(|s| {
        s.spawn(|_| {
            read_files_into_queue(record_queue.clone(), &log_files, &config.fields).unwrap();
            record_queue.push(vec![]);
        });
        s.spawn(|_| {
            write::write_queue_to_file(
                record_queue.clone(),
                config.output_file,
                &config.fields,
                config.use_tab,
            )
            .unwrap();
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
                .help("Input directory")
                .value_name("DIR")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .required(true)
                .help("Output here")
                .value_name("FILE")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("keep")
                .short("k")
                .multiple(true)
                .value_name("field")
                .takes_value(true)
                .help("Keep these fields")
                .possible_values(&FIELD_NAMES),
        )
        .arg(Arg::with_name("tab").long("tab").help("Use tab separator"))
        .get_matches();
    let input_dir = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let fields: Vec<&str> = matches
        .values_of("keep")
        .map_or_else(|| FIELD_NAMES.iter().copied().collect(), |m| m.collect());
    let use_tab = matches.is_present("tab");

    let config = Config {
        input_dir,
        output_file,
        fields,
        use_tab,
    };
    process(&config).unwrap();
}
