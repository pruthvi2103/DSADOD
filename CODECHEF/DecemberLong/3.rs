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
        let N = scan.next::<usize>();
        let D = scan.next::<usize>();
        let mut days=0;
        //let ages: Vec<usize> = (0..N).map(|_| scan.next()).collect();
        //let mut crit: Vec<usize> = Vec::new();
        //let mut noncrit: Vec<usize> = Vec::new();
        let mut crit = 0;
        let mut noncrit=0;
        for j in 0..N{
            let inp = scan.next::<usize>();
            if inp>=80 || inp <= 9{
                //crit.push(inp);
                crit+=1;
            }else{
                //noncrit.push(inp);
                noncrit+=1;
            };
        }
        days += if crit%D != 0 {(crit/D) +1} else {crit/D};
        days += if noncrit%D != 0 {(noncrit/D) +1} else {noncrit/D};
        println!("{}",days)

 
    }
}