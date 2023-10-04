import {
    Int64,
    add,
    gt,
    subtract,
} from "../src/lib.js";

describe("Add", function () {
    it("should add", function () {
        let n: number | Int64 = 0;

        for (let i = 0;i < 10000;++i) {
            n = add(n, 98766455);
        }

        expect(n.toString()).toBe("987664550000");

        const m = new Int64();

        for (let i = 0;i < 10;++i) {
            m.add(n);
        }

        expect(m.toDecimal()).toBe("9876645500000");
    });
});

describe("Subtract", function () {
    it("should subtract", function () {
        let n: number | Int64 = 0;

        for (let i = 0;i < 10000;++i) {
            n = subtract(n, 98766455);
        }

        expect(n.toString()).toBe("-987664550000");

        const m = new Int64();

        for (let i = 0;i < 10;++i) {
            m.subtract(n);
        }

        expect(m.toDecimal()).toBe("9876645500000");
    });
});

describe("Multiply", function () {
    it("should multiply", function () {
        const n = new Int64(1);

        for (let i = 1;i < 20;++i) {
            n.multiply(i);
        }

        expect(n.toDecimal()).toBe("121645100408832000");

        n.multiply(2);

        expect(n.toDecimal()).toBe("243290200817664000");
    });
});

describe("Divide", function () {
    it("should divide", function () {
        const n = new Int64("243290200817664000");

        n.divide(20);

        expect(n.toDecimal()).toBe("12164510040883200");

        n.divide(2);

        expect(n.toDecimal()).toBe("6082255020441600");
    });
});

describe("Mod", function () {
    it("should mod", function () {
        const n = new Int64("894453210654871");

        n.mod(8);

        expect(n.toNumber()).toBe(7);

        n.mod(2);

        expect(n.toNumber()).toBe(1);
    });
});

describe("Negative", function () {
    it("should negative", function () {
        const n = new Int64("894453210654871");

        n.negative();

        expect(n.toDecimal()).toBe("-894453210654871");

        n.negative();

        expect(n.toDecimal()).toBe("894453210654871");
    });
});

describe("Gt", function () {
    it("should greater than", function () {
        const a = new Int64("894453210654871");
        const b = new Int64("894453210654870");

        let n = gt(a, b);

        expect(n).toBe(true);

        n = a.gt(b);

        expect(n).toBe(true);
    });
});
