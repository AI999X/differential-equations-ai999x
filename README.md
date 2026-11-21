# Differential Equations (differential-equations-ai999x)

[![tests](https://github.com/AI999X/differential-equations-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/differential-equations-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/differential-equations-ai999x)](https://pypi.org/project/differential-equations-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/differential-equations-ai999x)

Differential Equations covers first- and second-order ODEs, linear systems, Laplace transforms, stability and phase portraits, numerical methods, classic PDEs (heat, wave, Laplace) and dynamical systems: 78 calculations in 7 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Differential Equations desktop app: Second-order homogeneous linear ODE: initial value problem, with its worked example and live scene](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/hero.webp)](https://www.ai999x.com/models/differential-equations-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-differential-equations-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-differential-equations-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `differential-equations` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/differential-equations-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install differential-equations-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Differential Equations desktop app: every scene and chart is drawn from the computed answer.

[![Differential Equations desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/clip.mp4)*

| | |
|---|---|
| [![Second-order homogeneous linear ODE: initial value problem: the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-1.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-second_order_homogeneous_ivp)<br>Second-order homogeneous linear ODE: initial value problem | [![Logistic growth: population at a time: the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-2.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-logistic_value)<br>Logistic growth: population at a time |
| [![Inverse Laplace transform of a rational function: the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-3.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-inverse_laplace_transform)<br>Inverse Laplace transform of a rational function | [![Classical Runge-Kutta method (RK4): the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-4.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-runge_kutta_4)<br>Classical Runge-Kutta method (RK4) |
| [![Linear 2x2 system: equilibrium type and stability (trace-determinant plane): the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-5.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-phase_portrait_2x2)<br>Linear 2x2 system: equilibrium type and stability (trace-determinant plane) | [![Wave equation on the whole line: d'Alembert's formula: the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-6.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-dalembert_solution)<br>Wave equation on the whole line: d'Alembert's formula |
| [![Competing species: coexistence equilibrium and its stability: the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-7.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-competing_species_equilibrium)<br>Competing species: coexistence equilibrium and its stability | [![Bernoulli equation y' + p y = q y^n: the live scene in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-8.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-bernoulli_equation)<br>Bernoulli equation y' + p y = q y^n |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Second-order homogeneous linear ODE: initial value problem: a chart in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/chart-1.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-second_order_homogeneous_ivp)<br>Second-order homogeneous linear ODE: initial value problem | [![Logistic growth: population at a time: a chart in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/chart-2.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-logistic_value)<br>Logistic growth: population at a time | [![Inverse Laplace transform of a rational function: a chart in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/chart-3.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-inverse_laplace_transform)<br>Inverse Laplace transform of a rational function | [![Classical Runge-Kutta method (RK4): a chart in the Differential Equations desktop app](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/chart-4.webp)](https://www.ai999x.com/models/differential-equations-ai999x/#cap-runge_kutta_4)<br>Classical Runge-Kutta method (RK4) |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import differential_equations_ai999x as differential_equations

a = differential_equations.ask(
    "Radiocarbon dating: half-life T = 5730 and fraction remaining p = "
    "0.25. What is the age?"
)
print(a.outcome, a.values)
# SOLVED {'t': 11460.0}

a = differential_equations.ask(
    "Find the convolution of f(t) = exp(-t) and g(t) = sin(t) at t = "
    "2."
)
print(a.outcome, a.values)
# SOLVED {'value': 0.730389773305}

a = differential_equations.ask(
    "Use the backward Euler method for dy/dt = -2*y + t, y(0) = 1, h = "
    "0.1, n = 2."
)
print(a.outcome, a.values)
# SOLVED {'y_n': 0.718055555556, 't_n': 0.2}

a = differential_equations.ask(
    "Solve the Cauchy-Euler equation t^2 y'' + 2t y' - 6y = 0 with "
    "y(1) = 1, y'(1) = 0; find y(2)."
)
print(a.outcome, a.values)
# SOLVED {'y': 2.45, 'm1': -3.0, 'm2': 2.0}

a = differential_equations.ask(
    "Find the Robin eigenvalue with L = 1, h = 1, n = 1."
)
print(a.outcome, a.values)
# SOLVED {'lambda_n': 4.11585836569, 'sqrt_lambda': 2.02875783811}

a = differential_equations.ask(
    "Solve the Bernoulli equation y' + p y = q y^n with p = 1, q = 1, "
    "n = 2, y(0) = 0.5; find y(1)."
)
print(a.outcome, a.values)
# SOLVED {'y': 0.26894142137}

a = differential_equations.ask(
    "Find the impulse response of y'' + 2y' + 5y = delta(t) at t = 1."
)
print(a.outcome, a.values)
# SOLVED {'h': 0.16725591462}

a = differential_equations.ask(
    "Check the CFL condition for c = 2, dt = 0.01, dx = 0.05."
)
print(a.outcome, a.values)
# SOLVED {'courant': 0.4, 'stable': True, 'dt_max': 0.025}

a = differential_equations.ask(
    "Find the Fourier sine coefficient of f(x) = x*(1 - x) with L = 1, "
    "n = 1."
)
print(a.outcome, a.values)
# SOLVED {'b_n': 0.258012275466}

a = differential_equations.ask(
    "Find the doubling time for exponential growth with k = 0.07."
)
print(a.outcome, a.values)
# SOLVED {'t_char': 9.90210257943, 'kind': 'doubling time'}

a = differential_equations.ask(
    "Use RK4 for dy/dt = t + y, y(0) = 1, h = 0.1, n = 2."
)
print(a.outcome, a.values)
# SOLVED {'y_n': 1.2428051417, 't_n': 0.2}

a = differential_equations.ask(
    "Heat equation series with f(x) = x*(1 - x), L = 1, alpha = 1, N = "
    "10, x = 0.5, t = 0.1."
)
print(a.outcome, a.values)
# SOLVED {'u': 0.0961618714343, 'b_1': 0.258012275466}

a = differential_equations.ask(
    "A draining tank obeys Torricelli's law with h0 = 4, k = 0.2; find "
    "h(5) and the emptying time."
)
print(a.outcome, a.values)
# SOLVED {'h': 2.25, 't_empty': 20.0}

a = differential_equations.ask(
    "Find the Laplace transform of the delayed function u(t - c) f(t - "
    "c) with f(t) = t, c = 2, s = 1."
)
print(a.outcome, a.values)
# SOLVED {'F': 0.135335283237, 'G': 1.0}

a = differential_equations.ask(
    "Solve the boundary value problem y'' + p y' + q y = r with p = 0, "
    "q = -1, r = 0, xa = 0, xb = 1, ya = 0, yb = 1, x = 0.5."
)
print(a.outcome, a.values)
# SOLVED {'y': 0.443409441985, 'slope_a': 0.850918128239}
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Differential Equations on ai999x.com](https://www.ai999x.com/models/differential-equations-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/differential-equations-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/differential-equations-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `differential-equations` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/differential-equations`) and its data (`rust/models/differential_equations/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p differential-equations-ai999x
./target/release/differential-equations --help
cargo test -p differential-equations-ai999x
```
