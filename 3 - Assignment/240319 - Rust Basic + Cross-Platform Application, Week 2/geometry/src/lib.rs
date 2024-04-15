// Calculate the magnitude of a vector by summing the squares of its coordinates
// and taking the square root. Use the `sqrt()` method to calculate the square
// root, like `v.sqrt()`.
fn magnitude(point: &[f64; 3]) -> f64 {
    (point[0].powi(2) + point[1].powi(2) + point[2].powi(2)).sqrt()
}

// Normalize a vector by calculating its magnitude and dividing all of its
// coordinates by that magnitude.
fn normalize(point: &mut [f64; 3]) {
    let r = magnitude(point);
    for x in point {
        *x /= r;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn magnitude_normal() {
        let mut vec = [1.0, 2.0, 9.0];
        assert!((magnitude(&mut vec) - 9.273618495495704).abs() < f64::EPSILON);
    }

    #[test]
    fn normalize_normal() {
        let mut vec = [1.0, 2.0, 9.0];
        normalize(&mut vec);
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn magnitude_unit() {
        let mut vec = [0.0, 1.0, 0.0];
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn normalize_unit() {
        let mut vec = [0.0, 1.0, 0.0];
        normalize(&mut vec);
        assert!((magnitude(&mut vec) - 1.0).abs() < f64::EPSILON);
    }
}
