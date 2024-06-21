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
    // imprime o tabuleiro e algumas informações do jogo na tela
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

    // verifica se venceu após jogar na posição recebida e retorna bool
    fn venceu(&self, l: usize, c: usize) -> bool {
        // quantidade de peças alinhadas da mesma cor
        let mut cont = 1;

        //vertical |
        for i in 1..=3 {
            if l as i32 - (i as i32) < 0 { break; }
            if self.tabuleiro[c][l-i] != self.vez_de { break; }
            cont += 1;
        }
        for i in 1..=3 {
            if l + i >= 6 { break; }
            if self.tabuleiro[c][l+i] != self.vez_de { break; }
            cont += 1;
        }
        
        if cont >= 4 {
            return true;
        }

        //horizontal -
        cont = 1;

        for i in 1..=3 {
            if c as i32 - (i as i32) < 0 { break; }
            if self.tabuleiro[c-i][l] != self.vez_de { break; }
            cont += 1;
        }
        for i in 1..=3 {
            if c + i >= 7 { break; }
            if self.tabuleiro[c+i][l] != self.vez_de { break; }
            cont += 1;
        }

        if cont >= 4 {
            return true;
        }

        // diagonal \
        cont = 1;

        for i in 1..=3 {
            if l as i32 - (i as i32) < 0 || c as i32 - (i as i32) < 0 { break; }
            if self.tabuleiro[c-i][l-i] != self.vez_de { break; }
            cont += 1;
        }
        for i in 1..=3 {
            if l + i >= 6 || c + i >= 7 { break; }
            if self.tabuleiro[c+i][l+i] != self.vez_de { break; }
            cont += 1;
        }

        if cont >= 4 {
            return true;
        }

        // diagonal /
        cont = 1;

        for i in 1..=3 {
            if l as i32 - (i as i32) < 0 ||  c + i >= 7 { break; }
            if self.tabuleiro[c+i][l-i] != self.vez_de { break; }
            cont += 1;
        }
        for i in 1..=3 {
            if l + i >= 6 || c as i32 - (i as i32) < 0 { break; }
            if self.tabuleiro[c-i][l+i] != self.vez_de { break; }
            cont += 1;
        }

        if cont >= 4 {
            return true;
        }

        false
    }

    // tenta adicionar uma peça na ao tebuleiro na posição recebida. retorna se conseguiu e se ganhou
    fn adicionar_peça(&mut self, posição: usize) -> (bool, bool) {
        if posição < 7 {
            for i in (0..6).rev() {
                if self.tabuleiro[posição][i] == EspaçoTabuleiro::Vazio {
                    // adicionando a peça ao tabuleiro
                    self.tabuleiro[posição][i] = self.vez_de;

                    if self.venceu(i, posição) {
                        return (true, true);
                    }

                    return (true, false);
                }
            }
        }
        (false, false)
    }

    // recebe input do jogador e retorna a posição que ele escolheu
    fn receber_input() -> usize {
        let mut input_text = String::new();
            
        print!("Digite uma posição para jogar: ");
        io::Write::flush(&mut io::stdout()).expect("flush failed!");
        io::stdin().read_line(&mut input_text).expect("failed to read from stdin");
        let posição: usize = input_text.trim().parse().expect("Invalid input");
        posição -1
    }

    // recebe input do jogador, faz a jogada e troca a vez do jogador. retorna "true" caso um jogador tenha vencido
    fn jogar(&mut self) -> bool {
        loop {
            let posição: usize = Jogo::receber_input();

            let resultado = self.adicionar_peça(posição);
            // se conseguiu adicionar a peça
            if resultado.0 {
                // se venceu
                if resultado.1 {
                    return true;
                }

                // trocando a vez para o outro jogador
                if self.vez_de == EspaçoTabuleiro::Jogador1 { self.vez_de = EspaçoTabuleiro::Jogador2 }
                else { self.vez_de = EspaçoTabuleiro::Jogador1 }

                break;
            }
            println!("Posição inválida!");
        }
        false
    }

    // começa um novo jogo
    fn começar(&mut self) {
        loop {
            self.print_tabuleiro();

            if self.jogar() {
                // se alguém venceu
                self.print_tabuleiro();
                println!("Jogador {} venceu!!", self.vez_de.to_char());
                break;
            }
        }
    }
}

fn main() {
    let mut jogo = Jogo::novo();
    
    jogo.começar();
}
