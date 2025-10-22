use std::collections::HashMap;

use swc_common::{
    errors::{EmitterWriter, Handler},
    sync::Lrc,
    util::take::Take,
    FileName, Mark, SourceMap, Span, GLOBALS,
};
use swc_ecma_ast::{Id, Module, Pass, Program, TsTypeAnn};
use swc_ecma_codegen::to_code_default;
use swc_ecma_parser::{lexer::Lexer, Parser, StringInput, Syntax, TsSyntax};

use compat::{es2015, es2016, es2020};
use swc_ecma_transforms_base::helpers::{inject_helpers, Helpers, HELPERS};
use swc_ecma_transforms_base::resolver;
use swc_ecma_transforms_compat as compat; // удобный модуль с es20xx
use swc_ecma_transforms_typescript::strip;
use swc_ecma_visit::{swc_ecma_ast as visit_ast, Visit, VisitMut, VisitMutWith, VisitWith};

/// Результат парсинга/понижения с нужной SourceMap для кодогена.
pub struct ParsedModule {
    pub module: Module,
    pub js_module: Module,
    pub source_map: Lrc<SourceMap>,
}

/// Парсит TS/JS → понижает → возвращает Module + SourceMap.
/// Готов к классам, for-of, template, стрелкам, **, ?., ?? и т.д.
pub fn ast_with_sources(src: &str) -> ParsedModule {
    let cm: Lrc<SourceMap> = Default::default();
    let emitter = EmitterWriter::new(Box::new(std::io::stderr()), Some(cm.clone()), true, true);
    let handler = Handler::with_emitter(true, false, Box::new(emitter));

    let fm = cm.new_source_file(
        Lrc::new(FileName::Custom("input.ts".into())),
        src.to_owned(),
    );

    let lexer = Lexer::new(
        Syntax::Typescript(TsSyntax {
            tsx: false,
            decorators: true,
            dts: false,
            ..Default::default()
        }),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );
    let mut parser = Parser::new_from(lexer);

    let (module, js_module) = GLOBALS.set(&Default::default(), || {
        // 1) Парсинг
        let parsed: Module = parser
            .parse_module()
            .map_err(|e| e.into_diagnostic(&handler).emit())
            .expect("failed to parse module");

        let module_ir = transform_module_for_ir(parsed.clone());
        let module_js = transform_module_for_js(parsed);

        (module_ir, module_js)
    });

    ParsedModule {
        module,
        js_module,
        source_map: cm,
    }
}

/// Упрощённый вариант: только Module.
pub fn ast(src: &str) -> Module {
    ast_with_sources(src).module
}

/// Алиас для обратной совместимости.
pub fn ast_downleveled(src: &str) -> Module {
    ast(src)
}

/// Эмит нормализованного JS из Module (+ SourceMap).
pub fn module_to_js(module: &Module, source_map: Lrc<SourceMap>) -> String {
    to_code_default(source_map, None, module)
}

/// One-shot: TS → пониженный JS (строкой).
pub fn downleveled_js(src: &str) -> String {
    let parsed = ast_with_sources(src);
    module_to_js(&parsed.js_module, parsed.source_map)
}

fn apply_pass(module: &mut Module, mut pass: impl Pass) {
    let mut program = Program::Module(module.take());
    pass.process(&mut program);
    *module = match program {
        Program::Module(m) => m,
        Program::Script(_) => unreachable!("expected module program after pass"),
    };
}

fn transform_module_for_ir(mut module: Module) -> Module {
    let unresolved_mark = Mark::new();
    let top_level_mark = Mark::new();

    apply_pass(
        &mut module,
        resolver(unresolved_mark, top_level_mark, /*is_ts*/ true),
    );

    let mut type_table = TypeTable::default();
    type_table.collect(&module);

    apply_pass(&mut module, strip(unresolved_mark, top_level_mark));
    apply_pass(&mut module, es2015::arrow(unresolved_mark));
    apply_pass(
        &mut module,
        es2015::for_of(es2015::for_of::Config {
            assume_array: true,
            ..Default::default()
        }),
    );
    apply_pass(&mut module, es2016::exponentiation());
    apply_pass(
        &mut module,
        es2020::optional_chaining(
            es2020::optional_chaining::Config::default(),
            unresolved_mark,
        ),
    );
    apply_pass(
        &mut module,
        es2020::nullish_coalescing(es2020::nullish_coalescing::Config::default()),
    );

    type_table.apply(&mut module);
    module
}

fn transform_module_for_js(module: Module) -> Module {
    HELPERS.set(&Helpers::new(false), || {
        let unresolved_mark = Mark::new();
        let top_level_mark = Mark::new();

        let mut module = module;

        apply_pass(
            &mut module,
            resolver(unresolved_mark, top_level_mark, /*is_ts*/ true),
        );
        apply_pass(&mut module, strip(unresolved_mark, top_level_mark));
        apply_pass(&mut module, es2015::arrow(unresolved_mark));
        apply_pass(
            &mut module,
            es2015::classes(es2015::classes::Config::default()),
        );
        apply_pass(
            &mut module,
            es2015::for_of(es2015::for_of::Config {
                assume_array: true,
                ..Default::default()
            }),
        );
        apply_pass(&mut module, es2016::exponentiation());
        apply_pass(
            &mut module,
            es2020::optional_chaining(
                es2020::optional_chaining::Config::default(),
                unresolved_mark,
            ),
        );
        apply_pass(
            &mut module,
            es2020::nullish_coalescing(es2020::nullish_coalescing::Config::default()),
        );
        apply_pass(&mut module, inject_helpers(top_level_mark));

        module
    })
}

#[derive(Default)]
struct TypeTable {
    bindings: HashMap<Id, Box<TsTypeAnn>>,
    fn_returns: HashMap<Span, Box<TsTypeAnn>>,
}

impl TypeTable {
    fn collect(&mut self, module: &Module) {
        let mut collector = TypeCollector {
            bindings: &mut self.bindings,
            fn_returns: &mut self.fn_returns,
        };
        module.visit_with(&mut collector);
    }

    fn apply(&self, module: &mut Module) {
        let mut rewriter = TypeRewriter { table: self };
        module.visit_mut_with(&mut rewriter);
    }
}

struct TypeCollector<'a> {
    bindings: &'a mut HashMap<Id, Box<TsTypeAnn>>,
    fn_returns: &'a mut HashMap<Span, Box<TsTypeAnn>>,
}

impl<'a> Visit for TypeCollector<'a> {
    fn visit_binding_ident(&mut self, ident: &visit_ast::BindingIdent) {
        if let Some(ann) = &ident.type_ann {
            self.bindings.insert(ident.id.to_id(), ann.clone());
        }
        ident.visit_children_with(self);
    }

    fn visit_function(&mut self, function: &visit_ast::Function) {
        if let Some(ret) = &function.return_type {
            self.fn_returns.insert(function.span, ret.clone());
        }
        function.visit_children_with(self);
    }
}

struct TypeRewriter<'a> {
    table: &'a TypeTable,
}

impl VisitMut for TypeRewriter<'_> {
    fn visit_mut_binding_ident(&mut self, ident: &mut visit_ast::BindingIdent) {
        if let Some(ann) = self.table.bindings.get(&ident.id.to_id()) {
            ident.type_ann = Some(ann.clone());
        }
        ident.visit_mut_children_with(self);
    }

    fn visit_mut_function(&mut self, function: &mut visit_ast::Function) {
        if let Some(ret) = self.table.fn_returns.get(&function.span) {
            function.return_type = Some(ret.clone());
        }
        function.visit_mut_children_with(self);
    }
}
