use std::num::ParseIntError;

use napi::{bindgen_prelude::*, JsBuffer};

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

fn to_i64_inner(value: Either<&Int64, Unknown>) -> Result<i64> {
    match value {
        Either::A(int64) => Ok(int64.v),
        Either::B(value) => {
            let typ = value.get_type()?;

            match typ {
                ValueType::Number => value.coerce_to_number()?.get_int64(),
                ValueType::String => {
                    string_to_i64(value.coerce_to_string()?.into_utf8()?.as_str()?)
                        .map_err(|err| Error::from_reason(err.to_string()))
                },
                ValueType::Object => {
                    if value.is_buffer()? {
                        let buffer = unsafe { value.cast::<JsBuffer>() };

                        let buffer_value = buffer.into_value()?;
                        let data = buffer_value.as_ref();

                        if data.len() == 8 {
                            Ok(i64::from_le_bytes([
                                data[0], data[1], data[2], data[3], data[4], data[5], data[6],
                                data[7],
                            ]))
                        } else {
                            Err(Error::from_reason("the length of the input buffer is not 8"))
                        }
                    } else {
                        Err(Error::from_reason(
                            "the int64 (long) value is an object, but it's not a buffer",
                        ))
                    }
                },
                _ => Err(Error::from_reason(
                    "an int64 (long) value should be a number, a string or an object",
                )),
            }
        },
    }
}

#[inline]
pub(crate) fn to_i64(env: &Env, value: Either<&Int64, Unknown>) -> Result<i64> {
    match to_i64_inner(value) {
        Ok(value) => Ok(value),
        Err(error) => {
            env.throw_type_error(&error.reason, None)?;

            Err(Error::from_reason(""))
        },
    }
}
