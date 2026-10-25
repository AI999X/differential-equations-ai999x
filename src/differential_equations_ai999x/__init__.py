"""differential-equations-ai999x — offline differential equations answers with their formula, steps and verification.

    import differential_equations_ai999x as m
    m.ask("Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).").show()       # y = 0.600423599106
    m.solve("runge_kutta_4", f="t + y", t0=0, y0=1, h=0.1, n=2).show()                   # y_n = 1.2428051417
    m.capabilities()                   # every certified capability

Covers first-order models (exponential, logistic, Gompertz, Newton cooling, mixing, drag, Torricelli, separable,
Bernoulli, linear with integrating factor), second-order linear equations (characteristic roots, damping cases,
undetermined coefficients, resonance, Cauchy-Euler, boundary value problems), Laplace transforms (table transforms,
shifting, inverse by residues, partial fractions, convolution, impulse and step responses), linear systems and phase
portraits (trace-determinant classification, e^{At} x0, Routh-Hurwitz), stability of equilibria (phase line,
linearization, Lotka-Volterra, competing species), numerical methods (Euler, Heun, midpoint, RK4, backward Euler,
stability functions and step limits, CFL and FTCS limits, explicit and Crank-Nicolson heat steps), classic PDEs by
separation of variables (heat and wave series, d'Alembert, Laplace on a rectangle, Sturm-Liouville and Robin
eigenvalues) and dynamical systems (logistic map cycles and Lyapunov exponent, Lorenz, Brusselator, SIR).
Expressions such as f(t, y) are read by an allowlisted parser, never evaluated as code. Built by the AI999X model
machine from the same engine as every domain; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.0"
_P = Product("differential_equations", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
