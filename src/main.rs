pub mod core;
pub mod engine;
pub mod input;
pub mod view;

use engine::GameEngine;
use input::GameAction;
use macroquad::prelude::*;
use view::gui::draw_game;

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_string(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros() as u32;

    let mut engine = GameEngine::new(seed);

    loop {
        // --- ДВИЖЕНИЕ ВЛЕВО ---
        let left_pressed = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::H);
        let left_down = is_key_down(KeyCode::Left) || is_key_down(KeyCode::H);

        if left_pressed {
            engine.handle_action(GameAction::MoveLeft);
            engine.input_das_delay = 0.20; // Задержка перед началом быстрого повтора (200 мс)
            engine.input_arr_cooldown = 0.0;
        } else if left_down && engine.input_das_delay <= 0.0 && engine.input_arr_cooldown <= 0.0 {
            engine.handle_action(GameAction::MoveLeft);
            engine.input_arr_cooldown = 0.04; // Скорость перемещения при удержании (40 мс)
        }

        // --- ДВИЖЕНИЕ ВПРАВО ---
        let right_pressed = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::L);
        let right_down = is_key_down(KeyCode::Right) || is_key_down(KeyCode::L);

        if right_pressed {
            engine.handle_action(GameAction::MoveRight);
            engine.input_das_delay = 0.20; // Задержка перед началом быстрого повтора (200 мс)
            engine.input_arr_cooldown = 0.0;
        } else if right_down && engine.input_das_delay <= 0.0 && engine.input_arr_cooldown <= 0.0 {
            engine.handle_action(GameAction::MoveRight);
            engine.input_arr_cooldown = 0.04; // Скорость перемещения при удержании (40 мс)
        }

        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::K) {
            if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                engine.handle_action(GameAction::RotateCounterClockwise);
            } else {
                engine.handle_action(GameAction::RotateClockwise);
            }
        }

        let is_down_triggered = if engine.state == engine::GameState::Playing {
            macroquad::input::is_key_down(KeyCode::Down)
                || macroquad::input::is_key_down(KeyCode::J)
        } else {
            macroquad::input::is_key_pressed(KeyCode::Down)
                || macroquad::input::is_key_pressed(KeyCode::J)
        };

        if is_down_triggered {
            engine.handle_action(GameAction::SoftDrop);
        }
        if is_key_pressed(KeyCode::Space) {
            engine.handle_action(GameAction::HardDrop);
        }
        if is_key_pressed(KeyCode::M) {
            engine.handle_action(GameAction::Hold);
        }
        if is_key_pressed(KeyCode::Escape) {
            engine.handle_action(GameAction::Pause); // Служит кнопкой Ок / Пауза
        }
        if is_key_pressed(KeyCode::Enter) {
            engine.handle_action(GameAction::Confirm); // Enter теперь отвечает за выбор в меню
        }

        engine.update(get_frame_time());

        draw_game(&engine);

        next_frame().await
    }
}
