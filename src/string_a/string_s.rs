pub fn init() {
  let s1 = String::from("abcabcabc");
  let s2 = String::from("abcabcabcabc");
  test(s1,s2);
}

fn test (s1:String,s2:String)  -> String{
  if format!("{}{}", s1, s2) == format!("{}{}", s2, s1) {
    fn gcd(a:usize, b:usize) -> usize{
      if b == 0 {
        a
      } else {
        gcd(b, a % b)
      }
    }
    let len = gcd(s1.len(),s2.len());
    s1[0..len].to_string()
  } else {
    String::new()
  }
  
}
