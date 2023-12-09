pub fn get_roots(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    if a == 0.0 {
        if b == 0.0 {
            return None;
        }
        return Some((-c / b, -c / b));
    }
    let d = b * b - 4.0 * a * c;
    match d {
        d if d < 0.0 => None,
        d if d > 0.0 => {
            let sqrt_d = f64::sqrt(d);
            let r1 = (-b - sqrt_d) / (2.0 * a);
            let r2 = (-b + sqrt_d) / (2.0 * a);
            if r1 > r2 {
                return Some((r2, r1));
            }
            return Some((r1, r2));
        }
        _ => Some((-b / (2.0 * a), -b / (2.0 * a)))
    }
}