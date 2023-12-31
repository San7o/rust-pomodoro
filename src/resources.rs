#[derive(Debug, Default, PartialEq)]
pub enum State {
    Running,
    #[default]
    Stopped,
    Finished
}
