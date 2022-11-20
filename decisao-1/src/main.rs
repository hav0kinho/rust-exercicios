use std::{io};
fn main() {
    //Faça um Programa que peça dois números e imprima o maior deles.
    let mut number1 = String::new();
    let mut number2 = String::new(); 

    println!("Digite o primeiro número: ");
    io::stdin().read_line(&mut number1).expect("Digite um número válido");
    println!("Digite o segundo número: ");
    io::stdin().read_line(&mut number2).expect("Digite um número válido");

    let number1: f32 = number1.trim().parse::<f32>().expect("Algo deu errado na conversão");
    let number2: f32 = number2.trim().parse::<f32>().expect("Algo deu errado na conversão");

    if number1 > number2{
        println!("Numero 1 ({number1}) é maior que numero 2 ({number2})");
    } else if number2 > number1{
        println!("Numero 2 ({number2}) é maior que numero 1 ({number1})");
    } else {
        println!("Números são iguais");
    }


}
