use ir::{IrParam, IrType};
use swc_ecma_ast::{self as ast};

use crate::types::ts_type_ann_to_ir;

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
        pat_to_ir(self)
    }
}

impl<'a> ParamLower for &'a ast::Param {
    fn lower_param(self) -> IrParam {
        pat_to_ir(&self.pat)
    }
}

impl<'a> ParamLower for &'a ast::ParamOrTsParamProp {
    fn lower_param(self) -> IrParam {
        match self {
            ast::ParamOrTsParamProp::Param(param) => pat_to_ir(&param.pat),
            ast::ParamOrTsParamProp::TsParamProp(prop) => ts_param_prop_to_ir(prop),
        }
    }
}

fn pat_to_ir(pat: &ast::Pat) -> IrParam {
    match pat {
        ast::Pat::Ident(binding) => binding_ident_to_ir(binding),
        ast::Pat::Assign(assign) => pat_to_ir(&assign.left),
        ast::Pat::Rest(rest) => pat_to_ir(&rest.arg),
        _ => unsupported_param(),
    }
}

fn ts_param_prop_to_ir(prop: &ast::TsParamProp) -> IrParam {
    match &prop.param {
        ast::TsParamPropParam::Ident(binding) => binding_ident_to_ir(binding),
        ast::TsParamPropParam::Assign(assign) => pat_to_ir(&assign.left),
    }
}

fn binding_ident_to_ir(binding: &ast::BindingIdent) -> IrParam {
    let name = binding.id.sym.to_string();
    let ty = binding
        .type_ann
        .as_ref()
        .map(|ann| ts_type_ann_to_ir(ann))
        .unwrap_or(IrType::Any);

    IrParam { name, ty }
}

fn unsupported_param() -> IrParam {
    IrParam {
        name: "unsupported_param".to_string(),
        ty: IrType::Any,
    }
}
