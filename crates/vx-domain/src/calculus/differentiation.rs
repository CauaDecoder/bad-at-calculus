//! Symbolic differentiation that records every rule it applies, so the
//! student can follow the derivation step by step.

use crate::{BinaryOperator, Constant, Expr, MathFunction, PiecewiseBranch};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivativeRule {
    Constant,
    Sum,
    Difference,
    Negation,
    ConstantMultiple,
    Product,
    Quotient,
    Power,
    Exponential,
    NaturalExponential,
    GeneralPower,
    NaturalLogarithm,
    Logarithm10,
    Sine,
    Cosine,
    Tangent,
    SquareRoot,
    CubeRoot,
    AbsoluteValue,
    Piecewise,
}

/// `d/dx [expression] = derivative`, obtained with `rule`.
#[derive(Debug, Clone, PartialEq)]
pub struct DerivationStep {
    pub rule: DerivativeRule,
    /// The outer rule was applied to an inner function `u(x)`, so `· u'` appears.
    pub uses_chain_rule: bool,
    pub expression: Expr,
    pub derivative: Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Derivation {
    pub derivative: Expr,
    /// Top-down: the first step is the outermost rule.
    pub steps: Vec<DerivationStep>,
}

pub fn differentiate(expression: &Expr, variable: &str) -> Derivation {
    let mut differentiator = Differentiator {
        variable,
        steps: Vec::new(),
    };
    let derivative = differentiator.derive(expression);
    if differentiator.steps.is_empty() && !expression.depends_on(variable) {
        let constant_rule = RuleApplication::plain(DerivativeRule::Constant, derivative.clone());
        differentiator.finish(None, expression, constant_rule);
    }
    Derivation {
        derivative,
        steps: differentiator.steps,
    }
}

/// A rule applied to one node, before simplification.
struct RuleApplication {
    rule: DerivativeRule,
    uses_chain_rule: bool,
    raw: Expr,
}

impl RuleApplication {
    fn plain(rule: DerivativeRule, raw: Expr) -> Self {
        Self {
            rule,
            uses_chain_rule: false,
            raw,
        }
    }

    fn chained(rule: DerivativeRule, uses_chain_rule: bool, raw: Expr) -> Self {
        Self {
            rule,
            uses_chain_rule,
            raw,
        }
    }
}

struct Differentiator<'a> {
    variable: &'a str,
    steps: Vec<DerivationStep>,
}

impl Differentiator<'_> {
    fn derive(&mut self, expression: &Expr) -> Expr {
        if !expression.depends_on(self.variable) {
            return Expr::Number(0.0);
        }
        if let Expr::Variable(_) = expression {
            return Expr::Number(1.0);
        }
        let slot = self.reserve();
        let application = match expression {
            Expr::Negate(operand) => {
                RuleApplication::plain(DerivativeRule::Negation, Expr::negate(self.derive(operand)))
            }
            Expr::Binary {
                operator,
                left,
                right,
            } => self.binary_rule(*operator, left, right),
            Expr::Call { function, argument } => self.call_rule(*function, argument),
            Expr::Piecewise(branches) => {
                let derived = branches
                    .iter()
                    .map(|branch| PiecewiseBranch {
                        condition: branch.condition.clone(),
                        value: self.derive(&branch.value),
                    })
                    .collect();
                RuleApplication::plain(DerivativeRule::Piecewise, Expr::Piecewise(derived))
            }
            Expr::Number(_) | Expr::Constant(_) | Expr::Variable(_) => {
                RuleApplication::plain(DerivativeRule::Constant, Expr::Number(0.0))
            }
        };
        self.finish(slot, expression, application)
    }

    fn binary_rule(&mut self, operator: BinaryOperator, u: &Expr, v: &Expr) -> RuleApplication {
        let u_varies = u.depends_on(self.variable);
        let v_varies = v.depends_on(self.variable);
        match operator {
            BinaryOperator::Add => RuleApplication::plain(
                DerivativeRule::Sum,
                Expr::sum(self.derive(u), self.derive(v)),
            ),
            BinaryOperator::Subtract => RuleApplication::plain(
                DerivativeRule::Difference,
                Expr::subtract(self.derive(u), self.derive(v)),
            ),
            BinaryOperator::Multiply if !u_varies => RuleApplication::plain(
                DerivativeRule::ConstantMultiple,
                Expr::multiply(u.clone(), self.derive(v)),
            ),
            BinaryOperator::Multiply if !v_varies => RuleApplication::plain(
                DerivativeRule::ConstantMultiple,
                Expr::multiply(v.clone(), self.derive(u)),
            ),
            BinaryOperator::Multiply => {
                // (uv)' = u'v + uv'
                let (du, dv) = (self.derive(u), self.derive(v));
                let raw = Expr::sum(Expr::multiply(du, v.clone()), Expr::multiply(u.clone(), dv));
                RuleApplication::plain(DerivativeRule::Product, raw)
            }
            BinaryOperator::Divide if !v_varies => RuleApplication::plain(
                DerivativeRule::ConstantMultiple,
                Expr::divide(self.derive(u), v.clone()),
            ),
            BinaryOperator::Divide => {
                // (u/v)' = (u'v − uv') / v²
                let (du, dv) = (self.derive(u), self.derive(v));
                let numerator =
                    Expr::subtract(Expr::multiply(du, v.clone()), Expr::multiply(u.clone(), dv));
                let denominator = Expr::power(v.clone(), Expr::Number(2.0));
                RuleApplication::plain(
                    DerivativeRule::Quotient,
                    Expr::divide(numerator, denominator),
                )
            }
            BinaryOperator::Power => self.power_rule(u, v),
        }
    }

    fn power_rule(&mut self, base: &Expr, exponent: &Expr) -> RuleApplication {
        let base_varies = base.depends_on(self.variable);
        let exponent_varies = exponent.depends_on(self.variable);
        let whole = Expr::power(base.clone(), exponent.clone());
        if !exponent_varies {
            // n·u^(n−1)·u'
            let reduced = Expr::subtract(exponent.clone(), Expr::Number(1.0));
            let outer = Expr::multiply(exponent.clone(), Expr::power(base.clone(), reduced));
            let raw = Expr::multiply(outer, self.derive(base));
            return RuleApplication::chained(
                DerivativeRule::Power,
                self.is_inner_function(base),
                raw,
            );
        }
        if !base_varies {
            // a^u·ln(a)·u'  (and e^u·u')
            let is_natural = *base == Expr::Constant(Constant::E);
            let (rule, outer) = if is_natural {
                (DerivativeRule::NaturalExponential, whole)
            } else {
                let logarithm = Expr::call(MathFunction::Ln, base.clone());
                (
                    DerivativeRule::Exponential,
                    Expr::multiply(whole, logarithm),
                )
            };
            let raw = Expr::multiply(outer, self.derive(exponent));
            return RuleApplication::chained(rule, self.is_inner_function(exponent), raw);
        }
        // u^v·(v'·ln(u) + v·u'/u)
        let (du, dv) = (self.derive(base), self.derive(exponent));
        let bracket = Expr::sum(
            Expr::multiply(dv, Expr::call(MathFunction::Ln, base.clone())),
            Expr::divide(Expr::multiply(exponent.clone(), du), base.clone()),
        );
        RuleApplication::plain(DerivativeRule::GeneralPower, Expr::multiply(whole, bracket))
    }

    /// f(u)' = f'(u)·u'
    fn call_rule(&mut self, function: MathFunction, inner: &Expr) -> RuleApplication {
        let (rule, outer) = outer_derivative(function, inner);
        let raw = Expr::multiply(outer, self.derive(inner));
        RuleApplication::chained(rule, self.is_inner_function(inner), raw)
    }

    fn is_inner_function(&self, expression: &Expr) -> bool {
        *expression != Expr::variable(self.variable)
    }

    /// Reserves the step's position before recursing, so steps stay top-down.
    fn reserve(&mut self) -> Option<usize> {
        self.steps.push(DerivationStep {
            rule: DerivativeRule::Constant,
            uses_chain_rule: false,
            expression: Expr::Number(0.0),
            derivative: Expr::Number(0.0),
        });
        Some(self.steps.len() - 1)
    }

    /// Simplifies the result and stores the step in its reserved slot.
    fn finish(
        &mut self,
        slot: Option<usize>,
        expression: &Expr,
        application: RuleApplication,
    ) -> Expr {
        let derivative = application.raw.simplified();
        let step = DerivationStep {
            rule: application.rule,
            uses_chain_rule: application.uses_chain_rule,
            expression: expression.clone(),
            derivative: derivative.clone(),
        };
        match slot.and_then(|index| self.steps.get_mut(index)) {
            Some(reserved) => *reserved = step,
            None => self.steps.push(step),
        }
        derivative
    }
}

/// `f'(u)` for each elementary function, before multiplying by `u'`.
fn outer_derivative(function: MathFunction, u: &Expr) -> (DerivativeRule, Expr) {
    let u = u.clone();
    let one = || Expr::Number(1.0);
    match function {
        MathFunction::Sin => (DerivativeRule::Sine, Expr::call(MathFunction::Cos, u)),
        MathFunction::Cos => (
            DerivativeRule::Cosine,
            Expr::negate(Expr::call(MathFunction::Sin, u)),
        ),
        MathFunction::Tan => (
            DerivativeRule::Tangent,
            Expr::divide(
                one(),
                Expr::power(Expr::call(MathFunction::Cos, u), Expr::Number(2.0)),
            ),
        ),
        MathFunction::Ln => (DerivativeRule::NaturalLogarithm, Expr::divide(one(), u)),
        MathFunction::Log10 => (
            DerivativeRule::Logarithm10,
            Expr::divide(
                one(),
                Expr::multiply(u, Expr::call(MathFunction::Ln, Expr::Number(10.0))),
            ),
        ),
        MathFunction::Exp => (
            DerivativeRule::NaturalExponential,
            Expr::call(MathFunction::Exp, u),
        ),
        MathFunction::Sqrt => (
            DerivativeRule::SquareRoot,
            Expr::divide(
                one(),
                Expr::multiply(Expr::Number(2.0), Expr::call(MathFunction::Sqrt, u)),
            ),
        ),
        MathFunction::Cbrt => (
            DerivativeRule::CubeRoot,
            Expr::divide(
                one(),
                Expr::multiply(
                    Expr::Number(3.0),
                    Expr::power(Expr::call(MathFunction::Cbrt, u), Expr::Number(2.0)),
                ),
            ),
        ),
        MathFunction::Abs => (
            DerivativeRule::AbsoluteValue,
            Expr::divide(u.clone(), Expr::call(MathFunction::Abs, u)),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TOLERANCE: f64 = 1e-9;

    fn x() -> Expr {
        Expr::variable("x")
    }

    fn number(value: f64) -> Expr {
        Expr::Number(value)
    }

    fn derivative_latex(expression: &Expr) -> String {
        differentiate(expression, "x").derivative.to_latex()
    }

    fn assert_derivative_value(expression: &Expr, at: f64, expected: f64) {
        let value = differentiate(expression, "x")
            .derivative
            .evaluate_at("x", at);
        let is_close = value
            .as_ref()
            .is_ok_and(|value| (value - expected).abs() < TOLERANCE);
        assert!(is_close, "expected {expected}, got {value:?}");
    }

    #[test]
    fn power_rule() {
        assert_eq!(derivative_latex(&Expr::power(x(), number(2.0))), "2 x");
        assert_eq!(derivative_latex(&Expr::power(x(), number(3.0))), "3 x^{2}");
    }

    #[test]
    fn linear_function_from_study_guide() {
        // ED derivação, ex. 3: f(x) = mx + b with m = 5, b = 2.
        let line = Expr::sum(Expr::multiply(number(5.0), x()), number(2.0));
        assert_eq!(derivative_latex(&line), "5");
    }

    #[test]
    fn constant_has_zero_derivative() {
        let derivation = differentiate(&number(7.0), "x");
        assert_eq!(derivation.derivative, number(0.0));
        assert_eq!(derivation.steps[0].rule, DerivativeRule::Constant);
    }

    #[test]
    fn product_rule_is_recorded_first() {
        // Lista 04, ex. 12: f(x) = (x³ + 2)⁴ cos x
        let inner = Expr::sum(Expr::power(x(), number(3.0)), number(2.0));
        let expression = Expr::multiply(
            Expr::power(inner, number(4.0)),
            Expr::call(MathFunction::Cos, x()),
        );
        let derivation = differentiate(&expression, "x");

        assert_eq!(derivation.steps[0].rule, DerivativeRule::Product);
        let power_step = derivation
            .steps
            .iter()
            .find(|step| step.rule == DerivativeRule::Power);
        assert!(power_step.is_some_and(|step| step.uses_chain_rule));
        assert_derivative_value(&expression, 0.0, 0.0);
    }

    #[test]
    fn coefficients_are_collected_in_front() {
        // Lista 04, ex. 12 (power part): d/dx (x³ + 2)⁴ = 12x²(x³ + 2)³
        let inner = Expr::sum(Expr::power(x(), number(3.0)), number(2.0));
        assert_eq!(
            derivative_latex(&Expr::power(inner, number(4.0))),
            r"12 \left(x^{3} + 2\right)^{3} x^{2}"
        );
    }

    #[test]
    fn quotient_rule_matches_numeric_value() {
        // Lista 04, ex. 9: f(x) = −x / (x² + 1), f'(1) = 0
        let expression = Expr::divide(
            Expr::negate(x()),
            Expr::sum(Expr::power(x(), number(2.0)), number(1.0)),
        );
        assert_derivative_value(&expression, 1.0, 0.0);
        assert_derivative_value(&expression, 0.0, -1.0);
    }

    #[test]
    fn chain_rule_for_sine_of_x_squared() {
        let expression = Expr::call(MathFunction::Sin, Expr::power(x(), number(2.0)));
        let derivation = differentiate(&expression, "x");
        assert!(derivation.steps[0].uses_chain_rule);
        assert_eq!(
            derivation.derivative.to_latex(),
            r"2 x \cos\left(x^{2}\right)"
        );
    }

    #[test]
    fn exponential_rules() {
        let natural = Expr::power(Expr::Constant(Constant::E), x());
        assert_eq!(derivative_latex(&natural), "e^{x}");
        let base_two = Expr::power(number(2.0), x());
        assert_derivative_value(&base_two, 1.0, 2.0 * 2.0_f64.ln());
    }

    #[test]
    fn general_power_x_to_the_x() {
        let expression = Expr::power(x(), x());
        assert_derivative_value(&expression, 1.0, 1.0);
    }
}
