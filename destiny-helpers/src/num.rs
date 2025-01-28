pub fn truncate_float(val: f64, decimals: u32, round_up: bool) -> f64 {
    let pow10 = 10i64.pow(decimals) as f64;
    let mut val = (val * pow10) as u64;
    if round_up {
        val += 1;
    }
    val as f64 / pow10
}

pub fn is_zero(val: f64) -> bool {
    val.abs() < 1e-8
}
