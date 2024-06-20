use std::io;

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
        println!(" 1 2 3 4 5 6 7 ");

        for i in 0..6 {
            for j in 0..7 {
                print!("{}", self.tabuleiro[j][i].to_char());
            }
            println!();
        }
        println!("\nVez de: {}", self.vez_de.to_char());
    }

    // tenta adicionar uma peça na ao tebuleiro na posição recebida. retorna se conseguiu ou não
    fn adicionar_peça(&mut self, posição: usize) -> bool {
        if posição < 7 {
            for i in (0..6).rev() {
                if self.tabuleiro[posição][i] == EspaçoTabuleiro::Vazio {
                    // adicionando a peça ao tabuleiro
                    self.tabuleiro[posição][i] = self.vez_de;

                    return true;
                }
            }
        }
        false
    }

    // recebe input do jogador, faz a jogada e troca a vez do jogador
    fn jogar(&mut self) {
        loop {
            let mut input_text = String::new();
            
            print!("Digite uma posição para jogar: ");
            io::Write::flush(&mut io::stdout()).expect("flush failed!");
            io::stdin().read_line(&mut input_text).expect("failed to read from stdin");

            let posição: usize = input_text.trim().parse().expect("Invalid input");

            if self.adicionar_peça(posição-1) {
                // trocando a vez para o outro jogador
                if self.vez_de == EspaçoTabuleiro::Jogador1 { self.vez_de = EspaçoTabuleiro::Jogador2 }
                else { self.vez_de = EspaçoTabuleiro::Jogador1 }

                break;
            }
            println!("Posição inválida!");
        }
    }

    // começa um novo jogo
    fn começar(&mut self) {
        loop {
            self.print_tabuleiro();
            self.jogar();
        }
    }
}

fn main() {
    let mut jogo = Jogo::novo();
    
    jogo.começar();
}
