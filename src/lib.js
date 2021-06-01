const util = require("util");

const int64 = require("../index.node");

int64.Int64.prototype[util.inspect.custom] = function () {
    return this.toDecimal();
};

int64.Int64.prototype.clone = function () {
    return new int64.Int64(this);
};

int64.Int64.from = function () {
    return new int64.Int64(...arguments);
};

module.exports = int64;
