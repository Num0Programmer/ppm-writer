/// Holds metadata for a PPM file
pub struct Header
{
    format: String,
    width: String,
    height: String,
    max_color: String
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
            max_color: String::from("255")
        }
    }
}
