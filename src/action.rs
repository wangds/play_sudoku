// action.rs

pub enum SudokuAction {
    NoOp,
    Quit,

    // New(filename)
    New(Option<String>),

    Undo,
    Redo,

    // AssignValue(x,y,v)
    AssignValue(u8,u8,u8),

    // UnassignValue(x,y)
    UnassignValue(u8,u8),

    // CrossOutValue(x,y,v)
    CrossOutValue(u8,u8,u8),
}
