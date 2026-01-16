use enigo::Direction;
use gpui_component::{IndexPath, select::SelectItem};
use std::fmt::Display;

/// enum specific functions to help with [gpui_components::select]
pub trait ToVec {
    /// Convert this object into a vector.
    fn to_vec() -> Vec<Self>
    where
        Self: Sized;
    /// The default index of the enum to use when nothing is supplied.
    fn default_index() -> Option<IndexPath>;
}

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
            // self.data,
            match self.macro_type {
                MacroType::Mouse => SelectMouseAction::from(self.data as u64).to_string(),
                MacroType::Key => format!("{}", self.data),
            },
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
impl SelectItem for MacroType {
    type Value = Self;

    fn title(&self) -> gpui::SharedString {
        match self {
            MacroType::Mouse => "Mouse".into(),
            MacroType::Key => "Key".into(),
        }
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
impl ToVec for MacroType {
    fn to_vec() -> Vec<Self> {
        vec![Self::Mouse, Self::Key]
    }
    fn default_index() -> Option<IndexPath> {
        Some(IndexPath::new(1))
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectDirection {
    Press,
    Release,
    Click,
}
impl SelectItem for SelectDirection {
    type Value = Self;

    fn title(&self) -> gpui::SharedString {
        match self {
            SelectDirection::Press => "Press".into(),
            SelectDirection::Release => "Release".into(),
            SelectDirection::Click => "Click".into(),
        }
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
impl From<&SelectDirection> for Direction {
    fn from(value: &SelectDirection) -> Self {
        match value {
            SelectDirection::Press => Self::Press,
            SelectDirection::Release => Self::Release,
            SelectDirection::Click => Self::Release,
        }
    }
}
impl From<&Direction> for SelectDirection {
    fn from(value: &Direction) -> Self {
        match value {
            Direction::Press => SelectDirection::Press,
            Direction::Release => SelectDirection::Release,
            Direction::Click => SelectDirection::Click,
        }
    }
}
impl From<Direction> for SelectDirection {
    fn from(value: Direction) -> Self {
        Self::from(&value)
    }
}
impl ToVec for SelectDirection {
    fn to_vec() -> Vec<Self> {
        vec![Self::Press, Self::Release, Self::Click]
    }
    fn default_index() -> Option<IndexPath> {
        Some(IndexPath::new(2))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SelectMouseAction {
    /// left mouse button
    Left,
    /// middle mouse button
    Middle,
    /// right mouse button
    Right,
    /// fourth mouse button, aka backward
    Fourth,
    /// fifth mouse button, aka forward.
    Fifth,
}
impl SelectItem for SelectMouseAction {
    type Value = Self;

    fn title(&self) -> gpui::SharedString {
        match self {
            SelectMouseAction::Left => "Left".into(),
            SelectMouseAction::Middle => "Middle".into(),
            SelectMouseAction::Right => "Right".into(),
            SelectMouseAction::Fourth => "Fourth".into(),
            SelectMouseAction::Fifth => "Fifth".into(),
        }
    }

    fn value(&self) -> &Self::Value {
        self
    }
}
impl ToVec for SelectMouseAction {
    fn to_vec() -> Vec<Self>
    where
        Self: Sized,
    {
        vec![
            Self::Left,
            Self::Middle,
            Self::Right,
            Self::Fourth,
            Self::Fifth,
        ]
    }
    fn default_index() -> Option<IndexPath> {
        Some(IndexPath::new(4))
    }
}
impl From<u64> for SelectMouseAction {
    fn from(value: u64) -> Self {
        Self::from(&value)
    }
}
impl From<&u64> for SelectMouseAction {
    fn from(value: &u64) -> Self {
        match value {
            1 => Self::Left,
            2 => Self::Middle,
            3 => Self::Right,
            4 => Self::Fourth,
            5 => Self::Fifth,
            _ => panic!("Invalid mouse button value"),
        }
    }
}
impl From<SelectMouseAction> for u64 {
    fn from(value: SelectMouseAction) -> Self {
        Self::from(&value)
    }
}
impl From<&SelectMouseAction> for u64 {
    fn from(value: &SelectMouseAction) -> Self {
        match value {
            SelectMouseAction::Left => 1,
            SelectMouseAction::Middle => 2,
            SelectMouseAction::Right => 3,
            SelectMouseAction::Fourth => 4,
            SelectMouseAction::Fifth => 5,
        }
    }
}
impl Display for SelectMouseAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectMouseAction::Left => write!(f, "Left"),
            SelectMouseAction::Middle => write!(f, "Middle"),
            SelectMouseAction::Right => write!(f, "Right"),
            SelectMouseAction::Fourth => write!(f, "Fourth"),
            SelectMouseAction::Fifth => write!(f, "Fifth"),
        }
    }
}
