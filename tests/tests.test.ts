import { describe, it } from "mocha";
import { expect } from "chai";

import * as int64 from "..";

const Int64 = int64.Int64;

describe("Add", function () {
    it("should add", function () {
        let n = 0;
        for (let i = 0;i < 10000;++i) {
            n = int64.add(n, 98766455);
        }
        expect(n.toString()).to.equal("987664550000");
        const m = Int64.from(0);
        for (let i = 0;i < 10;++i) {
            m.add(n);
        }
        expect(m.toDecimal()).to.equal("9876645500000");
    });
});

describe("Subtract", function () {
    it("should subtract", function () {
        let n = 0;
        for (let i = 0;i < 10000;++i) {
            n = int64.subtract(n, 98766455);
        }
        expect(n.toString()).to.equal("-987664550000");
        const m = Int64.from(0);
        for (let i = 0;i < 10;++i) {
            m.subtract(n);
        }
        expect(m.toDecimal()).to.equal("9876645500000");
    });
});

describe("Multiply", function () {
    it("should multiply", function () {
        const n = Int64.from(1);
        for (let i = 1;i < 20;++i) {
            n.multiply(i);
        }
        expect(n.toDecimal()).to.equal("121645100408832000");
        n.multiply(2);
        expect(n.toDecimal()).to.equal("243290200817664000");
    });
});

describe("Divide", function () {
    it("should divide", function () {
        const n = Int64.from("243290200817664000");
        n.divide(20);
        expect(n.toDecimal()).to.equal("12164510040883200");
        n.divide(2);
        expect(n.toDecimal()).to.equal("6082255020441600");
    });
});

describe("Mod", function () {
    it("should mod", function () {
        const n = Int64.from("894453210654871");
        n.mod(8);
        expect(n.toNumber()).to.equal(7);
        n.mod(2);
        expect(n.toNumber()).to.equal(1);
    });
});

describe("Negative", function () {
    it("should negative", function () {
        const n = Int64.from("894453210654871");
        n.negative();
        expect(n.toDecimal()).to.equal("-894453210654871");
        n.negative();
        expect(n.toDecimal()).to.equal("894453210654871");
    });
});

describe("Gt", function () {
    it("should greater than", function () {
        const a = Int64.from("894453210654871");
        const b = Int64.from("894453210654870");
        let n = int64.gt(a, b);
        expect(n).to.equal(true);
        n = a.gt(b);
        expect(n).to.equal(true);
    });
});
