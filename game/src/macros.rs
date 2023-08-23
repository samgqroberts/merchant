#[macro_export]
macro_rules! comp {
    ($writer:expr $(, $command:expr)* $(,)?) => {{
        $($command.write_ansi($writer)?;)*
    }}
}
