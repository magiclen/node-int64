mod functions;
mod napi_static;

use functions::{
    EXPONENT, Int64Value, ROTATION_BIT_COUNT, SHIFT_BIT_COUNT, divide_i64, modulo_i64, random_i64,
    range_error, to_i64, to_u32,
};
use napi::{
    Env, Result,
    bindgen_prelude::{Buffer, This},
};
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Option<Int64Value<'_>>,
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

    #[allow(clippy::inherent_to_string)]
    /// To a decimal number in a string.
    #[napi(js_name = "toString")]
    pub fn to_string(&self) -> String {
        self.to_decimal()
    }

    /// To a binary number in a string.
    #[napi(js_name = "toBinary")]
    pub fn to_binary(&self, format: Option<bool>) -> String {
        if format.unwrap_or(false) { format!("{:#066b}", self.v) } else { format!("{:b}", self.v) }
    }

    /// To a octal number in a string.
    #[napi(js_name = "toOctal")]
    pub fn to_octal(&self, format: Option<bool>) -> String {
        if format.unwrap_or(false) { format!("{:#024o}", self.v) } else { format!("{:o}", self.v) }
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
            Err(range_error(&env, &format!("{} is bigger than {MAX_SAFE_INTEGER}", self.v)))
        } else if self.v < MIN_SAFE_INTEGER {
            Err(range_error(&env, &format!("{} is smaller than {MIN_SAFE_INTEGER}", self.v)))
        } else {
            Ok(self.v)
        }
    }

    /// Sets the value of this instance.
    #[napi]
    pub fn set<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let v = to_i64(&env, value)?;

        self.v = v;

        Ok(this)
    }

    /// Computes `self += value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn add<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_add(b);

        Ok(this)
    }

    /// Computes `self -= value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn subtract<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_sub(b);

        Ok(this)
    }

    /// Computes `self *= value`, wrapping around at the boundary of an 64-bit integer.
    #[napi]
    pub fn multiply<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = a.wrapping_mul(b);

        Ok(this)
    }

    /// Computes `self /= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `value` must not be zero
    #[napi]
    pub fn divide<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = divide_i64(&env, a, b)?;

        Ok(this)
    }

    /// Computes `self %= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `value` must not be zero
    #[napi(js_name = "mod")]
    pub fn modulo<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = modulo_i64(&env, a, b)?;

        Ok(this)
    }

    /// Computes `self %= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi]
    pub fn pow<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;
        let b = to_u32(&env, b, EXPONENT)?;

        self.v = a.wrapping_pow(b);

        Ok(this)
    }

    /// Computes `self <<= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "shiftLeft")]
    pub fn shift_left<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;
        let b = to_u32(&env, b, SHIFT_BIT_COUNT)?;

        self.v = a.wrapping_shl(b);

        Ok(this)
    }

    /// Computes `self >>= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "shiftRight")]
    pub fn shift_right<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;
        let b = to_u32(&env, b, SHIFT_BIT_COUNT)?;

        self.v = a.wrapping_shr(b);

        Ok(this)
    }

    /// Computes `self >>= value`, wrapping around at the boundary of an 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "shiftRightUnsigned")]
    pub fn shift_right_unsigned<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;
        let b = to_u32(&env, b, SHIFT_BIT_COUNT)?;

        self.v = (a as u64).wrapping_shr(b) as i64;

        Ok(this)
    }

    /// Shifts the bits to the left by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "rotateLeft")]
    pub fn rotate_left<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;
        let b = to_u32(&env, b, ROTATION_BIT_COUNT)?;

        self.v = a.rotate_left(b);

        Ok(this)
    }

    /// Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
    ///
    /// `b` must not be smaller than zero
    #[napi(js_name = "rotateRight")]
    pub fn rotate_right<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;
        let b = to_u32(&env, b, ROTATION_BIT_COUNT)?;

        self.v = a.rotate_right(b);

        Ok(this)
    }

    /// Computes `self &= value`.
    #[napi]
    pub fn and<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        self.v &= to_i64(&env, value)?;

        Ok(this)
    }

    /// Computes `self |= value`.
    #[napi]
    pub fn or<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        self.v |= to_i64(&env, value)?;

        Ok(this)
    }

    /// Computes `self ^= value`.
    #[napi]
    pub fn xor<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        self.v ^= to_i64(&env, value)?;

        Ok(this)
    }

    /// Computes `self = ~(self & value)`.
    #[napi]
    pub fn nand<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        self.v = !(self.v & to_i64(&env, value)?);

        Ok(this)
    }

    /// Computes `self = ~(self | value)`.
    #[napi]
    pub fn nor<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        self.v = !(self.v | to_i64(&env, value)?);

        Ok(this)
    }

    /// Computes `self = ~(self ^ value)`.
    #[napi]
    pub fn xnor<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        self.v = !(self.v ^ to_i64(&env, value)?);

        Ok(this)
    }

    /// Computes `self = ~self`.
    #[napi]
    pub fn not<'env>(&mut self, this: This<'env>) -> Result<This<'env>> {
        self.v = !self.v;

        Ok(this)
    }

    /// Computes `self = -self`.
    #[napi]
    pub fn negative<'env>(&mut self, this: This<'env>) -> Result<This<'env>> {
        self.v = self.v.wrapping_neg();

        Ok(this)
    }

    /// Computes `self === value`.
    #[napi]
    pub fn eq(
        &self,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
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
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<Ordering> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        Ok(a.cmp(&b).into())
    }

    /// Set a random 64-bit integer between `self` and `value`.
    #[napi]
    pub fn random<'env>(
        &mut self,
        this: This<'env>,
        env: Env,
        #[napi(ts_arg_type = "number | string | Buffer | Int64")] value: Int64Value<'_>,
    ) -> Result<This<'env>> {
        let a = self.v;
        let b = to_i64(&env, value)?;

        self.v = random_i64(a, b);

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
