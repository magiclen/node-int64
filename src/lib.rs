#![allow(clippy::wrong_self_convention)]

extern crate neon;

#[macro_use]
extern crate random_number;

mod functions;

use std::cmp::Ordering;
use std::sync::atomic::{self, AtomicI64};

use neon::handle::Managed;
use neon::prelude::*;

use functions::*;

// 2^53 - 1;
const MAX_SAFE_INTEGER: i64 = 9007199254740991;

// -(2^53 - 1);
const MIN_SAFE_INTEGER: i64 = -9007199254740991;

fn to_i64<'a, T: Managed>(
    cx: &mut FunctionContext<'a>,
    value: Handle<JsValue>,
) -> Result<i64, JsResult<'a, T>> {
    if let Ok(value) = value.downcast::<JsString, _>(cx).map(|v| v.value(cx)) {
        return string_to_i64(value).map_err(|err| cx.throw_error(err.to_string()));
    }

    if let Ok(value) = value.downcast::<JsBuffer, _>(cx) {
        let value = cx.borrow(&value, |buffer| {
            if buffer.len() == 8 {
                let b = buffer.as_slice::<u8>();
                Some(i64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]))
            } else {
                None
            }
        });

        return match value {
            Some(value) => Ok(value),
            None => Err(cx.throw_type_error("the length of the input buffer is not 8")),
        };
    }

    if let Ok(value) = value.downcast::<JsBox<Int64>, _>(cx) {
        return Ok(value.0.load(atomic::Ordering::Relaxed));
    }

    if let Ok(value) = value.downcast::<JsObject, _>(cx) {
        if let Ok(boxed) = value.get(cx, "boxed") {
            if let Ok(value) = boxed.downcast::<JsBox<Int64>, _>(cx) {
                return Ok(value.0.load(atomic::Ordering::Relaxed));
            }
        }
    }

    let value = value.downcast_or_throw::<JsNumber, _>(cx).map_err(Err)?.value(cx);

    if value.is_infinite() || value.is_nan() || value.fract() > f64::EPSILON {
        return Err(cx.throw_type_error(format!("{} is not an integer", value)));
    }

    Ok(value as i64)
}

fn to_js_number<'a>(cx: &mut FunctionContext<'a>, i: i64) -> JsResult<'a, JsNumber> {
    if i > MAX_SAFE_INTEGER {
        cx.throw_range_error(format!("{} is bigger than {}", i, MAX_SAFE_INTEGER))
    } else if i < MIN_SAFE_INTEGER {
        cx.throw_range_error(format!("{} is smaller than {}", i, MIN_SAFE_INTEGER))
    } else {
        Ok(JsNumber::new(cx, i as f64))
    }
}

#[inline]
fn js_number_0<'a>(cx: &mut FunctionContext<'a>) -> JsResult<'a, JsNumber> {
    Ok(JsNumber::new(cx, 0))
}

#[inline]
fn js_number_1<'a>(cx: &mut FunctionContext<'a>) -> JsResult<'a, JsNumber> {
    Ok(JsNumber::new(cx, 1))
}

#[inline]
fn js_number_n1<'a>(cx: &mut FunctionContext<'a>) -> JsResult<'a, JsNumber> {
    Ok(JsNumber::new(cx, -1))
}

#[inline]
fn to_js_boolean<'a>(cx: &mut FunctionContext<'a>, b: bool) -> JsResult<'a, JsBoolean> {
    Ok(JsBoolean::new(cx, b))
}

fn parse_arguments_number<'a>(
    cx: &mut FunctionContext<'a>,
) -> Result<(i64, i64), JsResult<'a, JsNumber>> {
    match cx.argument_opt(0) {
        Some(arg1) => {
            let arg1 = match to_i64(cx, arg1) {
                Ok(arg) => arg,
                Err(err) => return Err(err),
            };

            match cx.argument_opt(1) {
                Some(arg2) => {
                    let arg2 = match to_i64(cx, arg2) {
                        Ok(arg) => arg,
                        Err(err) => return Err(err),
                    };

                    Ok((arg1, arg2))
                }
                None => Err(to_js_number(cx, arg1)),
            }
        }
        None => Err(js_number_0(cx)),
    }
}

fn parse_arguments_boolean<'a>(
    cx: &mut FunctionContext<'a>,
) -> Result<(i64, i64), JsResult<'a, JsBoolean>> {
    if cx.len() < 2 {
        return Err(cx.throw_error("need two arguments"));
    }

    let arg1 = cx.argument_opt(0).unwrap();
    let arg2 = cx.argument_opt(1).unwrap();

    let arg1 = match to_i64(cx, arg1) {
        Ok(arg) => arg,
        Err(err) => return Err(err),
    };

    let arg2 = match to_i64(cx, arg2) {
        Ok(arg) => arg,
        Err(err) => return Err(err),
    };

    Ok((arg1, arg2))
}

fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let sum = arg1.wrapping_add(arg2);

    to_js_number(&mut cx, sum)
}

fn subtract(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let difference = arg1.wrapping_sub(arg2);

    to_js_number(&mut cx, difference)
}

fn multiply(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let product = arg1.wrapping_mul(arg2);

    to_js_number(&mut cx, product)
}

fn divide(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let quotient = arg1.wrapping_div(arg2);

    to_js_number(&mut cx, quotient)
}

fn r#mod(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let remainder = arg1.wrapping_rem(arg2);

    to_js_number(&mut cx, remainder)
}

fn pow(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    if arg2 < 0 {
        return cx
            .throw_range_error("the exponent of an integer number may not be smaller than zero");
    } else if arg2 > u32::MAX as i64 {
        return cx.throw_range_error(format!(
            "the exponent of an integer number may not be bigger than {}",
            u32::MAX
        ));
    }

    let product = arg1.wrapping_pow(arg2 as u32);

    to_js_number(&mut cx, product)
}

fn shift_left(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    if arg2 < 0 {
        return cx.throw_range_error("the bit count for shift may not be smaller than zero");
    } else if arg2 > u32::MAX as i64 {
        return cx.throw_range_error(format!(
            "the bit count for shift may not be bigger than {}",
            u32::MAX
        ));
    }

    let c = arg1.wrapping_shl(arg2 as u32);

    to_js_number(&mut cx, c)
}

fn shift_right(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    if arg2 < 0 {
        return cx.throw_range_error("the bit count for shift may not be smaller than zero");
    } else if arg2 > u32::MAX as i64 {
        return cx.throw_range_error(format!(
            "the bit count for shift may not be bigger than {}",
            u32::MAX
        ));
    }

    let c = arg1.wrapping_shr(arg2 as u32);

    to_js_number(&mut cx, c)
}

fn shift_right_unsigned(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    if arg2 < 0 {
        return cx.throw_range_error("the bit count for shift may not be smaller than zero");
    } else if arg2 > u32::MAX as i64 {
        return cx.throw_range_error(format!(
            "the bit count for shift may not be bigger than {}",
            u32::MAX
        ));
    }

    let c = (arg1 as u64).wrapping_shr(arg2 as u32);

    to_js_number(&mut cx, c as i64)
}

fn rotate_left(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    if arg2 < 0 {
        return cx.throw_range_error("the bit count for rotation may not be smaller than zero");
    } else if arg2 > u32::MAX as i64 {
        return cx.throw_range_error(format!(
            "the bit count for rotation may not be bigger than {}",
            u32::MAX
        ));
    }

    let c = arg1.rotate_left(arg2 as u32);

    to_js_number(&mut cx, c)
}

fn rotate_right(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    if arg2 < 0 {
        return cx.throw_range_error("the bit count for rotation may not be smaller than zero");
    } else if arg2 > u32::MAX as i64 {
        return cx.throw_range_error(format!(
            "the bit count for rotation may not be bigger than {}",
            u32::MAX
        ));
    }

    let c = arg1.rotate_right(arg2 as u32);

    to_js_number(&mut cx, c)
}

fn and(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let c = arg1 & arg2;

    to_js_number(&mut cx, c)
}

fn or(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let c = arg1 | arg2;

    to_js_number(&mut cx, c)
}

fn xor(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let c = arg1 ^ arg2;

    to_js_number(&mut cx, c)
}

fn nand(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let c = !(arg1 & arg2);

    to_js_number(&mut cx, c)
}

fn nor(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let c = !(arg1 | arg2);

    to_js_number(&mut cx, c)
}

fn xnor(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let c = !(arg1 ^ arg2);

    to_js_number(&mut cx, c)
}

fn not(mut cx: FunctionContext) -> JsResult<JsNumber> {
    match cx.argument_opt(0) {
        Some(arg) => {
            let arg = match to_i64(&mut cx, arg) {
                Ok(arg) => arg,
                Err(err) => return err,
            };

            let c = !arg;

            to_js_number(&mut cx, c)
        }
        None => js_number_0(&mut cx),
    }
}

fn negative(mut cx: FunctionContext) -> JsResult<JsNumber> {
    match cx.argument_opt(0) {
        Some(arg) => {
            let arg = match to_i64(&mut cx, arg) {
                Ok(arg) => arg,
                Err(err) => return err,
            };

            let c = -arg;

            to_js_number(&mut cx, c)
        }
        None => js_number_0(&mut cx),
    }
}

fn eq(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let (arg1, arg2) = match parse_arguments_boolean(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let b = arg1 == arg2;

    to_js_boolean(&mut cx, b)
}

fn ne(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let (arg1, arg2) = match parse_arguments_boolean(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let b = arg1 != arg2;

    to_js_boolean(&mut cx, b)
}

fn gt(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let (arg1, arg2) = match parse_arguments_boolean(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let b = arg1 > arg2;

    to_js_boolean(&mut cx, b)
}

fn gte(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let (arg1, arg2) = match parse_arguments_boolean(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let b = arg1 >= arg2;

    to_js_boolean(&mut cx, b)
}

fn lt(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let (arg1, arg2) = match parse_arguments_boolean(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let b = arg1 < arg2;

    to_js_boolean(&mut cx, b)
}

fn lte(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let (arg1, arg2) = match parse_arguments_boolean(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    let b = arg1 <= arg2;

    to_js_boolean(&mut cx, b)
}

fn comp(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let (arg1, arg2) = match parse_arguments_number(&mut cx) {
        Ok(args) => args,
        Err(err) => return err,
    };

    match arg1.cmp(&arg2) {
        Ordering::Equal => js_number_0(&mut cx),
        Ordering::Greater => js_number_1(&mut cx),
        Ordering::Less => js_number_n1(&mut cx),
    }
}

fn random(mut cx: FunctionContext) -> JsResult<JsNumber> {
    if cx.len() < 2 {
        return cx.throw_error("need two arguments");
    }

    let arg1 = cx.argument_opt(0).unwrap();
    let arg2 = cx.argument_opt(1).unwrap();

    let arg1 = match to_i64(&mut cx, arg1) {
        Ok(arg) => {
            if arg > MAX_SAFE_INTEGER {
                return cx
                    .throw_range_error(format!("{} is bigger than {}", arg, MAX_SAFE_INTEGER));
            } else if arg < MIN_SAFE_INTEGER {
                return cx
                    .throw_range_error(format!("{} is smaller than {}", arg, MIN_SAFE_INTEGER));
            } else {
                arg
            }
        }
        Err(err) => return err,
    };

    let arg2 = match to_i64(&mut cx, arg2) {
        Ok(arg) => {
            if arg > MAX_SAFE_INTEGER {
                return cx
                    .throw_range_error(format!("{} is bigger than {}", arg, MAX_SAFE_INTEGER));
            } else if arg < MIN_SAFE_INTEGER {
                return cx
                    .throw_range_error(format!("{} is smaller than {}", arg, MIN_SAFE_INTEGER));
            } else {
                arg
            }
        }
        Err(err) => return err,
    };

    let c = random!(arg1, arg2);

    to_js_number(&mut cx, c)
}

// TODO class

#[derive(Debug)]
pub struct Int64(AtomicI64);

impl Finalize for Int64 {}

#[inline]
fn to_js_string<'a, S: Into<String>>(cx: &mut FunctionContext<'a>, s: S) -> JsResult<'a, JsString> {
    Ok(JsString::new(cx, s.into()))
}

#[inline]
fn to_js_int64<'a>(cx: &mut FunctionContext<'a>, i: i64) -> JsResult<'a, JsBox<Int64>> {
    Ok(JsBox::new(cx, Int64(AtomicI64::new(i))))
}

impl Int64 {
    fn init(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let arg = match cx.argument_opt(0) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        to_js_int64(&mut cx, arg)
    }

    fn to_decimal(mut cx: FunctionContext) -> JsResult<JsString> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_string(&mut cx, format!("{}", i))
    }

    fn to_binary(mut cx: FunctionContext) -> JsResult<JsString> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Relaxed);

        if let Some(Ok(arg)) = cx.argument_opt(1).map(|arg| arg.downcast::<JsBoolean, _>(&mut cx)) {
            if arg.value(&mut cx) {
                return to_js_string(&mut cx, format!("{:#066b}", i));
            }
        }

        to_js_string(&mut cx, format!("{:b}", i))
    }

    fn to_octal(mut cx: FunctionContext) -> JsResult<JsString> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Relaxed);

        if let Some(Ok(arg)) = cx.argument_opt(1).map(|arg| arg.downcast::<JsBoolean, _>(&mut cx)) {
            if arg.value(&mut cx) {
                return to_js_string(&mut cx, format!("{:#024o}", i));
            }
        }

        to_js_string(&mut cx, format!("{:o}", i))
    }

    fn to_hex(mut cx: FunctionContext) -> JsResult<JsString> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Relaxed);

        let arg1 = if let Some(arg) = cx.argument_opt(1) {
            if let Ok(arg) = arg.downcast::<JsBoolean, _>(&mut cx) {
                arg.value(&mut cx)
            } else {
                false
            }
        } else {
            false
        };

        let arg2 = if let Some(arg) = cx.argument_opt(2) {
            if let Ok(arg) = arg.downcast::<JsBoolean, _>(&mut cx) {
                arg.value(&mut cx)
            } else {
                false
            }
        } else {
            false
        };

        if arg1 {
            if arg2 {
                to_js_string(&mut cx, format!("{:#018X}", i))
            } else {
                to_js_string(&mut cx, format!("{:#018x}", i))
            }
        } else if arg2 {
            to_js_string(&mut cx, format!("{:X}", i))
        } else {
            to_js_string(&mut cx, format!("{:#x}", i))
        }
    }

    fn to_buffer(mut cx: FunctionContext) -> JsResult<JsBuffer> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Relaxed);

        let mut buffer = unsafe { JsBuffer::uninitialized(&mut cx, 8)? };

        cx.borrow_mut(&mut buffer, |buffer| {
            buffer.as_mut_slice().copy_from_slice(&i.to_le_bytes());
        });

        Ok(buffer)
    }

    fn to_number(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_number(&mut cx, i)
    }

    fn set(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.store(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn add(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.fetch_add(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn subtract(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.fetch_sub(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn multiply(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.wrapping_mul(arg), atomic::Ordering::Release);

        Ok(this)
    }

    fn divide(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.wrapping_div(arg), atomic::Ordering::Release);

        Ok(this)
    }

    fn r#mod(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.wrapping_rem(arg), atomic::Ordering::Release);

        Ok(this)
    }

    fn pow(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        if arg < 0 {
            return cx.throw_range_error(
                "the exponent of an integer number may not be smaller than zero",
            );
        } else if arg > u32::MAX as i64 {
            return cx.throw_range_error(format!(
                "the exponent of an integer number may not be bigger than {}",
                u32::MAX
            ));
        }

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.wrapping_pow(arg as u32), atomic::Ordering::Release);

        Ok(this)
    }

    fn shift_left(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        if arg < 0 {
            return cx.throw_range_error("the bit count for shift may not be smaller than zero");
        } else if arg > u32::MAX as i64 {
            return cx.throw_range_error(format!(
                "the bit count for shift may not be bigger than {}",
                u32::MAX
            ));
        }

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.wrapping_shl(arg as u32), atomic::Ordering::Release);

        Ok(this)
    }

    fn shift_right(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        if arg < 0 {
            return cx.throw_range_error("the bit count for shift may not be smaller than zero");
        } else if arg > u32::MAX as i64 {
            return cx.throw_range_error(format!(
                "the bit count for shift may not be bigger than {}",
                u32::MAX
            ));
        }

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.wrapping_shr(arg as u32), atomic::Ordering::Release);

        Ok(this)
    }

    fn shift_right_unsigned(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        if arg < 0 {
            return cx.throw_range_error("the bit count for shift may not be smaller than zero");
        } else if arg > u32::MAX as i64 {
            return cx.throw_range_error(format!(
                "the bit count for shift may not be bigger than {}",
                u32::MAX
            ));
        }

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store((i as u64).wrapping_shr(arg as u32) as i64, atomic::Ordering::Release);

        Ok(this)
    }

    fn rotate_left(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        if arg < 0 {
            return cx.throw_range_error("the bit count for rotation may not be smaller than zero");
        } else if arg > u32::MAX as i64 {
            return cx.throw_range_error(format!(
                "the bit count for rotation may not be bigger than {}",
                u32::MAX
            ));
        }

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.rotate_left(arg as u32), atomic::Ordering::Release);

        Ok(this)
    }

    fn rotate_right(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        if arg < 0 {
            return cx.throw_range_error("the bit count for rotation may not be smaller than zero");
        } else if arg > u32::MAX as i64 {
            return cx.throw_range_error(format!(
                "the bit count for rotation may not be bigger than {}",
                u32::MAX
            ));
        }

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(i.rotate_right(arg as u32), atomic::Ordering::Release);

        Ok(this)
    }

    fn and(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.fetch_and(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn or(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.fetch_or(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn xor(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.fetch_xor(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn nand(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        this.0.fetch_nand(arg, atomic::Ordering::Relaxed);

        Ok(this)
    }

    fn nor(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(!(i | arg), atomic::Ordering::Release);

        Ok(this)
    }

    fn xnor(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(!(i ^ arg), atomic::Ordering::Release);

        Ok(this)
    }

    fn not(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(!i, atomic::Ordering::Release);

        Ok(this)
    }

    fn negative(mut cx: FunctionContext) -> JsResult<JsBox<Int64>> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let i = this.0.load(atomic::Ordering::Acquire);

        this.0.store(-i, atomic::Ordering::Release);

        Ok(this)
    }

    fn eq(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_boolean(&mut cx, i == arg)
    }

    fn ne(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_boolean(&mut cx, i != arg)
    }

    fn gt(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_boolean(&mut cx, i > arg)
    }

    fn gte(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_boolean(&mut cx, i >= arg)
    }

    fn lt(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_boolean(&mut cx, i < arg)
    }

    fn lte(mut cx: FunctionContext) -> JsResult<JsBoolean> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_boolean(&mut cx, i <= arg)
    }

    fn comp(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        let c = match i.cmp(&arg) {
            Ordering::Equal => 0,
            Ordering::Greater => 1,
            Ordering::Less => -1,
        };

        to_js_number(&mut cx, c)
    }

    fn random(mut cx: FunctionContext) -> JsResult<JsNumber> {
        let this = cx.argument::<JsBox<Int64>>(0).unwrap();

        let arg = match cx.argument_opt(1) {
            Some(arg) => {
                match to_i64(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(err) => return err,
                }
            }
            None => return cx.throw_error("need an argument"),
        };

        let i = this.0.load(atomic::Ordering::Relaxed);

        to_js_number(&mut cx, random!(i, arg))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("add", add)?;
    cx.export_function("subtract", subtract)?;
    cx.export_function("multiply", multiply)?;
    cx.export_function("divide", divide)?;
    cx.export_function("mod", r#mod)?;
    cx.export_function("pow", pow)?;
    cx.export_function("shiftLeft", shift_left)?;
    cx.export_function("shiftRight", shift_right)?;
    cx.export_function("shiftRightUnsigned", shift_right_unsigned)?;
    cx.export_function("rotateLeft", rotate_left)?;
    cx.export_function("rotateRight", rotate_right)?;
    cx.export_function("and", and)?;
    cx.export_function("or", or)?;
    cx.export_function("xor", xor)?;
    cx.export_function("nand", nand)?;
    cx.export_function("nor", nor)?;
    cx.export_function("xnor", xnor)?;
    cx.export_function("not", not)?;
    cx.export_function("negative", negative)?;
    cx.export_function("eq", eq)?;
    cx.export_function("ne", ne)?;
    cx.export_function("gt", gt)?;
    cx.export_function("gte", gte)?;
    cx.export_function("lt", lt)?;
    cx.export_function("lte", lte)?;
    cx.export_function("comp", comp)?;
    cx.export_function("random", random)?;

    cx.export_function("initMethod", Int64::init)?;
    cx.export_function("toDecimalMethod", Int64::to_decimal)?;
    cx.export_function("toBinaryMethod", Int64::to_binary)?;
    cx.export_function("toOctalMethod", Int64::to_octal)?;
    cx.export_function("toHexMethod", Int64::to_hex)?;
    cx.export_function("toBufferMethod", Int64::to_buffer)?;
    cx.export_function("toNumberMethod", Int64::to_number)?;
    cx.export_function("setMethod", Int64::set)?;
    cx.export_function("addMethod", Int64::add)?;
    cx.export_function("subtractMethod", Int64::subtract)?;
    cx.export_function("multiplyMethod", Int64::multiply)?;
    cx.export_function("divideMethod", Int64::divide)?;
    cx.export_function("modMethod", Int64::r#mod)?;
    cx.export_function("powMethod", Int64::pow)?;
    cx.export_function("shiftLeftMethod", Int64::shift_left)?;
    cx.export_function("shiftRightMethod", Int64::shift_right)?;
    cx.export_function("shiftRightUnsignedMethod", Int64::shift_right_unsigned)?;
    cx.export_function("rotateLeftMethod", Int64::rotate_left)?;
    cx.export_function("rotateRightMethod", Int64::rotate_right)?;
    cx.export_function("andMethod", Int64::and)?;
    cx.export_function("orMethod", Int64::or)?;
    cx.export_function("xorMethod", Int64::xor)?;
    cx.export_function("nandMethod", Int64::nand)?;
    cx.export_function("norMethod", Int64::nor)?;
    cx.export_function("xnorMethod", Int64::xnor)?;
    cx.export_function("notMethod", Int64::not)?;
    cx.export_function("negativeMethod", Int64::negative)?;
    cx.export_function("eqMethod", Int64::eq)?;
    cx.export_function("neMethod", Int64::ne)?;
    cx.export_function("gtMethod", Int64::gt)?;
    cx.export_function("gteMethod", Int64::gte)?;
    cx.export_function("ltMethod", Int64::lt)?;
    cx.export_function("lteMethod", Int64::lte)?;
    cx.export_function("compMethod", Int64::comp)?;
    cx.export_function("randomMethod", Int64::random)?;

    Ok(())
}
