use std::io;

//Faça um Programa que calcule a área de um quadrado, em seguida mostre o dobro desta área para o usuário.
fn main() {
    let mut lado_quadrado = String::new();

    println!("Digite o valor do lado do quadrado: ");
    io::stdin().read_line(&mut lado_quadrado).expect("O valor digitado é inválido");

    let lado_quadrado: f32 = lado_quadrado.trim().parse().expect("Ocorreu um problema ao converter os valores");
    let area_quadrado: f32 = lado_quadrado.powi(2);
    let dobro_area_quadrado: f32 = area_quadrado * 2.0;

    println!("O lado do quadrado é: {lado_quadrado}m");
    println!("A area do quadrado é: {area_quadrado}m²");
    println!("O dobro da area do quadrado é: {dobro_area_quadrado}m²");
}
