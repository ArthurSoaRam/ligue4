use game::Jogo;

mod game;

fn main() {
    let mut jogo = Jogo::novo();
    
    jogo.começar();
}
