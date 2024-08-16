fn main() {
  let a: &i32;
  {
    // b lifetime is not same as a
    let b = 3; 
    a = &b;
    println!("{}",b);
  }  
// dscdfgyghdftw_sgh_dfhdf
  println!("{}",a);
}
