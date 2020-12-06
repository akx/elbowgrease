use std::fs::{metadata, File};
use std::io::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crossbeam_queue::SegQueue;
use flate2::read::MultiGzDecoder;
use indicatif::{ParallelProgressIterator, ProgressBar};
use rayon::prelude::*;

use crate::elb_data::LINE_RE;

pub fn read_files_into_queue(
    record_queue: Arc<SegQueue<Vec<String>>>,
    log_files: &[String],
    fields: &[&str],
) -> anyhow::Result<usize> {
    let total_atom = Arc::new(AtomicUsize::new(0));
    log_files
        .par_iter()
        .progress_with(ProgressBar::new(log_files.len() as u64))
        .for_each(
            |filename| match read_file_into_queue(record_queue.clone(), &filename, fields) {
                Err(e) => {
                    eprintln!("Error with file {}: {}", filename, e);
                }
                Ok(n) => {
                    total_atom.fetch_add(n, Ordering::SeqCst);
                }
            },
        );

    let total = total_atom.load(Ordering::Relaxed);
    println!("Parsed {} log records", total);
    Ok(total)
}

fn read_file_into_queue(
    queue: Arc<SegQueue<Vec<String>>>,
    file: &str,
    fields: &[&str],
) -> anyhow::Result<usize> {
    let md = metadata(&file)?;
    if md.len() == 0 {
        return Ok(0);
    }
    let mut s = String::new();
    if file.ends_with(".gz") {
        MultiGzDecoder::new(File::open(&file)?).read_to_string(&mut s)?;
    } else {
        File::open(&file)?.read_to_string(&mut s)?;
    };
    // eprintln!("{}: {}", file, s.len());
    let mut n_rec: usize = 0;
    for line in s.lines() {
        match LINE_RE.captures(&line) {
            None => {
                eprintln!("{}: Invalid line {}", file, line);
            }
            Some(caps) => {
                let mut vec: Vec<String> = Vec::new();
                for n in fields.iter() {
                    vec.push(caps.name(n).unwrap().as_str().parse().unwrap());
                }
                queue.push(vec);
                n_rec += 1;
            }
        }
    }
    Ok(n_rec)
}
