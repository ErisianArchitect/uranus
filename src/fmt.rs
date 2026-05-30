
pub enum CheckFormatError<'a> {
    InvalidInterpolation(&'a str),
    UnclosedBrace(usize),
    Io(std::io::Error),
}

pub type CheckFormatResult<'a, T = (), E = CheckFormatError<'a>> = std::result::Result<T, E>;

impl<'a> From<std::io::Error> for CheckFormatError<'a> {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub fn check_format<'a, F: FnMut(&'a str) -> CheckFormatResult<'a>>(fmt: &'a str, mut f: F) -> CheckFormatResult<'a> {
    let mut index = 0;
    while index < fmt.len() {
        let Some(next_open) = fmt[index..].find('{') else {
            return Ok(());
        };
        let open_index = index + next_open;
        if fmt[open_index..].starts_with("{{") {
            index = open_index + 2;
            continue;
        }
        let Some(next_close) = fmt[open_index + 1..].find('}') else {
            return Err(CheckFormatError::UnclosedBrace(open_index));
        };
        let close_index = open_index + 1 + next_close;
        let inner_range = (open_index + 1)..close_index;
        f(&fmt[inner_range])?;
        index = close_index + 1;
    }
    Ok(())
}

pub fn format<'a, F: FnMut(&'a str, &mut String) -> CheckFormatResult<'a>>(s: &'a str, mut f: F) -> CheckFormatResult<'a, String> {
    let mut buffer = String::new();
    let mut index = 0;
    while index < s.len() {
        let Some(next_open) = s[index..].find('{') else {
            buffer.push_str(&s[index..]);
            return Ok(buffer);
        };
        let open_index = index + next_open;
        if s[open_index..].starts_with("{{") {
            buffer.push_str("{{");
            index = open_index + 2;
            continue;
        }
        buffer.push_str(&s[index..open_index]);
        
        let mut close_scan = open_index + 1;
        while close_scan < s.len() {
            let Some(next_close) = s[close_scan..].find('}') else {
                return Err(CheckFormatError::UnclosedBrace(open_index));
            };
            let next_close = close_scan + next_close;
            if s[next_close + 1..].starts_with('}') {
                close_scan = next_close + 2;
                continue;
            }
            // close_scan = next_close;
            close_scan = next_close;
            break
        };

        let inner_range = (open_index + 1)..close_scan;
        f(&s[inner_range], &mut buffer)?;
        index = close_scan + 1;
    }
    Ok(buffer)
}
