use ir::{IrParam, IrType};

pub(crate) fn make() -> IrParam {
    IrParam {
        name: "unsupported_param".to_string(),
        ty: IrType::Any,
    }
}
