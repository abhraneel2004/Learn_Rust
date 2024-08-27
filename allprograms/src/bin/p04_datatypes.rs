use colored::*;
fn main(){
  // scalar datatypes - single value
  // compound datatypes - group of values


  // SCALAR DATATYPES- 
  println!("{}","\n\nScalar Data Types\n".green());
  // integer
  //    length signed  unsigned
  //    8-bit    i8     u8
  //    16-bit   i16    u16
  //    32-bit   i32    u32     (default)
  //    64-bit   i64    u64
  //    128-bit  i128   u128
  //    arch     isize  usize


  let a: i32 =   98_222; // Decimal
  let b: i32 = 0xfff; // Hexadecimal
  let c: i32 = 0o77; // Octal
  let d: i32 = 0b1111_0000; // Binary
  let e: u8 = b'A'; // Byte (u8 only)
  // let f: u8 = 300;
  // it will throw error
  println!("a = {}, b = {}, c = {}, d = {}, e = {}",a,b,c,d,e);

  // Floating point integer

  let f = 2.4;
  let g = 3.0;
  println!("f = {}, g = {}",f,g);
  // Arithmetic operations
  let h = 5 + 90; // addition
  let i = 95.0 - 90.0; // subtraction
  let j = 5 * 90; // multiplication
  let k = 5 / 2; // division
  // let k1 = 5.0/2; // error
  let l = 5 % 90; // remainder

  println!("\nh = {}, i = {}, j = {}, k = {}, l = {}",h,i,j,k,l);

  // Boolean

  let c= true;
  let d=false;
  println!("c = {}, d = {}",c,d);
  
  //Charaacter
  let c1 = 'z';
  let c2 = 'Z';
  let map_arrow = '⩤';

  println!("c1 = {}, c2 = {}, map_arrow = {}",c1,c2,map_arrow);


  // COMPOUND Types
  println!("{}","\n\nCompound Types\n".green());
  let tup = ("abhraneel art", 25);
  println!("tup = {:?}",tup);
  let (channel, count) = tup; // destructuring
  println!("channel = {}, count = {}",channel,count);
  // destructuring helps in extracting values from a tuple
  // destructuring can be done in a single line as well
  let (channel, count) = ("abhraneel art", 25); // destructuring + shadowing
  println!("channel = {}, count = {}",channel,count);

  // Arrays
  let error_codes = [404, 418, 429, 500];
  println!("error_codes = {:?}",error_codes);
  let not_found = error_codes[0];
  // let x= error_codes[10]; // index out of bounds exception
  println!("not_found = {}",not_found);

  let _byte = [0;8]; // array of 8 zeroes
  println!("_byte = {:?}",_byte);
}