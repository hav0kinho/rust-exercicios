use std::io;
fn main() {
    let mut nota1: String = String::new();
    let mut nota2: String = String::new();
    let mut nota3: String = String::new();
    let mut nota4: String = String::new();

    println!("Digite a nota 1: ");
    io::stdin().read_line(&mut nota1).expect("Aconteceu algum problema ao ler o valor");

    println!("Digite a nota 2: ");
    io::stdin().read_line(&mut nota2).expect("Aconteceu algum problema ao ler o valor");

    println!("Digite a nota 3: ");
    io::stdin().read_line(&mut nota3).expect("Aconteceu algum problema ao ler o valor");

    println!("Digite a nota 4: ");
    io::stdin().read_line(&mut nota4).expect("Aconteceu algum problema ao ler o valor");

    let nota1: f32 = nota1.trim().parse().expect("O valor digitado não é válido");
    let nota2: f32 = nota2.trim().parse().expect("O valor digitado não é válido");
    let nota3: f32 = nota3.trim().parse().expect("Valor digitado não é válido");
    let nota4: f32 = nota4.trim().parse().expect("O valor digitado não é válido");

    let media: f32 = (nota1 + nota2 + nota3 + nota4) / 4.0;

    println!("A media das notas digitadas é: {media}");

}
