pub enum ActionEnum {
    FORWARD,
    LEFT,
    RIGHT,
    FLASH,
    HALT,
}

pub const ACTION_FLASH_STRING: &'static str = "f";
pub const ACTION_HALT_STRING: &'static str = "h";
pub const ACTION_FORWARD_STRING: &'static str = "w";
pub const ACTION_LEFT_STRING: &'static str = "a";
pub const ACTION_RIGHT_STRING: &'static str = "d";

pub const ACTION_STRINGS: &'static [&str; 5] = &[
    ACTION_LEFT_STRING,
    ACTION_RIGHT_STRING,
    ACTION_FORWARD_STRING,
    ACTION_FLASH_STRING,
    ACTION_HALT_STRING,
];
