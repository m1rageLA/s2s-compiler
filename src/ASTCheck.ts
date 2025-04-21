import * as ts from 'typescript';
import * as fs from 'fs';
import * as path from 'path';

// Читаем и парсим файл
const filePath = path.join(__dirname, './samples/example.ts');
const sourceCode = fs.readFileSync(filePath, 'utf-8');

// Парсим исходный код в AST
const sourceFile = ts.createSourceFile(
  'example.ts',
  sourceCode,
  ts.ScriptTarget.Latest,
  true,
  ts.ScriptKind.TS
);

// Печатаем AST в консоль
function printNode(node: ts.Node, indent: string = '') {
  console.log(`${indent}${ts.SyntaxKind[node.kind]} (${node.kind}) → "${node.getText()}"`);
  node.forEachChild(child => printNode(child, indent + '  '));
}

printNode(sourceFile);
