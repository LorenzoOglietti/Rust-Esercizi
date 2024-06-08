/// Questo modulo implementa un Conto Bancario con diversi stati (Rosso, Argento, Oro)
/// e le operazioni di deposito, prelievo e pagamento degli interessi, usando il pattern State.
pub mod conto_bancario {
    /// La struttura ContoBancario rappresenta un conto bancario con diverse proprietà e stati.
    pub struct ContoBancario {
        state: Option<Box<dyn State>>,
        customer_name: String,
        total: i64,
        max_limit: i64,
        min_limit: i64,
        interest: i64,
    }

    impl ContoBancario {
        /// Crea un nuovo conto bancario con il nome del cliente fornito.
        ///
        /// # Parametri
        ///
        /// * `name` - Una stringa che rappresenta il nome del cliente.

        pub fn new(name: String) -> ContoBancario {
            let mut  conto = ContoBancario {
                state: Some(Box::new(Argento {})),
                customer_name: name,
                total: 0,
                max_limit: 10000,
                min_limit: -10000,
                interest: 5,
            };
            conto.update_state();
            conto
        }

        /// Deposita una quantità nel saldo del conto bancario.
        ///
        /// # Parametri
        ///
        /// * `qty` - Un i64 che rappresenta la quantità da depositare.
        pub fn deposita(&mut self, qty: i64) {
            self.total += qty;
            self.update_state();
        }

        /// Restituisce il saldo corrente del conto bancario.
        ///
        /// # Ritorna
        ///
        /// Un i64 che rappresenta il saldo corrente.

        pub fn vedi_saldo(&self) -> i64 {
            return self.total;
        }

        /// Preleva una quantità dal saldo del conto bancario.
        ///
        /// # Parametri
        ///
        /// * `qty` - Un i64 che rappresenta la quantità da prelevare.

        pub fn preleva(&mut self, qty: i64) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.preleva(self, qty));
            }
            self.update_state()
        }

        /// Paga gli interessi sul saldo del conto bancario.

        pub fn paga_interessi(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = Some(state.paga_interessi(self));
            }
            self.update_state()
        }

        /// Aggiorna lo stato del conto bancario in base al saldo corrente.

        pub fn update_state(&mut self) {
            if let Some(state) = self.state.take() {
                self.state = if self.total < self.min_limit {
                    Some(Box::new(Rosso {}))
                } else if self.total > self.max_limit {
                    Some(Box::new(Oro {}))
                } else {
                    Some(Box::new(Argento {}))
                };
            }
        }
    }

    /// Trait che rappresenta lo stato del conto bancario.
    trait State {
        fn preleva(&self, conto: &mut ContoBancario, qty: i64) -> Box<dyn State>;
        fn paga_interessi(&self, conto: &mut ContoBancario) -> Box<dyn State>;
    }

    /// Struttura che rappresenta lo stato Rosso del conto bancario.
    struct Rosso {}

    impl State for Rosso {
        /// Implementazione del metodo di prelievo per lo stato Rosso.
        ///
        /// # Parametri
        ///
        /// * `conto` - Un riferimento mutabile a ContoBancario.
        /// * `qty` - Un i64 che rappresenta la quantità da prelevare.
        ///
        /// # Ritorna
        ///
        /// Una nuova istanza di stato.
        fn preleva(&self, _conto: &mut ContoBancario, _qty: i64) -> Box<dyn State> {
            println!("Il conto è in rosso, non puoi prelevare");
            Box::new(Rosso {})
        }

        /// Implementazione del metodo di pagamento degli interessi per lo stato Rosso.
        ///
        /// # Parametri
        ///
        /// * `conto` - Un riferimento mutabile a ContoBancario.
        ///
        /// # Ritorna
        ///
        /// Una nuova istanza di stato.
        fn paga_interessi(&self, _conto: &mut ContoBancario) -> Box<dyn State> {
            println!("Il conto è in rosso, non ti vengono pagati gli interessi");
            Box::new(Rosso {})
        }
    }

    /// Struttura che rappresenta lo stato Argento del conto bancario.
    struct Argento {}

    impl State for Argento {
        /// Implementazione del metodo di prelievo per lo stato Argento.
        ///
        /// # Parametri
        ///
        /// * `conto` - Un riferimento mutabile a ContoBancario.
        /// * `qty` - Un i64 che rappresenta la quantità da prelevare.
        ///
        /// # Ritorna
        ///
        /// Una nuova istanza di stato.
        fn preleva(&self, conto: &mut ContoBancario, qty: i64) -> Box<dyn State> {
            let newtotal = conto.total - qty;
            conto.total = newtotal;
            if newtotal < conto.min_limit {
                Box::new(Rosso {})
            } else if newtotal > conto.max_limit{
                Box::new(Oro {})
            } else {
                Box::new(Argento {})
            }
        }

        /// Implementazione del metodo di pagamento degli interessi per lo stato Argento.
        ///
        /// # Parametri
        ///
        /// * `conto` - Un riferimento mutabile a ContoBancario.
        ///
        /// # Ritorna
        ///
        /// Una nuova istanza di stato.
        fn paga_interessi(&self, conto: &mut ContoBancario) -> Box<dyn State> {
            let newtotal = conto.total + (conto.total * conto.interest) / 100;
            conto.total = newtotal;
            if newtotal < conto.min_limit {
                Box::new(Rosso {})
            } else if newtotal > conto.max_limit {
                Box::new(Oro {})
            } else {
                Box::new(Argento {})
            }
        }
    }

    /// Struttura che rappresenta lo stato Oro del conto bancario.
    struct Oro {}

    impl State for Oro {
        /// Implementazione del metodo di prelievo per lo stato Oro.
        ///
        /// # Parametri
        ///
        /// * `conto` - Un riferimento mutabile a ContoBancario.
        /// * `qty` - Un i64 che rappresenta la quantità da prelevare.
        ///
        /// # Ritorna
        ///
        /// Una nuova istanza di stato.
        fn preleva(&self, conto: &mut ContoBancario, qty: i64) -> Box<dyn State> {
            let newtotal = conto.total - qty;
            conto.total = newtotal;
            if newtotal < conto.min_limit {
                Box::new(Rosso {})
            } else if newtotal <= conto.max_limit && newtotal >= conto.min_limit {
                Box::new(Argento {})
            } else {
                Box::new(Oro {})
            }
        }

        /// Implementazione del metodo di pagamento degli interessi per lo stato Oro.
        ///
        /// # Parametri
        ///
        /// * `conto` - Un riferimento mutabile a ContoBancario.
        ///
        /// # Ritorna
        ///
        /// Una nuova istanza di stato.
        fn paga_interessi(&self, conto: &mut ContoBancario) -> Box<dyn State> {
            conto.total += (conto.total * conto.interest) / 100;
            Box::new(Oro {})
        }
    }
}
