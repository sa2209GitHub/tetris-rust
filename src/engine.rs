use crate::core::board::Board;
use crate::core::piece::ActivePiece;
use crate::core::types::{Point, TetrominoType};
use crate::input::GameAction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Menu,
    GameOver,
}

#[derive(Debug, Clone)]
struct SimpleRand {
    seed: u32,
}

impl SimpleRand {
    fn new(seed: u32) -> Self {
        Self { seed }
    }

    fn next(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        self.seed & 0x7FFFFFFF
    }
}

#[derive(Debug, Clone)]
pub struct GameEngine {
    pub board: Board,
    pub current_piece: ActivePiece,
    pub next_piece: TetrominoType,
    pub hold_piece: Option<TetrominoType>,
    pub can_hold: bool,

    pub score: u32,
    pub lines_cleared: u32,
    pub level: u32,
    pub state: GameState,
    pub selected_menu_item: usize,

    pub fall_cooldown: f32,
    pub is_soft_dropping: bool,

    rng: SimpleRand,
    bag: Vec<TetrominoType>,

    pub input_das_delay: f32, // Таймер первой задержки перед авто-повтором
    pub input_arr_cooldown: f32, // Таймер скорости самого авто-повтора
}

impl GameEngine {
    pub fn new(seed: u32) -> Self {
        let mut rng = SimpleRand::new(seed);
        let mut bag = Vec::new();

        let current_type = Self::pull_from_bag(&mut bag, &mut rng);
        let next_piece = Self::pull_from_bag(&mut bag, &mut rng);

        Self {
            board: Board::new(),
            current_piece: ActivePiece::new(current_type),
            next_piece,
            hold_piece: None,
            can_hold: true,
            score: 0,
            lines_cleared: 0,
            level: 1,
            state: GameState::Playing,
            selected_menu_item: 0,
            fall_cooldown: 1.0,
            is_soft_dropping: false,
            rng,
            bag,
            input_das_delay: 0.0,
            input_arr_cooldown: 0.0,
        }
    }

    fn pull_from_bag(bag: &mut Vec<TetrominoType>, rng: &mut SimpleRand) -> TetrominoType {
        if bag.is_empty() {
            let mut available = vec![
                TetrominoType::I,
                TetrominoType::J,
                TetrominoType::L,
                TetrominoType::O,
                TetrominoType::S,
                TetrominoType::T,
                TetrominoType::Z,
            ];

            for i in (1..available.len()).rev() {
                let j = (rng.next() % (i as u32 + 1)) as usize;
                available.swap(i, j);
            }
            *bag = available;
        }

        bag.pop().unwrap()
    }

    fn get_fall_speed(&self) -> f32 {
        if self.is_soft_dropping {
            return 0.03;
        }

        let speed = 1.0 - (self.level as f32 - 1.0) * 0.05;
        speed.max(0.05)
    }

    fn add_score(&mut self, lines: u32) {
        let base_points = match lines {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };

        self.score += base_points * self.level;
        self.lines_cleared += lines;
        self.level = (self.lines_cleared / 10) + 1;
    }

    pub fn start_game(&mut self) {
        *self = Self::new(self.rng.next());
        self.state = GameState::Playing;
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.state != GameState::Playing {
            return;
        }

        // Уменьшаем таймеры зажатия кнопок, если они активны
        if self.input_das_delay > 0.0 {
            self.input_das_delay -= delta_time;
        }
        if self.input_arr_cooldown > 0.0 {
            self.input_arr_cooldown -= delta_time;
        }

        self.fall_cooldown -= delta_time;

        if self.fall_cooldown <= 0.0 {
            // Если шаг вниз был успешным (фигура сдвинулась)
            // И в этот момент был зажат софт-дроп — добавляем 1 очко за ячейку!
            if self.step_down() && self.is_soft_dropping && self.current_piece.position.y >= 0 {
                self.score += 1;
            }

            // Сбрасываем таймер на скорость софт-дропа (0.03) или обычную уровня
            self.fall_cooldown = self.get_fall_speed();
        }

        self.is_soft_dropping = false
    }

    pub fn step_down(&mut self) -> bool {
        let mut moved_piece = self.current_piece.clone();
        moved_piece.position.y += 1; // Проверяем строго СЛЕДУЮЩИЙ шаг

        if !self.board.has_collision(&moved_piece) {
            self.current_piece = moved_piece;
            true // Шаг успешно сделан
        } else {
            self.lock_and_spawn_next();
            false // Фигура уперлась и зафиксировалась
        }
    }

    fn lock_and_spawn_next(&mut self) {
        self.board.freeze_piece(&self.current_piece);

        let cleared = self.board.clear_lines();
        if cleared > 0 {
            self.add_score(cleared);
        }

        let next_type = Self::pull_from_bag(&mut self.bag, &mut self.rng);
        self.current_piece = ActivePiece::new(self.next_piece);
        self.next_piece = next_type;

        self.can_hold = true;
        self.is_soft_dropping = false; // Сбрасываем флаг ускорения для новой фигуры

        // ИСПРАВЛЕНИЕ: Новая фигура всегда начинает с честным полным таймером скорости
        self.fall_cooldown = self.get_fall_speed();

        if self.board.has_collision(&self.current_piece) {
            self.state = GameState::GameOver;
        }
    }

    pub fn handle_action(&mut self, action: GameAction) {
        // 1. Если мы находимся в МЕНЮ или GAMEOVER
        if self.state == GameState::Menu || self.state == GameState::GameOver {
            let max_items = if self.state == GameState::Menu { 3 } else { 2 };

            match action {
                GameAction::MoveLeft => {
                    if self.state == GameState::Menu {
                        self.state = GameState::Playing; // Выход из меню назад в игру
                    }
                }
                GameAction::SoftDrop => {
                    self.selected_menu_item = (self.selected_menu_item + 1) % max_items;
                }
                GameAction::RotateClockwise => {
                    if self.selected_menu_item == 0 {
                        self.selected_menu_item = max_items - 1;
                    } else {
                        self.selected_menu_item -= 1;
                    }
                }
                // ТЕПЕРЬ ДЛЯ ВЫБОРА ИСПОЛЬЗУЕТСЯ СТРОГО CONFIRM (Enter)
                GameAction::Confirm => {
                    if self.state == GameState::Menu {
                        match self.selected_menu_item {
                            0 => self.state = GameState::Playing, // Continue
                            1 => self.start_game(),               // Restart
                            2 => std::process::exit(0),           // Exit
                            _ => {}
                        }
                    } else if self.state == GameState::GameOver {
                        match self.selected_menu_item {
                            0 => self.start_game(),
                            1 => std::process::exit(0),
                            _ => {}
                        }
                    }
                }
                // ЕСЛИ В МЕНЮ НАЖАТ ESCAPE — просто возвращаем в игру (если это не GameOver)
                GameAction::Pause if self.state == GameState::Menu => {
                    self.state = GameState::Playing;
                }
                _ => {}
            }
            return;
        }

        // 2. Логика обычной игры (GameState::Playing)
        match action {
            GameAction::MoveLeft => {
                self.try_move(-1, 0);
            }
            GameAction::MoveRight => {
                self.try_move(1, 0);
            }
            GameAction::SoftDrop => {
                self.is_soft_dropping = true;
                let target_speed = self.get_fall_speed();
                if self.fall_cooldown > target_speed {
                    self.fall_cooldown = target_speed;
                }
            }
            GameAction::HardDrop => {
                self.hard_drop();
            }
            GameAction::RotateClockwise => {
                self.try_rotate(true);
            }
            GameAction::RotateCounterClockwise => {
                self.try_rotate(false);
            }
            GameAction::Hold => {
                self.try_hold();
            }

            // ЕСЛИ ВО ВРЕМЯ ИГРЫ НАЖАТ ESCAPE — открываем меню
            GameAction::Pause => {
                self.state = GameState::Menu;
                self.selected_menu_item = 0; // Сразу подсвечиваем "Continue"
            }
            GameAction::Restart => {
                self.state = GameState::Menu;
                self.selected_menu_item = 1; // Подсвечиваем "Restart"
            }
            _ => {} // Игнорируем Confirm во время игры
        }
    }

    /// Пытается сдвинуть фигуру на относительное смещение (dx, dy)
    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let mut moved = self.current_piece.clone();
        moved.position.x += dx;
        moved.position.y += dy;

        if !self.board.has_collision(&moved) {
            self.current_piece = moved;
            true
        } else {
            false
        }
    }

    /// Пытается повернуть фигуру.
    /// Включает простейший базовый Wall Kick: если у стены фигура не может повернуться,
    /// игра пытается сместить её на 1 клетку влево или вправо.
    fn try_rotate(&mut self, clockwise: bool) {
        let mut rotated = self.current_piece.clone();
        rotated.rotation = if clockwise {
            self.current_piece.rotation.rotate_clockwise()
        } else {
            self.current_piece.rotation.rotate_counter_clockwise()
        };

        // Список смещений для проверки (Wall Kick)
        // Сначала проверяем чистый поворот (0,0), затем сдвиг влево (-1,0), затем вправо (1,0)
        let kicks = [
            Point { x: 0, y: 0 },
            Point { x: -1, y: 0 },
            Point { x: 1, y: 0 },
        ];

        for kick in kicks.iter() {
            let mut test_piece = rotated.clone();
            test_piece.position.x += kick.x;
            test_piece.position.y += kick.y;

            if !self.board.has_collision(&test_piece) {
                self.current_piece = test_piece;
                break; // Успешно повернулись, выходим из цикла
            }
        }
    }

    /// Мгновенно роняет фигуру вниз до упора и фиксирует её
    fn hard_drop(&mut self) {
        let mut drop_distance = 0;

        while self.try_move(0, 1) {
            if self.current_piece.position.y >= 0 {
                drop_distance += 1;
            }
        }

        self.score += drop_distance * 2;

        self.lock_and_spawn_next();
    }

    /// Механика "Холда" (сохранение фигуры в карман)
    fn try_hold(&mut self) {
        if !self.can_hold {
            return; // За ход можно использовать холд только один раз
        }

        let current_type = self.current_piece.ty;

        if let Some(held_type) = self.hold_piece {
            // Если в кармане уже что-то было — меняем местами
            self.current_piece = ActivePiece::new(held_type);
            self.hold_piece = Some(current_type);
        } else {
            // Если карман был пуст — сохраняем текущую, а спавним "следующую"
            self.hold_piece = Some(current_type);
            let next_type = Self::pull_from_bag(&mut self.bag, &mut self.rng);
            self.current_piece = ActivePiece::new(self.next_piece);
            self.next_piece = next_type;
        }

        self.can_hold = false; // Блокируем холд до следующего приземления
    }
}
