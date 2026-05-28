declare module '../../pkg/train_solver.js' {
  export type Model = unknown;
  export type ObjectiveType = {
    Maximize: unknown;
    Minimize: unknown;
  };
  export type ConstraintType = {
    LessEq: unknown;
    Eq: unknown;
    GreaterEq: unknown;
  };
  const init: () => Promise<void>;
  const solveModel: (model: Model) => unknown;
  export { init, ObjectiveType, ConstraintType, Model, solveModel };
}
