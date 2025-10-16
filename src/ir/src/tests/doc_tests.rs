use crate::doc::ir_docs;

#[test]
fn ir_docs_lists_ir_item_variants() {
    let docs = ir_docs();

    let ir_item = docs
        .iter()
        .find(|node| node.name == "IrItem")
        .expect("IrItem should be documented");

    assert_eq!(ir_item.kind, "enum");

    let variant_names: Vec<&str> = ir_item.variants.iter().map(|variant| variant.name.as_str()).collect();

    assert!(variant_names.contains(&"Variable"), "IrItem enum should document Variable variant");
    assert!(variant_names.contains(&"Function"), "IrItem enum should document Function variant");
}

