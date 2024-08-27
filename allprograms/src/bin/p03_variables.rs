fn main(){
  // let x = 5; by default variables are immutable
  let mut x = 5; // making a variable mutable
  println!("The value of x is: {}",x);
  x = 7;
  println!("The value of x is: {}",x);

  const MAX: u32 = 10_000;
  // we can add _ to increase readability of large numbers
  println!("The value of MAX is: {}",MAX);
  // constant varables can be set to constant expression
  // we can not make constant variables mutable
  // constant variables are immutable by default
  // constant variables can not be iniatilized with a value which calculated at runtime

  let y = 5;
  println!("The value of y is: {}",y);
  let y = 7;
  println!("The value of y is: {}",y);
  // this is shadowing
  // we can not shadow a constant variable
  // shadowing enables us to mutate a variable which is already declared immutable
  
}