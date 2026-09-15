use std::num::ParseIntError;

use napi::{
    Env, Error, Result, Status, ValueType,
    bindgen_prelude::{BufferSlice, Either5, Unknown},
};

use crate::Int64;

/// Every JavaScript representation of a 64-bit integer that this library accepts.
pub(crate) type Int64Value<'a> = Either5<&'a Int64, i64, String, BufferSlice<'a>, Unknown<'a>>;

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

pub(crate) fn to_i64(env: &Env, value: Int64Value<'_>) -> Result<i64> {
    match value {
        Either5::A(int64) => Ok(int64.v),
        Either5::B(number) => Ok(number),
        Either5::C(s) => string_to_i64(s).map_err(|error| type_error(env, &error.to_string())),
        Either5::D(buffer) => match <[u8; 8]>::try_from(buffer.as_ref()) {
            Ok(bytes) => Ok(i64::from_le_bytes(bytes)),
            Err(_) => Err(type_error(env, "the length of the input buffer is not 8")),
        },
        // Nothing else can be a 64-bit integer, so this arm only picks the right message.
        Either5::E(value) => Err(type_error(env, match value.get_type()? {
            ValueType::Object => "the int64 (long) value is an object, but it's not a buffer",
            _ => "an int64 (long) value should be a number, a string or an object",
        })),
    }
}

/// Throws a JavaScript `RangeError` and returns the error that keeps it pending.
pub(crate) fn range_error(env: &Env, message: &str) -> Error {
    match env.throw_range_error(message, None) {
        Ok(()) => Error::new(Status::PendingException, message),
        Err(error) => error,
    }
}

/// Throws a JavaScript `TypeError` and returns the error that keeps it pending.
pub(crate) fn type_error(env: &Env, message: &str) -> Error {
    match env.throw_type_error(message, None) {
        Ok(()) => Error::new(Status::PendingException, message),
        Err(error) => error,
    }
}

/// Validates the exponent of a power, or the bit count of a shift or a rotation.
pub(crate) fn to_u32(env: &Env, value: i64, subject: &str) -> Result<u32> {
    if value < 0 {
        return Err(range_error(env, &format!("{subject} must not be smaller than zero")));
    } else if value > u32::MAX as i64 {
        return Err(range_error(env, &format!("{subject} must not be bigger than {}", u32::MAX)));
    }

    Ok(value as u32)
}

pub(crate) const EXPONENT: &str = "the exponent of an integer number";
pub(crate) const SHIFT_BIT_COUNT: &str = "the bit count for shift";
pub(crate) const ROTATION_BIT_COUNT: &str = "the bit count for rotation";

/// Computes `a / b`, rejecting the zero divisor that would otherwise panic.
pub(crate) fn divide_i64(env: &Env, a: i64, b: i64) -> Result<i64> {
    if b == 0 {
        return Err(range_error(env, "the divisor must not be zero"));
    }

    Ok(a.wrapping_div(b))
}

/// Computes `a % b`, rejecting the zero divisor that would otherwise panic.
pub(crate) fn modulo_i64(env: &Env, a: i64, b: i64) -> Result<i64> {
    if b == 0 {
        return Err(range_error(env, "the divisor must not be zero"));
    }

    Ok(a.wrapping_rem(b))
}

/// Picks a random integer between `a` and `b`, in either order.
pub(crate) fn random_i64(a: i64, b: i64) -> i64 {
    let (min, max) = if a <= b { (a, b) } else { (b, a) };

    random_number::random!(min, max)
}
