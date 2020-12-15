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
    let N = scan.next::<usize>();
    for x in 0..N{
        let A = scan.next::<usize>();
        let B = scan.next::<usize>();
        let eA = A/2;
        let eB = B/2;
        let oA = if A%2 == 0 {A/2} else {(A/2)+1};
        let oB = if B%2 == 0 {B/2} else {(B/2)+1};
        let res = (eA*eB) + (oA*oB);
        println!("{}",res)
    }
}