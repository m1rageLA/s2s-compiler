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
