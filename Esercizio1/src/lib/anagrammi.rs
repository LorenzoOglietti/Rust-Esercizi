/// Un modulo per verificare se due stringhe sono anagrammi.
pub mod anagrammi {
    use std::collections::HashMap;

    /// Verifica se due stringhe sono anagrammi l'una dell'altra.
    ///
    /// # Argomenti
    ///
    /// * `str1` - Un slice di stringa da confrontare.
    /// * `str2` - Un altro slice di stringa da confrontare.
    ///
    /// # Ritorna
    ///
    /// * `true` se `str1` e `str2` sono anagrammi, altrimenti `false`.
    pub fn sono_anagrammi(str1: &str, str2: &str) -> bool {
        if str1.len() != str2.len() {
            return false;
        }

        let mut parola1: HashMap<char, i32> = HashMap::new();
        let mut parola2: HashMap<char, i32> = HashMap::new();

        for car in str1.chars() {
            *parola1.entry(car.to_ascii_lowercase()).or_insert(0) += 1;
        }
        for car in str2.chars() {
            *parola2.entry(car.to_ascii_lowercase()).or_insert(0) += 1;
        }
        if parola1 == parola2 {
            return true;
        }
        return false;
    }
}
