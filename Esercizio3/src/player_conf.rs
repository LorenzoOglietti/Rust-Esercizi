/// Questo modulo contiene le strutture e le funzioni per la configurazione del gioco
/// e la gestione del giocatore.
pub mod player_conf {
    use rand::{random, Rng};

    /// Una cella del campo di gioco.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Cell {
        Empty,
        Food(i32),
        Poison(i32),
        Wall,
    }

    /// La struttura che rappresenta il giocatore.
    pub struct Player {
        pub strength: i32,
        pub position: (usize, usize),
        pub direction: Direction,
    }

    /// Le direzioni in cui il giocatore può muoversi.
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    /// La configurazione del campo di gioco.
    pub struct Configurazione {
        pub dimension: (usize, usize),
        pub campo: Vec<Vec<Cell>>,
        pub mosse: usize,
    }

    impl Configurazione {
        /// Crea una nuova configurazione del campo di gioco.
        ///
        /// # Argomenti
        ///
        /// * `dimensione` - La dimensione del campo di gioco (campo quadrato).
        /// * `mosse` - Il numero massimo di mosse.
        /// * `m` - Il numero di celle con cibo e veleno.
        ///
        /// # Ritorna
        ///
        /// Una tupla contenente la configurazione del campo di gioco e il giocatore.
        pub fn new(dimensione: usize, mosse: usize, m: usize) -> (Self, Player) {
            let mut matrix: Vec<Vec<Cell>> = vec![vec![Cell::Empty; dimensione]; dimensione];
            let mut rng = rand::thread_rng();
            let randx: usize = rng.gen_range(1..dimensione - 1);
            let randy: usize = rng.gen_range(1..dimensione - 1);

            // Inserisci muri
            for i in 0..dimensione {
                for j in 0..dimensione {
                    if i == 0 || i == dimensione - 1 || j == 0 || j == dimensione - 1 {
                        matrix[i][j] = Cell::Wall;
                    }
                }
            }

            // Inserisci cibo e veleno
            let mut foodn = 0;
            let mut poisonn = 0;
            while foodn < m || poisonn < m {
                let randxint: usize = rng.gen_range(1..dimensione - 1);
                let randyint: usize = rng.gen_range(1..dimensione - 1);
                if matrix[randxint][randyint] == Cell::Empty {
                    if foodn < m {
                        matrix[randxint][randyint] = Cell::Food(10);
                        foodn += 1;
                    } else if poisonn < m {
                        matrix[randxint][randyint] = Cell::Poison(10);
                        poisonn += 1;
                    }
                }
            }

            let player = Player::new((randx, randy), 0);
            let config = Self {
                dimension: (dimensione, dimensione),
                campo: matrix,
                mosse,
            };
            (config, player)
        }

        /// Stampa la configurazione del campo di gioco e le informazioni del giocatore.
        ///
        /// # Argomenti
        ///
        /// * `player` - Il giocatore attuale.
        pub fn print(&self, player: &Player) {
            let mut campo_str = String::new();
            for (i, riga) in self.campo.iter().enumerate() {
                for (j, cella) in riga.iter().enumerate() {
                    let simbol = if (i, j) == player.position {
                        'P'  // Posizione del giocatore
                    } else {
                        match cella {
                            Cell::Empty => '0',
                            Cell::Wall => 'X',
                            Cell::Food(..) => 'F',
                            Cell::Poison(..) => 'V',
                        }
                    };
                    campo_str.push(simbol);
                    campo_str.push(' ');
                }
                campo_str.push('\n');
            }
            println!(
                "Campo:\n{}\nGiocatore:\nDirezione: {:?}\nForza: {}\nPosizione: {:?}\nMosse rimaste: {}",
                campo_str, player.direction, player.strength, player.position, self.mosse
            );
        }
    }

    impl Player {
        /// Crea un nuovo giocatore.
        ///
        /// # Argomenti
        ///
        /// * `position` - La posizione iniziale del giocatore.
        /// * `strength` - La forza iniziale del giocatore.
        ///
        /// # Ritorna
        ///
        /// Un nuovo giocatore.
        pub fn new(position: (usize, usize), strength: i32) -> Self {
            let mut rng = rand::thread_rng();
            let direction = match rng.gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Right,
                _ => Direction::Left,
            };
            Self {
                direction,
                position,
                strength,
            }
        }

        /// Muove il giocatore nella direzione specificata.
        ///
        /// # Argomenti
        ///
        /// * `input_dir` - La direzione in cui muovere il giocatore.
        /// * `conf` - La configurazione del campo di gioco.
        ///
        /// # Ritorna
        ///
        /// Un intero che rappresenta lo stato del gioco dopo il movimento:
        /// - `-1` se il gioco è finito per perdita di forza,
        /// - `1` se il giocatore ha vinto,
        /// - `0` altrimenti.
        pub fn move_player(&mut self, input_dir: Direction, conf: &mut Configurazione) -> i32 {
            let coin = random::<bool>();
            if coin == true {
                self.direction = input_dir;
                match input_dir {
                    Direction::Right => {
                        if conf.campo[self.position.0][self.position.1 + 1] != Cell::Wall {
                            self.update_position((self.position.0, self.position.1 + 1), conf);
                        } else {
                            self.direction = Direction::Left;
                        }
                    }
                    Direction::Left => {
                        if conf.campo[self.position.0][self.position.1 - 1] != Cell::Wall {
                            self.update_position((self.position.0, self.position.1 - 1), conf);
                        } else {
                            self.direction = Direction::Right;
                        }
                    }
                    Direction::Down => {
                        if conf.campo[self.position.0 + 1][self.position.1] != Cell::Wall {
                            self.update_position((self.position.0 + 1, self.position.1), conf);
                        } else {
                            self.direction = Direction::Up;
                        }
                    }
                    Direction::Up => {
                        if conf.campo[self.position.0 - 1][self.position.1] != Cell::Wall {
                            self.update_position((self.position.0 - 1, self.position.1), conf);
                        } else {
                            self.direction = Direction::Down;
                        }
                    }
                }
            } else {
                self.random_move(conf);
            }

            if self.check_gameover() {
                return -1;
            }

            if self.check_win(conf) {
                return 1;
            }

            0
        }

        fn update_position(&mut self, new_position: (usize, usize), conf: &mut Configurazione) {
            match conf.campo[new_position.0][new_position.1] {
                Cell::Food(val) => {
                    self.strength += val;
                    conf.campo[new_position.0][new_position.1] = Cell::Empty;
                }
                Cell::Poison(val) => {
                    self.strength -= val;
                    conf.campo[new_position.0][new_position.1] = Cell::Empty;
                }
                _ => (),
            }
            self.position = new_position;
            conf.mosse -= 1;
        }

        fn random_move(&mut self, conf: &mut Configurazione) {
            let mut rng = rand::thread_rng();
            let direction = match rng.gen_range(0..4) {
                0 => Direction::Up,
                1 => Direction::Down,
                2 => Direction::Right,
                _ => Direction::Left,
            };

            match direction {
                Direction::Right => {
                    if conf.campo[self.position.0][self.position.1 + 1] != Cell::Wall {
                        self.update_position((self.position.0, self.position.1 + 1), conf);
                    } else {
                        self.direction = Direction::Left;
                    }
                }
                Direction::Left => {
                    if conf.campo[self.position.0][self.position.1 - 1] != Cell::Wall {
                        self.update_position((self.position.0, self.position.1 - 1), conf);
                    } else {
                        self.direction = Direction::Right;
                    }
                }
                Direction::Down => {
                    if conf.campo[self.position.0 + 1][self.position.1] != Cell::Wall {
                        self.update_position((self.position.0 + 1, self.position.1), conf);
                    } else {
                        self.direction = Direction::Up;
                    }
                }
                Direction::Up => {
                    if conf.campo[self.position.0 - 1][self.position.1] != Cell::Wall {
                        self.update_position((self.position.0 - 1, self.position.1), conf);
                    } else {
                        self.direction = Direction::Down;
                    }
                }
            }
        }

        fn check_gameover(&self) -> bool {
            self.strength < 0
        }

        fn check_win(&self, conf: &Configurazione) -> bool {
            conf.mosse <= 0
        }
    }
}
