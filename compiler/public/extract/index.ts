import ts from "typescript";
import type { CompileOptions, ExtractedFunction } from "../api/types.js";
import { findUserCallsite, resolveTypeScriptSource } from "./callsite.js";
import { findCompileCall, normalizeCompileArgument, normalizeFunctionLike } from "./normalize.js";
import { loadInlineTypeScriptProgram, loadTypeScriptProgram } from "./program.js";

export function extractFunctionWithTypes(
  _fn: Function,
  options: CompileOptions = {},
): ExtractedFunction {
  const callsite = options.callsite ?? findUserCallsite();
  const sourcePath = resolveTypeScriptSource(callsite.file);
  const { sourceFile, checker } = loadTypeScriptProgram(sourcePath);
  const call = findCompileCall(sourceFile, callsite.line);

  if (!call) {
    throw new Error(`Cannot find compile(...) call in ${sourcePath}:${callsite.line}`);
  }

  const [argument] = call.arguments;
  if (!argument) {
    throw new Error(`compile(...) requires a function argument at ${sourcePath}:${callsite.line}`);
  }

  return normalizeCompileArgument(argument, sourceFile, checker);
}

export function normalizeSourceFunction(
  source: string,
  options: CompileOptions = {},
): ExtractedFunction {
  const { sourceFile, checker } = loadInlineTypeScriptProgram(
    source,
    options.fileName ?? "inline.ts",
  );
  const declaration = sourceFile.statements.find(ts.isFunctionDeclaration);

  if (!declaration) {
    throw new Error("compileSource(...) expects a TypeScript function declaration");
  }

  return normalizeFunctionLike(declaration, sourceFile, checker);
}