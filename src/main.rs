use std::fmt::Display;
use std::thread;
use std::time::Duration;

mod ownership_sample;
mod borrowing_sample;
mod slices_sample;
mod enum_sample;
mod module_sample;

use module_sample::movies::play;

fn main(){
  // ownership
  ownership_sample::fn1();
  ownership_sample::fn2();

  // borrowing
  borrowing_sample::brw1();
  borrowing_sample::brw2();
  borrowing_sample::brw3();

  // slice
  slices_sample::slices1();
  slices_sample::slices2();
  slices_sample::slices3();

  // enum
  enum_sample::enum1();
  enum_sample::enum2();
  enum_sample::enum3();
  enum_sample::enum4();
  enum_sample::enum5();
  enum_sample::enum6();

  // module
  play("Herold and Kumar ".to_string());

  // Error Handling
  print_pro(10 as u8);
  print_pro(20 as u16);
  print_pro("Hello TutorialsPoint");

  // Input & Output
  let cmd_line = std::env::args();
  println!("No of elements in arguments is :{}",cmd_line.len()); 
  //print total number of values passed
  for arg in cmd_line {
    println!("[{}]",arg); //print all values passed as commandline arguments
  }

  // Smart Pointers
  let var_i32 = 5; 
  //stack
  let b = Box::new(var_i32); 
  //heap
  println!("b = {}, var_i32 = {}", b, var_i32);

  let x = 5; 
  //value type variable
  let y = Box::new(x); 
  //y points to a new value 5 in the heap

  println!("{}",5==x);
  println!("{}",5==*y); 
  //dereferencing y

  // Concurrency
  let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
        }
    });

    // Main thread work
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}

fn print_pro<T:Display>(t:T) {
  println!("Inside print_pro generic function:");
  println!("{}",t);
}
