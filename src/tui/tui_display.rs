use std::error::Error;

pub trait TuiDisplay {
    fn display(&self) -> Result<String, Box<dyn Error>>;
}
