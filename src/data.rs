use std::{
    fs::File,
    io::{Write, Result}
};

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
    
    pub fn write_buf(self, path: &str, buf: &Vec<u8>) -> Result<()>
    {
        let mut file = File::create(path)?;
        writeln!(&mut file, "{}", self.format)?;
        writeln!(&mut file, "{} {}", self.width, self.height)?;
        writeln!(&mut file, "{}", self.max)?;

        for byte in buf
        {
            writeln!(&mut file, "{}", byte)?;
        }

        Ok(())
    }
}

impl Default for Header
{
    /// forces a P3 ppm file with dimensions of 64 64 and max color value of
    /// 255
    fn default() -> Self
    {
        Self
        {
            format: String::from("P3"),
            width: 64,
            height: 64,
            max: 255
        }
    }
}
