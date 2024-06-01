/// Un modulo per la gestione dei numeri razionali.
pub mod razionale {
    use std::ops::{Add, Mul};

    /// Una struttura che rappresenta un numero razionale.
    #[derive(Clone)]
    pub struct Razionale {
        pub num: i32,
        pub denum: i32,
    }

    impl Razionale {
        /// Crea un nuovo numero razionale.
        ///
        /// # Argomenti
        ///
        /// * `a` - Numeratore.
        /// * `b` - Denominatore (non può essere 0).
        ///
        /// # Panics
        ///
        /// Panica se `b` è 0.
        pub fn new(a: i32, b: i32) -> Razionale {
            assert!(b != 0, "Denominatore non può essere 0");
            Razionale {
                num: a,
                denum: b,
            }
        }

        /// Converte un intero in un numero razionale.
        ///
        /// # Argomenti
        ///
        /// * `a` - Intero da convertire.

        pub fn int_to_raz(a: i32) -> Razionale {
            Razionale::new(a, 1)
        }

        /// Calcola il prodotto di due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `moltiplicatore` - Un altro numero razionale.
        ///
        /// # Ritorna
        ///
        /// Un nuovo numero razionale che è il prodotto di `self` e `moltiplicatore`.
        ///

        pub fn prodotto(&self, moltiplicatore: &Razionale) -> Razionale {
            let nnum = self.num * moltiplicatore.num;
            let ndenum = self.denum * moltiplicatore.denum;
            Razionale::new(nnum, ndenum)
        }

        /// Riduce un numero razionale ai minimi termini.
        ///
        /// # Ritorna
        ///
        /// Un nuovo numero razionale ridotto ai minimi termini.

        pub fn riduzione_minimi_termini(&self) -> Razionale {
            let mcd = Self::mcd(self.num, self.denum);
            Razionale::new(self.num / mcd, self.denum / mcd)
        }

        /// Calcola il massimo comune divisore (MCD) di due numeri.
        ///
        /// # Argomenti
        ///
        /// * `a` - Primo numero.
        /// * `b` - Secondo numero.
        ///
        /// # Ritorna
        ///
        /// L'MCD di `a` e `b`.
        fn mcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a.abs()
        }
    }

    impl Add<Razionale> for Razionale {
        type Output = Razionale;

        /// Implementa l'operazione di addizione per due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `self` - Il primo numero razionale.
        /// * `rhs` - Il secondo numero razionale.
        ///
        /// # Ritorna
        ///
        /// Un nuovo numero razionale che è la somma di `self` e `rhs`.

        fn add(self, rhs: Razionale) -> Razionale {
            let nnum = self.num * rhs.denum + self.denum * rhs.num;
            let ndenum = self.denum * rhs.denum;
            Razionale::new(nnum, ndenum)
        }
    }

    impl Mul<Razionale> for Razionale {
        type Output = Razionale;

        /// Implementa l'operazione di moltiplicazione per due numeri razionali.
        ///
        /// # Argomenti
        ///
        /// * `self` - Il primo numero razionale.
        /// * `rhs` - Il secondo numero razionale.
        ///
        /// # Ritorna
        ///
        /// Un nuovo numero razionale che è il prodotto di `self` e `rhs`.

        fn mul(self, rhs: Razionale) -> Razionale {
            let nnum = self.num * rhs.num;
            let ndenum = self.denum * rhs.denum;
            Razionale::new(nnum, ndenum)
        }
    }

    /// Implementazione del trait `Add` per `Razionale` e `i32`.
    impl Add<i32> for Razionale {
        type Output = Razionale;

        /// Implementa l'operazione di addizione per un numero razionale e un intero.
        ///
        /// # Argomenti
        ///
        /// * `self` - Il numero razionale.
        /// * `rhs` - L'intero da sommare.
        ///
        /// # Ritorna
        ///
        /// Un nuovo numero razionale che è la somma di `self` e `rhs`.

        fn add(self, rhs: i32) -> Razionale {
            Razionale::new(self.num + self.denum * rhs, self.denum)
        }
    }

    /// Implementazione del trait `Mul` per `Razionale` e `i32`.
    impl Mul<i32> for Razionale {
        type Output = Razionale;

        /// Implementa l'operazione di moltiplicazione per un numero razionale e un intero.
        ///
        /// # Argomenti
        ///
        /// * `self` - Il numero razionale.
        /// * `rhs` - L'intero da moltiplicare.
        ///
        /// # Ritorna
        ///
        /// Un nuovo numero razionale che è il prodotto di `self` e `rhs`.

        fn mul(self, rhs: i32) -> Razionale {
            Razionale::new(self.num * rhs, self.denum)
        }
    }
}