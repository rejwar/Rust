fn main() {
    let MyNumbers:[i32;5] = [10,20,30,40,50];
    let FirstMonth:[char;3] = ['j' , 'k' , 'l'];

    let AllZeros:[i32; 100] = [0 ;100];


    println!("MyNumbers first element : {}", MyNumbers[0]);
    println!("First month as char : {} ,{}", FirstMonth[0], FirstMonth[1]);
    println!("First element of AllZeros : {}", AllZeros[0]);
    println!("Last elememt of Allzeros :L {}", AllZeros[99]);
}
