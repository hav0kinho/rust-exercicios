use std::io;
fn main() {
    //Faça um Programa que verifique se uma letra digitada é "F" ou "M". Conforme a letra escrever: F - Feminino, M - Masculino, Sexo Inválido.
    let mut sexo = String::new();
    
    println!("Digite o seu sexo [F/M]:");
    io::stdin().read_line(&mut sexo).expect("Houve algum problema na leitura.");

    if sexo.to_uppercase().trim() == "F"{
        println!("Seu sexo é feminino");
    } else if sexo.to_uppercase().trim() == "M" {
        println!("Seu sexo é masculino");
    } else {
        println!("Digite um sexo válido!");
    }

}
