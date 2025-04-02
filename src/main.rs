use ablf::{BlfFile, ObjectTypes};
use acc_reader::AccReader;
use anyhow::Result;
use std::{fmt::Write, io::stdin};

fn main() -> Result<()> {
    let reader = AccReader::new(stdin());
    let blf = BlfFile::from_reader(reader).unwrap_or_else(|_| panic!("Unable to read file."));
    // date Fri Sep 6 10:07:01.310 am 2024
    let date = chrono::Local::now().format("%a %b %d %T.000 %P %Y");
    println!(
        "date {date}
base hex  timestamps absolute
internal events logged
// version 10.0.0"
    );
    for obj in blf {
        match obj.data {
            ObjectTypes::CanMessage86(m) => {
                let time = m.header.timestamp_ns as f64 / 1_000_000_000.0;
                let header = m.id & 0xFFFFFF;
                let priority = 0x1F & (m.id >> 24);
                let payload = as_hex(&m.data);
                let channel = m.channel;
                let txrx = match m.flags {
                    0x0 => "Rx",
                    0x1 => "Tx",
                    unknown => &format!("{unknown:2}"),
                };
                let length = m.data.len();
                println!(
                    "{time:11.6} {channel:<2} {priority:02X}{header:06X}x {txrx:>8}   d {length} {payload}"
                )
            }
            ObjectTypes::CanErrorExt73(can_error_frame_ext) => {
                eprintln!("CanErrorExt73: {can_error_frame_ext:?}")
            }
            ObjectTypes::LogContainer10(junk) => eprintln!("LogContainer10: {junk:?}"),
            ObjectTypes::AppText65(app_text) => eprintln!("{app_text:?}"),
            ObjectTypes::UnsupportedPadded { _last_data } => eprintln!("{_last_data:?}"),
            ObjectTypes::Unsupported(junk) => eprintln!("unsupported: {junk:?}"),
        }
    }
    Ok(())
}
fn as_hex(data: &[u8]) -> String {
    if data.is_empty() {
        return "".to_string();
    }
    // FIXME optimize
    let mut s = String::new();
    for byte in data {
        write!(&mut s, " {:02X}", byte).expect("Unable to write");
    }
    s[1..].to_string()
}
