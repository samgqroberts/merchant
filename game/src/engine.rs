use ansi_commands::frame::Renderer;
use crossterm::{
    cursor::{MoveToNextLine, Show},
    event::{poll, read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
};
use merchant_core::{
    components::{RequireResize, FRAME_HEIGHT, FRAME_WIDTH},
    engine::{render_scene, UpdateError, UpdateFn, UpdateSignal},
};
use std::{
    cell::RefCell,
    io::{self, Write},
    time::Duration,
};
use tracing::{debug, error, info};

use merchant_core::state::GameState;

use crate::renderer::CrosstermRenderer;

pub struct Engine<'a, Writer: Write> {
    pub writer: &'a RefCell<Writer>,
}

impl<'a, Writer: Write> Engine<'a, Writer> {
    pub fn new(writer: &'a RefCell<Writer>) -> Self {
        Self { writer }
    }

    pub fn draw_and_prompt(
        &mut self,
        game_state: &mut GameState,
    ) -> Result<UpdateSignal, UpdateError> {
        // check the terminal size, and if it needs to be taller or wider to fit the game, render those
        // instructions INSTEAD of the screen based on the game state
        let mut require_resize = false;
        if let Ok((current_x_cols, current_y_cols)) = crossterm::terminal::size() {
            debug!("Terminal size: {{x: {current_x_cols}, y: {current_y_cols}}}");
            if current_x_cols < FRAME_WIDTH || current_y_cols < FRAME_HEIGHT {
                require_resize = true;
                self.draw_need_resize(current_x_cols, current_y_cols)?;
            }
        } else {
            error!("Could not determine terminal size.");
        }
        // if terminal does not need to be resized draw the game state
        let mut update_fn: Option<_> = None;
        if !require_resize {
            update_fn = Some(self.draw_scene(game_state)?);
        }
        // Wait for any user event
        loop {
            // Wait up to 1s for some user event per loop iteration
            if poll(Duration::from_millis(1_000))? {
                // Read what even happened from the poll
                // It's guaranteed that read() won't block if `poll` returns `Ok(true)`
                match read()? {
                    Event::Key(event) => {
                        // only react to Press KeyEvents as opposed to Release
                        // on Windows, both Press and Release get triggered
                        // if we don't filter to just Press events we will double-update
                        if event.kind == KeyEventKind::Press {
                            info!("User Key Press: {:?} {:?}", event.code, event.modifiers);
                            // detect exit request
                            if event.modifiers == KeyModifiers::CONTROL
                                && event.code == KeyCode::Char('c')
                            {
                                return Ok(UpdateSignal::Quit);
                            }
                            // update game state (if we have an update_fn, we may not if
                            // terminal needs to be resized)
                            if let Some(update_fn) = update_fn {
                                return update_fn(convert_key_event(event), game_state);
                            } else {
                                return Ok(UpdateSignal::Continue);
                            }
                        }
                    }
                    Event::Resize(columns, rows) => {
                        info!("Terminal resized: {columns} columns, {rows} rows.");
                        return Ok(UpdateSignal::Continue); // trigger a rerender with no state updates
                    }
                    _ => continue,
                }
            } else {
                // Timeout expired, no event for 1s, wait for user input again
                continue;
            }
        }
    }

    pub fn draw_scene(&mut self, state: &mut GameState) -> io::Result<Box<UpdateFn>> {
        info!("Drawing scene: {:?}", state.mode);
        let writer = &mut *self.writer.borrow_mut();
        let (frame, update) = match render_scene(state) {
            Ok(result) => result,
            Err(e) => todo!("Error rendering scene: {:?}", e),
        };
        let render_result = CrosstermRenderer.render(&frame);
        writer.write_all(render_result.result.as_bytes())?;
        writer.flush()?;
        Ok(update)
    }

    pub fn draw_need_resize(&mut self, current_x_cols: u16, current_y_cols: u16) -> io::Result<()> {
        info!("Drawing screen requiring resize");
        let writer = &mut *self.writer.borrow_mut();
        let mut frame = ansi_commands::frame::Frame::new();
        frame
            .render(&RequireResize {
                current_x_cols,
                current_y_cols,
            })
            .unwrap(); // todo
        let render_result = CrosstermRenderer.render(&frame);
        writer.write_all(render_result.result.as_bytes())?;
        writer.flush()?;
        Ok(())
    }

    pub fn exit_message(&mut self, msg: &[&str]) -> io::Result<()> {
        let writer = &mut *self.writer.borrow_mut();
        execute!(writer, Show, MoveToNextLine(1),)?;
        for line in msg {
            execute!(writer, Show, MoveToNextLine(1), Print(line),)?;
        }
        execute!(writer, MoveToNextLine(1))?;
        Ok(())
    }
}

pub fn convert_key_event(event: crossterm::event::KeyEvent) -> ansi_commands::event::KeyEvent {
    ansi_commands::event::KeyEvent::new(
        match event.code {
            KeyCode::Char(c) => ansi_commands::event::KeyCode::Char(c),
            KeyCode::Backspace => ansi_commands::event::KeyCode::Backspace,
            KeyCode::Enter => ansi_commands::event::KeyCode::Enter,
            KeyCode::Left => ansi_commands::event::KeyCode::Left,
            KeyCode::Right => ansi_commands::event::KeyCode::Right,
            KeyCode::Up => ansi_commands::event::KeyCode::Up,
            KeyCode::Down => ansi_commands::event::KeyCode::Down,
            KeyCode::Home => ansi_commands::event::KeyCode::Home,
            KeyCode::End => ansi_commands::event::KeyCode::End,
            KeyCode::PageUp => ansi_commands::event::KeyCode::PageUp,
            KeyCode::PageDown => ansi_commands::event::KeyCode::PageDown,
            KeyCode::Tab => ansi_commands::event::KeyCode::Tab,
            KeyCode::BackTab => ansi_commands::event::KeyCode::BackTab,
            KeyCode::Delete => ansi_commands::event::KeyCode::Delete,
            KeyCode::Insert => ansi_commands::event::KeyCode::Insert,
            KeyCode::F(x) => ansi_commands::event::KeyCode::F(x),
            KeyCode::Null => ansi_commands::event::KeyCode::Null,
            KeyCode::Esc => ansi_commands::event::KeyCode::Esc,
            KeyCode::CapsLock => ansi_commands::event::KeyCode::CapsLock,
            KeyCode::ScrollLock => ansi_commands::event::KeyCode::ScrollLock,
            KeyCode::NumLock => ansi_commands::event::KeyCode::NumLock,
            KeyCode::PrintScreen => ansi_commands::event::KeyCode::PrintScreen,
            KeyCode::Pause => ansi_commands::event::KeyCode::Pause,
            KeyCode::Menu => ansi_commands::event::KeyCode::Menu,
            KeyCode::KeypadBegin => ansi_commands::event::KeyCode::KeypadBegin,
            KeyCode::Media(media_key_code) => {
                ansi_commands::event::KeyCode::Media(match media_key_code {
                    crossterm::event::MediaKeyCode::Play => {
                        ansi_commands::event::MediaKeyCode::Play
                    }
                    crossterm::event::MediaKeyCode::Pause => {
                        ansi_commands::event::MediaKeyCode::Pause
                    }
                    crossterm::event::MediaKeyCode::PlayPause => {
                        ansi_commands::event::MediaKeyCode::PlayPause
                    }
                    crossterm::event::MediaKeyCode::Reverse => {
                        ansi_commands::event::MediaKeyCode::Reverse
                    }
                    crossterm::event::MediaKeyCode::Stop => {
                        ansi_commands::event::MediaKeyCode::Stop
                    }
                    crossterm::event::MediaKeyCode::FastForward => {
                        ansi_commands::event::MediaKeyCode::FastForward
                    }
                    crossterm::event::MediaKeyCode::Rewind => {
                        ansi_commands::event::MediaKeyCode::Rewind
                    }
                    crossterm::event::MediaKeyCode::TrackNext => {
                        ansi_commands::event::MediaKeyCode::TrackNext
                    }
                    crossterm::event::MediaKeyCode::TrackPrevious => {
                        ansi_commands::event::MediaKeyCode::TrackPrevious
                    }
                    crossterm::event::MediaKeyCode::Record => {
                        ansi_commands::event::MediaKeyCode::Record
                    }
                    crossterm::event::MediaKeyCode::LowerVolume => {
                        ansi_commands::event::MediaKeyCode::LowerVolume
                    }
                    crossterm::event::MediaKeyCode::RaiseVolume => {
                        ansi_commands::event::MediaKeyCode::RaiseVolume
                    }
                    crossterm::event::MediaKeyCode::MuteVolume => {
                        ansi_commands::event::MediaKeyCode::MuteVolume
                    }
                })
            }
            KeyCode::Modifier(modifier_key_code) => {
                ansi_commands::event::KeyCode::Modifier(match modifier_key_code {
                    crossterm::event::ModifierKeyCode::LeftShift => {
                        ansi_commands::event::ModifierKeyCode::LeftShift
                    }
                    crossterm::event::ModifierKeyCode::LeftControl => {
                        ansi_commands::event::ModifierKeyCode::LeftControl
                    }
                    crossterm::event::ModifierKeyCode::LeftAlt => {
                        ansi_commands::event::ModifierKeyCode::LeftAlt
                    }
                    crossterm::event::ModifierKeyCode::LeftSuper => {
                        ansi_commands::event::ModifierKeyCode::LeftSuper
                    }
                    crossterm::event::ModifierKeyCode::LeftHyper => {
                        ansi_commands::event::ModifierKeyCode::LeftHyper
                    }
                    crossterm::event::ModifierKeyCode::LeftMeta => {
                        ansi_commands::event::ModifierKeyCode::LeftMeta
                    }
                    crossterm::event::ModifierKeyCode::RightShift => {
                        ansi_commands::event::ModifierKeyCode::RightShift
                    }
                    crossterm::event::ModifierKeyCode::RightControl => {
                        ansi_commands::event::ModifierKeyCode::RightControl
                    }
                    crossterm::event::ModifierKeyCode::RightAlt => {
                        ansi_commands::event::ModifierKeyCode::RightAlt
                    }
                    crossterm::event::ModifierKeyCode::RightSuper => {
                        ansi_commands::event::ModifierKeyCode::RightSuper
                    }
                    crossterm::event::ModifierKeyCode::RightHyper => {
                        ansi_commands::event::ModifierKeyCode::RightHyper
                    }
                    crossterm::event::ModifierKeyCode::RightMeta => {
                        ansi_commands::event::ModifierKeyCode::RightMeta
                    }
                    crossterm::event::ModifierKeyCode::IsoLevel3Shift => {
                        ansi_commands::event::ModifierKeyCode::IsoLevel3Shift
                    }
                    crossterm::event::ModifierKeyCode::IsoLevel5Shift => {
                        ansi_commands::event::ModifierKeyCode::IsoLevel5Shift
                    }
                })
            }
        },
        ansi_commands::event::KeyModifiers::empty(), // todo
    )
}
