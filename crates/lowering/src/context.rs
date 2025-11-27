use ir::IrType;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

thread_local! {
    static TYPE_STACK: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(Vec::new());
    static RETURN_STACK: RefCell<Vec<IrType>> = RefCell::new(Vec::new());
    static MUTATED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

pub(crate) fn mark_mutated(name: &str) {
    MUTATED.with(|m| {
        m.borrow_mut().insert(name.into());
    });
}
pub(crate) fn is_mutated(name: &str) -> bool {
    MUTATED.with(|m| m.borrow().contains(name))
}

pub(crate) fn reset() {
    TYPE_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    RETURN_STACK.with(|stack| stack.borrow_mut().clear());
    MUTATED.with(|set| set.borrow_mut().clear());
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

pub(crate) fn push_function_return(ty: IrType) {
    RETURN_STACK.with(|stack| {
        stack.borrow_mut().push(ty);
    });
}

pub(crate) fn pop_function_return() {
    RETURN_STACK.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.pop();
    });
}

pub(crate) fn current_function_return() -> Option<IrType> {
    RETURN_STACK.with(|stack| stack.borrow().last().copied())
}
