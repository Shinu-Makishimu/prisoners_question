use std::collections::BTreeMap;
use rand::seq::SliceRandom;
use rand::thread_rng;



struct PrisonNumBox {
    boxes: BTreeMap<usize, u32>,
}

impl PrisonNumBox {
    fn new() -> Self {
        let mut rng = thread_rng();
        let mut numbers: Vec<u32> = (1..=100).collect();
        numbers.shuffle(&mut rng);
        let boxes = (0..100).map(|index| (index + 1, numbers[index])).collect();
        PrisonNumBox { boxes }
    }
}

struct Prisoners {
    instances: BTreeMap<usize, bool>,
}

impl Prisoners {
    fn new() -> Self {
        let instances = (1..=100).map(|x| (x, false)).collect();
        Prisoners{instances}
    }
    fn acquitted(&mut self, number: &usize) -> Result<(), &'static str> {
        if let Some(prisoner) = self.instances.get_mut(number) {
            *prisoner = true;
            Ok(())
        } else {
            Err("out of prisoner")
        }
    }

    fn get_value(&self, key: &usize) -> Option<&bool> {
        self.instances.get(key)
    }
}


fn main(){
    let prison_num_box = PrisonNumBox::new();
    let mut prisoners = Prisoners::new();
    let prisoner_entries : Vec<(usize,bool)> = prisoners.instances.iter().map(|(k,v)| (*k,*v)).collect();

    prisoner_going_to_check(prison_num_box, &mut prisoners, prisoner_entries);


    for (prisoner, acquitted) in &prisoners.instances {
        println!("Prisoner number: {}, Acquitted: {}", prisoner, acquitted);
    }


    if prisoners.instances.values().any(|&acquitted| !acquitted) {
        println!("Fail");
    } else {
        println!("Success");
    }
}

fn prisoner_going_to_check(prison_num_box: PrisonNumBox, prisoners: &mut Prisoners, prisoner_entries: Vec<(usize, bool)>) {
    for (number, _is_acquitted) in prisoner_entries {
        let mut number_from_box = prison_num_box.boxes.get(&number).unwrap();

        for _ in 1..=50 {
            //println!("Prisoner number: {}, number from box: {}", number, number_from_box);
            if *number_from_box as usize == number {
                prisoners.acquitted(&number).expect("can't change prisoner");
                break
            } else {
                number_from_box = prison_num_box.boxes.get(&(*number_from_box as usize)).unwrap();
            }
        }
        let result = prisoners.get_value(&number).unwrap();
        println!("Prisoner number: {}, result: {}", number, result);
    }
}
