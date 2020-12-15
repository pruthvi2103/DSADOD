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
        let X = scan.next::<u32>();
        let mut A: Vec<u32> = (0..N).map(|_| scan.next()).collect();
        let mut ops = X;
        let mut i=0;
        let mut p=1;
        loop{
            if ops<=0{break;}
            
            for pos in i..N{
                if A[pos]==0{
                    i=pos;
                }
                else{
                    i=pos;
                    break;
                }
            }
            let mut j=i+1;
            if i==N-1{break;};
            let ltwo = log_2(A[i] as i32) as u32;
            p = 2_u32.pow(ltwo);
           // println!("ltow:{} p:{}",ltwo,p);
            if(A[j]^p > A[j]){
                for pos in j..N{
                    if A[pos]^p > A[pos]{
                        j=pos;
                    }
                    else{
                        j=pos;
                        break;
                    }
                }
            }
            //println!("i:{} j:{} N:{} X:{} ops:{} A:{:?} p:{}",i,j,N,X,ops,A,p);
            A[i] = A[i]^p;
            A[j] = A[j]^p;
            ops-=1;
        }
        if ops>0{
            if N<3{
                if ops%2 ==0{

                }
                else{
                    A[N-1]=A[N-1]^1;
                    A[N-2]=A[N-2]^1;
                }
            }
            else{

            };
        }
        print!("{}",A[0]);
        for elm in 1..N{
            print!(" {}",A[elm]);
        }
        print!("\n");
    }

}