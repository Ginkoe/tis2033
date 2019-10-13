#[derive(Debug)]
pub enum INSTRUCTIONS {
    SWP,
    SAV,
    MOV(LOCATION, LOCATION),
    JMP(String),
    JEZ(String),
    JLZ(String),
    JGZ(String),
    ADD(i16),
    SUB(i16),
    IGNORE
}

#[derive(Debug, PartialEq)]
pub enum LOCATION {
    VALUE(i16),
    RIGHT,
    LEFT,
    DOWN,
    UP,
    REGISTER(u8),
    ACC,
    DISPLAY
}