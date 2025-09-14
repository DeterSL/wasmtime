use core::any::TypeId;

use alloc::boxed::Box;

/// HERE Keyvan
pub type FilterFn = Box<dyn Fn(&dyn std::any::Any) -> bool + Send + Sync>;

/// HERE Keyvan
pub trait EventHandler {
    /// Register a type-erased filter for the given TypeId.
    fn register(&mut self, _ty: TypeId, _f: FilterFn) {}

    /// Check acceptance by passing a type-erased event reference.
    fn accepts(&self, _ev: &dyn std::any::Any) -> anyhow::Result<bool> { Ok(true) }
}
