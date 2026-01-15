use enigo::Direction;
use std::fmt::Display;

/// Controlling element for every item in a macro.
///
///
/// ```txt
/// Share/Store this in a way of
/// |----||- -------| |------- -------- -------- --||---- -------- -------|
/// 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
/// press/releaase/click (2), mouse/key (1), data(13), length (27), repeat (21)
/// ```
#[derive(Default, Clone, Copy, Debug)]
pub struct RawMacroEntry {
    /// The direction of a key or button
    pub direction: Direction,
    /// The input we are going with
    pub macro_type: MacroType,
    /// The data of the input
    pub data: u16,
    /// The length of time (in ms) of the input
    pub length: u32,
    /// How many times can we repeat said input.
    pub repeat: u32,
}

impl Display for RawMacroEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}ms repeat {} times",
            match self.direction {
                Direction::Press => "Press",
                Direction::Release => "Release",
                Direction::Click => "Click",
            },
            self.macro_type,
            self.data,
            // match self.macro_type {
            //     MacroType::Mouse => MouseAction::try_from(self.data)
            //         .expect("How to deal with this...")
            //         .to_string(),
            //     MacroType::Key => format!("{}", self.data),
            // },
            if self.direction == Direction::Click {
                "every"
            } else {
                "for"
            },
            self.length,
            self.repeat,
        )
    }
}

impl TryFrom<u64> for RawMacroEntry {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // Extract direction (2 bits)
        let direction_bits = (value >> 62) & 0x3;
        let direction = match direction_bits {
            0b00 => Direction::Press,
            0b01 => Direction::Release,
            0b10 => Direction::Click,
            _ => return Err(format!("Invalid direction bits ({:?})", direction_bits)),
        };

        // Extract macro_type (1 bit)
        let macro_type_bit = (value >> 61) & 0x1;
        let macro_type = match macro_type_bit {
            0 => MacroType::Mouse,
            1 => MacroType::Key,
            _ => return Err("An error that should not be reached...".into()),
        };

        // Extract data (13 bits)
        let data = ((value >> 48) & 0x1FFF) as u16;

        // Extract length (27 bits)
        let length = ((value >> 21) & 0x7FFFFFF) as u32;

        // Extract repeat (21 bits)
        let repeat = (value & 0x1FFFFF) as u32;

        Ok(RawMacroEntry {
            direction,
            macro_type,
            data,
            length,
            repeat,
        })
    }
}
impl From<&RawMacroEntry> for u64 {
    fn from(value: &RawMacroEntry) -> Self {
        let mut result: u64 = 0;

        // press/release/click (2 bits)
        let direction_bits = match value.direction {
            Direction::Press => 0b00,
            Direction::Release => 0b01,
            Direction::Click => 0b10,
        };
        result |= (direction_bits as u64) << 62;

        // mouse/key (1 bit)
        let macro_type_bit = match value.macro_type {
            MacroType::Mouse => 0,
            MacroType::Key => 1,
        };
        result |= (macro_type_bit as u64) << 61;

        // data (13 bits)
        result |= ((value.data as u64) & 0x1FFF) << 48;

        // length (27 bits)
        result |= ((value.length as u64) & 0x7FFFFFF) << 21;

        // repeat (21 bits)
        result |= (value.repeat as u64) & 0x1FFFFF;

        result
    }
}
impl From<RawMacroEntry> for u64 {
    fn from(value: RawMacroEntry) -> Self {
        u64::from(&value)
    }
}

/// This defines the data stored in RawMacroEntry
#[derive(Default, Clone, Copy, Debug)]
pub enum MacroType {
    /// This is a mouse action
    Mouse,
    /// This is a keyboard action
    #[default]
    Key,
}
impl Display for MacroType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroType::Mouse => write!(f, "Mouse"),
            MacroType::Key => write!(f, "Key"),
        }
    }
}
impl From<&bool> for MacroType {
    fn from(value: &bool) -> Self {
        match value {
            true => Self::Mouse,
            false => Self::Key,
        }
    }
}
impl From<bool> for MacroType {
    fn from(value: bool) -> Self {
        Self::from(&value)
    }
}
impl TryFrom<&&str> for MacroType {
    type Error = String;
    fn try_from(value: &&str) -> Result<Self, Self::Error> {
        Ok(match *value {
            "Mouse" => Self::Mouse,
            "Key" => Self::Key,
            _ => return Err("How??".into()),
        })
    }
}

pub fn from_direction_str(dir_str: &&str) -> Result<Direction, String> {
    Ok(match *dir_str {
        "Press" => Direction::Press,
        "Release" => Direction::Release,
        "Click" => Direction::Click,
        _ => return Err("Invalid direction!".into()),
    })
}

pub fn from_mouse_str(dir_str: &&str) -> Result<u16, String> {
    Ok(match *dir_str {
        "Left" => 1,
        "Middle" => 2,
        "Right" => 3,
        "Fourth" => 4,
        "Fifth" => 5,
        _ => return Err("Invalid direction!".into()),
    })
}
