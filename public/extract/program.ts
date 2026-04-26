import path from "node:path";
import ts from "typescript";

export function loadTypeScriptProgram(sourcePath: string): {
  sourceFile: ts.SourceFile;
  checker: ts.TypeChecker;
} {
  const compilerOptions = compilerOptionsForExtraction();
  const program = ts.createProgram([sourcePath], compilerOptions);
  const sourceFile = program.getSourceFile(sourcePath);
  if (!sourceFile) {
    throw new Error(`Cannot load TypeScript source ${sourcePath}`);
  }

  return {
    sourceFile,
    checker: program.getTypeChecker(),
  };
}

export function loadInlineTypeScriptProgram(
  source: string,
  fileName: string,
): {
  sourceFile: ts.SourceFile;
  checker: ts.TypeChecker;
} {
  const compilerOptions = compilerOptionsForExtraction();
  const sourcePath = path.resolve(fileName);
  const host = ts.createCompilerHost(compilerOptions);
  const getSourceFile = host.getSourceFile.bind(host);

  host.getSourceFile = (requestedFile, languageVersion, onError, shouldCreateNewSourceFile) => {
    if (path.resolve(requestedFile) === sourcePath) {
      return ts.createSourceFile(requestedFile, source, languageVersion, true, ts.ScriptKind.TS);
    }
    return getSourceFile(requestedFile, languageVersion, onError, shouldCreateNewSourceFile);
  };
  host.readFile = (requestedFile) =>
    path.resolve(requestedFile) === sourcePath ? source : ts.sys.readFile(requestedFile);
  host.fileExists = (requestedFile) =>
    path.resolve(requestedFile) === sourcePath || ts.sys.fileExists(requestedFile);

  const program = ts.createProgram([sourcePath], compilerOptions, host);
  const sourceFile = program.getSourceFile(sourcePath);
  if (!sourceFile) {
    throw new Error("Cannot load inline TypeScript source");
  }

  return {
    sourceFile,
    checker: program.getTypeChecker(),
  };
}

function compilerOptionsForExtraction(): ts.CompilerOptions {
  return {
    target: ts.ScriptTarget.ES2022,
    module: ts.ModuleKind.NodeNext,
    moduleResolution: ts.ModuleResolutionKind.NodeNext,
    strict: true,
    skipLibCheck: true,
    noEmit: true,
  };
}
