pub fn fn1 () {
    let v = vec![1,2,3];     // vector v owns the object in heap
    let v2 = v;              // moves ownership to v2
    display1(v2);             // v2 is moved to display and v2 is invalidated
    // println!("In main {:?}",v2);    //v2 is No longer usable here
}

fn display1(v:Vec<i32>){
   println!("inside display {:?}",v);
}

pub fn fn2(){
   let v = vec![1,2,3];       // vector v owns the object in heap
   let v2 = v;                // moves ownership to v2
   let v2_return = display2(v2);    
   println!("In main {:?}",v2_return);
}

fn display2(v:Vec<i32>)->Vec<i32> { 
   // returning same vector
   println!("inside display {:?}",v);
   v
}