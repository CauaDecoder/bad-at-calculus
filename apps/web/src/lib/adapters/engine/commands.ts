/** Engine command names — keep in sync with `crates/mv-api/src/lib.rs` (`commands`). */
export const Commands = {
  expressionLatex: 'expression.latex',
  calculusSample: 'calculus.sample',
  calculusTangent: 'calculus.tangent',
  calculusDerivative: 'calculus.derivative',
  calculusCriticalPoints: 'calculus.criticalPoints',
  calculusLimit: 'calculus.limit',
  calculusIntegral: 'calculus.integral',
  vectorsAnalyze: 'vectors.analyze',
  geometryPlane: 'geometry.plane',
  matrixAnalyze: 'matrix.analyze',
  matrixSolve: 'matrix.solve',
} as const;

export type Command = (typeof Commands)[keyof typeof Commands];
