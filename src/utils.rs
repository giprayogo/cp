#[macro_export]
macro_rules! stdins {
    ($buf:expr, $stdin:expr) => {{
        $buf.clear();
        $stdin.read_line(&mut $buf)?;
        $buf.trim().split_whitespace()
    }};
}

#[macro_export]
macro_rules! stdin {
    ($buf:expr, $stdin:expr) => {{
        $buf.clear();
        $stdin.read_line(&mut $buf)?;
        $buf.trim().parse()?
    }};
}