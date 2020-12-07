use crossbeam_queue::SegQueue;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

const RECORD_SEP: &str = "\n";

pub fn write_queue_to_file(
    queue: Arc<SegQueue<Vec<String>>>,
    filename: &str,
    fields: &[&str],
    use_tab: bool,
) -> anyhow::Result<()> {
    let file = File::create(filename)?;
    let mut file = BufWriter::with_capacity(524_288, file);
    let mut total: usize = 0;
    let field_sep = if use_tab { "\t" } else { "\x1F" };

    let vec: Vec<String> = fields.iter().map(|s| String::from(*s)).collect();
    write_line(&mut file, &vec, field_sep, RECORD_SEP)?;
    loop {
        match queue.pop() {
            None => {}
            Some(svec) => {
                if svec.is_empty() {
                    // Final element marker
                    break;
                }
                write_line(&mut file, svec.as_ref(), field_sep, RECORD_SEP)?;
                total += 1;
            }
        }
    }

    println!("Wrote {} log records", total);
    Ok(())
}

fn write_line(
    file: &mut BufWriter<File>,
    svec: &[String],
    field_sep: &str,
    record_sep: &str,
) -> anyhow::Result<()> {
    for (index, item) in svec.iter().enumerate() {
        file.write_all(item.as_ref())?;
        let sep = if index == svec.len() - 1 {
            record_sep
        } else {
            field_sep
        };
        file.write_all(sep.as_ref())?;
    }
    Ok(())
}
