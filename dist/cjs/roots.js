"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.repositoryRoot = exports.packageRoot = void 0;
const path_1 = __importDefault(require("path"));
exports.packageRoot = path_1.default.resolve(path_1.default.join(__dirname, '..'));
exports.repositoryRoot = path_1.default.resolve(path_1.default.join(exports.packageRoot, '../..'));
//# sourceMappingURL=roots.js.map