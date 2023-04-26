use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::{to_i64, Int64, Ordering};

/// Computes `a + b`, wrapping around at the boundary of an 64-bit integer.
#[napi]
pub fn add(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a.wrapping_add(b)
    })
}

/// Computes `a - b`, wrapping around at the boundary of an 64-bit integer.
#[napi]
pub fn subtract(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a.wrapping_sub(b)
    })
}

/// Computes `a * b`, wrapping around at the boundary of an 64-bit integer.
#[napi]
pub fn multiply(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a.wrapping_mul(b)
    })
}

/// Computes `a / b`, wrapping around at the boundary of an 64-bit integer.
#[napi]
pub fn divide(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a.wrapping_div(b)
    })
}

/// Computes `a % b`.
#[napi(js_name = "mod")]
pub fn modulo(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a.wrapping_rem(b)
    })
}

/// Computes `a ^ b`, wrapping around at the boundary of an 64-bit integer.
///
/// `b` must not be smaller than zero
#[napi]
pub fn pow(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    if b < 0 {
        env.throw_range_error(
            "the exponent of an integer number must not be smaller than zero",
            None,
        )?;

        return Err(Error::from_reason(""));
    } else if b > u32::MAX as i64 {
        env.throw_range_error(
            &format!("the exponent of an integer number must not be bigger than {}", u32::MAX),
            None,
        )?;

        return Err(Error::from_reason(""));
    }

    Ok(Int64 {
        v: a.wrapping_pow(b as u32)
    })
}

/// Computes `a << b`, wrapping around at the boundary of an 64-bit integer.
///
/// `b` must not be smaller than zero
#[napi(js_name = "shiftLeft")]
pub fn shift_left(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    if b < 0 {
        env.throw_range_error("the bit count for shift must not be smaller than zero", None)?;

        return Err(Error::from_reason(""));
    } else if b > u32::MAX as i64 {
        env.throw_range_error(
            &format!("the bit count for shift must not be bigger than {}", u32::MAX),
            None,
        )?;

        return Err(Error::from_reason(""));
    }

    Ok(Int64 {
        v: a.wrapping_shl(b as u32)
    })
}

/// Computes `a >> b`, wrapping around at the boundary of an 64-bit integer.
///
/// `b` must not be smaller than zero
#[napi(js_name = "shiftRight")]
pub fn shift_right(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    if b < 0 {
        env.throw_range_error("the bit count for shift must not be smaller than zero", None)?;

        return Err(Error::from_reason(""));
    } else if b > u32::MAX as i64 {
        env.throw_range_error(
            &format!("the bit count for shift must not be bigger than {}", u32::MAX),
            None,
        )?;

        return Err(Error::from_reason(""));
    }

    Ok(Int64 {
        v: a.wrapping_shr(b as u32)
    })
}

/// Computes `a >>> b`, wrapping around at the boundary of an 64-bit integer.
///
/// `b` must not be smaller than zero
#[napi(js_name = "shiftRightUnsigned")]
pub fn shift_right_unsigned(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    if b < 0 {
        env.throw_range_error("the bit count for shift must not be smaller than zero", None)?;

        return Err(Error::from_reason(""));
    } else if b > u32::MAX as i64 {
        env.throw_range_error(
            &format!("the bit count for shift must not be bigger than {}", u32::MAX),
            None,
        )?;

        return Err(Error::from_reason(""));
    }

    Ok(Int64 {
        v: (a as u64).wrapping_shr(b as u32) as i64
    })
}

/// Shifts the bits to the left by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
///
/// `b` must not be smaller than zero
#[napi(js_name = "rotateLeft")]
pub fn rotate_left(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    if b < 0 {
        env.throw_range_error("the bit count for rotation must not be smaller than zero", None)?;

        return Err(Error::from_reason(""));
    } else if b > u32::MAX as i64 {
        env.throw_range_error(
            &format!("the bit count for rotation must not be bigger than {}", u32::MAX),
            None,
        )?;

        return Err(Error::from_reason(""));
    }

    Ok(Int64 {
        v: a.rotate_left(b as u32)
    })
}

/// Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
///
/// `b` must not be smaller than zero
#[napi(js_name = "rotateRight")]
pub fn rotate_right(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    if b < 0 {
        env.throw_range_error("the bit count for rotation must not be smaller than zero", None)?;

        return Err(Error::from_reason(""));
    } else if b > u32::MAX as i64 {
        env.throw_range_error(
            &format!("the bit count for rotation must not be bigger than {}", u32::MAX),
            None,
        )?;

        return Err(Error::from_reason(""));
    }

    Ok(Int64 {
        v: a.rotate_right(b as u32)
    })
}

/// Computes `a & b`.
#[napi]
pub fn and(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a & b
    })
}

/// Computes `a | b`.
#[napi]
pub fn or(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a | b
    })
}

/// Computes `a ^ b`.
#[napi]
pub fn xor(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: a ^ b
    })
}

/// Computes `~(a & b)`.
#[napi]
pub fn nand(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let mut n = and(env, a, b)?;

    n.v = !n.v;

    Ok(n)
}

/// Computes `~(a | b)`.
#[napi]
pub fn nor(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let mut n = or(env, a, b)?;

    n.v = !n.v;

    Ok(n)
}

/// Computes `~(a ^ b)`.
#[napi]
pub fn xnor(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let mut n = xor(env, a, b)?;

    n.v = !n.v;

    Ok(n)
}

/// Computes `~a`.
#[napi]
pub fn not(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;

    Ok(Int64 {
        v: !a
    })
}

/// Computes `-a`.
#[napi]
pub fn negative(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;

    Ok(Int64 {
        v: -a
    })
}

/// Computes `a === b`.
#[napi]
pub fn eq(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<bool> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a == b)
}

/// Computes `a !== b`.
#[napi]
pub fn ne(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<bool> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a != b)
}

/// Computes `a > b`.
#[napi]
pub fn gt(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<bool> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a > b)
}

/// Computes `a >= b`.
#[napi]
pub fn gte(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<bool> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a >= b)
}

/// Computes `a < b`.
#[napi]
pub fn lt(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<bool> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a < b)
}

/// Computes `a <= b`.
#[napi]
pub fn lte(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<bool> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a <= b)
}

/// If `a < b`, returns `-1`.
/// If `a === b`, returns `0`.
/// If `a > b`, returns `1`.
#[napi]
pub fn comp(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Ordering> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(a.cmp(&b).into())
}

/// Gets a random 64-bit integer between `a` and `b`.
#[napi]
pub fn random(
    env: Env,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] a: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
    #[napi(ts_arg_type = "number | string | Buffer | Int64")] b: Either4<
        &Int64,
        i64,
        String,
        Buffer,
    >,
) -> Result<Int64> {
    let a = to_i64(&env, a)?;
    let b = to_i64(&env, b)?;

    Ok(Int64 {
        v: random_number::random!(a, b)
    })
}
