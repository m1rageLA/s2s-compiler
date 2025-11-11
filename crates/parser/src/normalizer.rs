use swc_common::{comments::SingleThreadedComments, Globals, Mark, GLOBALS};
use swc_ecma_ast::{Module, Program};
use swc_ecma_compat_es2015::es2015;
use swc_ecma_transforms_base::{fixer::fixer, hygiene::hygiene};
use swc_ecma_transforms::resolver;
use swc_ecma_transforms_typescript::strip;
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

    // 2️⃣ strip TypeScript -> чистый JS AST
    let mut program = Program::Module(ast);
    program = program.apply(strip(unresolved, top_level));

    // 3️⃣ es2015 — Pass (теперь принимает 3 аргумента)
    program = program.apply(es2015(
        unresolved,       // Mark
        None::<SingleThreadedComments>, // нет комментариев
        Default::default() // конфиг
    ));

    // 4️⃣ hygiene и fixer — Fold
    program = program.apply(hygiene());
    program = program.apply(fixer(None));

    match program {
        Program::Module(module) => module,
        Program::Script(_) => unreachable!("нормализатор ожидает модуль"),
    }
}
