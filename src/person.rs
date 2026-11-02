#[derive(Debug)]
pub struct Person {
    pub name: &'static str,
    pub year: usize,
    pub month: usize,
    pub day: usize,
}