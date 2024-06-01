#[cfg(test)]
mod tests {
    use Esercizio8::count_vocali::count_vocali::*;

    #[test]
    fn test_folds() {
        let a = String::from("Ciao Paola come stai? Ok. Tu John come stai? Ok");

        // Test per num_vocali_tuple
        assert_eq!(TuplaVocali(5, 2, 3, 7, 1), TuplaVocali::num_vocali_tuple(&a));

        // Test per num_vocali_struct
        assert_eq!(NumVocali { a: 5, e: 2, i: 3, o: 7, u: 1 }, NumVocali::num_vocali_struct(&a));
    }
}
