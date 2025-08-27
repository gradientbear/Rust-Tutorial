use std::fmt::Display;
use std::thread;
use std::time::Duration;

use crate::module_sample::movies::play;

pub fn others () {
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
