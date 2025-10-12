fn fib (n : i128) -> i128 { if (n) <= (1) { return n ; } return ((fib) ((n) - (1))) + ((fib) ((n) - (2))) ; }
let input : i128 = 50 ;
runtime :: console :: log (vec ! [runtime :: console :: stringify (& (format ! ("{}" , runtime :: console :: stringify (& ((fib) (input))))))]) ;

let tymur = "tymur";