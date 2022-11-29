//use std::io;

/*
Classe Bola: Crie uma classe que modele uma bola:

Atributos: Cor, circunferência, material
Métodos: trocaCor e mostraCor
*/

struct Bola{
    cor: String,
    circunferencia: f64,
    material: String,
}

impl Bola{
    fn trocar_cor(&mut self, nova_cor: &str){
        self.cor = nova_cor.to_ascii_lowercase();
    }

    fn mostra_cor(&self){
        println!("A cor dessa bola é {}", &self.cor);
    }
}

fn main() {
    let mut minha_bola = Bola{
        cor: "vermelho".to_string(),
        circunferencia: 55.5,
        material: "cerâmica".to_string()
    };


    println!("**Bola**\nCor: {}\nCircuferencia: {}\nMaterial: {}", minha_bola.cor, minha_bola.circunferencia, minha_bola.material);

    minha_bola.trocar_cor("rosa");
    minha_bola.mostra_cor();

    println!("\n\n");
    println!("**Bola**\nCor: {}\nCircuferencia: {}\nMaterial: {}", minha_bola.cor, minha_bola.circunferencia, minha_bola.material);
}
