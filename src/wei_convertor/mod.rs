pub fn wei_convertor(wei: &str) -> f64 {
    wei.parse::<f64>().unwrap() / 1_000_000_000_000_000_000f64
}
