pub fn brw1 () {
    let v = vec![10, 20, 30];
    print_vector1(&v);
    println!("Printing the value from main() v[0]={}", v[0]);
}

fn print_vector1(x : &Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}

pub fn brw2() {
    let mut i = 3;
    add_one(&mut i);
    println!("{}", i);
}

fn add_one(e : &mut i32) {
    *e += 1;
}

pub fn brw3 () {
    let mut name : String = String::from("TutorialsPoint");
    display(&mut name);
    println!("The value of name after modification is:{}", name);
}

fn display(param_name : &mut String) {
    println!("Param_name value is : {}", param_name);
    param_name.push_str(" Rocks");
}
