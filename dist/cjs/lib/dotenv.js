"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const path_1 = __importDefault(require("path"));
const dotenv_1 = require("dotenv");
const dotenv_expand_1 = require("dotenv-expand");
const roots_1 = require("../roots");
const DOTENV_FILENAME = process.env.DOTENV_FILENAME === undefined
    ? '.env'
    : process.env.DOTENV_FILENAME;
(0, dotenv_expand_1.expand)((0, dotenv_1.config)({
    path: path_1.default.join(roots_1.packageRoot, DOTENV_FILENAME),
}));
(0, dotenv_expand_1.expand)((0, dotenv_1.config)({
    path: path_1.default.join(roots_1.repositoryRoot, DOTENV_FILENAME),
}));
//# sourceMappingURL=dotenv.js.map