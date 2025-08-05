use ansi_commands::{frame::Frame, Component};

pub fn render_component<T: Component>(x: T) -> String {
    let mut frame = Frame::new();
    frame.render(&x).unwrap();
    frame.render_raw().result
}

pub fn render_component_crossterm<T: Component>(_x: T) -> String {
    todo!();
}
