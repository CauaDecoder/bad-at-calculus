use super::*;
use crate::{Expr, MathFunction};

fn x() -> Expr {
    Expr::variable("x")
}
fn settings() -> Result<SamplingSettings, &'static str> {
    SamplingSettings::new(
        7,
        [("x".into(), [-5.0, 5.0])].into(),
        12,
        Tolerance::default(),
    )
    .ok_or("settings")
}

#[test]
fn equations_require_constant_nonzero_scale_even_when_scale_is_tiny() -> Result<(), &'static str> {
    let expected = Expr::subtract(x(), Expr::Number(1.0));
    let settings = settings()?;
    for factor in [-2.0, 0.5, 1e-30] {
        let response = Expr::multiply(Expr::Number(factor), expected.clone());
        assert_eq!(
            compare_equations(&expected, &response, &settings).verdict,
            Verdict::Correct
        );
    }
    for response in [
        Expr::Number(0.0),
        Expr::power(expected.clone(), Expr::Number(2.0)),
        Expr::sum(expected.clone(), Expr::Number(1.0)),
    ] {
        assert_eq!(
            compare_equations(&expected, &response, &settings).verdict,
            Verdict::Incorrect
        );
    }
    Ok(())
}

#[test]
fn identity_equations_and_invalid_expected_domains_are_indeterminate() -> Result<(), &'static str> {
    let zero = Expr::Number(0.0);
    assert_eq!(
        compare_equations(&zero, &zero, &settings()?).verdict,
        Verdict::Indeterminate
    );
    let invalid = Expr::call(MathFunction::Sqrt, Expr::Number(-1.0));
    assert_eq!(
        compare_equations(&invalid, &invalid, &settings()?).verdict,
        Verdict::Indeterminate
    );
    Ok(())
}

#[test]
fn ordered_collections_propagate_uncertainty_without_hiding_wrong_components() {
    assert_eq!(combine_verdicts([]), Verdict::Correct);
    assert_eq!(
        combine_verdicts([Verdict::Correct, Verdict::Indeterminate]),
        Verdict::Indeterminate
    );
    assert_eq!(
        combine_verdicts([Verdict::Indeterminate, Verdict::Incorrect]),
        Verdict::Incorrect
    );
}

#[test]
fn sets_use_augmenting_paths_and_never_reuse_an_element() {
    use Verdict::{Correct as C, Incorrect as W, Indeterminate as U};
    assert_eq!(match_set(&[vec![C, C], vec![C, W]]), C);
    assert_eq!(match_set(&[vec![C, W], vec![C, W]]), W);
    assert_eq!(match_set(&[vec![C, W], vec![W, U]]), U);
    assert_eq!(match_set(&[vec![U, W], vec![U, W]]), W);
    assert_eq!(match_set(&[]), C);
    assert_eq!(match_set(&[vec![C, C]]), W);
}

#[test]
fn sets_match_every_permutation_but_reject_a_missing_column() {
    use Verdict::{Correct as C, Incorrect as W};
    for permutation in [
        [0, 1, 2],
        [0, 2, 1],
        [1, 0, 2],
        [1, 2, 0],
        [2, 0, 1],
        [2, 1, 0],
    ] {
        let edges: Vec<Vec<_>> = permutation
            .iter()
            .map(|&column| {
                (0..3)
                    .map(|index| if index == column { C } else { W })
                    .collect()
            })
            .collect();
        assert_eq!(match_set(&edges), C);
    }
    assert_eq!(match_set(&[vec![C, W, W], vec![W, C, W], vec![C, W, W]]), W);
}

#[test]
fn structural_forms_inspect_the_unsimplified_tree() {
    let plus = Expr::sum(x(), Expr::Number(1.0));
    let minus = Expr::subtract(x(), Expr::Number(1.0));
    let product = Expr::multiply(plus.clone(), minus);
    let expanded = Expr::subtract(Expr::power(x(), Expr::Number(2.0)), Expr::Number(1.0));
    assert_eq!(
        check_form(&product, ExpressionForm::Factored),
        Verdict::Correct
    );
    assert_eq!(
        check_form(&expanded, ExpressionForm::Factored),
        Verdict::Incorrect
    );
    assert_eq!(
        check_form(&product, ExpressionForm::Expanded),
        Verdict::Incorrect
    );
    assert_eq!(
        check_form(&expanded, ExpressionForm::Expanded),
        Verdict::Correct
    );
    assert_eq!(
        check_form(
            &Expr::power(plus, Expr::Number(2.0)),
            ExpressionForm::Expanded
        ),
        Verdict::Incorrect
    );
    let power = Expr::power(x(), Expr::Number(-1.0));
    assert_eq!(
        check_form(&power, ExpressionForm::NoNegativeExponents),
        Verdict::Incorrect
    );
    let variable_power = Expr::power(x(), x());
    assert_eq!(
        check_form(&variable_power, ExpressionForm::NoNegativeExponents),
        Verdict::Indeterminate
    );
}

#[test]
fn objective_comparison_has_no_partial_credit_or_order_requirement() {
    assert_eq!(
        compare_choices(&["a".into(), "b".into()], &["b".into(), "a".into()]),
        Verdict::Correct
    );
    let expected = [("a".into(), "1".into()), ("b".into(), "2".into())].into();
    assert_eq!(
        compare_pairs(&expected, &[("a".into(), "1".into())].into()),
        Verdict::Incorrect
    );
}

#[test]
fn unknown_form_in_one_branch_does_not_hide_a_definite_violation_in_another() {
    let expression = Expr::Piecewise(vec![
        crate::PiecewiseBranch {
            condition: None,
            value: Expr::power(x(), x()),
        },
        crate::PiecewiseBranch {
            condition: None,
            value: Expr::power(x(), Expr::Number(-1.0)),
        },
    ]);
    assert_eq!(
        check_form(&expression, ExpressionForm::NoNegativeExponents),
        Verdict::Incorrect
    );
}
