use crate::core::types::{BOARD_HEIGHT, BOARD_WIDTH, Color as TetrisColor};
use crate::engine::{GameEngine, GameState};
use macroquad::prelude::*;

// === ЦВЕТОВАЯ ПАЛИТРА SPACEDUCK ===
// Константы интерфейса
const SPACEDUCK_BG: Color = Color::new(0.063, 0.059, 0.102, 1.0); // #100f1a - Основной фон
const SPACEDUCK_BOARD: Color = Color::new(0.094, 0.086, 0.149, 1.0); // #181626 - Фон стакана
const SPACEDUCK_BORDER: Color = Color::new(0.459, 0.443, 0.761, 1.0); // #7571c2 - Основные рамки / текст
const SPACEDUCK_TEXT_MAIN: Color = Color::new(0.925, 0.941, 0.945, 1.0); // #ecf0f1 - Главный текст (белый)
const SPACEDUCK_TEXT_MUTED: Color = Color::new(0.302, 0.314, 0.471, 1.0); // #4d5078 - Приглушенный текст
const SPACEDUCK_ACCENT: Color = Color::new(0.925, 0.365, 0.525, 1.0); // #ec5d86 - Акцентный розовый (Пауза/Game Over)

// Настраиваемая палитра фигур SpaceDuck (можно менять цвета прямо здесь)
struct ColorPalette {
    cyan: Color,
    blue: Color,
    orange: Color,
    yellow: Color,
    green: Color,
    purple: Color,
    red: Color,
}

const PALETTE: ColorPalette = ColorPalette {
    cyan: Color::new(0.0, 0.698, 0.741, 1.0),     // #00b2bd
    blue: Color::new(0.0, 0.549, 0.812, 1.0),     // #008cb6
    orange: Color::new(0.886, 0.533, 0.259, 1.0), // #e28842
    yellow: Color::new(0.965, 0.824, 0.361, 1.0), // #f6d25c
    green: Color::new(0.361, 0.722, 0.518, 1.0),  // #5cb884
    purple: Color::new(0.631, 0.494, 0.831, 1.0), // #a17ed4
    red: Color::new(0.894, 0.32, 0.357, 1.0),     // #e4515b
};

fn get_mq_color(color: TetrisColor) -> Color {
    match color {
        TetrisColor::Cyan => PALETTE.cyan,
        TetrisColor::Blue => PALETTE.blue,
        TetrisColor::Orange => PALETTE.orange,
        TetrisColor::Yellow => PALETTE.yellow,
        TetrisColor::Green => PALETTE.green,
        TetrisColor::Purple => PALETTE.purple,
        TetrisColor::Red => PALETTE.red,
    }
}

// Вспомогательная функция для генерации цвета обводки (делает цвет на 40% темнее)
fn get_stroke_color(fill_color: Color) -> Color {
    Color::new(
        fill_color.r * 0.6,
        fill_color.g * 0.6,
        fill_color.b * 0.6,
        1.0,
    )
}

// Вспомогательная функция отрисовки блока с толстой обводкой 4px внутри
fn draw_tetris_block(x: f32, y: f32, size: f32, fill_color: Color) {
    draw_rectangle(x, y, size, size, fill_color);
    let stroke_color = get_stroke_color(fill_color);
    draw_rectangle_lines(x, y, size, size, 4.0, stroke_color);
}

pub fn draw_game(engine: &GameEngine) {
    // Очищаем экран основным глубоким космическим цветом SpaceDuck
    clear_background(SPACEDUCK_BG);

    let screen_w = screen_width();
    let screen_h = screen_height();

    // === 1. РАСЧЕТ РАЗМЕРОВ И ЦЕНТРИРОВАНИЯ ===
    let padding_y = 8.0;
    let available_h = screen_h - (padding_y * 2.0);

    let cell_size = (available_h / BOARD_HEIGHT as f32).floor();

    let board_pixel_w = BOARD_WIDTH as f32 * cell_size;
    let board_pixel_h = BOARD_HEIGHT as f32 * cell_size;

    let board_x = (screen_w - board_pixel_w) / 2.0;
    let board_y = padding_y;

    // === 2. ОТРИСОВКА СТАКАНА (ЗАДНИЙ ФОН И СЕТКА) ===
    draw_rectangle(
        board_x,
        board_y,
        board_pixel_w,
        board_pixel_h,
        SPACEDUCK_BOARD,
    );

    // СОЗДАЕМ ЦВЕТ СЕТКИ: берем цвет рамки и ставим 10% непрозрачности (альфа = 0.1)
    let grid_color = Color::new(
        SPACEDUCK_BORDER.r,
        SPACEDUCK_BORDER.g,
        SPACEDUCK_BORDER.b,
        0.1,
    );

    // РИСУЕМ ВНУТРЕННЮЮ СЕТКУ
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            let cell_x = board_x + x as f32 * cell_size;
            let cell_y = board_y + y as f32 * cell_size;

            // Рисуем тонкий контур каждой ячейки толщиной 1 пиксель
            draw_rectangle_lines(cell_x, cell_y, cell_size, cell_size, 2.0, grid_color);
        }
    }

    // Рисуем внешнюю рамку стакана (толщина 6.0 перекроет внешние края сетки)
    draw_rectangle_lines(
        board_x,
        board_y,
        board_pixel_w,
        board_pixel_h,
        6.0,
        SPACEDUCK_BORDER,
    );
    // Рисуем зафиксированные блоки в стакане
    for y in 0..BOARD_HEIGHT {
        for x in 0..BOARD_WIDTH {
            if let Some(color) = engine.board.cells[y][x] {
                let block_x = board_x + x as f32 * cell_size;
                let block_y = board_y + y as f32 * cell_size;

                draw_tetris_block(block_x, block_y, cell_size, get_mq_color(color));
            }
        }
    }

    // === 3. ОТРИСОВКА ПАДАЮЩЕЙ ФИГУРЫ ===
    if engine.state == GameState::Playing {
        let piece = &engine.current_piece;
        let matrix = piece.matrix();
        let color = get_mq_color(piece.ty.default_color());

        for (row, matrix_row) in (0..).zip(matrix) {
            for (col, &is_occupied) in (0..).zip(matrix_row) {
                if is_occupied {
                    let global_x = piece.position.x + col;
                    let global_y = piece.position.y + row;

                    if global_x >= 0
                        && global_x < BOARD_WIDTH as i32
                        && global_y >= 0
                        && global_y < BOARD_HEIGHT as i32
                    {
                        let block_x = board_x + global_x as f32 * cell_size;
                        let block_y = board_y + global_y as f32 * cell_size;

                        draw_tetris_block(block_x, block_y, cell_size, color);
                    }
                }
            }
        }
    }

    // === 4. ОТРИСОВКА ИНТЕРФЕЙСА СПРАВА ===
    let ui_x = board_x + board_pixel_w + 30.0;
    let mut ui_y = board_y + 40.0;

    // Название игры
    draw_text("TETRIS", ui_x, ui_y, 36.0, SPACEDUCK_TEXT_MAIN);
    ui_y += 50.0;

    // Счёт
    draw_text(
        format!("SCORE: {:06}", engine.score),
        ui_x,
        ui_y,
        24.0,
        SPACEDUCK_BORDER,
    );
    ui_y += 35.0;

    // Уровень
    draw_text(
        format!("LEVEL: {}", engine.level),
        ui_x,
        ui_y,
        24.0,
        SPACEDUCK_BORDER,
    );
    ui_y += 35.0;

    // Линии
    draw_text(
        format!("LINES: {}", engine.lines_cleared),
        ui_x,
        ui_y,
        24.0,
        SPACEDUCK_BORDER,
    );
    ui_y += 60.0;

    // Окно "СЛЕДУЮЩАЯ ФИГУРА"
    draw_text("NEXT:", ui_x, ui_y, 20.0, SPACEDUCK_TEXT_MUTED);
    ui_y += 15.0;

    draw_rectangle_lines(
        ui_x,
        ui_y,
        cell_size * 4.0,
        cell_size * 4.0,
        1.0,
        SPACEDUCK_TEXT_MUTED,
    );

    let next_color = get_mq_color(engine.next_piece.default_color());
    let next_matrix = &crate::core::piece::TETROMINO_SHAPES[engine.next_piece as usize][0];

    for (row, next_matrix_row) in (0..).zip(next_matrix) {
        for (col, &is_occupied) in (0..).zip(next_matrix_row) {
            if is_occupied {
                let mini_x = ui_x + col as f32 * cell_size;
                let mini_y = ui_y + row as f32 * cell_size;
                draw_tetris_block(mini_x, mini_y, cell_size, next_color);
            }
        }
    }
    ui_y += cell_size * 4.0 + 40.0;

    // Окно "ХОЛД"
    draw_text("HOLD:", ui_x, ui_y, 20.0, SPACEDUCK_TEXT_MUTED);
    ui_y += 15.0;
    draw_rectangle_lines(
        ui_x,
        ui_y,
        cell_size * 4.0,
        cell_size * 4.0,
        1.0,
        SPACEDUCK_TEXT_MUTED,
    );

    if let Some(hold_type) = engine.hold_piece {
        let hold_color = get_mq_color(hold_type.default_color());
        let hold_matrix = &crate::core::piece::TETROMINO_SHAPES[hold_type as usize][0];

        for (row, hold_matrix_row) in (0..).zip(hold_matrix) {
            for (col, &is_occupied) in (0..).zip(hold_matrix_row) {
                if is_occupied {
                    let mini_x = ui_x + col as f32 * cell_size;
                    let mini_y = ui_y + row as f32 * cell_size;
                    draw_tetris_block(mini_x, mini_y, cell_size, hold_color);
                }
            }
        }
    }

    // === 5. ОВЕРЛЕИ СОСТОЯНИЙ (МЕНЮ И GAME OVER) ===
    let center_x = screen_w / 2.0;
    let center_y = screen_h / 2.0;

    if engine.state == GameState::Menu {
        draw_rectangle(
            0.0,
            0.0,
            screen_w,
            screen_h,
            Color::new(0.063, 0.059, 0.102, 0.85),
        );

        draw_text(
            "PAUSE / MENU",
            center_x - 100.0,
            center_y - 80.0,
            36.0,
            SPACEDUCK_ACCENT,
        );

        let items = ["1. CONTINUE", "2. RESTART", "3. EXIT GAME"];
        for (i, item) in items.iter().enumerate() {
            let color = if engine.selected_menu_item == i {
                SPACEDUCK_TEXT_MAIN
            } else {
                SPACEDUCK_TEXT_MUTED
            };
            let prefix = if engine.selected_menu_item == i {
                "> "
            } else {
                "  "
            };

            draw_text(
                format!("{}{}", prefix, item),
                center_x - 100.0,
                center_y - 10.0 + (i as f32 * 40.0),
                28.0,
                color,
            );
        }
        draw_text(
            "Use UP/DOWN arrows to select, ENTER to confirm",
            center_x - 220.0,
            center_y + 140.0,
            18.0,
            SPACEDUCK_BORDER,
        );
    }

    if engine.state == GameState::GameOver {
        draw_rectangle(
            0.0,
            0.0,
            screen_w,
            screen_h,
            Color::new(0.063, 0.059, 0.102, 0.9),
        );

        draw_text(
            "GAME OVER",
            center_x - 90.0,
            center_y - 80.0,
            42.0,
            SPACEDUCK_ACCENT,
        );
        draw_text(
            format!("FINAL SCORE: {}", engine.score),
            center_x - 95.0,
            center_y - 30.0,
            24.0,
            SPACEDUCK_TEXT_MAIN,
        );

        let items = ["1. RESTART", "2. EXIT GAME"];
        for (i, item) in items.iter().enumerate() {
            let color = if engine.selected_menu_item == i {
                SPACEDUCK_TEXT_MAIN
            } else {
                SPACEDUCK_TEXT_MUTED
            };
            let prefix = if engine.selected_menu_item == i {
                "> "
            } else {
                "  "
            };

            draw_text(
                format!("{}{}", prefix, item),
                center_x - 100.0,
                center_y + 30.0 + (i as f32 * 40.0),
                28.0,
                color,
            );
        }
        draw_text(
            "Use UP/DOWN arrows to select, ENTER to confirm",
            center_x - 220.0,
            center_y + 150.0,
            18.0,
            SPACEDUCK_BORDER,
        );
    }

    // === СИСТЕМНЫЙ ОТЛАДОЧНЫЙ БЛОК (DEBUG LOG) ===
    //     draw_rectangle(
    //         10.0,
    //         10.0,
    //         320.0,
    //         150.0,
    //         Color::new(0.094, 0.086, 0.149, 0.9),
    //     );
    //     draw_rectangle_lines(10.0, 10.0, 320.0, 150.0, 1.0, SPACEDUCK_ACCENT);
    //
    //     draw_text("=== SYSTEM DEBUG ===", 20.0, 30.0, 18.0, SPACEDUCK_ACCENT);
    //
    //     draw_text(
    //         format!("Engine State: {:?}", engine.state),
    //         20.0,
    //         55.0,
    //         16.0,
    //         PALETTE.green,
    //     );
    //     draw_text(
    //         format!("Fall Cooldown: {:.3} sec", engine.fall_cooldown),
    //         20.0,
    //         75.0,
    //         16.0,
    //         SPACEDUCK_TEXT_MAIN,
    //     );
    //     draw_text(
    //         format!(
    //             "Piece Pos: X={}, Y={}",
    //             engine.current_piece.position.x, engine.current_piece.position.y
    //         ),
    //         20.0,
    //         95.0,
    //         16.0,
    //         PALETTE.yellow,
    //     );
    //
    //     let matrix = engine.current_piece.matrix();
    //
    //     let active_blocks_count = matrix
    //         .iter()
    //         .flatten()
    //         .filter(|&&is_active| is_active)
    //         .count();
    //
    //     draw_text(
    //         format!("Blocks inside matrix 4x4: {}", active_blocks_count),
    //         20.0,
    //         115.0,
    //         16.0,
    //         PALETTE.cyan,
    //     );
    //     draw_text(
    //         format!("Screen FPS: {}", macroquad::time::get_fps()),
    //         20.0,
    //         135.0,
    //         16.0,
    //         SPACEDUCK_TEXT_MUTED,
    //     );
}
