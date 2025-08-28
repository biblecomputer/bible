use std::collections::HashMap;
use uuid::Uuid;

mod language;
mod translation;

struct Peter {
    current: WindowID,
    windows: HashMap<WindowID, Window>,
}

enum WindowInstr {
    NewWindow,
    DeleteWindow,
}

struct WindowID(Uuid);

struct Window {
    point: Point,
    color: Color,
}

enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

struct Point {
    x: u32,
    y: u32,
}
