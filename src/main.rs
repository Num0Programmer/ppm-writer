use std::{env, io::Result};
use data::Header;

fn main() -> Result<()>
{
    // TODO: incorporate invokation arguments later
    // let args: Vec<_> = env::args().collect();

    let header = Header::default();
    let buf: Vec<u8> = vec![0; 12288];  // magic number for 64 * 64 * 3

    header.write_buf("sample.ppm", &buf)?;

    Ok(())
}
