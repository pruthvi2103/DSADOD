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
        let N: usize = scan.next::<usize>();
        let mut A: Vec<u32> = (0..N).map(|_| scan.next()).collect();
        let mut min=0;
        let mut max=N-1;
        let mut mid =N/2;
        //let mut B:Vec<u32> = A;
        let mut ctr =0;
        print!("{}",A[min]);
        min+=1;
        while min <= max{
            //println!("min:{} max:{}",min,max);
            if max > mid{
            print!(" {}",A[max]);
            }
            if min <= mid{

            print!(" {}",A[min]);
            }
            min+=1;
            max-=1;

        }
        print!("\n");
       
    

    }
}