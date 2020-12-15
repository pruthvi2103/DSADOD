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
        let mut ops= X;
        let mut i=0;
        let mut j=1;
        let mut p =0;
        loop{
            if ops<=0{ break;}
            if i>=N-1{break;}//put later
            let curr =i;
            for psn in curr..N-1{
                if A[psn] == 0{
                    i=psn;
                }
                else{
                    i=psn;
                    break;
                }
            }
            j=i+1;
            let o= A[i].count_ones();
            loop{
                if j==N-1{break;}
                if A[j]^A[i] > A[j]{
                    j+=1;
                }
                else{
                    break;
                }
            }
            if o<=ops{
                //print!("n:{}",j);
                let p=A[i];
                A[i]=A[i]^p;
                A[j]=A[j]^p;
                ops-=o;
            }
            else{
                if o<= ops{
                    ops-=o;
                    p=A[i];
                    A[i]=A[i]^p;
                    A[j]=A[j]^p;
                    ops-=1;
                }
            }
        }

       
        if ops!=0{
            if N>2{
            }
            else{
                if ops%2 ==0{
                    ops=0;
                    
                }
                else{
                    A[N-1]=A[N-1]^1;
                    A[N-2]=A[N-2]^1;
                }
            }
            
        }
        print!("{}",A[0]);
        for vekt in 1..N{
            print!(" {}",A[vekt]);
        }

print!("\n");
        
    }
}