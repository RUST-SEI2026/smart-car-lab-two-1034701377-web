#[derive(Default, Copy, Clone)]
pub(crate) struct State {
    pub(crate) is_reverse: bool,
}

impl State {
    pub(crate) fn be_reverse(&mut self) {
        self.is_reverse = !self.is_reverse;
    }
}
