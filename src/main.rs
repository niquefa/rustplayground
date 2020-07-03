use std::time::{Duration, Instant};
use std::collections::HashSet;


fn next_permutation(element: &mut Vec<i32>) -> bool{
  let mut i:usize = element.len() - 1;
  while i > 0 && element[i - 1] >= element[i] {
    i-=1;
  }
  if i <= 0 {
    return false;
  }
  let mut j: usize = element.len() - 1;
  while element[j] <= element[i - 1] {
    j-=1;
  }
  let mut temp: i32 = element[i - 1];
  element[i - 1] = element[j];
  element[j] = temp;
  j = element.len() - 1;
  while i < j {
    temp = element[i];
    element[i] = element[j];
    element[j] = temp;
    i += 1;
    j -= 1;
  }
  return true;
}
const MAX:i32 = 9;
fn main() {
    for n in 1..MAX {
        let mut elements = Vec::with_capacity(n as usize);
        for i in 0..n {
            elements.push(i);
        }
        let mut all_permutations: HashSet<String> = HashSet::new();
        let now = Instant::now();
        let mut ok = true;
        loop {
            let mut str_elements = String::new();
            for i in 0..n {
                let tmp: i32 = elements[i as usize];
                let s: String = tmp.to_string();
                str_elements.push_str(&s);
            }
            all_permutations.insert(str_elements);
            for i in 0..n {
                ok = ok && 0 <= elements[i as usize] && elements[i as usize] < n;            
            }
            let go_on: bool = next_permutation(&mut elements);
            if !go_on {
                break;
            }
        }
        let mut f: i32 = 1;
        for i in 2..n+1 {
            f *= i;
        }
        if ! ok || all_permutations.len() as i32 != f {
            println!("\n\nSome big error occur\n\n");
            break;
        }        
        println!("{} ({}=?={}) {} seconds",n,all_permutations.len(),f,now.elapsed().as_secs());
    }
}