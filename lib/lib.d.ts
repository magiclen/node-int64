/// <reference types="node" />
import util from 'util';
declare type I64 = number | string | Buffer | Int64;
export declare class Int64 {
    private boxed;
    /**
     * Create a new <code>Int64</code> instance.
     */
    static from(a: I64): Int64;
    constructor(a: I64);
    /**
     * To a decimal number in a string.
     */
    toDecimal(): string;
    /**
    * To a binary number in a string.
    */
    toBinary(format?: boolean): string;
    /**
    * To an octal number in a string.
    */
    toOctal(format?: boolean): string;
    /**
     * To a hex number in a string.
     */
    toHex(format?: boolean, uppercase?: boolean): string;
    /**
     * To a 64-bit buffer in Little-Endian byte order.
     */
    toBuffer(): Buffer;
    /**
    * To a number. If this 64-bit integer number is bigger than <code>2^53 - 1</code>, or smaller than <code>-(2^53 - 1)</code>, then throws an error.
    */
    toNumber(): number;
    /**
     * Sets the value of this instance.
     */
    set(a: I64): Int64;
    /**
    * Computes <code>self + a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    add(a: I64): Int64;
    /**
     * Computes <code>self - a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    subtract(a: I64): Int64;
    /**
    * Computes <code>self * a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    multiply(a: I64): Int64;
    /**
     * Computes <code>self / a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    divide(a: I64): Int64;
    /**
    * Computes <code>self % a</code>.
    */
    mod(a: I64): Int64;
    /**
     * Computes <code>self ^ a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    pow(a: I64): Int64;
    /**
    * Computes <code>self << a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    shiftLeft(a: I64): Int64;
    /**
    * Computes <code>self >> a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    shiftRight(a: I64): Int64;
    /**
     * Computes <code>self >>> a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    shiftRightUnsigned(a: I64): Int64;
    /**
    * Computes <code>self & a</code>.
    */
    and(a: I64): Int64;
    /**
   * Computes <code>self | a</code>.
   */
    or(a: I64): Int64;
    /**
     * Computes <code>self ^ a</code>.
     */
    xor(a: I64): Int64;
    /**
    * Computes <code>~(self & a)</code>.
    */
    nand(a: I64): Int64;
    /**
   * Computes <code>~(self | a)</code>.
   */
    nor(a: I64): Int64;
    /**
    * Computes <code>~(self ^ a)</code>.
    */
    xnor(a: I64): Int64;
    /**
   * Computes <code>~self</code>.
   */
    not(): Int64;
    /**
    * Computes <code>-self</code>.
    */
    negative(): Int64;
    /**
   * Computes <code>self === a</code>.
   */
    eq(a: I64): boolean;
    /**
     * Computes <code>self !== a</code>.
     */
    ne(a: I64): boolean;
    /**
    * Computes <code>self > a</code>.
    */
    gt(a: I64): boolean;
    /**
   * Computes <code>self >= a</code>.
   */
    gte(a: I64): boolean;
    /**
     * Computes <code>self < a</code>.
     */
    lt(a: I64): boolean;
    /**
     * Computes <code>self <= a</code>.
     */
    lte(a: I64): boolean;
    /**
     * If <code>self > a</code>, returns 1.
     *
     * If <code>self < a</code>, returns -1.
     *
     * If <code>self === a</code>, returns 0.
     */
    comp(a: I64): 0 | 1 | 2;
    /**
     * Gets a random number between <code>self</code> and <code>a</code>.
     */
    random(a: I64): Int64;
    /**
     * Clones this <code>Int64</code> object.
     */
    clone(): Int64;
    [util.inspect.custom](): string;
}
/**
 * Computes <code>a + b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const add: (a: I64, b: I64) => number;
/**
 * Computes <code>a - b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const subtract: (a: I64, b: I64) => number;
/**
 * Computes <code>a * b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const multiply: (a: I64, b: I64) => number;
/**
 * Computes <code>a / b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const divide: (a: I64, b: I64) => number;
/**
 * Computes <code>a % b</code>.
 */
export declare const mod: (a: I64, b: I64) => number;
/**
 * Computes <code>a ^ b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const pow: (a: I64, b: I64) => number;
/**
 * Computes <code>a << b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const shiftLeft: (a: I64, b: I64) => number;
/**
 * Computes <code>a >> b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const shiftRight: (a: I64, b: I64) => number;
/**
 * Computes <code>a >>> b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export declare const shiftRightUnsigned: (a: I64, b: I64) => number;
/**
 * Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
 */
export declare const rotateLeft: (a: I64, b: I64) => number;
/**
 * Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
 */
export declare const rotateRight: (a: I64, b: I64) => number;
/**
 * Computes <code>a & b</code>.
 */
export declare const and: (a: I64, b: I64) => number;
/**
 * Computes <code>a | b</code>.
 */
export declare const or: (a: I64, b: I64) => number;
/**
 * Computes <code>a ^ b</code>.
 */
export declare const xor: (a: I64, b: I64) => number;
/**
 * Computes <code>~(a & b)</code>.
 */
export declare const nand: (a: I64, b: I64) => number;
/**
 * Computes <code>~(a | b)</code>.
 */
export declare const nor: (a: I64, b: I64) => number;
/**
 * Computes <code>~(a ^ b)</code>.
 */
export declare const xnor: (a: I64, b: I64) => number;
/**
 * Computes <code>~a</code>.
 */
export declare const not: (a: I64) => number;
/**
 * Computes <code>-a</code>.
 */
export declare const negative: (a: I64) => number;
/**
 * Computes <code>a === b</code>.
 */
export declare const eq: (a: I64, b: I64) => boolean;
/**
 * Computes <code>a !== b</code>.
 */
export declare const ne: (a: I64, b: I64) => boolean;
/**
 * Computes <code>a > b</code>.
 */
export declare const gt: (a: I64, b: I64) => boolean;
/**
 * Computes <code>a >= b</code>.
 */
export declare const gte: (a: I64, b: I64) => boolean;
/**
 * Computes <code>a < b</code>.
 */
export declare const lt: (a: I64, b: I64) => boolean;
/**
 * Computes <code>a <= b</code>.
 */
export declare const lte: (a: I64, b: I64) => boolean;
/**
 * If the first one is bigger than the second one, returns 1.
 *
 * If the first one is smaller than the second one, returns -1.
 *
 * If the first one is equal to the second one, returns 0.
 */
export declare const comp: (a: I64, b: I64) => 0 | 1 | 2;
/**
* Gets a random number between a and b.
*/
export declare const random: (a: I64, b: I64) => number;
export {};
