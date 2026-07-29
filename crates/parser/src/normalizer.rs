use logger::Logger;
use swc_common::GLOBALS;
use swc_common::{Mark, comments::SingleThreadedComments};
use swc_ecma_ast::{EsVersion, Module, Pass, Program};
use swc_ecma_preset_env::transform_from_es_version;
use swc_ecma_transforms_base::assumptions::Assumptions;


pub fn normalizer(ast: Module) -> Module {
    let normalized = GLOBALS.set(&Default::default(), || {
        let mut program: Program = Program::Module(ast);
        let unresolved_mark = Mark::new();
        let mut pass = transform_from_es_version(
            unresolved_mark,
            None::<SingleThreadedComments>,
            EsVersion::Es5,
            Assumptions::default(),
            false,
        );
        pass.process(&mut program);
        let module = match program {
            Program::Module(module) => module,
            Program::Script(_) => unreachable!(),
        };
        module
    });
    
    Logger::step("normalize ast", "parser");
    normalized
}
