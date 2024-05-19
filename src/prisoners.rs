use std::collections::BTreeMap;

pub(crate) struct Prisoners {
    pub(crate) instances: BTreeMap<usize, bool>,
}

impl Prisoners {
    pub(crate) fn new() -> Self {
        let instances = (1..=100).map(|x| (x, false)).collect();
        Prisoners { instances }
    }
    pub(crate) fn acquitted(&mut self, number: &usize) -> Result<(), &'static str> {
        if let Some(prisoner) = self.instances.get_mut(number) {
            *prisoner = true;
            Ok(())
        } else {
            Err("out of prisoner")
        }
    }

    fn _get_value(&self, key: &usize) -> Option<&bool> {
        self.instances.get(key)
    }
}
