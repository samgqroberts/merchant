use captured_write::CapturedWrite;
use crossterm::{execute, Command};

pub fn render_component<T: Command>(x: T) -> String {
    let mut writer = CapturedWrite::new();
    execute!(&mut writer, x).unwrap();
    raw_format_ansi::raw_format_ansi(&writer.buffer)
}
