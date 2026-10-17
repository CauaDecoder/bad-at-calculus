use super::*;
use crate::{Expr, MathFunction};

fn settings(domain: [f64; 2]) -> Result<SamplingSettings, &'static str> {
    SamplingSettings::new(42, [("x".into(), domain)].into(), 12, Tolerance::default())
        .ok_or("Invalid settings")
}

fn x() -> Expr {
    Expr::variable("x")
}

#[test]
fn constant_comparison_uses_absolute_and_relative_tolerances() {
    assert_eq!(
        compare_numbers(0.5, 1.0 / 2.0, &Tolerance::default()).verdict,
        Verdict::Correct
    );
    assert_eq!(
        compare_numbers(1e10, 1e10 + 1.0, &Tolerance::default()).verdict,
        Verdict::Correct
    );
    assert_eq!(
        compare_numbers(0.0, 1e-13, &Tolerance::default()).verdict,
        Verdict::Correct
    );
    assert_eq!(
        compare_numbers(0.0, 1e-6, &Tolerance::default()).verdict,
        Verdict::Incorrect
    );
    assert_eq!(
        compare_numbers(f64::MAX, -f64::MAX, &Tolerance::default()).verdict,
        Verdict::Incorrect
    );
    assert_eq!(
        compare_numbers(f64::INFINITY, f64::INFINITY, &Tolerance::default()).verdict,
        Verdict::Incorrect
    );
    let exact = Tolerance {
        abs: Some(0.0),
        rel: Some(0.0),
    };
    assert_eq!(
        compare_numbers(1.0, 1.0 + f64::EPSILON, &exact).verdict,
        Verdict::Incorrect
    );
}

#[test]
fn never_simplifies_away_response_holes() -> Result<(), &'static str> {
    let ratio = Expr::divide(x(), x());
    assert_eq!(
        compare_expressions(&Expr::Number(1.0), &ratio, &settings([-10.0, 10.0])?).verdict,
        Verdict::Incorrect
    );
    let restricted = compare_expressions(&ratio, &Expr::Number(1.0), &settings([-10.0, 10.0])?);
    assert_eq!(restricted.verdict, Verdict::Correct);
    assert_eq!(restricted.sampling.ok_or("Missing sampling")?.valid, 12);
    assert_eq!(
        compare_expressions(&Expr::Number(1.0), &ratio, &settings([1.0, 3.0])?).verdict,
        Verdict::Correct
    );
    Ok(())
}

#[test]
fn square_root_of_square_is_not_x_on_negative_domain() -> Result<(), &'static str> {
    let root = Expr::call(MathFunction::Sqrt, Expr::power(x(), Expr::Number(2.0)));
    assert_eq!(
        compare_expressions(&root, &x(), &settings([-10.0, 10.0])?).verdict,
        Verdict::Incorrect
    );
    assert_eq!(
        compare_expressions(&root, &x(), &settings([0.0, 10.0])?).verdict,
        Verdict::Correct
    );
    Ok(())
}

#[test]
fn rejects_response_undefined_where_expected_is_defined() -> Result<(), &'static str> {
    let root = Expr::call(MathFunction::Sqrt, x());
    assert_eq!(
        compare_expressions(&x(), &root, &settings([-10.0, 10.0])?).verdict,
        Verdict::Incorrect
    );
    Ok(())
}

#[test]
fn discards_expected_invalid_points_and_requires_twelve_random_points() -> Result<(), &'static str>
{
    let root = Expr::call(MathFunction::Sqrt, x());
    let report = compare_expressions(&root, &root, &settings([-10.0, 10.0])?)
        .sampling
        .ok_or("Missing sampling")?;
    assert_eq!(report.valid, 12);
    assert!(report.discarded > 0);
    let impossible = compare_expressions(&root, &root, &settings([-10.0, -1.0])?);
    assert_eq!(impossible.verdict, Verdict::Indeterminate);
    let report = impossible.sampling.ok_or("Missing sampling")?;
    assert_eq!(report.valid, 0);
    assert_eq!(report.discarded, 120);
    Ok(())
}

#[test]
fn seed_and_domain_make_sampling_reproducible() -> Result<(), &'static str> {
    let expression = Expr::call(MathFunction::Ln, x());
    let first = compare_expressions(&expression, &expression, &settings([-10.0, 10.0])?);
    let second = compare_expressions(&expression, &expression, &settings([-10.0, 10.0])?);
    assert_eq!(first, second);
    Ok(())
}

#[test]
fn settings_reject_unbounded_work_and_invalid_intervals() {
    for samples in [0, 11, 257, u32::MAX] {
        assert!(SamplingSettings::new(
            0,
            [("x".into(), [0.0, 1.0])].into(),
            samples,
            Tolerance::default()
        )
        .is_none());
    }
    assert!(SamplingSettings::new(
        0,
        [("x".into(), [f64::NAN, 1.0])].into(),
        12,
        Tolerance::default()
    )
    .is_none());
}

#[test]
fn evaluates_all_bindings_and_piecewise_conditions() {
    let expression = Expr::sum(x(), Expr::variable("y"));
    let bindings = |name: &str| match name {
        "x" => Some(2.0),
        "y" => Some(3.0),
        _ => None,
    };
    assert_eq!(expression.evaluate_with(&bindings), Ok(5.0));
    let piecewise = Expr::Piecewise(vec![crate::PiecewiseBranch {
        condition: Some(crate::Condition {
            comparisons: vec![crate::Comparison {
                left: Expr::variable("y"),
                relation: crate::Relation::Greater,
                right: x(),
            }],
        }),
        value: expression,
    }]);
    assert_eq!(piecewise.evaluate_with(&bindings), Ok(5.0));
}

#[test]
fn equivalent_polynomials_match_and_perturbations_do_not() -> Result<(), &'static str> {
    for exponent in 1..=5 {
        let expected = Expr::power(x(), Expr::Number(f64::from(exponent)));
        let mut product = x();
        for _ in 1..exponent {
            product = Expr::multiply(product, x());
        }
        assert_eq!(
            compare_expressions(&expected, &product, &settings([-5.0, 5.0])?).verdict,
            Verdict::Correct
        );
        let wrong = Expr::sum(product, Expr::Number(1.0));
        assert_eq!(
            compare_expressions(&expected, &wrong, &settings([-5.0, 5.0])?).verdict,
            Verdict::Incorrect
        );
    }
    Ok(())
}
