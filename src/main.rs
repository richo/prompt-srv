use std::fmt;
use std::path::{self, Path, PathBuf};

#[derive(Debug)]
enum TerminalColours {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Reset,
}

impl fmt::Display for TerminalColours {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.escape_code())
    }
}

impl TerminalColours {
    fn escape_code(&self) -> &'static str {
        match self {
            TerminalColours::Black => "\u{001b}[30m",
            TerminalColours::Red => "\u{001b}[31m",
            TerminalColours::Green => "\u{001b}[32m",
            TerminalColours::Yellow => "\u{001b}[33m",
            TerminalColours::Blue => "\u{001b}[34m",
            TerminalColours::Magenta => "\u{001b}[35m",
            TerminalColours::Cyan => "\u{001b}[36m",
            TerminalColours::White => "\u{001b}[37m",
            TerminalColours::BrightBlack => "\u{001b}[30;1m",
            TerminalColours::BrightRed => "\u{001b}[31;1m",
            TerminalColours::BrightGreen => "\u{001b}[32;1m",
            TerminalColours::BrightYellow => "\u{001b}[33;1m",
            TerminalColours::BrightBlue => "\u{001b}[34;1m",
            TerminalColours::BrightMagenta => "\u{001b}[35;1m",
            TerminalColours::BrightCyan => "\u{001b}[36;1m",
            TerminalColours::BrightWhite => "\u{001b}[37;1m",
            TerminalColours::Reset => "\u{001b}[0m",
        }
    }
}

#[derive(Debug)]
enum ConversionError {
    /// This path contained invalid UTF8 characters
    Utf8,
    StripPrefix(path::StripPrefixError),

}

fn format_path(path: &Path) -> Result<String, ConversionError> {
    // TODO(richo) no longer hardcoded colours

    // Walk backward until we find a directory with a .git
    let ancestors = path.ancestors();
    for location in ancestors {
        if location.join(".git").exists() {
            let containing_dir = location.parent()
                .ok_or(ConversionError::Utf8)?;
            let repo_name = location.file_name()
                .ok_or(ConversionError::Utf8)?;
            let repo_local_path = path.strip_prefix(location)
                .map_err(|x| ConversionError::StripPrefix(x))?;

            return Ok(format!("{}{:?}/{}{:?}{}/{:?}{}", TerminalColours::Blue, containing_dir, TerminalColours::BrightWhite, repo_name, TerminalColours::Blue, repo_local_path, TerminalColours::Reset))
        }
    }

    path.to_str()
        .ok_or(ConversionError::Utf8)
        .map(|x| x.to_string())
}

fn main() -> Result<(), ConversionError> {
    let buf = PathBuf::from("/Users/richo/src/prompt-srv/src");
    let formatted = format_path(&buf)?;
    println!("{}", formatted);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_path() {
    }
}
