import util from "util";

const int64 = require("../index.node");

type I64 = number | string | Buffer | Int64;

export class Int64 {
    private boxed: any;

    /**
     * Create a new <code>Int64</code> instance.
     */
    static from(a: I64) {
        return new Int64(a);
    }

    constructor(a: I64) {
        this.boxed = int64.initMethod(a);
    }

    /**
     * To a decimal number in a string.
     */
    toDecimal(): string {
        return int64.toDecimalMethod(this.boxed);
    }

    /**
    * To a binary number in a string.
    */
    toBinary(format?: boolean): string {
        return int64.toBinaryMethod(this.boxed, format);
    }

    /**
    * To an octal number in a string.
    */
    toOctal(format?: boolean): string {
        return int64.toOctalMethod(this.boxed, format);
    }

    /**
     * To a hex number in a string.
     */
    toHex(format?: boolean, uppercase?: boolean): string {
        return int64.toHexMethod(this.boxed, format, uppercase);
    }

    /**
     * To a 64-bit buffer in Little-Endian byte order.
     */
    toBuffer(): Buffer {
        return int64.toBufferMethod(this.boxed);
    }

    /**
    * To a number. If this 64-bit integer number is bigger than <code>2^53 - 1</code>, or smaller than <code>-(2^53 - 1)</code>, then throws an error.
    */
    toNumber(): number {
        return int64.toNumberMethod(this.boxed);
    }

    /**
     * Sets the value of this instance.
     */
    set(a: I64): Int64 {
        return int64.setMethod(this.boxed, a);
    }

    /**
    * Computes <code>self + a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    add(a: I64): Int64 {
        return int64.addMethod(this.boxed, a);
    }

    /**
     * Computes <code>self - a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    subtract(a: I64): Int64 {
        return int64.subtractMethod(this.boxed, a);
    }

    /**
    * Computes <code>self * a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    multiply(a: I64): Int64 {
        return int64.multiplyMethod(this.boxed, a);
    }

    /**
     * Computes <code>self / a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    divide(a: I64): Int64 {
        return int64.divideMethod(this.boxed, a);
    }

    /**
    * Computes <code>self % a</code>.
    */
    mod(a: I64): Int64 {
        return int64.modMethod(this.boxed, a);
    }

    /**
     * Computes <code>self ^ a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    pow(a: I64): Int64 {
        return int64.powMethod(this.boxed, a);
    }

    /**
    * Computes <code>self << a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    shiftLeft(a: I64): Int64 {
        return int64.shiftLeftMethod(this.boxed, a);
    }

    /**
    * Computes <code>self >> a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    shiftRight(a: I64): Int64 {
        return int64.shiftRightMethod(this.boxed, a);
    }

    /**
     * Computes <code>self >>> a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    shiftRightUnsigned(a: I64): Int64 {
        return int64.shiftRightUnsignedMethod(this.boxed, a);
    }

    /**
    * Computes <code>self & a</code>.
    */
    and(a: I64): Int64 {
        return int64.andMethod(this.boxed, a);
    }

    /**
   * Computes <code>self | a</code>.
   */
    or(a: I64): Int64 {
        return int64.orMethod(this.boxed, a);
    }

    /**
     * Computes <code>self ^ a</code>.
     */
    xor(a: I64): Int64 {
        return int64.xorMethod(this.boxed, a);
    }

    /**
    * Computes <code>~(self & a)</code>.
    */
    nand(a: I64): Int64 {
        return int64.nandMethod(this.boxed, a);
    }

    /**
   * Computes <code>~(self | a)</code>.
   */
    nor(a: I64): Int64 {
        return int64.norMethod(this.boxed, a);
    }

    /**
    * Computes <code>~(self ^ a)</code>.
    */
    xnor(a: I64): Int64 {
        return int64.xnorMethod(this.boxed, a);
    }

    /**
   * Computes <code>~self</code>.
   */
    not(): Int64 {
        return int64.notMethod(this.boxed);
    }

    /**
    * Computes <code>-self</code>.
    */
    negative(): Int64 {
        return int64.negativeMethod(this.boxed);
    }

    /**
   * Computes <code>self === a</code>.
   */
    eq(a: I64): boolean {
        return int64.eqMethod(this.boxed, a);
    }

    /**
     * Computes <code>self !== a</code>.
     */
    ne(a: I64): boolean {
        return int64.neMethod(this.boxed, a);
    }

    /**
    * Computes <code>self > a</code>.
    */
    gt(a: I64): boolean {
        return int64.gtMethod(this.boxed, a);
    }

    /**
   * Computes <code>self >= a</code>.
   */
    gte(a: I64): boolean {
        return int64.gteMethod(this.boxed, a);
    }

    /**
     * Computes <code>self < a</code>.
     */
    lt(a: I64): boolean {
        return int64.ltMethod(this.boxed, a);
    }

    /**
     * Computes <code>self <= a</code>.
     */
    lte(a: I64): boolean {
        return int64.lteMethod(this.boxed, a);
    }

    /**
     * If <code>self > a</code>, returns 1.
     *
     * If <code>self < a</code>, returns -1.
     *
     * If <code>self === a</code>, returns 0.
     */
    comp(a: I64): 0 | 1 | 2 {
        return int64.compMethod(this.boxed, a);
    }

    /**
     * Gets a random number between <code>self</code> and <code>a</code>.
     */
    random(a: I64): Int64 {
        return int64.randomMethod(this.boxed, a);
    }

    /**
     * Clones this <code>Int64</code> object.
     */
    clone(): Int64 {
        return new Int64(this.boxed);
    }

    [util.inspect.custom](): string {
        return this.toDecimal();
    }
}

/**
 * Computes <code>a + b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const add = (a: I64, b: I64): number => int64.add(a, b);

/**
 * Computes <code>a - b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const subtract = (a: I64, b: I64): number => int64.subtract(a, b);

/**
 * Computes <code>a * b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const multiply = (a: I64, b: I64): number => int64.multiply(a, b);

/**
 * Computes <code>a / b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const divide = (a: I64, b: I64): number => int64.divide(a, b);

/**
 * Computes <code>a % b</code>.
 */
export const mod = (a: I64, b: I64): number => int64.mod(a, b);

/**
 * Computes <code>a ^ b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const pow = (a: I64, b: I64): number => int64.pow(a, b);

/**
 * Computes <code>a << b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const shiftLeft = (a: I64, b: I64): number => int64.shiftLeft(a, b);

/**
 * Computes <code>a >> b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const shiftRight = (a: I64, b: I64): number => int64.shiftRight(a, b);

/**
 * Computes <code>a >>> b</code>, wrapping around at the boundary of an 64-bit integer.
 */
export const shiftRightUnsigned = (a: I64, b: I64): number => int64.shiftRightUnsigned(a, b);

/**
 * Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
 */
export const rotateLeft = (a: I64, b: I64): number => int64.rotateLeft(a, b);

/**
 * Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
 */
export const rotateRight = (a: I64, b: I64): number => int64.rotateRight(a, b);

/**
 * Computes <code>a & b</code>.
 */
export const and = (a: I64, b: I64): number => int64.and(a, b);

/**
 * Computes <code>a | b</code>.
 */
export const or = (a: I64, b: I64): number => int64.or(a, b);

/**
 * Computes <code>a ^ b</code>.
 */
export const xor = (a: I64, b: I64): number => int64.xor(a, b);

/**
 * Computes <code>~(a & b)</code>.
 */
export const nand = (a: I64, b: I64): number => int64.nand(a, b);

/**
 * Computes <code>~(a | b)</code>.
 */
export const nor = (a: I64, b: I64): number => int64.nor(a, b);

/**
 * Computes <code>~(a ^ b)</code>.
 */
export const xnor = (a: I64, b: I64): number => int64.xnor(a, b);

/**
 * Computes <code>~a</code>.
 */
export const not = (a: I64): number => int64.not(a);

/**
 * Computes <code>-a</code>.
 */
export const negative = (a: I64): number => int64.negative(a);

/**
 * Computes <code>a === b</code>.
 */
export const eq = (a: I64, b: I64): boolean => int64.eq(a, b);

/**
 * Computes <code>a !== b</code>.
 */
export const ne = (a: I64, b: I64): boolean => int64.ne(a, b);

/**
 * Computes <code>a > b</code>.
 */
export const gt = (a: I64, b: I64): boolean => int64.gt(a, b);

/**
 * Computes <code>a >= b</code>.
 */
export const gte = (a: I64, b: I64): boolean => int64.gte(a, b);

/**
 * Computes <code>a < b</code>.
 */
export const lt = (a: I64, b: I64): boolean => int64.lt(a, b);

/**
 * Computes <code>a <= b</code>.
 */
export const lte = (a: I64, b: I64): boolean => int64.lte(a, b);

/**
 * If the first one is bigger than the second one, returns 1.
 *
 * If the first one is smaller than the second one, returns -1.
 *
 * If the first one is equal to the second one, returns 0.
 */
export const comp = (a: I64, b: I64): 0 | 1 | 2 => int64.comp(a, b);

/**
* Gets a random number between a and b.
*/
export const random = (a: I64, b: I64): number => int64.random(a, b);

export default Int64;
