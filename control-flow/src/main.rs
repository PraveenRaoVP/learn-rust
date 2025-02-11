use std::thread;
use std::time::Duration;

fn main() {
    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("Count: {count}");

    //     let mut remaining = 10;
    //     loop {
    //         println!("Remaining: {remaining}");
    //         if remaining == 9 {
    //             break;
    //         } 
    //         if count == 2 {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    
    // println!("End of counting at {count}");


    // for i in (1..=10).rev() {
    //     println!("{i}");
    //     // sleep for 100 ms
    //     thread::sleep(Duration::from_millis(1000));
    // }

    // println!("You are gay!");

    let mut i: i32 = 0;

    while i < 10 {
        println!("{i}");
        i += 1;
    }
}
