#[cfg(test)]
mod test {
    use Esercizio2_4::razionale::razionale::Razionale;

    #[test]
    fn test_new() {
        let raz = Razionale::new(3, 4);
        assert_eq!(raz.num, 3);
        assert_eq!(raz.denum, 4);
    }

    #[test]
    #[should_panic]
    fn test_new_panic() {
        // Questo test dovrebbe fallire poiché il denominatore non può essere 0
        let _ = Razionale::new(3, 0);
    }

    #[test]
    fn test_int_to_raz() {
        let raz = Razionale::int_to_raz(5);
        assert_eq!(raz.num, 5);
        assert_eq!(raz.denum, 1);
    }

    #[test]
    fn test_prodotto() {
        let raz1 = Razionale::new(3, 4);
        let raz2 = Razionale::new(2, 3);
        let prod = raz1.prodotto(&raz2);
        assert_eq!(prod.num, 6);
        assert_eq!(prod.denum, 12);
    }

    #[test]
    fn test_addition() {
        let raz1 = Razionale::new(1, 2);
        let raz2 = Razionale::new(1, 3);
        let sum = raz1 + raz2;
        assert_eq!(sum.num, 5);
        assert_eq!(sum.denum, 6);
    }

    #[test]
    fn test_multiplication() {
        let raz1 = Razionale::new(1, 2);
        let raz2 = Razionale::new(2, 3);
        let prod = raz1 * raz2;
        assert_eq!(prod.num, 2);
        assert_eq!(prod.denum, 6);
    }

    #[test]
    fn test_addition_with_int() {
        let raz = Razionale::new(1, 2);
        let sum = raz + 3;
        assert_eq!(sum.num, 7);
        assert_eq!(sum.denum, 2);
    }

    #[test]
    fn test_multiplication_with_int() {
        let raz = Razionale::new(1, 2);
        let prod = raz * 3;
        assert_eq!(prod.num, 3);
        assert_eq!(prod.denum, 2);
    }

    #[test]
    fn test_riduzione_minimi_termini() {
        let raz = Razionale::new(6, 8); // 6/8 ridotto è 3/4
        let reduced = raz.riduzione_minimi_termini();
        assert_eq!(reduced.num, 3);
        assert_eq!(reduced.denum, 4);
    }
}