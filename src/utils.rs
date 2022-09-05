pub enum Direction {
    Up,
    Down,
    Right,
    Left
}


pub enum Todo {
    Quit,
    Write(char),
    Save,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    //columns, rows
    Resize(u16, u16)
}