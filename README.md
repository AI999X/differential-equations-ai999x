# differential-equations-ai999x: offline, verified differential equations solver with step-by-step answers

[![tests](https://github.com/AI999X/differential-equations-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/differential-equations-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/differential-equations-ai999x)](https://pypi.org/project/differential-equations-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/differential-equations-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/differential-equations-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/differential-equations-ai999x/)** · **[Guides](https://huggingface.co/AI999X/differential-equations-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/differential-equations-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.5.0

Field: **Mathematics & Statistics** · Tasks: Question Answering

differential-equations-ai999x is an offline, deterministic differential equations solver written in Rust: a desktop app, a command-line program, a browser module and a Python package. It answers differential equations questions with verified results from 77 certified capabilities, gives 269 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**differential-equations-ai999x** reads a question in plain English (or a typed call), picks a certified differential equations capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions, or a refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** differential-equations-ai999x comes as the AI999X desktop app and as the command-line program `differential-equations` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Differential Equations desktop app** 1.5.0 | Windows, Linux. This model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-differential-equations-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-differential-equations-1.5.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.5.0 | Windows, Linux. Every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-1.5.0-linux-x86_64.tar.gz) |
| **Command line** (`differential-equations`) | Windows, Linux. One program per model, offline, no Python | [Windows (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-windows-x86_64.zip) · [Linux (x86-64)](https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz) |
| **In your browser** | any device. Nothing to install | [Compute on the model page](https://www.ai999x.com/models/differential-equations-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install differential-equations-ai999x` |

Checksums: [SHA256SUMS](https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS) · every version: [https://downloads.ai999x.com/desktop/latest/](https://downloads.ai999x.com/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Quick start

### Desktop app

Download **AI999X Differential Equations** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Differential Equations among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-differential-equations-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-differential-equations-1.5.0-linux-x86_64.tar.gz
./ai999x-differential-equations-1.5.0-linux-x86_64/ai999x-differential-equations
```

### Command line

Linux:

```bash
curl -LO https://downloads.ai999x.com/desktop/1.5.0/ai999x-cli-1.5.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.5.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.5.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
differential-equations --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.5.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.5.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.5.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
differential-equations --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words**: Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ differential-equations ask "Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1)."
Read as: Second-order homogeneous linear ODE: initial value problem
y = 0.6004235991
damping = overdamped
  by: Second-order homogeneous linear ODE: initial value problem  (y = e^{mt}[y0 C(t) + (v0 - m y0) S(t)], m = -b/(2a), C, S = cosh/sinh, 1/t or cos/sin by the sign of the discriminant)
  a = 1
  b = 3
  c = 2
  t = 1
  v0 = 0
  y0 = 1
  assumes: a != 0
```

**What a capability needs**: The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ differential-equations info second_order_homogeneous_ivp
Second-order homogeneous linear ODE: initial value problem
  y = e^{mt}[y0 C(t) + (v0 - m y0) S(t)], m = -b/(2a), C, S = cosh/sinh, 1/t or cos/sin by the sign of the discriminant
  in   a        coefficient of y'' [1]
  in   b        coefficient of y' [1]
  in   c        coefficient of y [1]
  in   y0       initial value y(0) [1]
  in   v0       initial slope y'(0) [1]
  in   t        time [1]
  out  y        value y(t) [1]
  out  damping  damping case []
```

**Solve**: Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ differential-equations solve second_order_homogeneous_ivp a=1.0 b=3.0 c=2.0 y0=1.0 v0=0.0 t=1.0
y = 0.6004235991
damping = overdamped
  by: Second-order homogeneous linear ODE: initial value problem  (y = e^{mt}[y0 C(t) + (v0 - m y0) S(t)], m = -b/(2a), C, S = cosh/sinh, 1/t or cos/sin by the sign of the discriminant)
  a = 1
  b = 3
  c = 2
  y0 = 1
  v0 = 0
  t = 1
  assumes: a != 0
```

**Many questions at once**: One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | differential-equations batch

`cases.jsonl`:

```json
{"key": "second_order_homogeneous_ivp", "inputs": {"a": 1.0, "b": 3.0, "c": 2.0, "y0": 1.0, "v0": 0.0, "t": 1.0}}
{"key": "logistic_value", "inputs": {"P0": 10.0, "r": 0.5, "K": 1000.0, "t": 8.0}}
```

```console
$ differential-equations batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.600423599106,"unit":"","solved_for":"y","si":[["a",1.0],["b",3.0],["c",2.0],["y0",1.0],["v0",0.0],["t",1.0]],"values":[["y",0.600423599106,""],["damping","overdamped",""]],"target":"y","phrase":"value y(t)","assumptions":["a != 0"],"name":"Second-order homogeneous linear ODE: initial value problem","equation":"y = e^{mt}[y0 C(t) + (v0 - m y0) S(t)], m = -b/(2a), C, S = cosh/sinh, 1/t or cos/sin by the sign of the discriminant"}
{"outcome":"SOLVED","message":"","value":355.460987137,"unit":"","solved_for":"P","si":[["P0",10.0],["r",0.5],["K",1000.0],["t",8.0]],"values":[["P",355.460987137,""]],"target":"P","phrase":"population P(t)","assumptions":["P0 > 0","r > 0","K > 0"],"name":"Logistic growth: population at a time","equation":"P = K / (1 + (K/P0 - 1) e^{-r t})"}
```

### In your browser

Open [Differential Equations on ai999x.com](https://www.ai999x.com/models/differential-equations-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `differential_equations_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install differential-equations-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Eigenvalues of y'' + lambda y = 0 with Dirichlet/Neumann ends

The n-th eigenvalue and eigenfunction of y'' + lambda y = 0 on [0, L] with Dirichlet (y = 0) or Neumann (y' = 0) conditions at each end.

Command line:

```console
$ differential-equations solve sturm_liouville_eigenvalues L=1 n=2 bc=dirichlet-dirichlet
lambda_n = 39.4784176
sqrt_lambda = 6.283185307
eigenfunction = sin(6.28318530718 x)
  by: Eigenvalues of y'' + lambda y = 0 with Dirichlet/Neumann ends  (sin/cos solutions; the boundary conditions quantise sqrt(lambda))
  L = 1
  bc = dirichlet-dirichlet
  assumes: L > 0; n >= 1
```

Python:

```python
import differential_equations_ai999x as m

a = m.solve("sturm_liouville_eigenvalues", L="1", n="2", bc="dirichlet-dirichlet")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/differential-equations-ai999x/#cap-sturm_liouville_eigenvalues) and press **Compute in your browser**.

_What this means:_ This calculation finds the specific energy levels for a vibrating string or heat distribution on an interval of length 1. Applying boundary conditions at both ends shows that the n-th mode has an eigenvalue of 39.4784. The corresponding spatial pattern, known as the eigenfunction, oscillates with a frequency proportional to 6.28319, which dictates how many half-waves fit within the domain.

### Eigenvalues with a Robin (convective) end condition

The n-th eigenvalue of y'' + lambda y = 0, y(0) = 0, y'(L) + h y(L) = 0 (heat loss at one end): root of tan(mu L) = -mu/h.

Command line:

```console
$ differential-equations solve robin_eigenvalues L=1 h=1 n=1
lambda_n = 4.115858366
sqrt_lambda = 2.028757838
  by: Eigenvalues with a Robin (convective) end condition  (bisection for mu cos(mu L) + h sin(mu L) = 0 in ((n - 1/2) pi/L, n pi/L))
  L = 1
  h = 1
  assumes: L > 0; h > 0
```

Python:

```python
import differential_equations_ai999x as m

a = m.solve("robin_eigenvalues", L="1", h="1", n="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/differential-equations-ai999x/#cap-robin_eigenvalues) and press **Compute in your browser**.

_What this means:_ This calculation finds the nth eigenvalue for a rod of length 1 losing heat at one end with coefficient 1. The result 4.11586 corresponds to the frequency parameter 2.02876, derived from the transcendental equation governing convective boundary conditions.

### Fourier sine series coefficient (half-range expansion)

The coefficient b_n of the sine series of f(x) on [0, L], used for the heat and wave equations with fixed ends.

Command line:

```console
$ differential-equations solve fourier_sine_coefficient "f=x*(1 - x)" L=1 n=1
b_n = 0.2580122755
  by: Fourier sine series coefficient (half-range expansion)  (exact integration by parts of x^k sin(n pi x / L))
  L = 1
  f = x*(1 - x)
  assumes: f a polynomial; L > 0
```

Python:

```python
import differential_equations_ai999x as m

a = m.solve("fourier_sine_coefficient", f="x*(1 - x)", L="1", n="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/differential-equations-ai999x/#cap-fourier_sine_coefficient) and press **Compute in your browser**.

_What this means:_ This calculation determines the Fourier sine series coefficient, denoted as 0.258012, by integrating the product of the initial profile x*(1 - x) and the sine basis function over the domain defined by length 1. The result represents the amplitude of the nth mode in the half-range expansion, which is crucial for solving boundary value problems such as the heat or wave equations with fixed ends. The computation assumes that x*(1 - x) is a polynomial and 1 is strictly positive.

### Heat equation on a rod with ends at zero: Fourier series value

u(x, t) for u_t = alpha u_xx, u(0, t) = u(L, t) = 0, u(x, 0) = f(x) (a polynomial), summed over the first N series terms.

Command line:

```console
$ differential-equations solve heat_equation_series "f=x*(1 - x)" L=1 alpha=1 N=10 x=0.5 t=0.1
u = 0.09616187143
b_1 = 0.2580122755
  by: Heat equation on a rod with ends at zero: Fourier series value  (separation of variables: u = sum b_n e^{-alpha (n pi/L)^2 t} sin(n pi x/L))
  L = 1
  alpha = 1
  x = 0.5
  t = 0.1
  f = x*(1 - x)
  assumes: f a polynomial; Dirichlet ends at 0; series truncated after N terms (stated)
```

Python:

```python
import differential_equations_ai999x as m

a = m.solve("heat_equation_series", f="x*(1 - x)", L="1", alpha="1", N="10", x="0.5", t="0.1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/differential-equations-ai999x/#cap-heat_equation_series) and press **Compute in your browser**.

_What this means:_ This calculation determines the temperature distribution along a rod of length 1 with diffusivity 1, given an initial polynomial profile. By applying separation of variables, the solution is expressed as a truncated Fourier sine series with 10 terms. The resulting temperature value at the specified location and time is 0.0961619, while the first coefficient in this series expansion is 0.258012. This method assumes both ends of the rod are held at zero temperature throughout the process.

## Documentation

- **Rust user guide**: the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_rust_guide.md)
- **Python guide**: every capability with a Python example: [PDF](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/differential-equations-ai999x/blob/main/docs/differential_equations_ai999x_capabilities.md)
- **Desktop app & command line**: downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page**: compute in your browser, every capability explained: https://www.ai999x.com/models/differential-equations-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified differential equations capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes (SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR), never a silent guess. Every one of its 269 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `differential-equations` command line for Windows and Linux, and WebAssembly on the model page, no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** first- and second-order ODEs, Laplace transforms, linear systems and phase portraits, stability, numerical methods, heat/wave/Laplace equations and dynamical systems.

## Model overview

| Property | Value |
|---|---|
| Package | `differential-equations-ai999x` 1.5.0 (import `differential_equations_ai999x`) |
| Type | Deterministic neuro-symbolic differential equations engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 77 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 269 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `differential-equations-ai999x "<question>"`; for the native program use `differential-equations` (above). The outputs above were produced by running this exact code on differential-equations-ai999x 1.5.0.

## How every answer is verified

1. **Reading**: the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning**: only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing**: exact symbolic or high-precision numeric evaluation.
4. **Checking**: dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

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

![Phase portrait of the damped pendulum, computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/nonlinear-equilibrium-type-phase.svg)

_Phase portrait of the damped pendulum x′ = y, y′ = -sin(x) - 0.5*y: 12 trajectories integrated by the package's fourth-order Runge–Kutta system solver (step 0.05, t = 0 … 16), equilibria classified by it from the Jacobian (x = 0: stable spiral; x = 3.14159: saddle; the pattern repeats every 2π)._

**Slope field and solutions of the logistic equation**

![Slope field and solutions of the logistic equation, computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/slope-field-phase.svg)

_Slope field of y′ = y*(1 - y) (direction at 187 grid points, each slope from the package) with five solutions integrated by its fourth-order Runge–Kutta solver (step 0.05); equilibria found by the package: y = 0 (unstable), y = 1 (stable)._

**Equilibrium types on the trace–determinant plane**

![Equilibrium types on the trace–determinant plane, computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/phase-portrait-2x2-stability.svg)

_Eight 2×2 systems x′ = A x placed by trace and determinant, each classified by the package: [[0, 1], [-2, -3]]: stable node (trace -3, det 2); [[-1, 2], [-2, -1]]: stable spiral (trace -2, det 5); [[1, 2], [-2, 1]]: unstable spiral (trace 2, det 5); [[2, 1], [0, 3]]: unstable node (trace 5, det 6); [[0, 1], [-4, 0]]: center (trace 0, det 4); [[1, 0], [0, -2]]: saddle (trace -1, det -2); [[-1, 3], [1, 1]]: saddle (trace 0, det -4); [[0.5, 3], [-3, 0.5]]: unstable spiral (trace 1, det 9.25)._

**Bifurcation of the logistic map: fixed points and 2-cycles**

![Bifurcation of the logistic map: fixed points and 2-cycles, computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/logistic-map-two-cycle-bifurcation.svg)

_Period-doubling of the logistic map x → r x(1 − x): its fixed points and 2-cycles for 69 values of r from 0.05 to 3.45, with their stability, all from the package; the fixed point loses stability at r = 3 where a stable 2-cycle appears, marked: r = 3.2, cycle 0.513045 ↔ 0.799455 (multiplier 0.16)._

**Heat spreading along a rod, step by step**

![Heat spreading along a rod, step by step, computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/heat-ftcs-steps-heatmap.svg)

_Temperatures along a rod (9 nodes, ends held at 0) from the explicit heat-equation scheme with r = 0.25, starting from 0, 0, 20, 60, 100, 60, 20, 0, 0: row k is the state after k steps, each computed by the package; the peak spreads and decays (100 → 47.79 at the centre after 7 steps)._

**The three damping regimes of a second-order equation**

![The three damping regimes of a second-order equation, computed by differential-equations-ai999x](https://huggingface.co/AI999X/differential-equations-ai999x/resolve/main/figures/second-order-homogeneous-ivp-curve.svg)

_Solutions of y″ + b y′ + y = 0 with y(0) = 1, y′(0) = 0, evaluated by the package at 121 times each: b = 0.4 (underdamped, y(12) = 0.0492303); b = 2 (critically damped, y(12) = 7.9875e-05); b = 4 (overdamped, y(12) = 0.0432445)._

## Capabilities

77 certified procedures. A few of them:

- **Procedures:** Age from the fraction remaining (radioactive dating); Backward (implicit) Euler method; Bernoulli equation y' + p y = q y^n; Brusselator: equilibrium, its type, and the Hopf value b = 1 + a^2; CFL condition for the advection equation (upwind scheme); Cauchy-Euler (equidimensional) equation: initial value problem; Classical Runge-Kutta method (RK4); Competing species: coexistence equilibrium and its stability; Convolution integral (f * g)(t); Crank-Nicolson steps for the heat equation; Damped oscillator: resonance frequency and peak amplitude; Doubling time or half-life from a rate constant; Draining tank (Torricelli's law): height and emptying time; Eigenvalues of y'' + lambda y = 0 with Dirichlet/Neumann ends; Eigenvalues with a Robin (convective) end condition; Equation y' = k y^2: solution and blow-up time; Equilibria of y' = f(y) and their stability (phase line); Euler's method; Euler's method error for exponential growth; Explicit finite-difference (FTCS) steps for the heat equation; Explicit midpoint method (RK2); Exponential growth or decay: value at a time; Exponential model through two measurements; Falling body with linear drag: velocity …

`differential_equations_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified differential equations capabilities, and any question whose stated numbers it cannot all use: it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (first- and second-order ODEs, Laplace transforms, linear systems and phase portraits, stability, numerical methods, heat/wave/Laplace equations and dynamical systems); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused; use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{differential_equations_ai999x_1_5_0,
  title  = {differential-equations-ai999x: an offline, verified differential equations solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.5.0},
  url    = {https://huggingface.co/AI999X/differential-equations-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/differential-equations-ai999x/discussions or visit https://ai999x.com.

## Support the project

differential-equations-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/differential-equations-ai999x/) (`pip install .` builds it). Releases are tagged `v1.5.0`; the [GitHub release](https://github.com/AI999X/differential-equations-ai999x/releases/tag/v1.5.0) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `differential-equations` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/differential-equations`) and its data (`rust/models/differential_equations/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p differential-equations-ai999x
./target/release/differential-equations --help
cargo test -p differential-equations-ai999x
```
