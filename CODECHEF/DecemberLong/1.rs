#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write};
const BITS: usize = 19;
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}
fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    
    let D1 = scan.next::<usize>();
    let V1 = scan.next::<usize>();
    let D2 = scan.next::<usize>();
    let V2 = scan.next::<usize>();
    let P = scan.next::<usize>();
    let mut vac=0;
    let mut days=1;
    loop{
        if days >= D1{
            vac +=V1
        } 
        if days>= D2{
            vac+=V2
        }
        if P<=vac{
            break;
        }

        days +=1;
    }
    println!("{}",days);
}