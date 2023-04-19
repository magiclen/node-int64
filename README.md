Int64
==========

[![CI](https://github.com/magiclen/node-int64/actions/workflows/ci.yml/badge.svg)](https://github.com/magiclen/node-int64/actions/workflows/ci.yml)

Use Rust to compute 64-bit signed integers.

You need to set up the Rust development environment: [rustup](https://rustup.rs/)

## Usage

First of all, an int64(long) value can be represented by

1. An Int64 object (instance).
1. A 53-bit integer number.
1. A string of a decimal number.
1. A string of a hexadecimal number, starting with `0x`.
1. A string of an octal number, starting with `0o`.
1. A string of a binary number, starting with `0b`.
1. A buffer with 8 bytes

### Static

Static funtions are used for quickly getting the final result as a JavaScript number. If you have several 64-bit integer computations to be done, it is better to use class methods instead.

A JavaScript number can safely represent only from `-(2^53 - 1)` to `2^53 - 1`, and this library does not allow you to convert an unsafe-ranged integer to a JavaScript number.

#### Random

```typescript
import { random } from "int64-napi";

const n = random(9876543210, "12345678901234"); // 5724595911391
```

#### Add

```typescript
import { add } from "int64-napi";

const n = add("0x0000000000000001", 0x00000002); // 1 + 2 = 3
```

#### Subtract

```typescript
import { subtract } from "int64-napi";

const n = subtract(1, 2); // 1 - 2 = -1
```

#### multiply

```typescript
import { multiply } from "int64-napi";

const n = multiply(2, 6); // 2 * 6 = 12
```

#### divide

```typescript
import { divide } from "int64-napi";

const n = divide(6, 4); // 6 / 4 = 1
```

#### mod

```typescript
import { mod } from "int64-napi";

const n = mod(6, 4); // 6 % 4 = 2
```

#### shiftLeft

```typescript
import { shiftLeft } from "int64-napi";

const n = shiftLeft(0b00101, 2); // 0b000101 << 2 = 0b010100
```

#### shiftRight

```typescript
import { shiftRight } from "int64-napi";

const n1 = shiftRight(0b0101, 2); // 0b0101 >> 2 = 0b0001
const n2 = shiftRight(0b0110, 1); // 0b0110 >> 1 = 0b0011
const n3 = shiftRight("0b1111111111111111111111111111111111111111111111111111111111111011", 1); // 0b1111111111111111111111111111111111111111111111111111111111111011 >> 1 = 0b1111111111111111111111111111111111111111111111111111111111111101
```

#### shiftRightUnsigned

```typescript
import { shiftRightUnsigned } from "int64-napi";

const n = shiftRightUnsigned("0b1111111111111111111111111111111111111111111111111111111111111011", 32); // 0b1111111111111111111111111111111111111111111111111111111111111011 >>> 32 = 0b0000000000000000000000000000000011111111111111111111111111111111
```

#### rotateRight

```typescript
import { rotateRight } from "int64-napi";

const n = rotateRight("0x0000000080000100", 20); // 0x0010000000000800
```

#### rotateLeft

```typescript
import { rotateLeft } from "int64-napi";

const n = rotateLeft("0x0010000000000800", 20); // 0x0000000080000100
```

#### and

```typescript
import { and } from "int64-napi";

const n = and("0x000000000000FFFF", "0x0123456789ABCDEF"); // 0x000000000000CDEF
```

#### or

```typescript
import { or } from "int64-napi";

const n = or("0x0000FFFF0000FFFF", "0xFFFFFFFFFFFF0000"); // 0xFFFFFFFFFFFFFFFF
```

#### xor

```typescript
import { xor } from "int64-napi";

const n = xor("0x0000FFFF0000FFFF", "0xFFFFFFFFFFFF0000"); // 0xFFFF0000FFFFFFFF
```

#### nand

```typescript
import { nand } from "int64-napi";

const n = nand("0x000000000000FFFF", "0x0123456789ABCDEF"); // 0xFFFFFFFFFFFF3210
```

#### nor

```typescript
import { nor } from "int64-napi";

const n = nor("0x0000FFFF0000FFFF", "0xFFFFFFFFFFFF0000"); // 0x0000000000000000
```

#### xnor

```typescript
import { xnor } from "int64-napi";

const n = xnor("0x0000FFFF0000FFFF", "0xFFFFFFFFFFFF0000"); // 0x0000FFFF00000000
```

#### not

```typescript
import { nor } from "int64-napi";

const n = nor("0x0000FFFF0000FFFF", "0xFFFFFFFFFFFF0000"); // 0x0000000000000000
```

#### eq (Equal)

```typescript
import { eq } from "int64-napi";

const n = eq("0x0000FFFF0000FFFF", "281470681808895"); // true
```

#### ne (Not Equal)

```typescript
import { ne } from "int64-napi";

const n = ne("0x0000FFFF0000FFFF", "0x0000FFFF00000000"); // true
```

#### gt (Greater Than)

```typescript
import { gt } from "int64-napi";

const n = gt("0x0000FFFF0000FFFF", "0x0000FFFF00000000"); // true
```

#### gte (Greater Than or Equal)

```typescript
import { gte } from "int64-napi";

const n = gte("0x0000FFFF0000FFFF", "0x0000FFFF00000000"); // true
```

#### lt (Less Than)

```typescript
import { lt } from "int64-napi";

const n = lt("0x0000FFFF0000FFFF", "0x0000FFFF0000FFFF"); // false
```

#### lte (Less Than or Equal)

```typescript
import { lte } from "int64-napi";

const n = lte("0x0000FFFF0000FFFF", "0x0000FFFF0000FFFF"); // true
```

#### comp (Compare)

If `a < b`, returns `-1`.

If `a === b`, returns `0`.

If `a > b`, returns `1`.

```typescript
import { comp } from "int64-napi";

const a = comp("0x0000FFFF0000FFFF", "0x0000FFFF0000FFFF"); // 0
const b = comp("0x0000FFFF0000FFFF", "0x0000FFFF00000000"); // 1
const c = comp("0x0000FFFF00000000", "0x0000FFFF0000FFFF"); // -1
```

### Instance / Object

#### Create an Instance

```typescript
import { Int64 } from "int64-napi";

const i64 = new Int64(1);
```

#### Methods

`Int64` instance has methods which are corresponding to static functions. `Int64` instances are mutable and reusable, which means operations may modify its value.

```typescript
import { Int64 } from "int64-napi";

const i64 = new Int64(1);
const n1 = i64.add(1).multiply(3).subtract(3).divide(3).toDecimal(); // "1"

i64.set("0xFFFF000000000000");
const n2 = i64.shiftLeft(8).shiftRight(56).toHex(true); // "0xffffffffffffffff"

i64.set("0xFFFF000000000000");
const n3 = i64.shiftLeft(8).shiftRightUnsigned(56).toHex(true); // "0x00000000000000ff"

i64.set("0x000000010001");
const n4 = i64.rotateRight(8).toHex(true); // "0x0100000000000100"

i64.set("0x0000FFFFFFFF0000");
const n51 = i64.toHex(true); // "0x0000ffffffff0000"
const n52 = i64.toHex(); // "ffffffff0000"
const n53 = i64.toDecimal() + 1; // "2814749766451201"
const n54 = i64.toNumber() + 1; // 281474976645121
```

To clone an `Int64` instance.

```typescript
const i64_2 = i64.clone();
```

## License

[MIT](LICENSE)
