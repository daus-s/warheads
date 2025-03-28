pub trait BoxScore {
    fn season(&self) -> i32;

    fn game(&self) -> String;

    fn id(&self) -> u64;
}