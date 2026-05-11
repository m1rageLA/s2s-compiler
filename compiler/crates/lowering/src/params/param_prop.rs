use ir::IrParam;
use swc_ecma_ast::{self as ast};

use super::{binding, pat};

pub(crate) fn from_param(param: &ast::ParamOrTsParamProp) -> IrParam {
    match param {
        ast::ParamOrTsParamProp::Param(param) => pat::from_pat(&param.pat),
        ast::ParamOrTsParamProp::TsParamProp(prop) => from_ts_param_prop(prop),
    }
}

fn from_ts_param_prop(prop: &ast::TsParamProp) -> IrParam {
    match &prop.param {
        ast::TsParamPropParam::Ident(binding_ident) => binding::from_binding(binding_ident),
        ast::TsParamPropParam::Assign(assign) => pat::from_pat(&assign.left),
    }
}
