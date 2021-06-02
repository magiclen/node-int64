"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.random = exports.comp = exports.lte = exports.lt = exports.gte = exports.gt = exports.ne = exports.eq = exports.negative = exports.not = exports.xnor = exports.nor = exports.nand = exports.xor = exports.or = exports.and = exports.rotateRight = exports.rotateLeft = exports.shiftRightUnsigned = exports.shiftRight = exports.shiftLeft = exports.pow = exports.mod = exports.divide = exports.multiply = exports.subtract = exports.add = exports.Int64 = void 0;
const util_1 = __importDefault(require("util"));
const int64 = require("../index.node");
class Int64 {
    constructor(a) {
        this.boxed = int64.initMethod(a);
    }
    /**
     * Create a new <code>Int64</code> instance.
     */
    static from(a) {
        return new Int64(a);
    }
    /**
     * To a decimal number in a string.
     */
    toDecimal() {
        return int64.toDecimalMethod(this.boxed);
    }
    /**
    * To a binary number in a string.
    */
    toBinary(format) {
        return int64.toBinaryMethod(this.boxed, format);
    }
    /**
    * To an octal number in a string.
    */
    toOctal(format) {
        return int64.toOctalMethod(this.boxed, format);
    }
    /**
     * To a hex number in a string.
     */
    toHex(format, uppercase) {
        return int64.toHexMethod(this.boxed, format, uppercase);
    }
    /**
     * To a 64-bit buffer in Little-Endian byte order.
     */
    toBuffer() {
        return int64.toBufferMethod(this.boxed);
    }
    /**
    * To a number. If this 64-bit integer number is bigger than <code>2^53 - 1</code>, or smaller than <code>-(2^53 - 1)</code>, then throws an error.
    */
    toNumber() {
        return int64.toNumberMethod(this.boxed);
    }
    /**
     * Sets the value of this instance.
     */
    set(a) {
        return int64.setMethod(this.boxed, a);
    }
    /**
    * Computes <code>self + a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    add(a) {
        return int64.addMethod(this.boxed, a);
    }
    /**
     * Computes <code>self - a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    subtract(a) {
        return int64.subtractMethod(this.boxed, a);
    }
    /**
    * Computes <code>self * a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    multiply(a) {
        return int64.multiplyMethod(this.boxed, a);
    }
    /**
     * Computes <code>self / a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    divide(a) {
        return int64.divideMethod(this.boxed, a);
    }
    /**
    * Computes <code>self % a</code>.
    */
    mod(a) {
        return int64.modMethod(this.boxed, a);
    }
    /**
     * Computes <code>self ^ a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    pow(a) {
        return int64.powMethod(this.boxed, a);
    }
    /**
    * Computes <code>self << a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    shiftLeft(a) {
        return int64.shiftLeftMethod(this.boxed, a);
    }
    /**
    * Computes <code>self >> a</code>, wrapping around at the boundary of an 64-bit integer.
    */
    shiftRight(a) {
        return int64.shiftRightMethod(this.boxed, a);
    }
    /**
     * Computes <code>self >>> a</code>, wrapping around at the boundary of an 64-bit integer.
     */
    shiftRightUnsigned(a) {
        return int64.shiftRightUnsignedMethod(this.boxed, a);
    }
    /**
    * Computes <code>self & a</code>.
    */
    and(a) {
        return int64.andMethod(this.boxed, a);
    }
    /**
   * Computes <code>self | a</code>.
   */
    or(a) {
        return int64.orMethod(this.boxed, a);
    }
    /**
     * Computes <code>self ^ a</code>.
     */
    xor(a) {
        return int64.xorMethod(this.boxed, a);
    }
    /**
    * Computes <code>~(self & a)</code>.
    */
    nand(a) {
        return int64.nandMethod(this.boxed, a);
    }
    /**
   * Computes <code>~(self | a)</code>.
   */
    nor(a) {
        return int64.norMethod(this.boxed, a);
    }
    /**
    * Computes <code>~(self ^ a)</code>.
    */
    xnor(a) {
        return int64.xnorMethod(this.boxed, a);
    }
    /**
   * Computes <code>~self</code>.
   */
    not() {
        return int64.notMethod(this.boxed);
    }
    /**
    * Computes <code>-self</code>.
    */
    negative() {
        return int64.negativeMethod(this.boxed);
    }
    /**
   * Computes <code>self === a</code>.
   */
    eq(a) {
        return int64.eqMethod(this.boxed, a);
    }
    /**
     * Computes <code>self !== a</code>.
     */
    ne(a) {
        return int64.neMethod(this.boxed, a);
    }
    /**
    * Computes <code>self > a</code>.
    */
    gt(a) {
        return int64.gtMethod(this.boxed, a);
    }
    /**
   * Computes <code>self >= a</code>.
   */
    gte(a) {
        return int64.gteMethod(this.boxed, a);
    }
    /**
     * Computes <code>self < a</code>.
     */
    lt(a) {
        return int64.ltMethod(this.boxed, a);
    }
    /**
     * Computes <code>self <= a</code>.
     */
    lte(a) {
        return int64.lteMethod(this.boxed, a);
    }
    /**
     * If <code>self > a</code>, returns 1.
     *
     * If <code>self < a</code>, returns -1.
     *
     * If <code>self === a</code>, returns 0.
     */
    comp(a) {
        return int64.compMethod(this.boxed, a);
    }
    /**
     * Gets a random number between <code>self</code> and <code>a</code>.
     */
    random(a) {
        return int64.randomMethod(this.boxed, a);
    }
    /**
     * Clones this <code>Int64</code> object.
     */
    clone() {
        return new Int64(this.boxed);
    }
    [util_1.default.inspect.custom]() {
        return this.toDecimal();
    }
}
exports.Int64 = Int64;
/**
 * Computes <code>a + b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const add = (a, b) => int64.add(a, b);
exports.add = add;
/**
 * Computes <code>a - b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const subtract = (a, b) => int64.subtract(a, b);
exports.subtract = subtract;
/**
 * Computes <code>a * b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const multiply = (a, b) => int64.multiply(a, b);
exports.multiply = multiply;
/**
 * Computes <code>a / b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const divide = (a, b) => int64.divide(a, b);
exports.divide = divide;
/**
 * Computes <code>a % b</code>.
 */
const mod = (a, b) => int64.mod(a, b);
exports.mod = mod;
/**
 * Computes <code>a ^ b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const pow = (a, b) => int64.pow(a, b);
exports.pow = pow;
/**
 * Computes <code>a << b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const shiftLeft = (a, b) => int64.shiftLeft(a, b);
exports.shiftLeft = shiftLeft;
/**
 * Computes <code>a >> b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const shiftRight = (a, b) => int64.shiftRight(a, b);
exports.shiftRight = shiftRight;
/**
 * Computes <code>a >>> b</code>, wrapping around at the boundary of an 64-bit integer.
 */
const shiftRightUnsigned = (a, b) => int64.shiftRightUnsigned(a, b);
exports.shiftRightUnsigned = shiftRightUnsigned;
/**
 * Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
 */
const rotateLeft = (a, b) => int64.rotateLeft(a, b);
exports.rotateLeft = rotateLeft;
/**
 * Shifts the bits to the right by a specified amount n, wrapping the truncated bits to the beginning of the resulting 64-bit integer.
 */
const rotateRight = (a, b) => int64.rotateRight(a, b);
exports.rotateRight = rotateRight;
/**
 * Computes <code>a & b</code>.
 */
const and = (a, b) => int64.and(a, b);
exports.and = and;
/**
 * Computes <code>a | b</code>.
 */
const or = (a, b) => int64.or(a, b);
exports.or = or;
/**
 * Computes <code>a ^ b</code>.
 */
const xor = (a, b) => int64.xor(a, b);
exports.xor = xor;
/**
 * Computes <code>~(a & b)</code>.
 */
const nand = (a, b) => int64.nand(a, b);
exports.nand = nand;
/**
 * Computes <code>~(a | b)</code>.
 */
const nor = (a, b) => int64.nor(a, b);
exports.nor = nor;
/**
 * Computes <code>~(a ^ b)</code>.
 */
const xnor = (a, b) => int64.xnor(a, b);
exports.xnor = xnor;
/**
 * Computes <code>~a</code>.
 */
const not = (a) => int64.not(a);
exports.not = not;
/**
 * Computes <code>-a</code>.
 */
const negative = (a) => int64.negative(a);
exports.negative = negative;
/**
 * Computes <code>a === b</code>.
 */
const eq = (a, b) => int64.eq(a, b);
exports.eq = eq;
/**
 * Computes <code>a !== b</code>.
 */
const ne = (a, b) => int64.ne(a, b);
exports.ne = ne;
/**
 * Computes <code>a > b</code>.
 */
const gt = (a, b) => int64.gt(a, b);
exports.gt = gt;
/**
 * Computes <code>a >= b</code>.
 */
const gte = (a, b) => int64.gte(a, b);
exports.gte = gte;
/**
 * Computes <code>a < b</code>.
 */
const lt = (a, b) => int64.lt(a, b);
exports.lt = lt;
/**
 * Computes <code>a <= b</code>.
 */
const lte = (a, b) => int64.lte(a, b);
exports.lte = lte;
/**
 * If the first one is bigger than the second one, returns 1.
 *
 * If the first one is smaller than the second one, returns -1.
 *
 * If the first one is equal to the second one, returns 0.
 */
const comp = (a, b) => int64.comp(a, b);
exports.comp = comp;
/**
* Gets a random number between a and b.
*/
const random = (a, b) => int64.random(a, b);
exports.random = random;
exports.default = Int64;
