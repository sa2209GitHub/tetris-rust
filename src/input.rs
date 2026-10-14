#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    RotateClockwise,
    RotateCounterClockwise,
    SoftDrop,
    HardDrop,
    Hold,
    Pause,
    Restart,
    Confirm,
}
