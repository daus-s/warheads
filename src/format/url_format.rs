use crate::format::season::season_fmt;

pub trait UrlFormatter {
    fn url(&self) -> String;
}

impl UrlFormatter for i32 {
    fn url(&self) -> String {
        season_fmt(*self)
    }
}
