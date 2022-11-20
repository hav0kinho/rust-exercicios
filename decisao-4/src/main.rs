use std::io;
fn main() {
    //Faça um Programa que verifique se uma letra digitada é vogal ou consoante.
    let vogais = ['a', 'e', 'i', 'o', 'u'];
    let mut letra = String::new();

    io::stdin().read_line(&mut letra).expect("Houve um erro ao ler o valor!");


    if letra.contains(&vogais){
        println!("É Vogal")
    } else {
        println!("É Consoante");
    } 

}
