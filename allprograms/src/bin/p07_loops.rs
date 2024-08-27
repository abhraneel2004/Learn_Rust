fn main(){
  // LOOPS
  // Type 1
  /*
  loop:
  this loop will execute until it reaches a break statement
  the syntax:
  */
  println!("loop\n");
  let mut counter = 0;
  loop {
      println!("count = {}", counter);
      counter += 1;
      if counter == 5 {
          break;
      }
  }
  // Type 2
  /*
  while:
  The while loop executes a block of code as long as a specified condition is true.
  */
  println!("---------------");
  println!("\nwhile loop\n");
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number -= 1;
  }
  println!("Out of the loop");
  // Type 3
  /*
  for:
  The for loop iterates over a sequence of elements, executing the code block for each element.
  */
  println!("---------------");
  println!("for loop\n");
  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
      println!("the value is: {}", element);
  }

  println!("------------");
  for number in (1..4).rev() {
    println!("{}!", number);
  }

  println!("---------------");
  for num in 1..4  {
    println!("num = {}", num);
  }

  println!("---------------");
  for n in 1..=4 {
    println!("n = {}", n);
  }
  println!("---------------");
  //loop with continue
  let mut count = 0;
  loop {
      count += 1;
      if count % 2 == 0 {
          continue; // Skip even numbers
      }
      println!("{}", count);
      if count == 9 {
          break; // Exit the loop after printing 9
      }
  }
}