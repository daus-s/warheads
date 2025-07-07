pub trait Makes {
    fn makes(&self) -> u8;
}

pub trait Attempts {
    fn attempts(&self) -> Option<u8>;
}
