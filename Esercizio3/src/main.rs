mod player_conf;

use std::io;
use std::io::Write;
use crate::player_conf::player_conf::{Configurazione, Direction}; // Per flush()

fn main() {
    let dimensione = 10; // Dimensione del campo di gioco
    let mosse = 20; // Numero massimo di mosse
    let m = 5; // Numero di celle con cibo e veleno

    let (mut configurazione, mut player) = Configurazione::new(dimensione, mosse, m);
    configurazione.print(&player);

    loop {
        println!("Inserisci direzione (WASD):");
        io::stdout().flush().unwrap(); // Assicurati che il testo venga stampato prima dell'input

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input_char = input.trim().chars().next().unwrap();

        let direction = match input_char {
            'w' | 'W' => Direction::Up,
            's' | 'S' => Direction::Down,
            'a' | 'A' => Direction::Left,
            'd' | 'D' => Direction::Right,
            _ => {
                println!("Input non valido!");
                continue;
            }
        };

        player.move_player(direction, &mut configurazione);
        configurazione.print(&player);


        if player.strength < 0 {
            println!("Game Over: Hai perso tutte le tue forze!");
            break;
        }

        if configurazione.mosse <= 0 {
            println!("Congratulazioni! Hai esaurito tutte le mosse!");
            break;
        }
    }

    println!("Gioco finito!");
}
