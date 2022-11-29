/*

Classe Quadrado: Crie uma classe que modele um quadrado:

Atributos: Tamanho do lado
Métodos: Mudar valor do Lado, Retornar valor do Lado e calcular Área;
*/



fn main() {
    let mut meu_quadrado = Quadrado{tamanho_do_lado: 8.5};
    println!("Quadrado: {}", meu_quadrado.tamanho_do_lado);
    

    println!("{}", meu_quadrado.calcular_area());

    meu_quadrado.mudar_valor_lado(12.0);

    println!("{} {}", meu_quadrado.calcular_area(), meu_quadrado.retorna_lado());
}

struct Quadrado{
    tamanho_do_lado: f64,
}

impl Quadrado{
    fn mudar_valor_lado(&mut self, lado: f64){
        self.tamanho_do_lado = lado;
    }

    fn retorna_lado(&self) -> f64{
        return self.tamanho_do_lado;
    }

    fn calcular_area(&self) -> f64{
        return self.tamanho_do_lado * self.tamanho_do_lado;
    }
}