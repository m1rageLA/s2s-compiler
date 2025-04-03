use syn::{parse_file, File, Item, Expr, Stmt};
use quote::ToTokens; // To convert syn types to token streams
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Node {
    kind: String,
    text: String,
    children: Vec<Node>,
}

fn main() {
    let code = r#"
        pub fn fibonacci(n: u32) -> u32 {
            if n <= 1 {
                return n;
            }
            fibonacci(n - 1) + fibonacci(n - 2)
        }
    "#;

    // Parse the Rust source code.
    let syntax_tree: File = parse_file(code).expect("Failed to parse Rust code");

    // Build our custom AST node from the syn File.
    let root_node = file_to_node(&syntax_tree);

    // Serialize to JSON and print.
    let json_ast = serde_json::to_string_pretty(&root_node).unwrap();
    println!("{}", json_ast);
}

/// Convert a syn::File (top-level AST) into our custom Node.
fn file_to_node(file_ast: &File) -> Node {
    let children = file_ast.items.iter().map(item_to_node).collect();
    Node {
        kind: "SourceFile".to_string(),
        text: "".to_string(),
        children,
    }
}

/// Convert a syn::Item into a Node.
fn item_to_node(item: &Item) -> Node {
    match item {
        Item::Fn(func) => {
            let mut children = Vec::new();

            // Add a signature node.
            children.push(Node {
                kind: "Signature".to_string(),
                text: format!("fn {}(...)", func.sig.ident),
                children: Vec::new(),
            });

            // Convert each statement in the function block.
            for stmt in &func.block.stmts {
                children.push(stmt_to_node(stmt));
            }

            Node {
                kind: "FunctionDeclaration".to_string(),
                text: format!("fn {}", func.sig.ident),
                children,
            }
        }
        _ => Node {
            kind: "OtherItem".to_string(),
            // Use ToTokens to print the item.
            text: format!("{}", item.to_token_stream()),
            children: Vec::new(),
        },
    }
}

/// Convert a syn::Stmt into a Node.
fn stmt_to_node(stmt: &Stmt) -> Node {
    match stmt {
        Stmt::Local(local) => Node {
            kind: "Local".to_string(),
            text: format!("{}", local.to_token_stream()),
            children: Vec::new(),
        },
        Stmt::Item(item) => item_to_node(item),
        // For Stmt::Expr, note that it now contains two fields.
        Stmt::Expr(expr, _) => expr_to_node(expr),
        // You might also want to handle macro statements if needed.
        _ => Node {
            kind: "OtherStmt".to_string(),
            text: format!("{}", stmt.to_token_stream()),
            children: Vec::new(),
        },
    }
}

/// Convert a syn::Expr into a Node.
fn expr_to_node(expr: &Expr) -> Node {
    match expr {
        Expr::If(expr_if) => {
            let mut children = Vec::new();
            // Process the condition.
            children.push(expr_to_node(&expr_if.cond));
            // Process each statement in the 'then' block.
            for stmt in &expr_if.then_branch.stmts {
                children.push(stmt_to_node(stmt));
            }
            Node {
                kind: "IfExpression".to_string(),
                text: "if (...) { ... }".to_string(),
                children,
            }
        }
        Expr::Binary(expr_bin) => {
            let mut children = Vec::new();
            children.push(expr_to_node(&expr_bin.left));
            children.push(expr_to_node(&expr_bin.right));
            Node {
                kind: "BinaryExpression".to_string(),
                text: format!("Binary op: {}", expr_bin.op.to_token_stream()),
                children,
            }
        }
        Expr::Call(expr_call) => {
            let mut children = Vec::new();
            children.push(expr_to_node(&expr_call.func));
            for arg in &expr_call.args {
                children.push(expr_to_node(arg));
            }
            Node {
                kind: "CallExpression".to_string(),
                text: "function_call(...)".to_string(),
                children,
            }
        }
        Expr::Return(expr_return) => {
            let mut children = Vec::new();
            if let Some(ret_expr) = &expr_return.expr {
                children.push(expr_to_node(ret_expr));
            }
            Node {
                kind: "ReturnExpression".to_string(),
                text: "return ...".to_string(),
                children,
            }
        }
        _ => Node {
            kind: "OtherExpression".to_string(),
            text: format!("{}", expr.to_token_stream()),
            children: Vec::new(),
        },
    }
}
