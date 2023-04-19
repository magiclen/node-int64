import { inspect } from "node:util";

import { Int64 } from "../index.cjs";

export * from "../index.cjs";

Object.assign(Int64.prototype, {
    [inspect.custom](): string {
        // eslint-disable-next-line no-extra-parens
        return (this as unknown as Int64).toDecimal();
    },
    toString(): string {
        // eslint-disable-next-line no-extra-parens
        return (this as unknown as Int64).toDecimal();
    },
});
