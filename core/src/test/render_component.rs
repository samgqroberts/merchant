use terminal_commands::{frame::Frame, Component};

pub fn render_component<T: Component>(x: T) -> String {
    let mut frame = Frame::new();
    frame.render(&x).unwrap();
    frame.render_raw().unwrap().result
}
