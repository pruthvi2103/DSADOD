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
            if ops <=0{break;}
            if A[i]!=0{}
            else{
            for mos in i+1..N-1{
                if A[mos]==0{
                    i=mos;
                }
                else{
                    i=mos;
                    break;
                }
                if mos==N-1{
                    i=N-2;
                    j=N-1;
                    break;
                }
            }
        }
        if A[i+1]!=0{
        j=i+1;
        }
        else{
            for mos in j..N{
                if A[j]==0{
                    j=mos;
                }
                else{
                    j=mos;
                    break;
                }
            }
        }

        //println!("i:{} j:{} N:{} X:{} ops:{} A:{:?}",i,j,N,X,ops,A);
        let o = A[i].count_ones();
        //println!("{}",o);
        if(o<= ops){
            ops-=o;
            p=A[i];
            //let xtemp=j;
            if A[j]^p > A[j]{
                for zoop in j..N{
                    if A[zoop]^p > A[zoop]{
                        j=zoop;
                    }else{
                        j=zoop;
                        break;
                    }
                }
            }
            A[i]=A[i]^p;
            A[j]=A[j]^p;
            //j=xtemp;
        }
        else{
            let diff = o-ops;
            p = A[i]>>diff;
            p = p <<diff;
            //println!("{}",p);
            //p = u32::pow(2,p);
            //let xtemp=j;
            if A[j]^p > A[j]{
                for zoop in j..N{
                    if A[zoop]^p > A[zoop]{
                        j=zoop;
                    }else{
                        j=zoop;
                        break;
                    }
                }
            }
            A[i]=A[i]^p;
            A[j]=A[j]^p;
            //j=xtemp;
            ops-=ops;
        }
        if i==N-2 && j==N-1{
            break;}
        }
        if ops!=0{
            if N>2{

            }
            else{
                if ops%2 ==0{
                    ops=0;
                }
                else{
                    A[i]=A[i]^1;
                    A[j]=A[j]^1;
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