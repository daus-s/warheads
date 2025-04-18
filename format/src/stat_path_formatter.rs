pub trait StatPathFormatter {
    fn path_specifier(&self) -> &'static str;

    fn ext(&self) -> &'static str;

}
