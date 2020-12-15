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
fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

fn log_2(x: i32) -> u32 {
    assert!(x > 0);
    num_bits::<i32>() as u32 - x.leading_zeros() - 1
}


fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let T = scan.next::<usize>();
    for x in 0..T{
        let N = scan.next::<usize>();
        let X = scan.next::<usize>();
        let mut A: Vec<u32> = (0..N).map(|_| scan.next()).collect();
        let mut ops= X;
        let mut i=0;
        let mut j=1;
        let mut p =0;
        loop{
            if ops==0{
                break;
            }
            else{
                if j+1 == N{
                }
                else{
                   if A[i] == 0 && A[j] == 0 {
                       if j+2 ==N{
                        i=j;
                        j=j+1;
                       }
                       else{
                       i=j+1;
                       j=j+2;
                       }
                   }
                   else if A[i] == 0{
                    i=j;
                    j=j+1;
                   }
                   else if A[j] ==0{
                    j+=1;
                   }

                }
                if A[i] !=0{
                    p = log_2(A[i] as i32);
                    //println!("{}",p);
                    p = u32::pow(2,p);
                    //println!("{}",p);
                   }
                   else{
                    p = log_2(A[j] as i32);
                    //println!("{}",p);
                    p = u32::pow(2,p);
                    //println!("{}",p);
                   }
                   A[i] = A[i] ^ p;
                   A[j] = A[j] ^ p;

                println!("N : {} X : {} ops: {} i: {} j: {} p: {} \n",N,X,ops,i,j,p);
                println!("{} {}",A[i],A[j]);
                println!("{:?}",A);

            }
            ops-=1;
        }
        print!("{}",A[0]);
        for q in 1..N{
            print!(" {}",A[q]);
        }
        print!("\n");
    }
}