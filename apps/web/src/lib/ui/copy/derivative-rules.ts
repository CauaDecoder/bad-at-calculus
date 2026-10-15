/** Names and formulas of the differentiation rules, as taught in Cálculo A. */

export interface RuleDescription {
  readonly name: string;
  readonly formula: string;
}

const RULES: Record<string, RuleDescription> = {
  constant: { name: 'Derivada da constante', formula: "(c)' = 0" },
  sum: { name: 'Regra da soma', formula: "(f + g)' = f' + g'" },
  difference: { name: 'Regra da diferença', formula: "(f - g)' = f' - g'" },
  negation: { name: 'Sinal negativo', formula: "(-f)' = -f'" },
  'constant-multiple': { name: 'Constante multiplicando', formula: "(c\\,f)' = c\\,f'" },
  product: { name: 'Regra do produto', formula: "(f\\,g)' = f'g + f\\,g'" },
  quotient: {
    name: 'Regra do quociente',
    formula: "\\left(\\frac{f}{g}\\right)' = \\frac{f'g - f\\,g'}{g^{2}}",
  },
  power: { name: 'Regra da potência', formula: "(x^{n})' = n\\,x^{n-1}" },
  exponential: { name: 'Exponencial de base a', formula: "(a^{x})' = a^{x}\\ln a" },
  'natural-exponential': { name: 'Exponencial natural', formula: "(e^{x})' = e^{x}" },
  'general-power': {
    name: 'Potência geral',
    formula: "(u^{v})' = u^{v}\\left(v'\\ln u + \\frac{v\\,u'}{u}\\right)",
  },
  'natural-logarithm': { name: 'Logaritmo natural', formula: "(\\ln x)' = \\frac{1}{x}" },
  'logarithm-10': {
    name: 'Logaritmo na base 10',
    formula: "(\\log_{10} x)' = \\frac{1}{x\\ln 10}",
  },
  sine: { name: 'Derivada do seno', formula: "(\\sin x)' = \\cos x" },
  cosine: { name: 'Derivada do cosseno', formula: "(\\cos x)' = -\\sin x" },
  tangent: { name: 'Derivada da tangente', formula: "(\\tan x)' = \\sec^{2} x" },
  'square-root': { name: 'Raiz quadrada', formula: "(\\sqrt{x})' = \\frac{1}{2\\sqrt{x}}" },
  'cube-root': { name: 'Raiz cúbica', formula: "(\\sqrt[3]{x})' = \\frac{1}{3\\sqrt[3]{x^{2}}}" },
  'absolute-value': { name: 'Valor absoluto', formula: "(|x|)' = \\frac{x}{|x|},\\ x \\ne 0" },
  piecewise: { name: 'Função por partes', formula: '\\text{deriva cada parte}' },
};

export const CHAIN_RULE: RuleDescription = {
  name: 'Regra da cadeia',
  formula: "(f(u))' = f'(u)\\cdot u'",
};

export function describeRule(rule: string): RuleDescription {
  return RULES[rule] ?? { name: rule, formula: '' };
}
