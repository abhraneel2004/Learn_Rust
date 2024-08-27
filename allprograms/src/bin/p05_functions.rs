fn main(){
  myfunc();
  my_func2(10, 20, "Abhraneel");
  let res = my_func3(3, 4);
  println!("The result is {}\nThala for a reason",res);
}

fn myfunc(){
  println!("Another function");
}

fn my_func2(p1: i32, p2: u16, x: &str){
  println!("p1 = {}, p2 = {}, x = {}",p1,p2,x);
}

fn my_func3(p1: i32, p2: i32) -> i32 {
  // function with return type
  println!("The value of p1 = {}", p1);
  println!("\nThe value of p2 = {}", p2);
  let sum = p1+p2;
  // implicitly the last line of the function becomes the return statement
  sum // works similar to return sum 
  // we dont have to give semicolon in last line
}