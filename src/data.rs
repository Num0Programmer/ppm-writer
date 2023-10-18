use std::{
    fs::File,
    io::{Write, Result}
};

/// Holds metadata for a PPM file
pub struct Header
{
    format: String,
    width: String,
    height: String,
    max: String
}

impl Header
{
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
    fn default() -> Self
    {
        Self
        {
            format: String::from("P3"),
            width: String::from("64"),
            height: String::from("64"),
            max: String::from("255")
        }
    }
}
