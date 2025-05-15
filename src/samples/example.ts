// Sample program demonstrating all supported IRNode types

// FunctionNode, ProgramNode
function demo(x) {
  // VariableDeclarationNode: let sum = 0;
  let sum = 0;

  // VariableDeclarationNode & ArrayLiteralNode
  const data = [1, { val: 2 }, 3];

  // BlockNode with an ExpressionStatementNode (AssignmentNode, BinaryNode, ElementAccessNode)
  {
    sum = x + data[0];
  }

  // IfNode with BinaryNode, AssignmentNode, PropertyAccessNode, LiteralNode
  if (sum > 1) {
    sum = sum * data[1].val;
  } else {
    sum = sum - data[2];
  }

  // ForNode with VariableDeclarationNode, BinaryNode, CallExpressionNode (data.length)
  for (let i = 0; i < data.length; i++) {
    // IfNode, BinaryNode, LiteralNode
    if (data[i] % 2 === 0) {
      // ContinueNode
      continue;
    }
    // ExpressionStatementNode, AssignmentNode, BinaryNode
    sum += data[i];
  }

  // WhileNode
  while (sum < 10) {
    // ExpressionStatementNode, AssignmentNode, LiteralNode
    sum++;
    // IfNode, BinaryNode, LiteralNode, BreakStatementNode
    if (sum === 5) break;
  }

  // SwitchNode with CaseClauses, BreakStatementNode, AssignmentNode, LiteralNode
  switch (sum) {
    case 5:
      sum = 'five';
      break;
    case 6:
      sum = 'six';
      break;
    default:
      sum = 'other';
  }

  // NewExpressionNode & CallExpressionNode & PropertyAccessNode
  const date = new Date();
  console.log(date.getFullYear(), sum);

  // ReturnNode & IdentifierNode
  return sum;
}

// ObjectLiteralNode
const objLit = { a: 1, b: "test", nested: { c: true } };

// ArrayLiteralNode containing IdentifierNode & FunctionNode
const arrLit = [objLit, demo];

// CallExpressionNode with IdentifierNode & ElementAccessNode & PropertyAccessNode
demo(objLit.nested.c);
