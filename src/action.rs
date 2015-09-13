// action.rs

pub enum SudokuAction {
    Quit,

    // AssignValue(x,y,v)
    AssignValue(u8,u8,u8),

    // CrossOutValue(x,y,v)
    CrossOutValue(u8,u8,u8),
}
