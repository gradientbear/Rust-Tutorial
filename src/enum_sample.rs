// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory {
   Male,Female
}
pub fn enum1() {
   let male = GenderCategory::Male;
   let female = GenderCategory::Female;

   println!("{:?}",male);
   println!("{:?}",female);
}



#[derive(Debug)]
struct Person {
  name: String,
  gender: GenderCategory
}

pub fn enum2() {
  let p1 = Person {
    name:String::from("Mohtashim"),
    gender:GenderCategory::Male
  };
  let p2 = Person {
    name:String::from("Amy"),
    gender:GenderCategory::Female
  };
  println!("{:?}",p1);
  println!("{:?}",p2);
}



pub fn enum3() {
  let result = is_even(3);
  println!("{:?}",result);
  println!("{:?}",is_even(30));
}

fn is_even(no:i32) -> Option<bool> {
  if no%2 == 0 {
    Some(true)
  } else {
    None
  }
}



enum CarType {
  Hatch,
  Sedan,
  SUV
}

fn print_size(car:CarType) {
  match car {
    CarType::Hatch => {
      println!("Small sized car");
    },
    CarType::Sedan => {
      println!("medium sized car");
    },
    CarType::SUV => {
      println!("Large sized Sports Utility car");
    }
  }
}

pub fn enum4() {
  print_size(CarType::SUV);
  print_size(CarType::Hatch);
  print_size(CarType::Sedan);
}



pub fn enum5() {
  match is_even(5) {
    Some(data) => {
      if data==true {
        println!("Even no");
      }
    },
    None => {
      println!("not even");
    }
  }
}



// The `derive` attribute automatically creates the implementation
// required to make this `enum` printable with `fmt::Debug`.
#[derive(Debug)]
enum GenderCategory1 {
  Name(String),Usr_ID(i32)
}
pub fn enum6() {
  let p1 = GenderCategory1::Name(String::from("Mohtashim"));
  let p2 = GenderCategory1::Usr_ID(100);
  println!("{:?}",p1);
  println!("{:?}",p2);

  match p1 {
    GenderCategory1::Name(val)=> {
      println!("{}",val);
    }
    GenderCategory1::Usr_ID(val)=> {
      println!("{}",val);
    }
  }
}
