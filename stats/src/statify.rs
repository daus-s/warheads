pub trait Statify<T> {
    fn unwrap_f(&self, default: &str) -> String;
}
impl<T> Statify<T> for Option<T>
where
    T: ToString,
{
    fn unwrap_f(&self, default: &str) -> String {
        match self {
            Some(t) => t.to_string(),
            None => default.to_string(),
        }
    }
}