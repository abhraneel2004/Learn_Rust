fn main(){
  let num = 5;
  // the condition must be boolean [no integer value accepted]
  if num<5 {
    println!("Statement 1");
  }
  else if num>5 {
    println!("Statement 2");
  }
  else{
    println!("Statement 3");
  }

  let cond = true;

  let x = if cond{ 7} else{8};
  println!("x = {}",x);
}