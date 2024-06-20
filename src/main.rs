#[derive(Clone, Copy)]
enum EspaçoTabuleiro {
    Vazio,
    Jogador1,
    Jogador2,
}

impl EspaçoTabuleiro {
    fn print_espaço(&self) {
        match self {
            EspaçoTabuleiro::Vazio => print!("⚫"),
            EspaçoTabuleiro::Jogador1 => print!("🔵"),
            EspaçoTabuleiro::Jogador2 => print!("🔴"),
        }
    }
}

struct Jogo {
    //tabuleiro com 6 linhas e 7 colunas, começando da ponta superior esquerda
    tabuleiro: [[EspaçoTabuleiro; 6]; 7],
    vez_de: EspaçoTabuleiro,
}

impl Jogo {
    fn novo() -> Self {
        Self {
            tabuleiro: [[EspaçoTabuleiro::Vazio; 6]; 7],
            vez_de: EspaçoTabuleiro::Jogador1,
        }
    }

    fn print_tabuleiro(&self) {
        for i in 0..6 {
            for j in 0..7 {
                self.tabuleiro[j][i].print_espaço();
            }
            println!();
        }
    }
}

fn main() {
    let mut jogo = Jogo::novo();
    
    jogo.print_tabuleiro();
}
