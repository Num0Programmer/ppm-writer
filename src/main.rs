use std::env;
use data::Header;

fn main()
{
    let args: Vec<String> = env::args().collect();

    let header = Header::new(
        args[1].parse().unwrap(),
        args[2].parse().unwrap()
    );
    let buf: Vec<u8> = vec![0; header.width * header.height * 3];

    header.write_buf(&args[0], &buf);
}
