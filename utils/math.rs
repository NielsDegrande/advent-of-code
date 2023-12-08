fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        // No real solutions.
        None
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let x1 = (-b + sqrt_discriminant) / (2.0 * a);
        let x2 = (-b - sqrt_discriminant) / (2.0 * a);
        Some((x1, x2))
    }
}

fn is_integer(n: f32) -> bool {
    n.fract() == 0.0
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    // a * b / gcd(a, b), divide first to avoid overflow.
    a / gcd(a, b) * b
}

fn lcm_of_vec(numbers: &Vec<u64>) -> u64 {
    numbers
        .iter()
        .copied()
        .reduce(|a, b| lcm(a, b))
        .unwrap_or(1)
}
