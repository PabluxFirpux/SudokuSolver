#[derive(Debug)]
pub struct Cell {
    pub value: u32,
    pub x_pos: u32,
    pub y_pos: u32,
}

impl Cell {
    pub fn set_value(&mut self, value: u32) {
        self.value = value
    }
}