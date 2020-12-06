use crossbeam_queue::SegQueue;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

const FIELD_SEP: &str = "\x1F";
const RECORD_SEP: &str = "\n";

pub fn write_queue_to_file(
    queue: Arc<SegQueue<Vec<String>>>,
    filename: &str,
    fields: &[&str],
) -> anyhow::Result<()> {
    let file = File::create(filename)?;
    let mut file = BufWriter::with_capacity(524_288, file);

    let mut total: usize = 0;

    let vec: Vec<String> = fields.iter().map(|s| String::from(*s)).collect();
    write_line(&mut file, &vec)?;
    loop {
        match queue.pop() {
            None => {}
            Some(svec) => {
                if svec.is_empty() {
                    // Final element marker
                    break;
                }
                write_line(&mut file, svec.as_ref())?;
                total += 1;
            }
        }
    }

    println!("Wrote {} log records", total);
    Ok(())
}

fn write_line(file: &mut BufWriter<File>, svec: &[String]) -> anyhow::Result<()> {
    for (index, item) in svec.iter().enumerate() {
        file.write_all(item.as_ref())?;
        let sep = if index == svec.len() - 1 {
            RECORD_SEP
        } else {
            FIELD_SEP
        };
        file.write_all(sep.as_ref())?;
    }
    Ok(())
}
