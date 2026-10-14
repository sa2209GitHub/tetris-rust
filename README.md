# Rust Tetris 🦀🕹️

A classic **Tetris game clone** written in **Rust**. This project is a clean, modern implementation of the beloved retro arcade game, focusing on performance, safety, and simple controls.

## 🚀 Features

- **Classic Gameplay:** Standard Tetris mechanics, including line clears, scoring, and increasing difficulty levels.
- **Smooth Controls:** Responsive keyboard inputs with supporting Vim-style shortcuts.
- **Next Piece Preview:** See what block is coming next to plan your strategy.
- **Hold Piece Functionality:** Store a tetromino in the hold queue and switch between the active and held pieces with visual on-screen feedback.
- **High Performance:** Built with Rust's efficiency, ensuring zero lag and low resource usage.

## 🛠️ Tech Stack & Dependencies

- **Language:** Rust (Stable)
- **Graphics/Window Handling:** _(Specify your library here, e.g., SDL2 / ggez / piston / macroquad / crossterm)_

## 🎮 How to Play

1. **Clone the repository:**
   ```bash
   git clone https://github.com
   cd rust-tetris
   ```
2. **Run the game:**
   ```bash
   cargo run --release
   ```

## ⌨️ Controls

| Action           | Keyboard Option 1 | Keyboard Option 2 (Vim-style) |
| :--------------- | :---------------- | :---------------------------- |
| **Move Left**    | `Left Arrow`      | `H`                           |
| **Soft Drop**    | `Down Arrow`      | `J`                           |
| **Rotate Piece** | `Up Arrow`        | `K`                           |
| **Move Right**   | `Right Arrow`     | `L`                           |
| **Hard Drop**    | `Spacebar`        | —                             |
| **Hold Piece**   | `M`               | —                             |
| **Pause Game**   | `P`               | —                             |
