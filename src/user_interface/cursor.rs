#[derive(Default)]
pub struct Cursor {
    position: Option<(u16, u16)>,
}

impl Cursor {
    pub fn set(&mut self, x: u16, y: u16) {
        self.position = Some((x, y));
    }

    pub fn take(&mut self) -> Option<(u16, u16)> {
        self.position.take()
    }
}
