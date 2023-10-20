use std::{
    fs::File,
    io::Write
};

pub const FILEPATH_ACCESS: usize = 1;
pub const WIDTH_ACCESS: usize = 2;
pub const HEIGHT_ACCESS: usize = 3;

/// Holds metadata for a PPM file
pub struct Header
{
    format: String,
    pub width: usize,
    pub height: usize,
    max: usize
}

impl Header
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Self
        {
            format: String::from("P3"),
            width,
            height,
            max: 255
        }
    }
    
    pub fn write_buf(self, path: &str, buf: &Vec<u8>)
    {
        let mut file = File::create(path).unwrap();
        writeln!(&mut file, "{}", self.format).unwrap();
        writeln!(&mut file, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut file, "{}", self.max).unwrap();

        for byte in buf
        {
            writeln!(&mut file, "{}", byte).unwrap();
        }
    }
}
