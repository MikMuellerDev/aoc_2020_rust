use std::time;
mod day_1;
mod day_2;
mod day_3;

fn main() {
    let start = time::Instant::now();
    
    day_1::run();
    day_2::run();
    day_3::run();


    println!("\n<------------->\n Execution took {:?}", start.elapsed())
}
