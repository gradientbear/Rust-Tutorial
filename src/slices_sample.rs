pub fn slices1 () {
  let n1 = "Tutorials".to_string();
  println!("length of string is {}", n1.len());
  let c1 = &n1[4..9];
  println!("{}", c1);
}

pub fn slices2(){
   let data = [10,20,30,40,50];
   use_slice(&data[1..4]);
   //this is effectively borrowing elements for a while
}

fn use_slice(slice:&[i32]) { 
   // is taking a slice or borrowing a part of an array of i32s
   println!("length of slice is {:?}",slice.len());
   println!("{:?}",slice);
}

pub fn slices3(){
  let mut data = [10,20,30,40,50];
  use_slice2(&mut data[1..4]);
  // passes references of 20, 30 and 40
  println!("{:?}",data);
}
fn use_slice2(slice:&mut [i32]) {
  println!("length of slice is {:?}",slice.len());
  println!("{:?}",slice);
  slice[0] = 1010; // replaces 20 with 1010
}

