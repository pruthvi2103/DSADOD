N = 10;
K=5;
kc=0
nk=3;
a=Array.from({length: N}, (_, index) => {
    sum = index + 1
    if(sum == nk){
        nk+=2
        kc+=1
        return -sum;


    }
    return sum;
});


sum = 0;
b=Array.from({length: N}, (_, index) => { sum=a[index]+sum; return sum;} );
kount=0
for(let i =0;i<N;i++){
    if(b[i] >0){
        kount+=1
    }
}
console.dir(a,{'maxArrayLength': null});
console.dir(b,{'maxArrayLength': null});
console.log("Count of K:",N-kc);
console.log("Cald'd K:",kount);

