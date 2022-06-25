// basic thread usage:
use std::sync::Arc;
use std::thread;
use std::thread::Builder;

pub fn basic_thread_usage() {
    let thread_result = thread::spawn(|| {
        println!("thread!");
        "Much concurrent, such wow!".to_string()
    });

    let status = thread_result.join();
    println!("Status = {:?}", status);
}

pub fn customize_thread_usage() {
    let my_thread = Builder::new()
        .name("Worker Thread".to_string())
        .stack_size(1024 * 4);

    let thread_result = my_thread.spawn(|| {
        panic!("Oops!");
    });

    let status = thread_result.unwrap().join();
    println!("Status = {:?}", status);
}

pub fn thread_usage_arc_usage() {
    let nums = Arc::new(vec![0, 1, 2, 3, 4]);
    let mut childs = vec![];

    for index in 0..nums.len() {
        let ns = Arc::clone(&nums);
        let c = thread::spawn(move || {
            println!("The index is ({})!", ns[index]);
        });
        childs.push(c);
    }

    for c in childs {
        c.join().unwrap();
    }
}
#[test]
#[ignore]
fn test_basic_thread_usage() {
    basic_thread_usage();
}

#[test]
#[ignore]
fn test_customized_thread_usage() {
    customize_thread_usage();
}

#[test]
fn test_thread_arc_usage() {
    thread_usage_arc_usage();
}
