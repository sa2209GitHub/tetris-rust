use crate::core::piece::ActivePiece;
use crate::core::types::{BOARD_HEIGHT, BOARD_WIDTH, BoardCells};

#[derive(Default, Debug, Clone)]
pub struct Board {
    pub cells: BoardCells,
}

impl Board {
    pub fn new() -> Self {
        Self {
            cells: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
        }
    }

    pub fn has_collision(&self, piece: &ActivePiece) -> bool {
        let matrix = piece.matrix();

        for (row_idx, matrix_row) in matrix.iter().enumerate() {
            // Считаем global_y один раз для всей строки матрицы
            let global_y = piece.position.y + row_idx as i32;

            for (col_idx, &is_occupied) in matrix_row.iter().enumerate() {
                if is_occupied {
                    let global_x = piece.position.x + col_idx as i32;

                    if global_x < 0
                        || global_x >= BOARD_WIDTH as i32
                        || global_y >= BOARD_HEIGHT as i32
                    {
                        return true;
                    }

                    if global_y >= 0 && self.cells[global_y as usize][global_x as usize].is_some() {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn freeze_piece(&mut self, piece: &ActivePiece) {
        let matrix = piece.matrix();
        let color = piece.ty.default_color();

        for (row_idx, matrix_row) in matrix.iter().enumerate() {
            for (col_idx, &is_occupied) in matrix_row.iter().enumerate() {
                if is_occupied {
                    let global_x = piece.position.x + col_idx as i32;
                    let global_y = piece.position.y + row_idx as i32;

                    if global_x >= 0
                        && global_x < BOARD_WIDTH as i32
                        && global_y >= 0
                        && global_y < BOARD_HEIGHT as i32
                    {
                        self.cells[global_y as usize][global_x as usize] = Some(color);
                    }
                }
            }
        }
    }

    pub fn clear_lines(&mut self) -> u32 {
        let mut lines_cleared = 0;
        let mut row = BOARD_HEIGHT - 1;

        while row > 0 {
            let is_full = self.cells[row].iter().all(|cell| cell.is_some());

            if is_full {
                lines_cleared += 1;

                for move_row in (1..=row).rev() {
                    self.cells[move_row] = self.cells[move_row - 1];
                }

                self.cells[0] = [None; BOARD_WIDTH];
            } else {
                row -= 1;
            }
        }

        lines_cleared
    }
}

#[cfg(test)]
mod tests {
    use crate::core::board::Board;
    use crate::core::piece::ActivePiece;
    use crate::core::types::{BOARD_HEIGHT, BOARD_WIDTH, Color, TetrominoType};

    #[test]
    fn test_empty_board_has_no_colliisions() {
        let board = Board::new();
        let piece = ActivePiece::new(TetrominoType::O);

        assert!(!board.has_collision(&piece));
    }

    #[test]
    fn test_collision_with_floor() {
        let board = Board::new();
        let mut piece = ActivePiece::new(TetrominoType::O);

        piece.position.y = 20;

        assert!(board.has_collision(&piece));
    }

    #[test]
    fn test_collision_with_left_wall() {
        let board = Board::new();
        let mut piece = ActivePiece::new(TetrominoType::I);

        piece.position.x = -2;

        assert!(board.has_collision(&piece));
    }

    #[test]
    fn test_clear_full_lines() {
        let mut board = Board::new();

        for x in 0..BOARD_WIDTH {
            board.cells[BOARD_HEIGHT - 1][x] = Some(Color::Red);
        }

        let cleared = board.clear_lines();
        assert_eq!(cleared, 1);

        assert!(
            board.cells[BOARD_HEIGHT - 1]
                .iter()
                .all(|cell| cell.is_none())
        );
    }
}
