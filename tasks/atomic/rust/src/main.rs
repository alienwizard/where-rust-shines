extern crate crossbeam;

fn main() {
    let thread_count = 10;
    let increments_per_thread = 100000;
    let i = 0;
    crossbeam::scope(|scope| {
        for _ in 0..thread_count {
            scope.spawn(|| {
                            for _ in 0..increments_per_thread {
                                //i+=1;
                            }
                        });
        }
    });
    println!("Result of {}*{} increments: {}",
             thread_count,
             increments_per_thread,
             i);
}
