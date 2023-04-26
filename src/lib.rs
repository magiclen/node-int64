mod functions;
mod napi_static;

use functions::*;
use napi::bindgen_prelude::*;
use napi_derive::napi;
pub use napi_static::*;

// 2^53 - 1;
const MAX_SAFE_INTEGER: i64 = 9007199254740991;

// -(2^53 - 1);
const MIN_SAFE_INTEGER: i64 = -9007199254740991;

// We need this enum to build the definition of Ordering.
#[napi]
pub enum Ordering {
    Less    = -1,
    Equal   = 0,
    Greater = 1,
}

impl From<std::cmp::Ordering> for Ordering {
    #[inline]
    fn from(value: std::cmp::Ordering) -> Self {
        match value {
            std::cmp::Ordering::Less => Ordering::Less,
            std::cmp::Ordering::Equal => Ordering::Equal,
            std::cmp::Ordering::Greater => Ordering::Greater,
        }
    }
}

#[napi]
pub struct Int64 {
    v: i64,
}

#[napi]
impl Int64 {
    /// @param value The initial value. Default: `0`.
    #[napi(constructor)]
    pub fn new(
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Option<
            Either4<&Int64, i64, String, Buffer>,
        >,
    ) -> Result<Self> {
        match value {
            Some(value) => {
                let v = to_i64(&env, value)?;

                Ok(Int64 {
                    v,
                })
            },
            None => Ok(Int64 {
                v: 0
            }),
        }
    }

    /// To a decimal number in a string.
    #[napi(js_name = "toDecimal")]
    pub fn to_decimal(&self) -> String {
        format!("{}", self.v)
    }

    /// To a binary number in a string.
    #[napi(js_name = "toBinary")]
    pub fn to_binary(&self, format: Option<bool>) -> String {
        if format.unwrap_or(false) {
            format!("{:#066b}", self.v)
        } else {
            format!("{:b}", self.v)
        }
    }

    /// To a octal number in a string.
    #[napi(js_name = "toOctal")]
    pub fn to_octal(&self, format: Option<bool>) -> String {
        if format.unwrap_or(false) {
            format!("{:#024o}", self.v)
        } else {
            format!("{:o}", self.v)
        }
    }

    /// To a hex number in a string.
    #[napi(js_name = "toHex")]
    pub fn to_hex(&self, format: Option<bool>, uppercase: Option<bool>) -> String {
        if uppercase.unwrap_or(false) {
            if format.unwrap_or(false) {
                format!("{:#018X}", self.v)
            } else {
                format!("{:X}", self.v)
            }
        } else if format.unwrap_or(false) {
            format!("{:#018x}", self.v)
        } else {
            format!("{:x}", self.v)
        }
    }

    /// To a 64-bit buffer in Little-Endian byte order.
    #[napi(js_name = "toBuffer")]
    pub fn to_buffer(&self) -> Buffer {
        Buffer::from(self.v.to_le_bytes().to_vec())
    }

    /// To a number. If this 64-bit integer number is bigger than `2^53 - 1`, or smaller than `-(2^53 - 1)`, then throws a RangeError.
    #[napi(js_name = "toNumber")]
    pub fn to_number(&self, env: Env) -> Result<i64> {
        if self.v > MAX_SAFE_INTEGER {
            env.throw_range_error(
                format!("{} is bigger than {MAX_SAFE_INTEGER}", self.v).as_str(),
                None,
            )?;

            Err(Error::from_reason(""))
        } else if self.v < MIN_SAFE_INTEGER {
            env.throw_range_error(
                format!("{} is smaller than {MIN_SAFE_INTEGER}", self.v).as_str(),
                None,
            )?;

            Err(Error::from_reason(""))
        } else {
            Ok(self.v)
        }
    }

    /// Sets the value of this instance.
    #[napi]
    pub fn set(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let v = to_i64(&env, value)?;

        self.v = v;

        Ok(this)
    }

    /// Computes `self += value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn add(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_add(b);

        Ok(this)
    }

    /// Computes `self -= value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn subtract(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_sub(b);

        Ok(this)
    }

    /// Computes `self *= value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn multiply(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_mul(b);

        Ok(this)
    }

    /// Computes `self /= value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn divide(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_div(b);

        Ok(this)
    }

    /// Computes `self %= value`, wrapping around at the boundary of an 64-bit integer.
    #[napi(js_name = "mod")]
    pub fn modulo(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_rem(b);

        Ok(this)
    }

    /// Computes `self %= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi]
    pub fn pow(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

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

        self.v = a.wrapping_pow(b as u32);

        Ok(this)
    }

    /// Computes `self <<= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "shiftLeft")]
    pub fn shift_left(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

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

        self.v = a.wrapping_shl(b as u32);

        Ok(this)
    }

    /// Computes `self >>= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "shiftRight")]
    pub fn shift_right(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

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

        self.v = a.wrapping_shr(b as u32);

        Ok(this)
    }

    /// Computes `self >>= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "shiftRightUnsigned")]
    pub fn shift_right_unsigned(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

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

        self.v = (a as u64).wrapping_shr(b as u32) as i64;

        Ok(this)
    }

    /// Shifts the bits to the left by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "rotateLeft")]
    pub fn rotate_left(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        if b < 0 {
            env.throw_range_error(
                "the bit count for rotation must not be smaller than zero",
                None,
            )?;

            return Err(Error::from_reason(""));
        } else if b > u32::MAX as i64 {
            env.throw_range_error(
                &format!("the bit count for rotation must not be bigger than {}", u32::MAX),
                None,
            )?;

            return Err(Error::from_reason(""));
        }

        self.v = a.rotate_left(b as u32);

        Ok(this)
    }

    /// Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "rotateRight")]
    pub fn rotate_right(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        if b < 0 {
            env.throw_range_error(
                "the bit count for rotation must not be smaller than zero",
                None,
            )?;

            return Err(Error::from_reason(""));
        } else if b > u32::MAX as i64 {
            env.throw_range_error(
                &format!("the bit count for rotation must not be bigger than {}", u32::MAX),
                None,
            )?;

            return Err(Error::from_reason(""));
        }

        self.v = a.rotate_right(b as u32);

        Ok(this)
    }

    /// Computes `self &= value`.
    #[napi]
    pub fn and(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        self.v &= to_i64(&env, value)?;

        Ok(this)
    }

    /// Computes `self |= value`.
    #[napi]
    pub fn or(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        self.v |= to_i64(&env, value)?;

        Ok(this)
    }

    /// Computes `self ^= value`.
    #[napi]
    pub fn xor(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        self.v ^= to_i64(&env, value)?;

        Ok(this)
    }

    /// Computes `self = ~(self & value)`.
    #[napi]
    pub fn nand(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        self.v = !(self.v & to_i64(&env, value)?);

        Ok(this)
    }

    /// Computes `self = ~(self | value)`.
    #[napi]
    pub fn nor(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        self.v = !(self.v | to_i64(&env, value)?);

        Ok(this)
    }

    /// Computes `self = ~(self ^ value)`.
    #[napi]
    pub fn xnor(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        self.v = !(self.v ^ to_i64(&env, value)?);

        Ok(this)
    }

    /// Computes `self = ~self`.
    #[napi]
    pub fn not(&mut self, this: This) -> Result<This> {
        self.v = !self.v;

        Ok(this)
    }

    /// Computes `self = -self`.
    #[napi]
    pub fn negative(&mut self, this: This) -> Result<This> {
        self.v = -self.v;

        Ok(this)
    }

    /// Computes `self === value`.
    #[napi]
    pub fn eq(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<bool> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a == b)
    }

    /// Computes `self !== value`.
    #[napi]
    pub fn ne(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<bool> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a != b)
    }

    /// Computes `self > value`.
    #[napi]
    pub fn gt(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<bool> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a > b)
    }

    /// Computes `self >= value`.
    #[napi]
    pub fn gte(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<bool> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a >= b)
    }

    /// Computes `self < value`.
    #[napi]
    pub fn lt(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<bool> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a < b)
    }

    /// Computes `self <= value`.
    #[napi]
    pub fn lte(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<bool> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a <= b)
    }

    /// If `self < value`, returns `-1`.
    /// If `self === value`, returns `0`.
    /// If `self > value`, returns `1`.
    #[napi]
    pub fn comp(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<Ordering> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a.cmp(&b).into())
    }

    /// Set a random 64-bit integer between `self` and `value`.
    #[napi]
    pub fn random(
        &mut self,
        this: This,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Either4<
            &Int64,
            i64,
            String,
            Buffer,
        >,
    ) -> Result<This> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = random_number::random!(a, b);

        Ok(this)
    }

    #[allow(clippy::should_implement_trait)]
    /// Clones this `Int64` object.
    #[napi]
    pub fn clone(&self) -> Int64 {
        Int64 {
            v: self.v
        }
    }
}
