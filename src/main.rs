#[derive(Clone, Copy, PartialEq)]
enum EspaçoTabuleiro {
    Vazio,
    Jogador1,
    Jogador2,
}

impl EspaçoTabuleiro {
    fn to_char(&self) -> char {
        match self {
            EspaçoTabuleiro::Vazio => '⚫',
            EspaçoTabuleiro::Jogador1 => '🔵',
            EspaçoTabuleiro::Jogador2 => '🔴',
        }
    }
}

struct Jogo {
    // tabuleiro com 6 linhas e 7 colunas, começando da ponta superior esquerda
    tabuleiro: [[EspaçoTabuleiro; 6]; 7],
    vez_de: EspaçoTabuleiro,
}

impl Jogo {
    // cria um novo jogo
    fn novo() -> Self {
        Self {
            tabuleiro: [[EspaçoTabuleiro::Vazio; 6]; 7],
            vez_de: EspaçoTabuleiro::Jogador1,
        }
    }

    fn print_tabuleiro(&self) {
        println!("===============");

        for i in 0..6 {
            for j in 0..7 {
                print!("{}", self.tabuleiro[j][i].to_char());
            }
            println!();
        }
        println!("\nVez de: {}", self.vez_de.to_char());
    }

    // tenta adicionar uma peça na ao tebuleiro na posição recebida e troca a vez do jogador. retorna se conseguiu ou não
    fn adicionar_peça(&mut self, posição: usize) -> bool {
        if posição >=0 && posição < 7 {
            for i in (0..6).rev() {
                if self.tabuleiro[posição][i] == EspaçoTabuleiro::Vazio {
                    // adicionando a peça ao tabuleiro
                    self.tabuleiro[posição][i] = self.vez_de;

                    // trocando a vez para o outro jogador
                    if self.vez_de == EspaçoTabuleiro::Jogador1 { self.vez_de = EspaçoTabuleiro::Jogador2 }
                    else { self.vez_de = EspaçoTabuleiro::Jogador1 }

                    return true;
                }
            }
        }
        false
    }

    fn jogar(&mut self) {
        todo!()
    }
}

fn main() {
    let mut jogo = Jogo::novo();
    
    jogo.print_tabuleiro();
    println!("{}", jogo.adicionar_peça(0));
    jogo.print_tabuleiro();
    println!("{}", jogo.adicionar_peça(0));
    jogo.print_tabuleiro();
}
