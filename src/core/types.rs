pub const BOARD_WIDTH: usize = 10;
pub const BOARD_HEIGHT: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Cyan,
    Blue,
    Orange,
    Yellow,
    Green,
    Purple,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TetrominoType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl TetrominoType {
    pub fn default_color(&self) -> Color {
        match self {
            TetrominoType::I => Color::Cyan,
            TetrominoType::J => Color::Blue,
            TetrominoType::L => Color::Orange,
            TetrominoType::O => Color::Yellow,
            TetrominoType::S => Color::Green,
            TetrominoType::T => Color::Purple,
            TetrominoType::Z => Color::Red,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}

impl Rotation {
    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::R90,
            Rotation::R90 => Rotation::R180,
            Rotation::R180 => Rotation::R270,
            Rotation::R270 => Rotation::R0,
        }
    }

    pub fn rotate_counter_clockwise(&self) -> Self {
        match self {
            Rotation::R0 => Rotation::R270,
            Rotation::R90 => Rotation::R0,
            Rotation::R180 => Rotation::R90,
            Rotation::R270 => Rotation::R180,
        }
    }
}

pub type PieceMatrix = [[bool; 4]; 4];

pub type BoardCells = [[Option<Color>; BOARD_WIDTH]; BOARD_HEIGHT];
