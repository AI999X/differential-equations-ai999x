# Differential Equations: the desktop app, user guide

**Differential Equations**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Differential Equations covers first- and second-order ODEs, linear systems, Laplace transforms, stability and phase portraits, numerical methods, classic PDEs (heat, wave, Laplace) and dynamical systems. Its desktop app holds all 78 calculations in 7 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-differential-equations-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-differential-equations-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-differential-equations-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-differential-equations.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-differential-equations-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-differential-equations-1.6.5-linux-x86_64.tar.gz
./ai999x-differential-equations-1.6.5-linux-x86_64/ai999x-differential-equations
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Differential Equations app, opened on Second-order homogeneous linear ODE: initial value problem with its worked example.](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/hero.webp)

*The Differential Equations app, opened on Second-order homogeneous linear ODE: initial value problem with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Second-order ODES › Second-order homogeneous linear ODE: initial value problem (or type part of its name in the search box). It opens with a worked example filled in: “Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).” The answer: y = 0.6004235991, damping = overdamped.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Second-order homogeneous linear ODE: initial value problem: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-1.webp)

*Second-order homogeneous linear ODE: initial value problem: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).” The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Second-order homogeneous linear ODE: initial value problem
y = 0.6004235991
damping = overdamped
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Logistic growth: population at a time](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-2.webp)

*Logistic growth: population at a time*

![Inverse Laplace transform of a rational function](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-3.webp)

*Inverse Laplace transform of a rational function*

![Classical Runge-Kutta method (RK4)](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-4.webp)

*Classical Runge-Kutta method (RK4)*

![Linear 2x2 system: equilibrium type and stability (trace-determinant plane)](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-5.webp)

*Linear 2x2 system: equilibrium type and stability (trace-determinant plane)*

![Wave equation on the whole line: d'Alembert's formula](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-6.webp)

*Wave equation on the whole line: d'Alembert's formula*

![Competing species: coexistence equilibrium and its stability](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-7.webp)

*Competing species: coexistence equilibrium and its stability*

![Bernoulli equation y' + p y = q y^n](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/scene-8.webp)

*Bernoulli equation y' + p y = q y^n*

![Second-order homogeneous linear ODE: initial value problem: a chart, enlarged.](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/chart-1.webp)

*Second-order homogeneous linear ODE: initial value problem: a chart, enlarged.*

![Logistic growth: population at a time: a chart, enlarged.](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/chart-2.webp)

*Logistic growth: population at a time: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Differential Equations app in the dark theme.](https://www.ai999x.com/?ax_shot=differential-equations-ai999x/hero-dark.webp)

*The Differential Equations app in the dark theme.*

## 7. Every calculation

78 calculations in 7 topics; each has its own entry on the model's page (https://www.ai999x.com/models/differential-equations-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Classic PDES | Eigenvalues of y'' + lambda y = 0 with Dirichlet/Neumann ends, Eigenvalues with a Robin (convective) end condition, Fourier sine series coefficient (half-range expansion), Heat equation on a rod with ends at zero: Fourier series value, Laplace's equation on a rectangle: series value, Vibrating string with fixed ends: Fourier series value, Wave equation on the whole line: d'Alembert's formula |
| Dynamical systems | Brusselator: equilibrium, its type, and the Hopf value b = 1 + a^2, Competing species: coexistence equilibrium and its stability, Logistic map: fixed points and their stability, Logistic map: period-2 cycle and its stability, Lorenz system: equilibria and stability of the origin, Lorenz system: Hopf threshold rho_H (loss of stability of C+-), Lotka-Volterra predator-prey: coexistence equilibrium and oscillation period, Lyapunov exponent of the logistic map (periodic regime), Nonlinear 2-D system: type of an equilibrium by linearization, SIR epidemic: R0, peak infections and final size |
| First-order ODES | Age from the fraction remaining (radioactive dating), Bernoulli equation y' + p y = q y^n, Doubling time or half-life from a rate constant, Draining tank (Torricelli's law): height and emptying time, Equation y' = k y^2: solution and blow-up time, Equilibria of y' = f(y) and their stability (phase line), Exponential growth or decay: value at a time, Exponential model through two measurements, Falling body with linear drag: velocity, Falling body with quadratic drag: velocity, First-order linear ODE with constant coefficients (integrating factor), First-order linear ODE with exponential forcing, First-order linear ODE y' + P(x) y = Q(x) (integrating factor), Gompertz growth: value at a time, Logistic growth with constant harvesting: equilibria, Logistic growth: inflection time and maximum growth rate, Logistic growth: population at a time, Mixing tank (brine): amount of salt at a time, Newton's law of cooling: cooling constant from a measurement, Newton's law of cooling: temperature at a time, Newton's law of cooling: time to reach a temperature, Radioactive decay from a half-life, Separable equation y' = a t^n y, Slope (direction) field value, Time for exponential growth or decay to reach a value |
| Laplace transforms | Convolution integral (f * g)(t), Impulse response of a second-order ODE, Inverse Laplace transform of a rational function, Laplace transform of a delayed function u(t - c) f(t - c), Laplace transform of a function at a point, Partial fractions of a rational function (simple poles), Unit-step response of a second-order ODE |
| Linear systems | Linear 2x2 system: equilibrium type and stability (trace-determinant plane), Linear system x' = A x: solution at a time, Routh-Hurwitz stability test of a characteristic polynomial, Stability of x' = A x (n x n) from the characteristic polynomial |
| Numerical methods | Backward (implicit) Euler method, CFL condition for the advection equation (upwind scheme), Classical Runge-Kutta method (RK4), Crank-Nicolson steps for the heat equation, Euler's method, Euler's method error for exponential growth, Explicit finite-difference (FTCS) steps for the heat equation, Explicit midpoint method (RK2), Improved Euler (Heun) method, Largest stable step of an explicit method for y' = lambda y, RK4 for a system of two first-order equations, Stability function (amplification factor) of a one-step method, Stability of the explicit (FTCS) scheme for the heat equation |
| Second-order ODES | Cauchy-Euler (equidimensional) equation: initial value problem, Damped oscillator: resonance frequency and peak amplitude, Forced damped oscillator: steady-state amplitude and phase, Linear two-point boundary value problem (constant coefficients), Mass-spring-damper: natural frequency, damping ratio, damped frequency, Second-order constant-coefficient ODE: characteristic roots, Second-order homogeneous linear ODE: initial value problem, Simple harmonic motion: position, amplitude, period, Sinusoidally forced linear oscillator: initial value problem, Undamped resonance: growing response, Undetermined coefficients: exponential forcing, Undetermined coefficients: polynomial forcing |

## 8. Other ways to use it

The same engine runs on the command line (`differential-equations`, its own guide), in Python (`pip install differential-equations-ai999x`, `import differential_equations_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/differential-equations-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/differential-equations-ai999x/
