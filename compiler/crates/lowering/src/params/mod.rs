use ir::IrParam;
use swc_ecma_ast::{self as ast};

mod binding;
mod param_prop;
mod pat;
mod unsupported;

pub(crate) fn params_to_ir<I>(params: I) -> Vec<IrParam>
where
    I: IntoIterator,
    I::Item: ParamLower,
{
    params.into_iter().map(ParamLower::lower_param).collect()
}

pub(crate) trait ParamLower {
    fn lower_param(self) -> IrParam;
}

impl<'a> ParamLower for &'a ast::Pat {
    fn lower_param(self) -> IrParam {
        pat::from_pat(self)
    }
}

impl<'a> ParamLower for &'a ast::Param {
    fn lower_param(self) -> IrParam {
        pat::from_pat(&self.pat)
    }
}

impl<'a> ParamLower for &'a ast::ParamOrTsParamProp {
    fn lower_param(self) -> IrParam {
        param_prop::from_param(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ir::IrType;
    use swc_ecma_ast::{ClassMember, ModuleItem};

    #[test]
    fn lowers_function_params_with_annotations() {
        let module = parser::ast(
            r#"
            function sum(value: number, label: string) {}
        "#,
        );
        let fn_decl = match module.body.first().expect("expected function declaration") {
            ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Fn(fn_decl))) => fn_decl,
            other => panic!("expected function declaration, got {other:?}"),
        };
        let params = params_to_ir(&fn_decl.function.params);

        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "value");
        assert_eq!(params[0].ty, IrType::Number);
        assert_eq!(params[1].name, "label");
        assert_eq!(params[1].ty, IrType::Str);
    }

    #[test]
    fn lowers_assignment_and_rest_patterns() {
        let module = parser::ast(
            r#"
            function collect(value = 1, ...rest: number[]) {}
        "#,
        );
        let fn_decl = match module.body.first().expect("expected function declaration") {
            ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Fn(fn_decl))) => fn_decl,
            other => panic!("expected function declaration, got {other:?}"),
        };

        let params = params_to_ir(&fn_decl.function.params);
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "value");
        assert_eq!(params[0].ty, IrType::Any);

        assert_eq!(params[1].name, "rest");
        assert_eq!(params[1].ty, IrType::Any);
    }

    #[test]
    fn marks_unsupported_patterns() {
        let module = parser::ast(
            r#"
            function acceptsObject({ value }) {}
        "#,
        );
        let fn_decl = match module.body.first().expect("expected function declaration") {
            ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Fn(fn_decl))) => fn_decl,
            other => panic!("expected function declaration, got {other:?}"),
        };

        let params = params_to_ir(&fn_decl.function.params);
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].name, "unsupported_param");
        assert_eq!(params[0].ty, IrType::Any);
    }

    #[test]
    fn lowers_constructor_parameter_properties() {
        let module = parser::ast(
            r#"
            class Example {
                constructor(private value: number, readonly flag = true) {}
            }
        "#,
        );

        let class_decl = module
            .body
            .iter()
            .find_map(|item| match item {
                ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Class(class_decl))) => Some(class_decl),
                _ => None,
            })
            .expect("expected class declaration");

        let ctor = class_decl
            .class
            .body
            .iter()
            .find_map(|member| match member {
                ClassMember::Constructor(ctor) => Some(ctor),
                _ => None,
            })
            .expect("expected constructor member");

        let params = params_to_ir(&ctor.params);
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].name, "value");
        assert_eq!(params[0].ty, IrType::Number);
        assert_eq!(params[1].name, "flag");
        assert_eq!(params[1].ty, IrType::Any);
    }

    #[test]
    fn supports_direct_pat_iterators() {
        let module = parser::ast("const arrow = ({ x }: { x: number }) => x;");
        let binding = match module.body.first().expect("expected variable declaration") {
            ModuleItem::Stmt(ast::Stmt::Decl(ast::Decl::Var(var_decl))) => {
                var_decl.decls.first().expect("expected declarator")
            }
            other => panic!("expected variable declaration, got {other:?}"),
        };

        let arrow = match binding
            .init
            .as_ref()
            .expect("expected initializer")
            .as_ref()
        {
            ast::Expr::Arrow(arrow) => arrow,
            other => panic!("expected arrow expression, got {other:?}"),
        };

        let params = params_to_ir(&arrow.params);
        assert_eq!(params.len(), 1, "expected single destructured param");
        assert_eq!(params[0].name, "unsupported_param");
    }
}
