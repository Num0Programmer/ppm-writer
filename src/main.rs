use std::env;
use ppm_writer::*;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() < 4
    {
        println!("Missing an image filepath/name, width and height!");
    }
    else
    {        
        let header = Header::new(
            args[WIDTH_ACCESS].parse().unwrap(),
            args[HEIGHT_ACCESS].parse().unwrap()
        );
        let buf: Vec<u8> = vec![0; header.width * header.height * 3];

        header.write_buffer(&args[FILEPATH_ACCESS], &buf);
    }
}
