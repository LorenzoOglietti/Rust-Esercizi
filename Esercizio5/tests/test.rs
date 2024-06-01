#[cfg(test)]
mod tests {
    use Esercizio5::conto_bancario::conto_bancario::ContoBancario;

    #[test]
    fn test_deposita() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.deposita(500);
        assert_eq!(conto.vedi_saldo(), 500);
    }

    #[test]
    fn test_preleva_in_rosso() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.preleva(100000);
        conto.preleva(500);
        assert_eq!(conto.vedi_saldo(), -100000);
    }

    #[test]
    fn test_preleva_in_argento() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.deposita(5000);
        conto.preleva(3000);
        assert_eq!(conto.vedi_saldo(), 2000);
    }

    #[test]
    fn test_preleva_in_oro() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.deposita(15000);
        conto.preleva(5000);
        assert_eq!(conto.vedi_saldo(), 10000);
    }

    #[test]
    fn test_paga_interessi_in_argento() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.deposita(5000);
        conto.paga_interessi();
        assert_eq!(conto.vedi_saldo(), 5250); // 5% di 5000 = 250
    }

    #[test]
    fn test_paga_interessi_in_oro() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.deposita(15000);
        conto.paga_interessi();
        assert_eq!(conto.vedi_saldo(), 15750); // 5% di 15000 = 750
    }

    #[test]
    fn test_paga_interessi_in_rosso() {
        let mut conto = ContoBancario::new(String::from("Mario"));
        conto.preleva(100000);
        conto.paga_interessi();
        assert_eq!(conto.vedi_saldo(), -100000); // Saldo non cambia
    }
}
