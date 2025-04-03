import ts from 'typescript';
import fs from 'fs';
export default function parseCodeToAST(fileName: string): ts.SourceFile {
    const sourceCode = fs.readFileSync(fileName, 'utf-8');
    const sourceFile = ts.createSourceFile(
        fileName,
        sourceCode,
        ts.ScriptTarget.Latest,
        true
    );

    function toSimpleAST(node: ts.Node): any {
        const kind = ts.SyntaxKind[node.kind];
        const text = node.getText(sourceFile);
        const childer = node.getChildren(sourceFile).map(toSimpleAST);

        return {
            kind,
            text,
            children: childer,
        }
    }
    return toSimpleAST(sourceFile);
}