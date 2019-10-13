#[derive(Debug)]
pub enum INSTRUCTIONS {
    JMP(usize),
    SWP,
    SAV,
    MOV(LOCATION, LOCATION),
    JEZ(usize),
    JLZ(usize),
    JGZ(usize),
    ADD(i16),
    SUB(i16)
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