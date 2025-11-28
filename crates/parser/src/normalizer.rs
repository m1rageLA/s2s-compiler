#![allow(dead_code)]
use swc_common::{
    comments::{Comments, SingleThreadedComments},
    Globals, Mark, GLOBALS,
};
use swc_ecma_ast::{Module, Pass, Program};
use swc_ecma_compat_common::regexp::{self, regexp};
use swc_ecma_compat_es2015::{
    arrow, block_scoped_functions, computed_properties, destructuring, duplicate_keys,
    function_name, generator, instance_of, new_target, object_super, parameters, shorthand, spread,
    sticky_regex, template_literal, typeof_symbol, Config as Es2015Config,
};
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene};
use swc_ecma_transforms::resolver;
use swc_ecma_visit::VisitMutWith;

pub(crate) fn ast_normalize(ast: Module) -> Module {
    let globals = Globals::new();
    GLOBALS.set(&globals, || normalize(ast))
}

fn normalize(mut ast: Module) -> Module {
    let unresolved = Mark::new();
    let top_level = Mark::new();

    // 1️⃣ resolver — мутирует AST (VisitMut)
    ast.visit_mut_with(&mut resolver(unresolved, top_level, true));

    let mut program = Program::Module(ast);

    // 2️⃣ es2015 — кастомный Pass, сохраняющий class / const / let
    program = program.apply(es2015_with_es6_preserved(
        unresolved,
        None::<SingleThreadedComments>,
        Default::default(),
    ));

    // 3️⃣ hygiene и fixer — Fold
    program = program.apply(hygiene());
    program = program.apply(fixer(None));

    match program {
        Program::Module(module) => module,
        Program::Script(_) => unreachable!("нормализатор ожидает модуль"),
    }
}

fn es2015_with_es6_preserved<C>(
    unresolved_mark: Mark,
    comments: Option<C>,
    config: Es2015Config,
) -> impl Pass
where
    C: Comments + Clone,
{
    (
        (
            regexp(regexp::Config {
                dot_all_regex: false,
                has_indices: false,
                lookbehind_assertion: false,
                named_capturing_groups_regex: false,
                sticky_regex: true,
                unicode_property_regex: false,
                unicode_regex: true,
                unicode_sets_regex: false,
            }),
            block_scoped_functions(),
            template_literal(config.template_literal),
            // classes() убираем, чтобы сохранить синтаксис class
            new_target(),
            spread(config.spread, unresolved_mark),
        ),
        if !config.typescript {
            Some(object_super())
        } else {
            None
        },
        shorthand(),
        function_name(),

        // Should come before parameters (см. swc issue #1036)
        parameters(config.parameters, unresolved_mark),
        (
            exprs(unresolved_mark),
            typeof_symbol(config.typeof_symbol),
            computed_properties(config.computed_props),
            destructuring(config.destructuring),
            // block_scoping() пропускаем, чтобы не занижать let/const
            generator::generator(unresolved_mark, comments),
        ),
    )
}

fn exprs(unresolved_mark: Mark) -> impl Pass {
    (
        arrow(unresolved_mark),
        duplicate_keys(),
        sticky_regex(),
        instance_of(),
    )
}
