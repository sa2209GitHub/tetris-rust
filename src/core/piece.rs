use crate::core::types::{PieceMatrix, Point, Rotation, TetrominoType};

const IDX_I: usize = 0;
const IDX_J: usize = 1;
const IDX_L: usize = 2;
const IDX_O: usize = 3;
const IDX_S: usize = 4;
const IDX_T: usize = 5;
const IDX_Z: usize = 6;

const IDX_R0: usize = 0;
const IDX_R90: usize = 1;
const IDX_R180: usize = 2;
const IDX_R270: usize = 3;

pub const TETROMINO_SHAPES: [[PieceMatrix; 4]; 7] = [
    // === I ===
    [
        [
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
            [false, false, false, false],
        ], // R0
        [
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
            [false, false, true, false],
        ], // R90
        [
            [false, false, false, false],
            [false, false, false, false],
            [true, true, true, true],
            [false, false, false, false],
        ], // R180
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
        ], // R270
    ],
    // === J ===
    [
        [
            [true, false, false, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, true, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, false, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
    ],
    // === L ===
    [
        [
            [false, false, true, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, true, false],
            [true, false, false, false],
            [false, false, false, false],
        ],
        [
            [true, true, false, false],
            [false, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    ],
    // === O ===
    [
        [
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, true, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
    ],
    // === S ===
    [
        [
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, true, false],
            [false, false, true, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [false, true, true, false],
            [true, true, false, false],
            [false, false, false, false],
        ],
        [
            [true, false, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    ],
    // === T ===
    [
        [
            [false, true, false, false],
            [true, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [true, true, false, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
    ],
    // === Z ===
    [
        [
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, true, false],
            [false, true, true, false],
            [false, true, false, false],
            [false, false, false, false],
        ],
        [
            [false, false, false, false],
            [true, true, false, false],
            [false, true, true, false],
            [false, false, false, false],
        ],
        [
            [false, true, false, false],
            [true, true, false, false],
            [true, false, false, false],
            [false, false, false, false],
        ],
    ],
];

#[derive(Debug, Clone)]
pub struct ActivePiece {
    pub ty: TetrominoType,
    pub rotation: Rotation,
    pub position: Point,
}

impl ActivePiece {
    pub fn new(ty: TetrominoType) -> Self {
        Self {
            ty,
            rotation: Rotation::R0,
            position: Point { x: 3, y: -1 },
        }
    }

    pub fn matrix(&self) -> &PieceMatrix {
        let type_idx = match self.ty {
            TetrominoType::I => IDX_I,
            TetrominoType::J => IDX_J,
            TetrominoType::L => IDX_L,
            TetrominoType::O => IDX_O,
            TetrominoType::S => IDX_S,
            TetrominoType::T => IDX_T,
            TetrominoType::Z => IDX_Z,
        };

        let rot_idx = match self.rotation {
            Rotation::R0 => IDX_R0,
            Rotation::R90 => IDX_R90,
            Rotation::R180 => IDX_R180,
            Rotation::R270 => IDX_R270,
        };

        &TETROMINO_SHAPES[type_idx][rot_idx]
    }
}
