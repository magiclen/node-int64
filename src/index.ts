import { inspect } from "node:util";

import { Int64 } from "../native/index.cjs";

export {
    add,
    and,
    comp,
    divide,
    eq,
    gt,
    gte,
    Int64,
    lt,
    lte,
    mod,
    multiply,
    nand,
    ne,
    negative,
    nor,
    not,
    or,
    Ordering,
    pow,
    random,
    rotateLeft,
    rotateRight,
    shiftLeft,
    shiftRight,
    shiftRightUnsigned,
    subtract,
    xnor,
    xor,
} from "../native/index.cjs";

// Node.js prints the decimal value instead of the opaque native object.
Object.assign(Int64.prototype, {
    [inspect.custom](this: Int64): string {
        return this.toDecimal();
    },
});
