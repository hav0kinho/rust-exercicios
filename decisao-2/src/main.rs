use std::{io};
fn main() {
    //Faça um Programa que peça um valor e mostre na tela se o valor é positivo ou negativo.
    let mut number = String::new();

    println!("Digite um número: ");
    io::stdin().read_line(&mut number).expect("Digite um número válido");

    let number: i32 = number.trim().parse().expect("O número não é válido");

    if number < 0 {
        println!("O número é negativo");
    } else {
        println!("O número é positivo");
    }
}
