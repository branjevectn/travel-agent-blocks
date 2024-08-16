fn main() {
  let a: &i32;
  {
    // b lifetime is not same as a
    let b = 3; 
    a = &b;
    println!("{}",b);
  }  
// cgj_sfgxf_gidfcd_dgghegbg_egbss_wfg
  println!("{}",a);
}
