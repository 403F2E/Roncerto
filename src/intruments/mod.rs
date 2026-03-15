mod guitar;
mod piano;

pub(crate) use guitar::Guitar;
pub(crate) use piano::Piano;

pub(crate) trait Instrument {
    fn run(&mut self) -> ();
    fn load_notes(&self) -> Option<std::collections::HashMap<char, Vec<u8>>>;
}
