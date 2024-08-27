fn main(){
  /*
  Ownership Rules:
  1. Each value in rust has a variable that's called its owner.
  2. There can only be one owner at a time.
  3. When the owner goes out of scope, the value will be dropped.
  */

  { // s is not valid here, because it is not yet declared
    let s = String::from("Hello"); // s is valid from this point forward
    println!("Inside block: s = {}",s);
    // do stuff with s
  } // here, s goes out of scope and is dropped

  let x = 5;
  let y = x; //copy
  println!("x = {}\ny = {}", x,y);
  // This will create a copy of x value in y

  let s1 = String::from("Hello");
  /*
  let s2 = s1;  // the reference pointing to "Hello" will be moved from s1 to s2
  println!("{}", s1);
  ~~~~~~~~~~~~~~~~~~~~~~
   -- value moved here
  21 |   println!("{}", s1);
     |                  ^^ value borrowed here after move
  */

  let s2 = s1.clone();
  println!("s1 = {}\ns2 = {}",s1,s2);

  let s3 = String::from("New String");
  
  //takes_ownership(s3);
  
  // here the value s3 is refering to is moved to some_string
  // the scope of some_string is inside the function
  // when the function is over the scope of some_string is dropped
  // println!("s = {}", s3);
  // this will throw error because s3 is moved to takes_onwership function

  // solution
  takes_ownership(s3.clone());
  println!("s = {}", s3); // original reference not lost

  let val = 5;
  makes_copy(val);
  // since integers are not moved, we can still use val after the function call
  println!("val = {}", val);

  // giving ownership
  let s4 = gives_ownership();
  println!("s4 = {}", s4);
  // The s variable refering to "Hello" string inside the fucntion
  // after the return statement the ownership of "Hello" came to s4

  let mut s5 = String::from("I am a string");
  s5 = takes_and_gives_back(s5);
  println!("s5 = {}", s5);
}

fn takes_ownership(some_string: String){
  println!("{}", some_string);
}

fn makes_copy(n: i32){
  println!("{}", n);
}

fn gives_ownership() -> String{
  let s = String::from("Hello");
  s
}

fn takes_and_gives_back(a_string: String) -> String{
  a_string
}