use std::collections::BTreeMap;

pub struct Context {
    type_map: BTreeMap<String, String>,
}

impl Context {
    pub new() -> Self {
        Context {
            type_map: BTreeMap::new(),
        }
    }
}
