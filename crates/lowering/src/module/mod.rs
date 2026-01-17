use crate::context;
use ir::{
    IrCatchClause, IrForInLeft, IrForInit, IrItem, IrModule, IrStmt, IrVariable,
};
use swc_ecma_ast::{self as ast};

mod stmt;
mod unsupported;

pub fn ast_to_ir(module: &ast::Module) -> IrModule {
    context::reset();
    let mut items: Vec<IrItem> = Vec::new();

    for statement in &module.body {
        match statement {
            ast::ModuleItem::Stmt(stmt) => stmt::lower(stmt, &mut items),
            _ => unsupported::handle(),
        }
    }

    apply_mutations(&mut items);

    IrModule { items }
}

fn apply_mutations(items: &mut [IrItem]) {
    for item in items {
        match item {
            IrItem::Variable(var) => apply_mutation_to_var(var),
            IrItem::Function(func) => apply_mutation_to_stmts(&mut func.body),
            IrItem::Block(stmts) => apply_mutation_to_stmts(stmts),
            IrItem::Expression(_) => {}
        }
    }
}

fn apply_mutation_to_stmts(stmts: &mut [IrStmt]) {
    for stmt in stmts {
        apply_mutation_to_stmt(stmt);
    }
}

fn apply_mutation_to_stmt(stmt: &mut IrStmt) {
    match stmt {
        IrStmt::VarDecl(vars) => {
            for var in vars {
                apply_mutation_to_var(var);
            }
        }
        IrStmt::Leteral(var) => apply_mutation_to_var(var),
        IrStmt::Block(stmts) => apply_mutation_to_stmts(stmts),
        IrStmt::Labeled { body, .. } => apply_mutation_to_stmt(body),
        IrStmt::If {
            then_branch,
            else_branch,
            ..
        } => {
            apply_mutation_to_stmts(then_branch);
            if let Some(branch) = else_branch {
                apply_mutation_to_stmts(branch);
            }
        }
        IrStmt::While(_, body) | IrStmt::DoWhile(body, _) => apply_mutation_to_stmts(body),
        IrStmt::For { init, body, .. } => {
            if let Some(init) = init {
                if let IrForInit::VarDecl(vars) = init {
                    for var in vars {
                        apply_mutation_to_var(var);
                    }
                }
            }
            apply_mutation_to_stmts(body);
        }
        IrStmt::ForIn { left, body, .. } => {
            if let IrForInLeft::Var(var) = left {
                apply_mutation_to_var(var);
            }
            apply_mutation_to_stmts(body);
        }
        IrStmt::Switch { cases, .. } => {
            for case in cases {
                apply_mutation_to_stmts(&mut case.consequent);
            }
        }
        IrStmt::Try {
            try_block,
            catch,
            finally,
        } => {
            apply_mutation_to_stmts(try_block);
            if let Some(IrCatchClause { body, .. }) = catch {
                apply_mutation_to_stmts(body);
            }
            if let Some(finally) = finally {
                apply_mutation_to_stmts(finally);
            }
        }
        _ => {}
    }
}

fn apply_mutation_to_var(var: &mut IrVariable) {
    if context::is_mutated(&var.name) {
        var.mutable = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{assert_identifier, expect_variable, lower};
    use ir::{IrItem, IrStmt};

    #[test]
    fn ast_to_ir_emits_items_for_supported_statements() {
        let ir_module = lower(
            r#"
            const value = 1;
            function run() {}
            value;
            {
                let scoped = 2;
            }
        "#,
        );

        assert_eq!(ir_module.items.len(), 4);

        let value = expect_variable(&ir_module.items[0], "value");
        assert!(!value.mutable);

        match &ir_module.items[1] {
            IrItem::Function(function) => {
                assert_eq!(function.name, "run");
            }
            other => panic!("expected function item, got {other:?}"),
        }

        match &ir_module.items[2] {
            IrItem::Expression(expr) => assert_identifier(expr, "value"),
            other => panic!("expected expression item, got {other:?}"),
        }

        match &ir_module.items[3] {
            IrItem::Block(stmts) => {
                assert_eq!(stmts.len(), 1);
                match &stmts[0] {
                    IrStmt::VarDecl(vars) => {
                        assert_eq!(vars.len(), 1);
                        assert_eq!(vars[0].name, "scoped");
                    }
                    other => panic!("expected variable declaration in block, got {other:?}"),
                }
            }
            other => panic!("expected block item, got {other:?}"),
        }
    }

    #[test]
    fn ast_to_ir_wraps_loops_into_block_items() {
        let ir_module = lower(
            r#"
            while (false) {}
            do {
                value();
            } while (true);
            for (let i = 0; i < 3; i = i + 1) {}
        "#,
        );

        assert_eq!(ir_module.items.len(), 3);

        match &ir_module.items[0] {
            IrItem::Block(stmts) => {
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], IrStmt::While(_, _)));
            }
            other => panic!("expected while loop block, got {other:?}"),
        }

        match &ir_module.items[1] {
            IrItem::Block(stmts) => {
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], IrStmt::DoWhile(_, _)));
            }
            other => panic!("expected do/while loop block, got {other:?}"),
        }

        match &ir_module.items[2] {
            IrItem::Block(stmts) => {
                assert_eq!(stmts.len(), 1);
                assert!(matches!(stmts[0], IrStmt::For { .. }));
            }
            other => panic!("expected for loop block, got {other:?}"),
        }
    }

    #[test]
    fn ast_to_ir_ignores_module_declarations() {
        let ir_module = lower(
            r#"
            const value = 1;
            export { value };
        "#,
        );

        assert_eq!(ir_module.items.len(), 1);
        expect_variable(&ir_module.items[0], "value");
    }

    #[test]
    fn ast_to_ir_skips_unsupported_statements() {
        let ir_module = lower(
            r#"
            debugger;
        "#,
        );

        assert!(ir_module.items.is_empty());
    }
}
