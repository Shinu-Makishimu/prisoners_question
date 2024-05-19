use rand::prelude::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;

pub(crate) struct PrisonNumBox {
    pub(crate) boxes: BTreeMap<usize, u32>,
}

impl PrisonNumBox {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let mut numbers: Vec<u32> = (1..=100).collect();
        numbers.shuffle(&mut rng);
        let boxes = (0..100).map(|index| (index + 1, numbers[index])).collect();
        PrisonNumBox { boxes }
    }
}
