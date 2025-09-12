use crate::rust_second_tutor::structs_data::struct_main;

mod rust_second_tutor;

mod ownership_sample;
mod borrowing_sample;
mod slices_sample;
mod enum_sample;
mod module_sample;
mod other_sample;

fn main(){
  // structs_data
  struct_main();

  // let mut s = String::from("hello");
  // println!("len = {}, cap = {}", s.len(), s.capacity());

  // Second Tutor
  // rust_second_tutor::guessing_game::guessing_game();


  // // ownership
  // ownership_sample::fn1();
  // ownership_sample::fn2();

  // // borrowing
  // borrowing_sample::brw1();
  // borrowing_sample::brw2();
  // borrowing_sample::brw3();

  // // slice
  // slices_sample::slices1();
  // slices_sample::slices2();
  // slices_sample::slices3();

  // // enum
  // enum_sample::enum1();
  // enum_sample::enum2();
  // enum_sample::enum3();
  // enum_sample::enum4();
  // enum_sample::enum5();
  // enum_sample::enum6();

  // // others
  // other_sample::others();

}
