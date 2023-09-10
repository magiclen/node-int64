import { inspect } from "node:util";

import { Int64 } from "../index.cjs";

export * from "../index.cjs";

Object.assign(Int64.prototype, {
    [inspect.custom](): string {
        return (this as unknown as Int64).toDecimal();
    },
});
