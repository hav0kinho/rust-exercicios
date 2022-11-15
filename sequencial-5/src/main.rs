use std::io;
fn main() {
    let mut centimetros = String::new();

    println!("Digite um valor em centimétros para ser convertido para metros");
    io::stdin().read_line(&mut centimetros).expect("Você digitou um valor inválido");

    let centimetros: f32 = centimetros.trim().parse().expect("O valor digitado é inválido");
    let metros: f32 = centimetros / 100.0;

    println!("{centimetros}cm valem {metros}m!");

}
