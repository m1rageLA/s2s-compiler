import tsParser from "@typescript-eslint/parser";
import ts2rust from "./eslint-plugin-ts2rust/index.mjs";

const HEAVY_FN_NAME = "heavy"; // <- меняешь тут

export default [
  {
    files: ["**/*.ts", "**/*.tsx"],
    languageOptions: {
      parser: tsParser,
      ecmaVersion: 2022,
      sourceType: "module",
    },
    plugins: { ts2rust },
    rules: {
      "ts2rust/no-dynamic-in-heavy": ["error", { heavyName: HEAVY_FN_NAME }],
    },
  },
];
