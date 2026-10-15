//! Renders an [`Expr`] as LaTeX, adding only the parentheses that are needed.

use super::{BinaryOperator, Condition, Constant, Expr, MathFunction, PiecewiseBranch, Relation};

/// How tightly an expression binds when shown; higher binds tighter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Sum = 1,
    Product = 2,
    Negation = 3,
    Power = 4,
    Atom = 5,
}

impl Expr {
    pub fn to_latex(&self) -> String {
        match self {
            Self::Number(number) => format_number(*number),
            Self::Constant(Constant::Pi) => r"\pi".to_owned(),
            Self::Constant(Constant::E) => "e".to_owned(),
            Self::Variable(name) => name.clone(),
            Self::Negate(operand) => negation_latex(operand),
            Self::Binary {
                operator,
                left,
                right,
            } => binary_latex(*operator, left, right),
            Self::Call { function, argument } => call_latex(*function, argument),
            Self::Piecewise(branches) => piecewise_latex(branches),
        }
    }

    fn precedence(&self) -> Precedence {
        match self {
            Self::Negate(_) => Precedence::Negation,
            Self::Number(number) if *number < 0.0 => Precedence::Negation,
            Self::Binary { operator, .. } => match operator {
                BinaryOperator::Add | BinaryOperator::Subtract => Precedence::Sum,
                BinaryOperator::Multiply => Precedence::Product,
                BinaryOperator::Power => Precedence::Power,
                BinaryOperator::Divide => Precedence::Atom,
            },
            _ => Precedence::Atom,
        }
    }

    fn is_negation(&self) -> bool {
        self.precedence() == Precedence::Negation
    }

    /// `e^{x}` or `\sqrt[3]{x}` can't take another superscript directly.
    fn has_own_superscript(&self) -> bool {
        matches!(
            self,
            Self::Call {
                function: MathFunction::Exp,
                ..
            }
        )
    }

    fn is_fraction(&self) -> bool {
        matches!(
            self,
            Self::Binary {
                operator: BinaryOperator::Divide,
                ..
            }
        )
    }

    fn starts_with_digit(&self) -> bool {
        match self {
            Self::Number(value) => *value >= 0.0,
            Self::Binary {
                operator: BinaryOperator::Multiply | BinaryOperator::Power,
                left,
                ..
            } => left.starts_with_digit(),
            _ => false,
        }
    }
}

fn negation_latex(operand: &Expr) -> String {
    let needs_parentheses = operand.precedence() == Precedence::Sum || operand.is_negation();
    format!("-{}", wrap_if(needs_parentheses, operand))
}

fn binary_latex(operator: BinaryOperator, left: &Expr, right: &Expr) -> String {
    match operator {
        BinaryOperator::Add => format!(
            "{} + {}",
            left.to_latex(),
            wrap_if(right.is_negation(), right)
        ),
        BinaryOperator::Subtract => {
            let needs_parentheses = right.precedence() == Precedence::Sum || right.is_negation();
            format!(
                "{} - {}",
                left.to_latex(),
                wrap_if(needs_parentheses, right)
            )
        }
        BinaryOperator::Multiply => product_latex(left, right),
        BinaryOperator::Divide => format!(r"\frac{{{}}}{{{}}}", left.to_latex(), right.to_latex()),
        BinaryOperator::Power => {
            let needs_parentheses = left.precedence() < Precedence::Atom
                || left.is_fraction()
                || left.has_own_superscript();
            format!(
                "{}^{{{}}}",
                wrap_if(needs_parentheses, left),
                right.to_latex()
            )
        }
    }
}

fn product_latex(left: &Expr, right: &Expr) -> String {
    let left_needs_parentheses = left.precedence() < Precedence::Product;
    let right_needs_parentheses = right.precedence() < Precedence::Product || right.is_negation();
    // Juxtapose like on paper (`2x`, `x\cos(x)`), except where it would be ambiguous:
    // `2 · 3` would read as 23 and `2\frac{1}{x}` as a mixed number.
    let is_ambiguous = !right_needs_parentheses
        && (right.starts_with_digit() || right.is_fraction() || left.is_fraction());
    let separator = if is_ambiguous { r" \cdot " } else { " " };
    format!(
        "{}{separator}{}",
        wrap_if(left_needs_parentheses, left),
        wrap_if(right_needs_parentheses, right)
    )
}

fn call_latex(function: MathFunction, argument: &Expr) -> String {
    let inner = argument.to_latex();
    match function {
        MathFunction::Sqrt => format!(r"\sqrt{{{inner}}}"),
        MathFunction::Cbrt => format!(r"\sqrt[3]{{{inner}}}"),
        MathFunction::Abs => format!(r"\left|{inner}\right|"),
        MathFunction::Exp => format!("e^{{{inner}}}"),
        MathFunction::Log10 => format!(r"\log_{{10}}\left({inner}\right)"),
        named => format!(r"\{}\left({inner}\right)", named.name()),
    }
}

fn wrap_if(condition: bool, expression: &Expr) -> String {
    if condition {
        format!(r"\left({}\right)", expression.to_latex())
    } else {
        expression.to_latex()
    }
}

fn piecewise_latex(branches: &[PiecewiseBranch]) -> String {
    let rows: Vec<String> = branches
        .iter()
        .map(|branch| {
            let condition = branch
                .condition
                .as_ref()
                .map(condition_latex)
                .unwrap_or_default();
            format!("{} & {condition}", branch.value.to_latex())
        })
        .collect();
    format!(r"\begin{{cases}} {} \end{{cases}}", rows.join(r" \\ "))
}

fn condition_latex(condition: &Condition) -> String {
    let mut latex = String::new();
    for (index, comparison) in condition.comparisons.iter().enumerate() {
        let continues_chain = index > 0
            && condition
                .comparisons
                .get(index - 1)
                .is_some_and(|previous| previous.right == comparison.left);
        if index == 0 {
            latex.push_str(&comparison.left.to_latex());
        } else if !continues_chain {
            latex.push_str(&format!(r",\ {}", comparison.left.to_latex()));
        }
        latex.push_str(&format!(
            " {} {}",
            relation_latex(comparison.relation),
            comparison.right.to_latex()
        ));
    }
    latex
}

fn relation_latex(relation: Relation) -> &'static str {
    match relation {
        Relation::Less => "<",
        Relation::LessOrEqual => r"\le",
        Relation::Greater => ">",
        Relation::GreaterOrEqual => r"\ge",
        Relation::Equal => "=",
    }
}

/// Numbers from folding (`0.1 + 0.2`) are rounded so no float noise shows up.
fn format_number(value: f64) -> String {
    const DISPLAY_SCALE: f64 = 1e10;
    let rounded = (value * DISPLAY_SCALE).round() / DISPLAY_SCALE;
    format!("{}", rounded + 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn x() -> Expr {
        Expr::variable("x")
    }

    fn number(value: f64) -> Expr {
        Expr::Number(value)
    }

    fn sum(left: Expr, right: Expr) -> Expr {
        Expr::binary(BinaryOperator::Add, left, right)
    }

    fn product(left: Expr, right: Expr) -> Expr {
        Expr::binary(BinaryOperator::Multiply, left, right)
    }

    fn power(base: Expr, exponent: Expr) -> Expr {
        Expr::binary(BinaryOperator::Power, base, exponent)
    }

    #[test]
    fn coefficient_is_written_next_to_variable() {
        assert_eq!(product(number(3.0), x()).to_latex(), "3 x");
    }

    #[test]
    fn integers_have_no_decimal_point() {
        assert_eq!(power(x(), number(2.0)).to_latex(), "x^{2}");
    }

    #[test]
    fn sum_inside_product_gets_parentheses() {
        let expression = product(number(2.0), sum(x(), number(1.0)));
        assert_eq!(expression.to_latex(), r"2 \left(x + 1\right)");
    }

    #[test]
    fn sum_as_base_of_power_gets_parentheses() {
        let expression = power(sum(x(), number(1.0)), number(2.0));
        assert_eq!(expression.to_latex(), r"\left(x + 1\right)^{2}");
    }

    #[test]
    fn division_becomes_fraction() {
        let expression = Expr::binary(BinaryOperator::Divide, number(1.0), x());
        assert_eq!(expression.to_latex(), r"\frac{1}{x}");
    }

    #[test]
    fn negated_power_keeps_minus_outside() {
        let expression = Expr::negate(power(x(), number(2.0)));
        assert_eq!(expression.to_latex(), "-x^{2}");
    }

    #[test]
    fn functions_use_latex_commands() {
        assert_eq!(
            Expr::call(MathFunction::Sin, x()).to_latex(),
            r"\sin\left(x\right)"
        );
        assert_eq!(Expr::call(MathFunction::Sqrt, x()).to_latex(), r"\sqrt{x}");
        assert_eq!(
            Expr::call(MathFunction::Abs, x()).to_latex(),
            r"\left|x\right|"
        );
    }
}
