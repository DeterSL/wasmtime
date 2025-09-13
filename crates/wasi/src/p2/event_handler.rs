use std::{collections::HashMap, any::{TypeId, Any}};


type FilterFn = Box<dyn Fn(&dyn Any) -> bool + Send + Sync>;

#[derive(Default)]
pub struct EventHandler {
    map: HashMap<TypeId, Vec<FilterFn>>,
}

impl EventHandler {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }

    pub fn register<K: 'static>(&mut self, f: impl Fn(&K) -> bool + Send + Sync + 'static) {
        let wrapper: FilterFn = Box::new(move |any: &dyn Any| {
            any.downcast_ref::<K>().map_or(false, |k| f(k))
        });
        self.map.entry(TypeId::of::<K>()).or_default().push(wrapper);
    }

    pub fn accepts<K: 'static>(&self, ev: &K) -> anyhow::Result<bool> {
        let id = TypeId::of::<K>();
        let vec = match self.map.get(&id) {
            Some(v) => v,
            None => return Ok(false),
        };

        for f in vec {
            // propagate errors; if a filter returns Ok(true) return immediately
            if f(ev as &dyn Any) {
                return Ok(true);
            }
        }
        Ok(false)
    }
}
