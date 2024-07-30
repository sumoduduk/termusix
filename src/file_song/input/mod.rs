mod crossterm;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Input {
    Up,
    Down,
    Left,
    Right,
    None,
}
