fn main() {
  let a: &i32;
  {
    // b lifetime is not same as a
    let b = 3; 
    a = &b;
    println!("{}",b);
  }  
// dscdffh_fgygh_dftw_sgh_dfhdf
  println!("{}",a);
}
