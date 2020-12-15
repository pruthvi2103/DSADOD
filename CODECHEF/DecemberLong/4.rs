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
        let K = scan.next::<usize>();
        let mut count = N-K;
        let mut thresh=N+2;
        let mut kc =3;
        let mut first=true;
        if count == N{
            thresh=1;
        }
        else if K <= N/2{
            let difference = (N/2) - (K);
            if N%2 == 0{
                thresh = N;

            }else{
                thresh = N-1;
            } 
            thresh -= difference*2;
        }
            let tempz=1;
            if(tempz>=thresh){
                print!("{}",-1*(tempz as i32));
                count -=1;
                
            }
            else if tempz==kc && count > 0{
                print!("{}",-1*(tempz as i32));
                kc+=2;
                count -=1;
            }
            else{
                print!("{}",tempz);
            };
        for z in 2..N+1{
            if(z>=thresh){
                print!(" {}",-1*(z as i32));
                count -=1;
                
            }
            else if z==kc && count > 0{
                print!(" {}",-1*(z as i32));
                kc+=2;
                count -=1;
            }
            else{
                print!(" {}",z);
            };
            
        }


        print!("\n");
        }

        
    }
    // 1 2 -3 4 -5 6 -7 8 -9 10
    // K = 6
    // K= 5
    // difference = 0
    // K =3
    // 1 2 -3 4 -5 6 -7 8 -9 10

    // 1 2 -3 4 -5 6 -7 8 -9 10 -11
    // K = 3 