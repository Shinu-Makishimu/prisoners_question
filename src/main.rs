mod prison;
mod prisoners;

use async_std::task;

use crate::prison::PrisonNumBox;
use crate::prisoners::Prisoners;
use async_std::task::JoinHandle;
use std::sync::{Arc, Mutex};

const COUNTS: f32 = 1000f32;

fn main() {
    let success_count = 0;
    let fail_count = 0;

    let success_count = Arc::new(Mutex::new(success_count)); // обертываем переменные в Arc и Mutex...
    let fail_count = Arc::new(Mutex::new(fail_count));

    let mut handle_vec = vec![];
    async_check_run(&success_count, &fail_count, &mut handle_vec);
    let _results = async_std::task::block_on(async {
        let res = futures::future::join_all(handle_vec).await;
        res
    });

    let final_success_count = *success_count.lock().unwrap();
    let final_fail_count = *fail_count.lock().unwrap();

    let probability_of_success = final_success_count as f32 / COUNTS;

    println!("Success: {}", final_success_count);
    println!("Failure: {}", final_fail_count);
    println!("Probability of Success: {}", probability_of_success);
}

fn async_check_run(
    success_count: &Arc<Mutex<i32>>,
    fail_count: &Arc<Mutex<i32>>,
    handle_vec: &mut Vec<JoinHandle<()>>,
) {
    for _ in 0..COUNTS as u32 {
        let prison_num_box = PrisonNumBox::new();
        let mut prisoners = Prisoners::new();
        let prisoner_entries: Vec<(usize, bool)> =
            prisoners.instances.iter().map(|(k, v)| (*k, *v)).collect();

        let success_count = Arc::clone(&success_count);
        let fail_count = Arc::clone(&fail_count);

        let handle = task::spawn(async move {
            //активируем асинхронные ветки
            prisoner_going_to_check(prison_num_box, &mut prisoners, prisoner_entries);
            if prisoners.instances.values().any(|&acquitted| !acquitted) {
                *fail_count.lock().unwrap() += 1;
            } else {
                *success_count.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle);
    }
}

fn prisoner_going_to_check(
    prison_num_box: PrisonNumBox,
    prisoners: &mut Prisoners,
    prisoner_entries: Vec<(usize, bool)>,
) {
    for (number, _is_acquitted) in prisoner_entries {
        let mut number_from_box = prison_num_box.boxes.get(&number).unwrap();

        for _ in 1..=50 {
            //println!("Prisoner number: {}, number from box: {}", number, number_from_box);
            if *number_from_box as usize == number {
                prisoners.acquitted(&number).expect("can't change prisoner");
                break;
            } else {
                number_from_box = prison_num_box
                    .boxes
                    .get(&(*number_from_box as usize))
                    .unwrap();
            }
        }
        //let result = prisoners.get_value(&number).unwrap();
        //println!("Prisoner number: {}, result: {}", number, result);
    }
}
