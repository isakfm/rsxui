//! Context management for component rendering.

use std::{
    any::{Any, TypeId},
    cell::RefCell,
    collections::HashMap,
};

tokio::task_local! {
    static CONTEXT: RefCell<HashMap<TypeId, Box<dyn Any>>>;
}

pub fn provide_context<T: 'static>(value: T) {
    CONTEXT.with(|context| {
        context
            .borrow_mut()
            .insert(TypeId::of::<T>(), Box::new(value));
    });
}

pub fn use_context<T>() -> Option<T>
where
    T: Clone + 'static,
{
    CONTEXT.with(|context| {
        context
            .borrow()
            .get(&TypeId::of::<T>())
            .and_then(|any| {
                any.downcast_ref::<T>().cloned()
            })
    })
}

pub fn expect_context<T>() -> T
where
    T: Clone + 'static,
{
    use_context::<T>()
        .unwrap_or_else(|| panic!("Context not found for type: {}", std::any::type_name::<T>()))
}
