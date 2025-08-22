use terminal_commands::{render_raw, Commands, Component};

pub fn render_component<T: Component>(x: T) -> String {
    let mut commands = Commands::new();
    x.render(&mut commands).unwrap();
    render_raw(&commands).result
}
