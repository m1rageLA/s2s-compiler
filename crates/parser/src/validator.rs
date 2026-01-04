#![allow(dead_code)]
use swc_ecma_ast::*;

pub(crate) fn assert_es5_strict(module: &Module) {
    let validator = Es5StrictValidator;
    validator.module(module);
}

struct Es5StrictValidator;

impl Es5StrictValidator {
    fn module(&self, module: &Module) {
        for item in &module.body {
            self.module_item(item);
        }
    }

    fn module_item(&self, item: &ModuleItem) {
        match item {
            ModuleItem::Stmt(stmt) => self.stmt(stmt),
            ModuleItem::ModuleDecl(decl) => match decl {
                ModuleDecl::Import(_) => self.reject("ImportDeclaration"),
                ModuleDecl::ExportDecl(_) => self.reject("ExportDeclaration"),
                ModuleDecl::ExportNamed(_) => self.reject("ExportNamedDeclaration"),
                ModuleDecl::ExportDefaultDecl(_) => self.reject("ExportDefaultDeclaration"),
                ModuleDecl::ExportDefaultExpr(_) => self.reject("ExportDefaultDeclaration"),
                ModuleDecl::ExportAll(_) => self.reject("ExportAllDeclaration"),
                ModuleDecl::TsImportEquals(_) => self.reject("TsImportEqualsDeclaration"),
                ModuleDecl::TsExportAssignment(_) => self.reject("TsExportAssignment"),
                ModuleDecl::TsNamespaceExport(_) => self.reject("TsNamespaceExportDeclaration"),
            },
        }
    }

    fn stmt(&self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr_stmt) => self.expr(&expr_stmt.expr),
            Stmt::Block(block) => self.block(block),
            Stmt::Empty(_) | Stmt::Debugger(_) => {}
            Stmt::With(with_stmt) => {
                self.expr(&with_stmt.obj);
                self.stmt(&with_stmt.body);
            }
            Stmt::Return(ret) => {
                if let Some(arg) = &ret.arg {
                    self.expr(arg);
                }
            }
            Stmt::Labeled(labeled) => self.stmt(&labeled.body),
            Stmt::Break(_) | Stmt::Continue(_) => {}
            Stmt::If(if_stmt) => {
                self.expr(&if_stmt.test);
                self.stmt(&if_stmt.cons);
                if let Some(alt) = &if_stmt.alt {
                    self.stmt(alt);
                }
            }
            Stmt::Switch(switch_stmt) => {
                self.expr(&switch_stmt.discriminant);
                for case in &switch_stmt.cases {
                    self.switch_case(case);
                }
            }
            Stmt::Throw(throw_stmt) => self.expr(&throw_stmt.arg),
            Stmt::Try(try_stmt) => {
                self.block(&try_stmt.block);
                if let Some(handler) = &try_stmt.handler {
                    self.catch_clause(handler);
                }
                if let Some(finalizer) = &try_stmt.finalizer {
                    self.block(finalizer);
                }
                if try_stmt.handler.is_none() && try_stmt.finalizer.is_none() {
                    self.reject("TryStatement without handler/finalizer");
                }
            }
            Stmt::While(while_stmt) => {
                self.expr(&while_stmt.test);
                self.stmt(&while_stmt.body);
            }
            Stmt::DoWhile(do_stmt) => {
                self.expr(&do_stmt.test);
                self.stmt(&do_stmt.body);
            }
            Stmt::For(for_stmt) => {
                if let Some(init) = &for_stmt.init {
                    match init {
                        VarDeclOrExpr::VarDecl(var) => self.var_decl(var),
                        VarDeclOrExpr::Expr(expr) => self.expr(expr),
                    }
                }
                if let Some(test) = &for_stmt.test {
                    self.expr(test);
                }
                if let Some(update) = &for_stmt.update {
                    self.expr(update);
                }
                self.stmt(&for_stmt.body);
            }
            Stmt::ForIn(for_in) => {
                self.for_in_head(&for_in.left);
                self.expr(&for_in.right);
                self.stmt(&for_in.body);
            }
            Stmt::Decl(decl) => self.decl(decl),
            Stmt::ForOf(_) => self.reject("ForOfStatement"),
        }
    }

    fn block(&self, block: &BlockStmt) {
        for stmt in &block.stmts {
            self.stmt(stmt);
        }
    }

    fn switch_case(&self, case: &SwitchCase) {
        if let Some(test) = &case.test {
            self.expr(test);
        }
        for stmt in &case.cons {
            self.stmt(stmt);
        }
    }

    fn catch_clause(&self, clause: &CatchClause) {
        if let Some(param) = &clause.param {
            self.binding_pat(param, "CatchClause");
        } else {
            self.reject("CatchClause without parameter");
        }
        self.block(&clause.body);
    }

    fn decl(&self, decl: &Decl) {
        match decl {
            Decl::Fn(fn_decl) => {
                if fn_decl.declare {
                    self.reject("DeclareFunction");
                }
                self.ident(&fn_decl.ident, "FunctionDeclaration");
                self.function(&fn_decl.function, "FunctionDeclaration");
            }
            Decl::Class(class_decl) => {
                if class_decl.declare {
                    self.reject("DeclareClass");
                }
                self.ident(&class_decl.ident, "ClassDeclaration");
                self.class(&class_decl.class);
            }
            Decl::Var(var_decl) => {
                if var_decl.declare {
                    self.reject("DeclareVariable");
                }
                self.var_decl(var_decl);
            }
            Decl::Using(_) => self.reject("UsingDeclaration"),
            Decl::TsInterface(_) => self.reject("TsInterfaceDeclaration"),
            Decl::TsTypeAlias(_) => self.reject("TsTypeAliasDeclaration"),
            Decl::TsEnum(_) => self.reject("TsEnumDeclaration"),
            Decl::TsModule(_) => self.reject("TsModuleDeclaration"),
        }
    }

    fn var_decl(&self, decl: &VarDecl) {
        for declarator in &decl.decls {
            if declarator.definite {
                self.reject("DefiniteAssignment");
            }
            self.binding_pat(&declarator.name, "VariableDeclarator");
            if let Some(init) = &declarator.init {
                self.expr(init);
            }
        }
    }

    fn binding_pat(&self, pat: &Pat, context: &str) {
        match pat {
            Pat::Ident(ident) => self.binding_ident(ident, context),
            _ => self.reject(context),
        }
    }

    fn assignment_pat(&self, pat: &Pat, context: &str) {
        match pat {
            Pat::Ident(ident) => self.binding_ident(ident, context),
            Pat::Expr(expr) => self.expr(expr),
            _ => self.reject(context),
        }
    }

    fn binding_ident(&self, ident: &BindingIdent, context: &str) {
        if ident.type_ann.is_some() {
            self.reject(context);
        }
        self.ident(&ident.id, context);
    }

    fn ident(&self, ident: &Ident, context: &str) {
        if ident.optional {
            self.reject(context);
        }
    }

    fn expr(&self, expr: &Expr) {
        match expr {
            Expr::This(_) => {}
            Expr::Ident(ident) => self.ident(ident, "Identifier"),
            Expr::Lit(lit) => self.lit(lit),
            Expr::Array(array_lit) => self.array_lit(array_lit),
            Expr::Object(object_lit) => self.object_lit(object_lit),
            Expr::Fn(fn_expr) => {
                if let Some(ident) = &fn_expr.ident {
                    self.ident(ident, "FunctionExpression");
                }
                self.function(&fn_expr.function, "FunctionExpression");
            }
            Expr::Unary(unary) => {
                self.unary_expr(unary);
            }
            Expr::Update(update) => {
                self.expr(&update.arg);
            }
            Expr::Bin(bin) => {
                self.binary_op(bin.op);
                self.expr(&bin.left);
                self.expr(&bin.right);
            }
            Expr::Assign(assign) => {
                self.assign_target(&assign.left);
                self.assign_op(assign.op);
                self.expr(&assign.right);
            }
            Expr::Member(member) => self.member_expr(member),
            Expr::SuperProp(super_prop) => self.super_prop_expr(super_prop),
            Expr::Cond(cond) => {
                self.expr(&cond.test);
                self.expr(&cond.cons);
                self.expr(&cond.alt);
            }
            Expr::Call(call_expr) => self.call_expr(call_expr),
            Expr::New(new_expr) => self.new_expr(new_expr),
            Expr::Seq(seq) => {
                for expr in &seq.exprs {
                    self.expr(expr);
                }
            }
            Expr::Class(class_expr) => self.class(&class_expr.class),
            Expr::Paren(paren) => self.expr(&paren.expr),
            Expr::Tpl(_) => self.reject("TemplateLiteral"),
            Expr::TaggedTpl(_) => self.reject("TaggedTemplateExpression"),
            Expr::Arrow(_) => self.reject("ArrowFunctionExpression"),
            Expr::Yield(_) => self.reject("YieldExpression"),
            Expr::MetaProp(_) => self.reject("MetaProperty"),
            Expr::Await(_) => self.reject("AwaitExpression"),
            Expr::JSXMember(_) | Expr::JSXNamespacedName(_) | Expr::JSXEmpty(_) | Expr::JSXElement(_)
            | Expr::JSXFragment(_) => self.reject("JSXExpression"),
            Expr::TsTypeAssertion(_) | Expr::TsConstAssertion(_) | Expr::TsNonNull(_)
            | Expr::TsAs(_) | Expr::TsInstantiation(_) | Expr::TsSatisfies(_) => {
                self.reject("TypeScriptExpression")
            }
            Expr::OptChain(_) => self.reject("OptionalChainingExpression"),
            Expr::PrivateName(_) => self.reject("PrivateNameExpression"),
            Expr::Invalid(_) => self.reject("InvalidExpression"),
        }
    }

    fn lit(&self, lit: &Lit) {
        match lit {
            Lit::Str(_) | Lit::Bool(_) | Lit::Null(_) | Lit::Num(_) | Lit::Regex(_) => {}
            Lit::BigInt(_) | Lit::JSXText(_) => self.reject("Literal"),
        }
    }

    fn array_lit(&self, array: &ArrayLit) {
        for elem in &array.elems {
            if let Some(elem) = elem {
                if elem.spread.is_some() {
                    self.reject("ArraySpreadElement");
                }
                self.expr(&elem.expr);
            }
        }
    }

    fn object_lit(&self, object: &ObjectLit) {
        for prop in &object.props {
            match prop {
                PropOrSpread::Prop(prop) => self.prop(prop),
                PropOrSpread::Spread(_) => self.reject("ObjectSpread"),
            }
        }
    }

    fn prop(&self, prop: &Prop) {
        match prop {
            Prop::KeyValue(kv) => {
                self.prop_name(&kv.key);
                self.expr(&kv.value);
            }
            Prop::Getter(getter) => {
                self.prop_name(&getter.key);
                if getter.type_ann.is_some() {
                    self.reject("GetterProperty");
                }
                if let Some(body) = &getter.body {
                    self.block(body);
                } else {
                    self.reject("GetterProperty");
                }
            }
            Prop::Setter(setter) => {
                self.prop_name(&setter.key);
                if setter.this_param.is_some() {
                    self.reject("SetterProperty");
                }
                self.binding_pat(&setter.param, "SetterProperty");
                if let Some(body) = &setter.body {
                    self.block(body);
                } else {
                    self.reject("SetterProperty");
                }
            }
            Prop::Method(_) => self.reject("MethodProperty"),
            Prop::Assign(_) => self.reject("AssignmentProperty"),
            Prop::Shorthand(_) => self.reject("ShorthandProperty"),
        }
    }

    fn prop_name(&self, name: &PropName) {
        match name {
            PropName::Ident(_) | PropName::Str(_) | PropName::Num(_) => {}
            PropName::Computed(_) => self.reject("ComputedPropertyName"),
            PropName::BigInt(_) => self.reject("BigIntPropertyName"),
        }
    }

    fn function(&self, function: &Function, context: &str) {
        if function.is_async || function.is_generator {
            self.reject(context);
        }
        if !function.decorators.is_empty() {
            self.reject(context);
        }
        if function.type_params.is_some() || function.return_type.is_some() {
            self.reject(context);
        }
        if let Some(body) = &function.body {
            self.block(body);
        } else {
            self.reject(context);
        }
        for param in &function.params {
            if !param.decorators.is_empty() {
                self.reject(context);
            }
            self.binding_pat(&param.pat, context);
        }
    }

    fn class(&self, class: &Class) {
        if !class.decorators.is_empty()
            || class.is_abstract
            || class.type_params.is_some()
            || class.super_type_params.is_some()
            || !class.implements.is_empty()
        {
            self.reject("ClassDeclaration");
        }
        if let Some(super_class) = &class.super_class {
            self.expr(super_class);
        }
        for member in &class.body {
            self.class_member(member);
        }
    }

    fn class_member(&self, member: &ClassMember) {
        match member {
            ClassMember::Constructor(ctor) => self.constructor(ctor),
            ClassMember::Method(method) => self.class_method(method),
            ClassMember::Empty(_) => {}
            ClassMember::ClassProp(_) => self.reject("ClassProperty"),
            ClassMember::PrivateProp(_) => self.reject("PrivateClassProperty"),
            ClassMember::TsIndexSignature(_) => self.reject("TsIndexSignature"),
            ClassMember::PrivateMethod(_) => self.reject("PrivateMethod"),
            ClassMember::StaticBlock(_) => self.reject("StaticBlock"),
            ClassMember::AutoAccessor(_) => self.reject("AutoAccessor"),
        }
    }

    fn constructor(&self, ctor: &Constructor) {
        if ctor.accessibility.is_some() || ctor.is_optional {
            self.reject("Constructor");
        }
        self.prop_name(&ctor.key);
        if let Some(body) = &ctor.body {
            self.block(body);
        } else {
            self.reject("Constructor");
        }
        for param in &ctor.params {
            match param {
                ParamOrTsParamProp::Param(param) => {
                    self.binding_pat(&param.pat, "Constructor");
                }
                ParamOrTsParamProp::TsParamProp(_) => self.reject("TsParameterProperty"),
            }
        }
    }

    fn class_method(&self, method: &ClassMethod) {
        if method.accessibility.is_some()
            || method.is_abstract
            || method.is_optional
            || method.is_override
        {
            self.reject("ClassMethod");
        }
        self.prop_name(&method.key);
        self.function(&method.function, "ClassMethod");
    }

    fn call_expr(&self, call: &CallExpr) {
        if call.type_args.is_some() {
            self.reject("CallExpression");
        }
        match &call.callee {
            Callee::Expr(expr) => self.expr(expr),
            Callee::Super(_) => {}
            Callee::Import(_) => self.reject("ImportCall"),
        }
        self.call_args(&call.args);
    }

    fn new_expr(&self, new_expr: &NewExpr) {
        if new_expr.type_args.is_some() {
            self.reject("NewExpression");
        }
        self.expr(&new_expr.callee);
        if let Some(args) = &new_expr.args {
            self.call_args(args);
        }
    }

    fn call_args(&self, args: &[ExprOrSpread]) {
        for arg in args {
            if arg.spread.is_some() {
                self.reject("SpreadArgument");
            }
            self.expr(&arg.expr);
        }
    }

    fn member_expr(&self, member: &MemberExpr) {
        self.expr(&member.obj);
        self.member_prop(&member.prop);
    }

    fn member_prop(&self, prop: &MemberProp) {
        match prop {
            MemberProp::Ident(_) => {}
            MemberProp::Computed(comp) => self.expr(&comp.expr),
            MemberProp::PrivateName(_) => self.reject("PrivateName"),
        }
    }

    fn super_prop_expr(&self, prop: &SuperPropExpr) {
        match &prop.prop {
            SuperProp::Ident(_) => {}
            SuperProp::Computed(comp) => self.expr(&comp.expr),
        }
    }

    fn unary_expr(&self, unary: &UnaryExpr) {
        self.expr(&unary.arg);
    }

    fn binary_op(&self, op: BinaryOp) {
        match op {
            BinaryOp::Exp | BinaryOp::NullishCoalescing => {
                self.reject("BinaryExpression");
            }
            _ => {}
        }
    }

    fn assign_op(&self, op: AssignOp) {
        match op {
            AssignOp::ExpAssign | AssignOp::AndAssign | AssignOp::OrAssign | AssignOp::NullishAssign => {
                self.reject("AssignmentExpression");
            }
            _ => {}
        }
    }

    fn assign_target(&self, target: &AssignTarget) {
        match target {
            AssignTarget::Simple(simple) => self.simple_assign_target(simple),
            AssignTarget::Pat(_) => self.reject("DestructuringAssignment"),
        }
    }

    fn simple_assign_target(&self, target: &SimpleAssignTarget) {
        match target {
            SimpleAssignTarget::Ident(ident) => {
                if ident.type_ann.is_some() {
                    self.reject("Identifier");
                }
                self.ident(&ident.id, "Identifier");
            }
            SimpleAssignTarget::Member(member) => self.member_expr(member),
            SimpleAssignTarget::SuperProp(prop) => self.super_prop_expr(prop),
            SimpleAssignTarget::Paren(paren) => self.expr(&paren.expr),
            SimpleAssignTarget::OptChain(_) => self.reject("OptionalChainingAssignmentTarget"),
            SimpleAssignTarget::TsAs(_)
            | SimpleAssignTarget::TsSatisfies(_)
            | SimpleAssignTarget::TsNonNull(_)
            | SimpleAssignTarget::TsTypeAssertion(_)
            | SimpleAssignTarget::TsInstantiation(_) => self.reject("TypeScriptAssignmentTarget"),
            SimpleAssignTarget::Invalid(_) => self.reject("InvalidAssignmentTarget"),
        }
    }

    fn for_in_head(&self, head: &ForHead) {
        match head {
            ForHead::VarDecl(var) => self.var_decl(var),
            ForHead::Pat(pat) => self.assignment_pat(pat, "ForInStatement"),
            ForHead::UsingDecl(_) => self.reject("UsingDeclaration"),
        }
    }

    fn reject(&self, node: &str) -> ! {
        panic!(
            "ES5 validation failed: node `{}` is not allowed after normalization. Only ES5 syntax plus const/let/class are permitted. Input is rejected.",
            node
        );
    }
}
