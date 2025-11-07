use std::fs::File;
use std::path::PathBuf;
use std::io::{self, Read, Seek, SeekFrom, Write};
use clap::Parser;

const CHUNK_SIZE: usize = 8192;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short = 'n', long = "lines")]
    lines_count: i64,

    file_name: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let mut file = File::open(&args.file_name)?;
    let file_size = file.metadata()?.len();

    let mut chunk_position: i64 = file_size as i64;
    let mut newline_count = 0;
    let mut tail_bytes = Vec::new();
    let mut chunk: [u8; CHUNK_SIZE] = [0; CHUNK_SIZE];

    while chunk_position > 0 {
        chunk.fill(0);
        let read_size = CHUNK_SIZE.min(chunk_position as usize) as u64;
        chunk_position -= read_size as i64;

        file.seek(SeekFrom::Start(chunk_position as u64))?;
        file.read(&mut chunk[..read_size as usize])?;

        for &byte in chunk.iter().rev() {
            if byte == b'\n' {
                newline_count += 1;
                if newline_count > args.lines_count {
                    break;
                }
            }
            tail_bytes.push(byte);
        }
    }

    tail_bytes.reverse();
    std::io::stdout().write_all(&tail_bytes)?;
    Ok(())
}
