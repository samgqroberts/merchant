#[macro_export]
macro_rules! comp {
    ($writer:expr $(, $command:expr)* $(,)?) => {{
        let writer: &mut $crate::Commands = $writer.as_mut();
        Result::<&mut $crate::Commands, String>::Ok(writer)
            $(.and_then(|writer| {
                $crate::Component::render(&$command, writer)?;
                Ok(writer)
            }))*.map(|_| ())
    }};
}
