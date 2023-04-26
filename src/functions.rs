use std::num::ParseIntError;

use napi::bindgen_prelude::*;

use crate::Int64;

pub(crate) fn string_to_i64<S: AsRef<str>>(s: S) -> std::result::Result<i64, ParseIntError> {
    let s = s.as_ref();

    if s.len() > 2 {
        let b = s.as_bytes();

        if b[0] == b'0' {
            match b[1].to_ascii_lowercase() {
                b'b' => {
                    return i64::from_str_radix(&s[2..], 2);
                },
                b'o' => {
                    return i64::from_str_radix(&s[2..], 8);
                },
                b'x' => {
                    return i64::from_str_radix(&s[2..], 16);
                },
                _ => (),
            }
        }
    }

    s.parse()
}

fn to_i64_inner(value: Either4<&Int64, i64, String, Buffer>) -> Result<i64> {
    match value {
        Either4::A(int64) => Ok(int64.v),
        Either4::B(value) => Ok(value),
        Either4::C(value) => {
            string_to_i64(value).map_err(|err| Error::from_reason(err.to_string()))
        },
        Either4::D(value) => {
            let data = value.as_ref();

            if data.len() == 8 {
                Ok(i64::from_le_bytes([
                    data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
                ]))
            } else {
                Err(Error::from_reason("the length of the input buffer is not 8"))
            }
        },
    }
}

#[inline]
pub(crate) fn to_i64(env: &Env, value: Either4<&Int64, i64, String, Buffer>) -> Result<i64> {
    match to_i64_inner(value) {
        Ok(value) => Ok(value),
        Err(error) => {
            env.throw_type_error(&error.reason, None)?;

            Err(Error::from_reason(""))
        },
    }
}
