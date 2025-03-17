pub trait StatPathFormatter {
    fn epath(&self) -> &'static str;

    fn ext(&self) -> &'static str;

    fn dbg_open(&self, season: i32) -> String;

    fn dbg_write(&self, season: i32) -> String;
}
