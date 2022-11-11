#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left
}


#[derive(Debug, Clone)]
pub enum Events {
    Quit,
    Write(char),
    Save,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Space,
    Erase,
    //columns, rows
    Resize(u16, u16)
}