import assert from "node:assert/strict";
import { describe, it } from "node:test";

import { add, divide, gt, Int64, mod, negative, random, subtract } from "../src/index.ts";

describe("Add", () => {
    it("should add", () => {
        let n: number | Int64 = 0;

        for (let i = 0; i < 10000; ++i) {
            n = add(n, 98766455);
        }

        assert.equal("987664550000", n.toString());

        const m = new Int64();

        for (let i = 0; i < 10; ++i) {
            m.add(n);
        }

        assert.equal("9876645500000", m.toDecimal());
    });
});

describe("Subtract", () => {
    it("should subtract", () => {
        let n: number | Int64 = 0;

        for (let i = 0; i < 10000; ++i) {
            n = subtract(n, 98766455);
        }

        assert.equal("-987664550000", n.toString());

        const m = new Int64();

        for (let i = 0; i < 10; ++i) {
            m.subtract(n);
        }

        assert.equal("9876645500000", m.toDecimal());
    });
});

describe("Multiply", () => {
    it("should multiply", () => {
        const n = new Int64(1);

        for (let i = 1; i < 20; ++i) {
            n.multiply(i);
        }

        assert.equal("121645100408832000", n.toDecimal());

        n.multiply(2);

        assert.equal("243290200817664000", n.toDecimal());
    });
});

describe("Divide", () => {
    it("should divide", () => {
        const n = new Int64("243290200817664000");

        n.divide(20);

        assert.equal("12164510040883200", n.toDecimal());

        n.divide(2);

        assert.equal("6082255020441600", n.toDecimal());
    });

    it("should reject a zero divisor", () => {
        assert.throws(() => divide(1, 0), {
            name: "RangeError",
            message: "the divisor must not be zero",
        });

        assert.throws(() => new Int64(1).divide(0), {
            name: "RangeError",
            message: "the divisor must not be zero",
        });
    });
});

describe("Mod", () => {
    it("should mod", () => {
        const n = new Int64("894453210654871");

        n.mod(8);

        assert.equal(7, n.toNumber());

        n.mod(2);

        assert.equal(1, n.toNumber());
    });

    it("should reject a zero divisor", () => {
        assert.throws(() => mod(1, 0), {
            name: "RangeError",
            message: "the divisor must not be zero",
        });
    });
});

describe("Negative", () => {
    it("should negative", () => {
        const n = new Int64("894453210654871");

        n.negative();

        assert.equal("-894453210654871", n.toDecimal());

        n.negative();

        assert.equal("894453210654871", n.toDecimal());
    });

    it("should wrap the minimum integer", () => {
        assert.equal("-9223372036854775808", negative("-9223372036854775808").toDecimal());
    });
});

describe("Gt", () => {
    it("should greater than", () => {
        const a = new Int64("894453210654871");
        const b = new Int64("894453210654870");

        let n = gt(a, b);

        assert.equal(true, n);

        n = a.gt(b);

        assert.equal(true, n);
    });
});

describe("Random", () => {
    it("should accept the bounds in either order", () => {
        for (const n of [random(1, 10), random(10, 1)]) {
            assert.equal(true, n.gte(1) && n.lte(10));
        }
    });
});
