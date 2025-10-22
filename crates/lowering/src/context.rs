use ir::IrType;
use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    static TYPE_STACK: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(Vec::new());
}

pub(crate) fn reset() {
    TYPE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
}

pub(crate) fn push_scope() {
    TYPE_STACK.with(|stack| {
        stack.borrow_mut().push(HashMap::new());
    });
}

pub(crate) fn pop_scope() {
    TYPE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
}

pub(crate) fn define(name: &str, ty: IrType) {
    TYPE_STACK.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), ty);
        }
    });
}

pub(crate) fn lookup(name: &str) -> Option<IrType> {
    TYPE_STACK.with(|stack| {
        let stack = stack.borrow();
        for scope in stack.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(*ty);
            }
        }
        None
    })
}
