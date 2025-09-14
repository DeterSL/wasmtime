use std::{collections::HashMap, any::{TypeId, Any}};

use wasmtime::{EventHandler, FilterFn};

#[derive(Default)]
pub struct EventHandlerImpl {
    map: HashMap<TypeId, Vec<FilterFn>>,
}

impl EventHandlerImpl {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
}

impl EventHandler for EventHandlerImpl {
    fn register(&mut self, ty: TypeId, f: FilterFn) {
        self.map.entry(ty).or_default().push(f);
    }

    fn accepts(&self, ev: &dyn Any) -> anyhow::Result<bool> {
        let id = ev.type_id();
        let vec = match self.map.get(&id) {
            Some(v) => v,
            None => return Ok(false),
        };

        for f in vec {
            if f(ev) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

pub struct NoopHandler;
impl EventHandler for NoopHandler {
    #[inline(always)]
    fn register(&mut self, _ty: TypeId, _f: FilterFn) {}
    #[inline(always)]
    fn accepts(&self, _ev: &dyn Any) -> anyhow::Result<bool> { Ok(true) }
}
