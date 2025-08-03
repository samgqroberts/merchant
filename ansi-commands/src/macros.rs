#[macro_export]
macro_rules! comp {
    ($writer:expr $(, $command:expr)* $(,)?) => {{
        let writer: &mut $crate::frame::Frame = $writer.as_mut();
        Result::<&mut $crate::frame::Frame, String>::Ok(writer)
            $(.and_then(|writer| {
                writer.render(&$command)?;
                Ok(writer)
            }))*.map(|_| ())
        // #[cfg(feature = "crossterm")]
        // {
        //     $(crossterm::Command::write_ansi(&$command, $writer)?;)*
        // }
        // #[cfg(not(feature = "crossterm"))]
        // {
        //     todo!()
        // }
    }};
}
