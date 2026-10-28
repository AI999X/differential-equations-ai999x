# differential-equations-ai999x — offline, verified differential equations solver with step-by-step answers

[![tests](https://github.com/AI999X/differential-equations-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/differential-equations-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/differential-equations-ai999x)](https://pypi.org/project/differential-equations-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/differential-equations-ai999x)

**[Install](https://pypi.org/project/differential-equations-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/differential-equations-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/differential-equations-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.3

Field: **Mathematics & Statistics** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

differential-equations-ai999x is an offline, deterministic Python solver that answers differential equations questions with verified results from 77 certified capabilities, gives 269 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**differential-equations-ai999x** is a compact, deterministic differential equations engine for Python. It reads a question in plain English (or a typed call), picks a certified differential equations capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified differential equations capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 269 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** first- and second-order ODEs, Laplace transforms, linear systems and phase portraits, stability, numerical methods, heat/wave/Laplace equations and dynamical systems.

## Model overview

| Property | Value |
|---|---|
| Package | `differential-equations-ai999x` 1.0.3 (import `differential_equations_ai999x`) |
| Type | Deterministic neuro-symbolic differential equations engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 77 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 269 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install differential-equations-ai999x
```

```python
import differential_equations_ai999x as differential_equations

differential_equations.ask("Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).").show()
# -> SOLVED — y = 0.600424, damping = overdamped  (second_order_homogeneous_ivp)
differential_equations.ask("Logistic growth with P0 = 10, r = 0.5, K = 1000; find P(8).").show()
# -> SOLVED — P = 355.461  (logistic_value)
differential_equations.ask("Find the inverse Laplace transform of (s + 3)/(s^2 + 3s + 2) at t = 1.").show()
# -> SOLVED — f = 0.600424, poles = [-2.0, -1.0]  (inverse_laplace_transform)
differential_equations.solve("runge_kutta_4", f="t + y", t0=0, y0=1, h=0.1, n=2).show()
# -> SOLVED — y_n = 1.24281, t_n = 0.2  (runge_kutta_4)
differential_equations.ask("Classify the equilibrium of the linear system x' = A x with A = [[0, 1], [-2, -3]].").show()
# -> SOLVED — type = stable node, stability = asymptotically stable, lambda1 = -2, lambda2 = -1  (phase_portrait_2x2)
differential_equations.ask("Who created you?").message
# -> differential-equations-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `differential-equations-ai999x "<question>"`, `differential-equations-ai999x --capabilities`. The outputs above were produced by running this exact code on differential-equations-ai999x 1.0.3.

## Find the right capability, and draw it

`differential_equations.search()` finds which certified capability a question is about — across **every** AI999X model, so a question for another model points you to it. It runs **[ai999x-encoder](https://huggingface.co/AI999X/ai999x-encoder)**, AI999X's own capability encoder (4.7M parameters, a 4.9 MB ONNX model, offline; the root `model.safetensors` of this repository): `pip install "differential-equations-ai999x[search]"`. `ask(question, suggest=True)` adds the same pointers to a question it does not answer; an answer itself never changes.

```python
import differential_equations_ai999x as differential_equations

differential_equations.search("Solve x^2 - 5x + 6 = 0", k=2)
# -> Roots of a quadratic equation (math-foundations-ai999x, 0.63); Roots of a cubic equation (math-foundations-ai999x, 0.59)
```

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Worked examples

Each question was put to `differential-equations-ai999x`; the answer is the one it returned, with the steps it took.

**Second-order homogeneous linear ODE: initial value problem** · _Second-order ODES_

> Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).

→ **y = 0.6004235991, damping = overdamped**

<details><summary>Steps</summary>

- Second-order homogeneous linear ODE: initial value problem: a = 1, b = 3, c = 2, y0 = 1, v0 = 0, t = 1 → y = 0.6004235991, damping = overdamped

</details>

**Logistic growth: population at a time** · _First-order ODES_

> Logistic growth with P0 = 10, r = 0.5, K = 1000; find P(8).

→ **P = 355.4609871**

<details><summary>Steps</summary>

- Logistic growth: population at a time: P0 = 10, r = 0.5, K = 1000, t = 8 → P = 355.4609871

</details>

**Inverse Laplace transform of a rational function** · _Laplace transforms_

> Find the inverse Laplace transform of (s + 3)/(s^2 + 3s + 2) at t = 1.

→ **f = 0.6004235991, poles = [-2, -1]**

<details><summary>Steps</summary>

- Inverse Laplace transform of a rational function: F = (s + 3)/(s^2 + 3s + 2), t = 1 → f = 0.6004235991, poles = [-2, -1]

</details>

**Classical Runge-Kutta method (RK4)** · _Numerical methods_

`m.solve("runge_kutta_4", f="t + y", t0=0, y0=1, h=0.1, n=2)`

→ **y_n = 1.242805142, t_n = 0.2**

<details><summary>Steps</summary>

- Classical Runge-Kutta method (RK4): f = t + y, t0 = 0, y0 = 1, h = 0.1, n = 2 → y_n = 1.242805142, t_n = 0.2

</details>

**Linear 2x2 system: equilibrium type and stability (trace-determinant plane)** · _Linear systems_

> Classify the equilibrium of the linear system x' = A x with A = [[0, 1], [-2, -3]].

→ **type = stable node, stability = asymptotically stable, lambda1 = -2, lambda2 = -1, trace = -3, det = 2**

<details><summary>Steps</summary>

- Linear 2x2 system: equilibrium type and stability (trace-determinant plane): A = [0 1; -2 -3] → type = stable node, stability = asymptotically stable, lambda1 = -2, lambda2 = -1, trace = -3, det = 2

</details>

**Wave equation on the whole line: d'Alembert's formula** · _Classic PDES_

> Use d'Alembert's formula with f(x) = sin(x), g(x) = 0, c = 2, x = 1, t = 0.5.

→ **u = 0.4546487134**

<details><summary>Steps</summary>

- Wave equation on the whole line: d'Alembert's formula: f = sin(x), g = 0, c = 2, x = 1, t = 0.5 → u = 0.4546487134

</details>

## Figures

Every point was computed by `differential-equations-ai999x`.

**Phase portrait of the damped pendulum**

![Phase portrait of the damped pendulum — computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/nonlinear-equilibrium-type-phase.svg)

_Phase portrait of the damped pendulum x′ = y, y′ = -sin(x) - 0.5*y: 12 trajectories integrated by the package's fourth-order Runge–Kutta system solver (step 0.05, t = 0 … 16), equilibria classified by it from the Jacobian (x = 0: stable spiral; x = 3.14159: saddle; the pattern repeats every 2π)._

**Slope field and solutions of the logistic equation**

![Slope field and solutions of the logistic equation — computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/slope-field-phase.svg)

_Slope field of y′ = y*(1 - y) (direction at 187 grid points, each slope from the package) with five solutions integrated by its fourth-order Runge–Kutta solver (step 0.05); equilibria found by the package: y = 0 (unstable), y = 1 (stable)._

**Equilibrium types on the trace–determinant plane**

![Equilibrium types on the trace–determinant plane — computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/phase-portrait-2x2-stability.svg)

_Eight 2×2 systems x′ = A x placed by trace and determinant, each classified by the package: [[0, 1], [-2, -3]]: stable node (trace -3, det 2); [[-1, 2], [-2, -1]]: stable spiral (trace -2, det 5); [[1, 2], [-2, 1]]: unstable spiral (trace 2, det 5); [[2, 1], [0, 3]]: unstable node (trace 5, det 6); [[0, 1], [-4, 0]]: center (trace 0, det 4); [[1, 0], [0, -2]]: saddle (trace -1, det -2); [[-1, 3], [1, 1]]: saddle (trace 0, det -4); [[0.5, 3], [-3, 0.5]]: unstable spiral (trace 1, det 9.25)._

**Bifurcation of the logistic map: fixed points and 2-cycles**

![Bifurcation of the logistic map: fixed points and 2-cycles — computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/logistic-map-two-cycle-bifurcation.svg)

_Period-doubling of the logistic map x → r x(1 − x): its fixed points and 2-cycles for 69 values of r from 0.05 to 3.45, with their stability, all from the package; the fixed point loses stability at r = 3 where a stable 2-cycle appears — marked: r = 3.2, cycle 0.513045 ↔ 0.799455 (multiplier 0.16)._

**Heat spreading along a rod, step by step**

![Heat spreading along a rod, step by step — computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/heat-ftcs-steps-heatmap.svg)

_Temperatures along a rod (9 nodes, ends held at 0) from the explicit heat-equation scheme with r = 0.25, starting from 0, 0, 20, 60, 100, 60, 20, 0, 0: row k is the state after k steps, each computed by the package; the peak spreads and decays (100 → 47.79 at the centre after 7 steps)._

**The three damping regimes of a second-order equation**

![The three damping regimes of a second-order equation — computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/second-order-homogeneous-ivp-curve.svg)

_Solutions of y″ + b y′ + y = 0 with y(0) = 1, y′(0) = 0, evaluated by the package at 121 times each: b = 0.4 (underdamped, y(12) = 0.0492303); b = 2 (critically damped, y(12) = 7.9875e-05); b = 4 (overdamped, y(12) = 0.0432445)._

## Capabilities

77 certified procedures. A few of them:

- **Procedures:** Age from the fraction remaining (radioactive dating); Backward (implicit) Euler method; Bernoulli equation y' + p y = q y^n; Brusselator: equilibrium, its type, and the Hopf value b = 1 + a^2; CFL condition for the advection equation (upwind scheme); Cauchy-Euler (equidimensional) equation: initial value problem; Classical Runge-Kutta method (RK4); Competing species: coexistence equilibrium and its stability; Convolution integral (f * g)(t); Crank-Nicolson steps for the heat equation; Damped oscillator: resonance frequency and peak amplitude; Doubling time or half-life from a rate constant; Draining tank (Torricelli's law): height and emptying time; Eigenvalues of y'' + lambda y = 0 with Dirichlet/Neumann ends; Eigenvalues with a Robin (convective) end condition; Equation y' = k y^2: solution and blow-up time; Equilibria of y' = f(y) and their stability (phase line); Euler's method; Euler's method error for exponential growth; Explicit finite-difference (FTCS) steps for the heat equation; Explicit midpoint method (RK2); Exponential growth or decay: value at a time; Exponential model through two measurements; Falling body with linear drag: velocity …

`differential_equations_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified differential equations capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (first- and second-order ODEs, Laplace transforms, linear systems and phase portraits, stability, numerical methods, heat/wave/Laplace equations and dynamical systems); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{differential_equations_ai999x_1_0_3,
  title  = {differential-equations-ai999x: an offline, verified differential equations solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.3},
  url    = {https://huggingface.co/AI999X/differential-equations-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/differential-equations-ai999x/discussions or visit https://ai999x.com.

## Support the project

differential-equations-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/differential-equations-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/differential-equations-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/differential-equations-ai999x/tree/main/onnx).
