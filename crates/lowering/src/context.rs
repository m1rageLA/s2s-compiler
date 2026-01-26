use ir::{IrType, IrTypeAlias, IrTypeAliasDef};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

thread_local! {
    static TYPE_STACK: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(Vec::new());
    static RETURN_STACK: RefCell<Vec<IrType>> = RefCell::new(Vec::new());
    static MUTATED: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
    static FN_RETURNS: RefCell<Vec<HashMap<String, IrType>>> = RefCell::new(Vec::new());
    static TYPE_ALIASES: RefCell<Vec<HashMap<String, IrTypeAlias>>> = RefCell::new(Vec::new());
    static TYPE_ALIAS_COUNTER: RefCell<u32> = RefCell::new(0);
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
    FN_RETURNS.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    RETURN_STACK.with(|stack| stack.borrow_mut().clear());
    MUTATED.with(|set| set.borrow_mut().clear());
    TYPE_ALIASES.with(|stack| {
        let mut stack = stack.borrow_mut();
        stack.clear();
        stack.push(HashMap::new());
    });
    TYPE_ALIAS_COUNTER.with(|counter| *counter.borrow_mut() = 0);
}

pub(crate) fn push_scope() {
    TYPE_STACK.with(|stack| {
        stack.borrow_mut().push(HashMap::new());
    });
    FN_RETURNS.with(|stack| {
        stack.borrow_mut().push(HashMap::new());
    });
    TYPE_ALIASES.with(|stack| {
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
    FN_RETURNS.with(|stack| {
        let mut stack = stack.borrow_mut();
        if stack.len() > 1 {
            stack.pop();
        }
    });
    TYPE_ALIASES.with(|stack| {
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

pub(crate) fn define_type_alias(name: &str, def: IrTypeAliasDef) -> IrTypeAlias {
    let id = TYPE_ALIAS_COUNTER.with(|counter| {
        let mut value = counter.borrow_mut();
        *value += 1;
        *value
    });

    let alias = IrTypeAlias {
        id,
        name: name.to_string(),
        def,
    };

    TYPE_ALIASES.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), alias.clone());
        }
    });

    alias
}

pub(crate) fn lookup_type_alias(name: &str) -> Option<IrTypeAlias> {
    TYPE_ALIASES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            if let Some(alias) = scope.get(name) {
                return Some(alias.clone());
            }
        }
        None
    })
}

pub(crate) fn lookup_type_alias_by_id(id: u32) -> Option<IrTypeAlias> {
    TYPE_ALIASES.with(|stack| {
        for scope in stack.borrow().iter().rev() {
            for alias in scope.values() {
                if alias.id == id {
                    return Some(alias.clone());
                }
            }
        }
        None
    })
}

pub(crate) fn define_function_return(name: &str, ty: IrType) {
    FN_RETURNS.with(|stack| {
        if let Some(scope) = stack.borrow_mut().last_mut() {
            scope.insert(name.to_string(), ty);
        }
    });
}

pub(crate) fn lookup_function_return(name: &str) -> Option<IrType> {
    FN_RETURNS.with(|stack| {
        for scope in stack.borrow().iter().rev() {
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
