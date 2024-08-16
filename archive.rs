fn main() {
  let a: &i32;
  {
    // b lifetime is not same as a
    let b = 3; 
    a = &b;
    println!("{}",b);
  }  
// dsw_sgh_df hufsfg_dd
  println!("{}",a);
}
