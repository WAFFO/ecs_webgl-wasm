

pub struct KeyMap {
    board: [bool;256],
    pub FORWARD: usize,
    pub BACKWARD: usize,
    pub LEFTWARD: usize,
    pub RIGHTWARD: usize,
    pub UP: usize,
    pub DOWN: usize,
    pub LEFT: usize,
    pub RIGHT: usize,
}

impl KeyMap {
    pub fn new() -> KeyMap {
        KeyMap {
            board: [false;256],
            FORWARD: 87,
            BACKWARD: 83,
            LEFTWARD: 65,
            RIGHTWARD: 68,
            UP: 38,
            DOWN: 40,
            LEFT: 37,
            RIGHT: 39,
        }
    }
    pub fn press(&self, key: usize) {
        board[key] = true;
    }
    pub fn release(&self, key: usize) {
        board[key] = false;
    }
    pub fn key(&self, key: usize) -> bool {
        board[key]
    }
}
