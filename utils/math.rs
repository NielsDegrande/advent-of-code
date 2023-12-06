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
