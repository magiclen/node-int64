extern crate neon;

#[macro_use]
extern crate random_number;

mod functions;

use std::cmp::Ordering;

use neon::handle::Managed;
use neon::object::This;
use neon::prelude::*;
use neon::result::Throw;

use functions::*;

// 2^53 - 1;
const MAX_SAFE_INTEGER: i64 = 9007199254740991;

// -(2^53 - 1);
const MIN_SAFE_INTEGER: i64 = -9007199254740991;

fn to_i64<'a, T: Managed, K: This>(
    cx: &mut CallContext<'a, K>,
    value: Handle<JsValue>,
) -> Result<i64, JsResult<'a, T>> {
    if let Ok(value) = value.downcast::<JsString>().map(|v| v.value()) {
        return string_to_i64(value).map_err(|err| cx.throw_error(err.to_string()));
    }

    if let Ok(value) = value.downcast::<JsBuffer>() {
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

    if let Ok(value) = value.downcast::<JsInt64>() {
        let value = cx.borrow(&value, |int64| int64.0);

        return Ok(value);
    }

    let value = match value.downcast_or_throw::<JsNumber, _>(cx) {
        Ok(value) => value,
        Err(_) => return Err(cx.throw_type_error("the input is not a number")),
    }
    .value();

    if value.is_infinite() || value.is_nan() || value.fract() > f64::EPSILON {
        return Err(cx.throw_type_error(format!("{} is not an integer", value)));
    }

    Ok(value as i64)
}

fn to_js_number<'a, K: This>(cx: &mut CallContext<'a, K>, i: i64) -> JsResult<'a, JsNumber> {
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
pub struct Int64(i64);

fn to_js_int64<'a, K: This>(cx: &mut CallContext<'a, K>, i: i64) -> JsResult<'a, JsInt64> {
    if (MIN_SAFE_INTEGER..=MAX_SAFE_INTEGER).contains(&i) {
        let arg = JsNumber::new(cx, i as f64);

        JsInt64::new(cx, vec![arg])
    } else {
        let mut arg = JsBuffer::new(cx, 8)?;

        cx.borrow_mut(&mut arg, |buffer| buffer.as_mut_slice().copy_from_slice(&i.to_le_bytes()));

        JsInt64::new(cx, vec![arg])
    }
}

declare_types! {
    pub class JsInt64 for Int64 {
        init(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            Ok(Int64(arg))
        }

        method toDecimal(mut cx) {
            let this = cx.this();

            let i = {
                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0
            };

            Ok(JsString::new(&mut cx, format!("{}", i)).upcast())
        }

        method toBinary(mut cx) {
            let this = cx.this();

            let i = {
                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0
            };

            if let Some(Ok(arg)) = cx.argument_opt(0).map(|arg| arg.downcast::<JsBoolean>()) {
                if arg.value() {
                    return Ok(JsString::new(&mut cx, format!("{:#066b}", i)).upcast());
                }
            }

            Ok(JsString::new(&mut cx, format!("{:b}", i)).upcast())
        }

        method toOctal(mut cx) {
            let this = cx.this();

            let i = {
                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0
            };

            if let Some(Ok(arg)) = cx.argument_opt(0).map(|arg| arg.downcast::<JsBoolean>()) {
                if arg.value() {
                    return Ok(JsString::new(&mut cx, format!("{:#024o}", i)).upcast());
                }
            }

            Ok(JsString::new(&mut cx, format!("{:o}", i)).upcast())
        }

        method toHex(mut cx) {
            let this = cx.this();

            let i = {
                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0
            };

            let arg1 = if let Some(arg) = cx.argument_opt(0) {
                if let Ok(arg) = arg.downcast::<JsBoolean>() {
                    arg.value()
                } else {
                    false
                }
            } else {
                false
            };

            let arg2 = if let Some(arg) = cx.argument_opt(1) {
                if let Ok(arg) = arg.downcast::<JsBoolean>() {
                    arg.value()
                } else {
                    false
                }
            } else {
                false
            };

            if arg1 {
                if arg2 {
                    Ok(JsString::new(&mut cx, format!("{:#018X}", i)).upcast())
                } else {
                    Ok(JsString::new(&mut cx, format!("{:#018x}", i)).upcast())
                }
            } else if arg2 {
                Ok(JsString::new(&mut cx, format!("{:X}", i)).upcast())
            } else {
                Ok(JsString::new(&mut cx, format!("{:x}", i)).upcast())
            }
        }

        method toBuffer(mut cx) {
            let this = cx.this();

            let i: i64 = {
                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0
            };

            let mut buffer = JsBuffer::new(&mut cx, 8)?;

            cx.borrow_mut(&mut buffer, |buffer| {
                buffer.as_mut_slice().copy_from_slice(&i.to_le_bytes());
            });

            Ok(buffer.upcast())
        }

        method toNumber(mut cx) {
            let this = cx.this();

            let i: i64 = {
                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0
            };

            to_js_number(&mut cx, i).map(|v| v.upcast())
        }

        method set(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = arg;
            }

            Ok(this.upcast())
        }

        method add(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_add(arg);
            }

            Ok(this.upcast())
        }

        method subtract(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_sub(arg);
            }

            Ok(this.upcast())
        }

        method multiply(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_mul(arg);
            }

            Ok(this.upcast())
        }

        method divide(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_div(arg);
            }

            Ok(this.upcast())
        }

        method mod(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_rem(arg);
            }

            Ok(this.upcast())
        }

        method pow(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            if arg < 0 {
                return cx
                    .throw_range_error("the exponent of an integer number may not be smaller than zero");
            } else if arg > u32::MAX as i64 {
                return cx.throw_range_error(format!(
                    "the exponent of an integer number may not be bigger than {}",
                    u32::MAX
                ));
            }

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_pow(arg as u32);
            }

            Ok(this.upcast())
        }

        method shiftLeft(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            if arg < 0 {
                return cx.throw_range_error("the bit count for shift may not be smaller than zero");
            } else if arg > u32::MAX as i64 {
                return cx.throw_range_error(format!(
                    "the bit count for shift may not be bigger than {}",
                    u32::MAX
                ));
            }

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_shl(arg as u32);
            }

            Ok(this.upcast())
        }

        method shiftRight(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            if arg < 0 {
                return cx.throw_range_error("the bit count for shift may not be smaller than zero");
            } else if arg > u32::MAX as i64 {
                return cx.throw_range_error(format!(
                    "the bit count for shift may not be bigger than {}",
                    u32::MAX
                ));
            }

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.wrapping_shr(arg as u32);
            }

            Ok(this.upcast())
        }

        method shiftRightUnsigned(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            if arg < 0 {
                return cx.throw_range_error("the bit count for shift may not be smaller than zero");
            } else if arg > u32::MAX as i64 {
                return cx.throw_range_error(format!(
                    "the bit count for shift may not be bigger than {}",
                    u32::MAX
                ));
            }

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = (me.0 as u64).wrapping_shr(arg as u32) as i64;
            }

            Ok(this.upcast())
        }

        method rotateLeft(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            if arg < 0 {
                return cx.throw_range_error("the bit count for rotation may not be smaller than zero");
            } else if arg > u32::MAX as i64 {
                return cx.throw_range_error(format!(
                    "the bit count for rotation may not be bigger than {}",
                    u32::MAX
                ));
            }

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.rotate_left(arg as u32);
            }

            Ok(this.upcast())
        }

        method rotateRight(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            if arg < 0 {
                return cx.throw_range_error("the bit count for rotation may not be smaller than zero");
            } else if arg > u32::MAX as i64 {
                return cx.throw_range_error(format!(
                    "the bit count for rotation may not be bigger than {}",
                    u32::MAX
                ));
            }

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = me.0.rotate_right(arg as u32);
            }

            Ok(this.upcast())
        }

        method and(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 &= arg;
            }

            Ok(this.upcast())
        }

        method or(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 |= arg;
            }

            Ok(this.upcast())
        }

        method xor(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 ^= arg;
            }

            Ok(this.upcast())
        }

        method nand(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = !(me.0 & arg);
            }

            Ok(this.upcast())
        }

        method nor(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = !(me.0 | arg);
            }

            Ok(this.upcast())
        }

        method xnor(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = !(me.0 ^ arg);
            }

            Ok(this.upcast())
        }

        method not(mut cx) {
            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = !me.0;
            }

            Ok(this.upcast())
        }

        method negative(mut cx) {
            let mut this = cx.this();

            {
                let guard = cx.lock();
                let mut me = this.borrow_mut(&guard);

                me.0 = -me.0;
            }

            Ok(this.upcast())
        }

        method eq(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0 == arg
            };

            Ok(JsBoolean::new(&mut cx, c).upcast())
        }

        method ne(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0 != arg
            };

            Ok(JsBoolean::new(&mut cx, c).upcast())
        }

        method gt(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0 > arg
            };

            Ok(JsBoolean::new(&mut cx, c).upcast())
        }

        method gte(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0 >= arg
            };

            Ok(JsBoolean::new(&mut cx, c).upcast())
        }

        method lt(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0 < arg
            };

            Ok(JsBoolean::new(&mut cx, c).upcast())
        }

        method lte(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                me.0 <= arg
            };

            Ok(JsBoolean::new(&mut cx, c).upcast())
        }

        method comp(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                match me.0.cmp(&arg) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                }
            };

            Ok(JsNumber::new(&mut cx, c).upcast())
        }

        method random(mut cx) {
            let arg = match cx.argument_opt(0) {
                Some(arg) => match to_i64::<JsNumber, _>(&mut cx, arg) {
                    Ok(arg) => arg,
                    Err(_) => return Err(Throw),
                }
                None => return cx.throw_error("need an argument")
            };

            let c = {
                let this = cx.this();

                let guard = cx.lock();
                let me = this.borrow(&guard);

                random!(me.0, arg)
            };

            to_js_int64(&mut cx, c).map(|v| v.upcast())
        }
    }
}

neon::register_module!(mut cx, {
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

    cx.export_class::<JsInt64>("Int64")?;

    Ok(())
});
