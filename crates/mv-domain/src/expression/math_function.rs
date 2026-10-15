/// Elementary functions available in expressions: `sin(x)`, `sqrt(x)`, ...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathFunction {
    Sin,
    Cos,
    Tan,
    Ln,
    Log10,
    Exp,
    Sqrt,
    Cbrt,
    Abs,
}

impl MathFunction {
    pub const ALL: [MathFunction; 9] = [
        Self::Sin,
        Self::Cos,
        Self::Tan,
        Self::Ln,
        Self::Log10,
        Self::Exp,
        Self::Sqrt,
        Self::Cbrt,
        Self::Abs,
    ];

    /// Name typed by the user, e.g. `sqrt` in `sqrt(x)`.
    pub fn name(self) -> &'static str {
        match self {
            Self::Sin => "sin",
            Self::Cos => "cos",
            Self::Tan => "tan",
            Self::Ln => "ln",
            Self::Log10 => "log",
            Self::Exp => "exp",
            Self::Sqrt => "sqrt",
            Self::Cbrt => "cbrt",
            Self::Abs => "abs",
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|function| function.name() == name)
    }

    /// Raw IEEE result; `NaN`/infinity are rejected by the caller.
    pub fn apply(self, argument: f64) -> f64 {
        match self {
            Self::Sin => argument.sin(),
            Self::Cos => argument.cos(),
            Self::Tan => argument.tan(),
            Self::Ln => argument.ln(),
            Self::Log10 => argument.log10(),
            Self::Exp => argument.exp(),
            Self::Sqrt => argument.sqrt(),
            Self::Cbrt => argument.cbrt(),
            Self::Abs => argument.abs(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_function_round_trips_through_its_name() {
        for function in MathFunction::ALL {
            assert_eq!(MathFunction::from_name(function.name()), Some(function));
        }
    }
}
