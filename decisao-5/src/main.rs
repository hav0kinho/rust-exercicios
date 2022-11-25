use std::io;
fn main() {
    /*Faça um programa para a leitura de duas notas parciais de um aluno. O programa deve calcular a média
     alcançada por aluno e apresentar:

    A mensagem "Aprovado", se a média alcançada for maior ou igual a sete;
    A mensagem "Reprovado", se a média for menor do que sete;
    A mensagem "Aprovado com Distinção", se a média for igual a dez.
    */
    
    let mut nota1 = String::new();
    let mut nota2 = String::new();

    println!("Digite a nota 1: ");
    io::stdin().read_line(&mut nota1).expect("Digite um número válido");

    println!("Digite a nota 2: ");
    io::stdin().read_line(&mut nota2).expect("Digite um número válido");

    let nota1: f64 = nota1.trim().parse().expect("Falha na conversão dos valores");
    let nota2: f64 = nota2.trim().parse().expect("Falha na conversão dos valores");

    let media = calcular_media(&nota1, &nota2);

    println!("Nota 1: {nota1}");
    println!("Nota 2: {nota2}");

    println!("\nMédia: {media}");
    if media == 10.0{
        println!("Aprovado com distinção");
    } else if media < 7.0{
        println!("Reprovado");
    } else if media >= 7.0{
        println!("Aprovado");
    }

}

fn calcular_media(nota1: &f64, nota2: &f64) -> f64{
    let media = (nota1 + nota2) / 2.0;
    return media;
}
