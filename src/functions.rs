use std::num::ParseIntError;

pub(crate) fn string_to_i64<S: AsRef<str>>(s: S) -> Result<i64, ParseIntError> {
    let s = s.as_ref();

    if s.len() > 2 {
        let b = s.as_bytes();

        if b[0] == b'0' {
            match b[1].to_ascii_lowercase() {
                b'b' => {
                    return Ok(u64::from_str_radix(&s[2..], 2)? as i64);
                }
                b'o' => {
                    return Ok(u64::from_str_radix(&s[2..], 8)? as i64);
                }
                b'x' => {
                    return Ok(u64::from_str_radix(&s[2..], 16)? as i64);
                }
                _ => (),
            }
        }
    }

    s.parse()
}
