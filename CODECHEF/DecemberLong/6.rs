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
    let T = scan.next::<usize>();
    for x in 0..T{
        let mut str:String = scan.next();
        let len =str.len();
        let mut mask:u128=1;
        let mut vec = Vec::new();
        for cnt in 0..len{
            if mask&1==1{
                vec.push(cnt);
            }
            mask=mask<<1;
            println!(" {} ",mask);

        }
        for idx in vec{
            print!(" {} ",idx);
        }
        

    }
}
//1101