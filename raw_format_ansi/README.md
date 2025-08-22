# raw_format_ansi

This crate exports a single function, `raw_format_ansi`, which accepts a string that may have ansi escape sequences and returns a string with those sequences parsed, accounted for, and removed.

Positioning ansi codes are respected and styling codes are removed.

For example, the string "\x1b[1mH\033[5CW\033[22m\033[6Dello\033[2Corld" would simply become "Hello World".
