
/// Modulo che fornisce strutture per contare il numero di vocali in una stringa.
pub mod count_vocali {
    /// Struttura che rappresenta il numero di vocali in una stringa, utilizzando un struct.
    #[derive(PartialEq, Debug)]
    pub struct NumVocali {
        /// Numero di occorrenze della vocale 'a'.
        pub a: i32,
        /// Numero di occorrenze della vocale 'e'.
        pub e: i32,
        /// Numero di occorrenze della vocale 'i'.
        pub i: i32,
        /// Numero di occorrenze della vocale 'o'.
        pub o: i32,
        /// Numero di occorrenze della vocale 'u'.
        pub u: i32,
    }

    impl NumVocali {
        /// Conta il numero di vocali in una stringa e restituisce una struttura `NumVocali`.
        ///
        /// **Input:**
        /// - `s`: Riferimento alla stringa di cui contare le vocali.
        ///
        /// **Valore di ritorno:** Una struttura `NumVocali` contenente il numero di occorrenze di ciascuna vocale.
        pub fn num_vocali_struct(s: &String) -> NumVocali {
            let vector = s.chars().fold((0,0,0,0,0), |count, c| {count_vocali(&count, &c)});
            NumVocali {
                a: vector.0,
                e: vector.1,
                i: vector.2,
                o: vector.3,
                u: vector.4,
            }
        }
    }
    /// Struttura che rappresenta il numero di vocali in una stringa, utilizzando una tupla.
    #[derive(PartialEq, Debug)]
    pub struct TuplaVocali(pub i32, pub i32, pub i32, pub i32, pub i32);

    impl TuplaVocali {
        /// Conta il numero di vocali in una stringa e restituisce una tupla `TuplaVocali`.
        ///
        /// **Input:**
        /// - `s`: Riferimento alla stringa di cui contare le vocali.
        ///
        /// **Valore di ritorno:** Una tupla `TuplaVocali` contenente il numero di occorrenze di ciascuna vocale.
        pub fn num_vocali_tuple(s: &String) -> TuplaVocali {
            let vector = s.chars().fold((0,0,0,0,0), |count, c| {count_vocali(&count, &c)});
            TuplaVocali(vector.0, vector.1, vector.2, vector.3, vector.4)
        }
    }
    /// Funzione ausiliaria per contare il numero di vocali in una stringa.
    fn count_vocali(counter: &(i32, i32, i32, i32, i32), car: &char) -> (i32, i32, i32, i32, i32) {
        match car.to_ascii_lowercase() {
            'a' => (counter.0+1, counter.1, counter.2, counter.3, counter.4),
            'e' => (counter.0, counter.1+1, counter.2, counter.3, counter.4),
            'i' => (counter.0, counter.1, counter.2+1, counter.3, counter.4),
            'o' => (counter.0, counter.1, counter.2, counter.3+1, counter.4),
            'u' => (counter.0, counter.1, counter.2, counter.3, counter.4+1),
            _ => *counter,
        }
    }
}
