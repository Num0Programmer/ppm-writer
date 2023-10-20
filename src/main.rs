use std::{env, io::Result};
use data::Header;

fn main() -> Result<()>
{
    let args: Vec<String> = env::args().collect();

    let header = Header::new(
        args[0].parse().unwrap(),
        args[1].parse().unwrap()
    );
    let buf: Vec<u8> = vec![0; header.width * header.height * 3];

    header.write_buf("sample.ppm", &buf)?;

    Ok(())
}
