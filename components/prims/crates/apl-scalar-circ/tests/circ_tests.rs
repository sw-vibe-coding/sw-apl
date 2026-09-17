//! Circular functions, factorial, binomial.

use apl_scalar_circ::{binomial, circular, factorial, gamma};
use apl_value::ErrorKind;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9 * b.abs().max(1.0)
}

#[test]
fn gamma_matches_factorials_and_half_integers() {
    assert!(close(gamma(5.0), 24.0));
    assert!(close(gamma(1.0), 1.0));
    assert!(close(gamma(0.5), std::f64::consts::PI.sqrt()));
    assert!(close(gamma(10.0), 362_880.0));
}

#[test]
fn factorial_is_exact_for_small_integers_and_extends_by_gamma() {
    assert!(close(factorial(0.0).unwrap(), 1.0));
    assert!(close(factorial(5.0).unwrap(), 120.0));
    assert!(close(factorial(20.0).unwrap(), 2_432_902_008_176_640_000.0));
    assert!(close(factorial(0.5).unwrap(), 0.886_226_925_452_758));
    assert_eq!(factorial(-1.0).unwrap_err().kind, ErrorKind::Domain);
    assert_eq!(factorial(-3.0).unwrap_err().kind, ErrorKind::Domain);
    assert!(close(factorial(-0.5).unwrap(), std::f64::consts::PI.sqrt()));
}

#[test]
fn binomial_counts_combinations() {
    assert!(close(binomial(2.0, 5.0).unwrap(), 10.0));
    assert!(close(binomial(0.0, 7.0).unwrap(), 1.0));
    assert!(close(binomial(3.0, 3.0).unwrap(), 1.0));
    assert!(close(binomial(1.0, 10.0).unwrap(), 10.0));
    assert!(
        close(binomial(5.0, 3.0).unwrap(), 0.0),
        "more than available is zero"
    );
    assert!(close(
        binomial(0.5, 2.0).unwrap(),
        2.0 / (gamma(1.5) * gamma(2.5))
    ));
}

#[test]
fn circular_table_by_k() {
    use std::f64::consts::{FRAC_PI_2, PI};
    assert!(close(circular(0.0, 0.6).unwrap(), 0.8));
    assert!(close(circular(1.0, FRAC_PI_2).unwrap(), 1.0));
    assert!(close(circular(2.0, PI).unwrap(), -1.0));
    assert!(close(circular(3.0, 0.0).unwrap(), 0.0));
    assert!(close(circular(4.0, 3.0).unwrap(), 10f64.sqrt()));
    assert!(close(circular(5.0, 0.0).unwrap(), 0.0));
    assert!(close(circular(6.0, 0.0).unwrap(), 1.0));
    assert!(close(circular(7.0, 0.0).unwrap(), 0.0));
    assert!(close(circular(-1.0, 1.0).unwrap(), FRAC_PI_2));
    assert!(close(circular(-2.0, 1.0).unwrap(), 0.0));
    assert!(close(circular(-3.0, 1.0).unwrap(), PI / 4.0));
    assert!(close(circular(-4.0, 5.0).unwrap(), 24f64.sqrt()));
    assert!(close(circular(-5.0, 0.0).unwrap(), 0.0));
    assert!(close(circular(-6.0, 1.0).unwrap(), 0.0));
    assert!(close(circular(-7.0, 0.0).unwrap(), 0.0));
    for (k, x) in [
        (0.0, 2.0),
        (-1.0, 2.0),
        (-4.0, 0.5),
        (-6.0, 0.5),
        (-7.0, 1.0),
        (8.0, 0.0),
        (0.5, 0.0),
    ] {
        assert_eq!(
            circular(k, x).unwrap_err().kind,
            ErrorKind::Domain,
            "{k} {x}"
        );
    }
}
