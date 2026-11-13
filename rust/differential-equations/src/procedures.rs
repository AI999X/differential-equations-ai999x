//! differential_equations: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/differential_equations/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "amplification_factor",
    "autonomous_equilibria",
    "backward_euler_method",
    "bernoulli_equation",
    "boundary_value_problem",
    "brusselator_hopf",
    "cauchy_euler_ivp",
    "cfl_condition",
    "characteristic_roots",
    "competing_species_equilibrium",
    "convolution_integral",
    "crank_nicolson_steps",
    "dalembert_solution",
    "damped_oscillator_parameters",
    "doubling_time",
    "euler_error_exponential",
    "euler_method",
    "explicit_heat_stability",
    "explicit_step_limit",
    "exponential_from_two_points",
    "exponential_growth_value",
    "exponential_time_to_reach",
    "forced_oscillator_ivp",
    "fourier_sine_coefficient",
    "gompertz_growth",
    "half_life_decay",
    "heat_equation_series",
    "heat_ftcs_steps",
    "improved_euler_method",
    "inverse_laplace_transform",
    "laplace_equation_rectangle",
    "laplace_time_shift",
    "laplace_transform",
    "linear_drag_velocity",
    "linear_first_order",
    "linear_first_order_exponential_forcing",
    "linear_first_order_variable",
    "linear_system_solution",
    "linear_system_stability",
    "logistic_harvesting",
    "logistic_inflection",
    "logistic_map_fixed_points",
    "logistic_map_lyapunov",
    "logistic_map_two_cycle",
    "logistic_value",
    "lorenz_equilibria",
    "lorenz_hopf_threshold",
    "lotka_volterra_equilibrium",
    "midpoint_method",
    "mixing_tank",
    "newton_cooling_constant",
    "newton_cooling_temperature",
    "newton_cooling_time",
    "nonlinear_equilibrium_type",
    "ode_impulse_response",
    "ode_step_response",
    "partial_fraction_residues",
    "phase_portrait_2x2",
    "practical_resonance",
    "quadratic_drag_velocity",
    "quadratic_ode_blowup",
    "radiometric_age",
    "robin_eigenvalues",
    "routh_hurwitz",
    "runge_kutta_4",
    "runge_kutta_4_system",
    "second_order_homogeneous_ivp",
    "separable_power",
    "simple_harmonic_motion",
    "sir_epidemic",
    "slope_field",
    "steady_state_amplitude",
    "sturm_liouville_eigenvalues",
    "torricelli_draining",
    "undamped_resonance",
    "undetermined_coefficients_exponential",
    "undetermined_coefficients_polynomial",
    "wave_equation_string_series",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "amplification_factor" => {
            let r = f_amplification(match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_20(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "autonomous_equilibria" => {
            let r = f_equilibria_1d(match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_71(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "backward_euler_method" => {
            let r = f_backward_euler(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_123(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bernoulli_equation" => {
            let r = f_bernoulli(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_175(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "boundary_value_problem" => {
            let r = f_bvp2(match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ya") { Some(x) => py::from_val(x), None => { return None } }, match v.get("yb") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_245(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "brusselator_hopf" => {
            let r = f_brusselator(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_301(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cauchy_euler_ivp" => {
            let r = f_cauchy_euler(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_365(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "cfl_condition" => {
            let r = f_cfl(match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_407(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "characteristic_roots" => {
            let r = f_char_roots(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_469(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "competing_species_equilibrium" => {
            let r = f_competing_species(match v.get("a1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b11") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b12") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b21") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b22") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_533(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "convolution_integral" => {
            let r = f_convolution(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_573(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "crank_nicolson_steps" => {
            let r = f_crank_nicolson(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_623(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "dalembert_solution" => {
            let r = f_dalembert(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_671(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "damped_oscillator_parameters" => {
            let r = f_oscillator_params(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_727(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "doubling_time" => {
            let r = f_char_time(match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_763(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "euler_error_exponential" => {
            let r = f_euler_error(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_813(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "euler_method" => {
            let r = s_840(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_863(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "explicit_heat_stability" => {
            let r = f_heat_stability(match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dx") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_905(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "explicit_step_limit" => {
            let r = f_step_limit(match v.get("method") { Some(x) => py::from_val(x), None => { return None } }, match v.get("lam") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_947(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_from_two_points" => {
            let r = f_two_points(match v.get("t1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_988(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_growth_value" => {
            let r = f_exp_growth(match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1022(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "exponential_time_to_reach" => {
            let r = f_time_to_reach(match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1065(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "forced_oscillator_ivp" => {
            let r = f_forced_cos(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1132(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "fourier_sine_coefficient" => {
            let r = f_sine_coefficient(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1183(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "gompertz_growth" => {
            let r = f_gompertz(match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1221(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "half_life_decay" => {
            let r = f_half_life_decay(match v.get("N0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1255(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heat_equation_series" => {
            let r = f_heat_series(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1319(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "heat_ftcs_steps" => {
            let r = f_ftcs(match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1364(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "improved_euler_method" => {
            let r = s_1391(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1414(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "inverse_laplace_transform" => {
            let r = f_inverse_laplace(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1441(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "laplace_equation_rectangle" => {
            let r = f_laplace_rect(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1507(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "laplace_time_shift" => {
            let r = f_laplace_shifted(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1557(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "laplace_transform" => {
            let r = f_laplace_value(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1605(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_drag_velocity" => {
            let r = f_drag_linear(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1643(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_first_order" => {
            let r = f_lin1(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1683(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_first_order_exponential_forcing" => {
            let r = f_lin1_exp(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1724(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_first_order_variable" => {
            let r = f_linear_first_order_variable(match v.get("p_terms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q_terms") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match f__verify_lfo(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_system_solution" => {
            let r = f_linear_system_solution(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_system_stability" => {
            let r = f_linear_stability(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1951(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_harvesting" => {
            let r = f_harvesting(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1995(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_inflection" => {
            let r = f_logistic_inflection(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2036(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_map_fixed_points" => {
            let r = f_lm_fixed(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2081(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_map_lyapunov" => {
            let r = f_lm_lyapunov(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2144(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_map_two_cycle" => {
            let r = f_lm_cycle(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2184(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_value" => {
            let r = f_logistic(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2225(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lorenz_equilibria" => {
            let r = f_lorenz_eq(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lorenz_hopf_threshold" => {
            let r = f_lorenz_hopf(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2312(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lotka_volterra_equilibrium" => {
            let r = f_lotka_volterra(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2346(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "midpoint_method" => {
            let r = s_2373(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2396(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mixing_tank" => {
            let r = f_mixing(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2440(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_cooling_constant" => {
            let r = f_cooling_k(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2481(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_cooling_temperature" => {
            let r = f_cooling_T(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2516(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_cooling_time" => {
            let r = f_cooling_time(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2559(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nonlinear_equilibrium_type" => {
            let r = f_jacobian_classify(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2636(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ode_impulse_response" => {
            let r = f_impulse2(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2693(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ode_step_response" => {
            let r = f_step2(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2758(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_fraction_residues" => {
            let r = f_partial_fractions(match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2832(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "phase_portrait_2x2" => {
            let r = f_phase_portrait(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2915(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "practical_resonance" => {
            let r = f_practical_resonance(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2957(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_drag_velocity" => {
            let r = f_drag_quadratic(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3007(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_ode_blowup" => {
            let r = f_riccati_sq(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3045(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radiometric_age" => {
            let r = f_decay_age(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3083(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "robin_eigenvalues" => {
            let r = f_robin_eigen(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3135(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "routh_hurwitz" => {
            let r = f_routh_hurwitz(match v.get("coeffs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3200(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "runge_kutta_4" => {
            let r = s_3227(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3250(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "runge_kutta_4_system" => {
            let r = f_rk4_system(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3300(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_homogeneous_ivp" => {
            let r = f_ivp2(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3375(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "separable_power" => {
            let r = f_separable_power(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_harmonic_motion" => {
            let r = f_shm(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3450(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sir_epidemic" => {
            let r = f_sir(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3507(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "slope_field" => {
            let r = f_slope_field(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3539(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "steady_state_amplitude" => {
            let r = f_steady_state(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3582(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sturm_liouville_eigenvalues" => {
            let r = f_sturm_liouville(match v.get("bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3621(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "torricelli_draining" => {
            let r = f_torricelli(match v.get("h0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3663(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "undamped_resonance" => {
            let r = f_pure_resonance(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3700(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "undetermined_coefficients_exponential" => {
            let r = f_uc_exp(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3769(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "undetermined_coefficients_polynomial" => {
            let r = f_uc_poly(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3834(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wave_equation_string_series" => {
            let r = f_wave_series(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("0")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_SS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("s")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_XS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("x")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(ax_engine::pysym::symbol("y")) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GL5_W() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.5688888888888889), Py::Float(0.47862867049936647), Py::Float(0.47862867049936647), Py::Float(0.23692688505618908), Py::Float(0.23692688505618908)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__GL5_X() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::list(vec![Py::Float(0.0), Py::Float(0.5384693101056831), Py::Float(-0.5384693101056831), Py::Float(0.906179845938664), Py::Float(-0.906179845938664)])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__STAB_POLY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::dict(vec![(Py::str("euler"), Py::list(vec![Py::Int(1i64), Py::Int(1i64)])), (Py::str("heun"), Py::list(vec![Py::Int(1i64), Py::Int(1i64), Py::Float(0.5)])), (Py::str("midpoint"), Py::list(vec![Py::Int(1i64), Py::Int(1i64), Py::Float(0.5)])), (Py::str("rk4"), Py::list(vec![Py::Int(1i64), Py::Int(1i64), Py::Float(0.5), Py::Float(0.16666666666666666), Py::Float(0.041666666666666664)]))])) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__CS(mut v_d2: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_z = py::mul(&py::mul(&v_d2.clone(), &v_t.clone())?, &v_t.clone())?;
    if ({ let t3812 = py::abs(&v_z.clone())?; Py::Bool({ let t3813 = Py::Int(1i64); if py::lt(&t3812, &t3813)? { true } else { false } }) }).truthy() {
        let t3814 = (Py::tuple(vec![Py::Float(1.0), v_t.clone(), Py::Float(1.0), v_t.clone(), Py::Int(1i64)])).items()?;
        if t3814.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_C = t3814[0].clone();
        v_S = t3814[1].clone();
        v_tc = t3814[2].clone();
        v_ts = t3814[3].clone();
        v_k = t3814[4].clone();
        while (Py::Bool(true)).truthy() {
            v_tc = py::mul(&v_tc, &py::truediv(&v_z.clone(), &py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?)?;
            v_ts = py::mul(&v_ts, &py::truediv(&v_z.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_k.clone())?, &py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?)?)?)?;
            let t3815 = (Py::tuple(vec![py::add(&v_C.clone(), &v_tc.clone())?, py::add(&v_S.clone(), &v_ts.clone())?, py::add(&v_k.clone(), &Py::Int(1i64))?])).items()?;
            if t3815.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_C = t3815[0].clone();
            v_S = t3815[1].clone();
            v_k = t3815[2].clone();
            if ({ let t3820 = { let t3816 = py::abs(&v_tc.clone())?; Py::Bool({ let t3817 = Py::Float(1e-18); if py::lt(&t3816, &t3817)? { true } else { false } }) }; if !t3820.truthy() { t3820 } else { { let t3818 = py::abs(&v_ts.clone())?; Py::Bool({ let t3819 = py::mul(&Py::Float(1e-18), &py::minmax(&[Py::Float(1.0), py::abs(&v_t.clone())?], true)?)?; if py::lt(&t3818, &t3819)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::tuple(vec![v_C.clone(), v_S.clone()]));
            }
        }
    }
    let t3823: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t3821 = v_d2.clone(); Py::Bool({ let t3822 = Py::Int(0i64); if py::gt(&t3821, &t3822)? { true } else { false } }) }).truthy() {
                v_d = py::math1("sqrt", &v_d2.clone())?;
                return Ok(Some(Py::tuple(vec![py::math1("cosh", &py::mul(&v_d.clone(), &v_t.clone())?)?, py::truediv(&py::math1("sinh", &py::mul(&v_d.clone(), &v_t.clone())?)?, &v_d.clone())?])));
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3823 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Overflow)) => {
            return Err(py::value_error(py::to_str(&Py::str("the solution overflows"))));
        }
        Err(e) => return Err(e),
    }
    v_w = py::math1("sqrt", &py::neg(&v_d2.clone())?)?;
    return Ok(Py::tuple(vec![py::math1("cos", &py::mul(&v_w.clone(), &v_t.clone())?)?, py::truediv(&py::math1("sin", &py::mul(&v_w.clone(), &v_t.clone())?)?, &v_w.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__R(mut v_method: Py, mut v_z: Py) -> R {
    if ({ let t6 = v_method.clone(); Py::Bool({ let t7 = c__STAB_POLY()?; if py::contains(&t7, &t6)? { true } else { false } }) }).truthy() {
        return Ok(py::sum(&{ let mut out = vec![]; for t8 in py::iter(&(py::enumerate(&py::getitem(&c__STAB_POLY()?, &v_method.clone())?, 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t9 = (t8).items()?;
if t9.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t9[0].clone();
v_c = t9[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_z.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?);
    }
    if ({ let t10 = v_method.clone(); Py::Bool({ let t11 = Py::str("backward euler"); if py::eq(&t10, &t11) { true } else { false } }) }).truthy() {
        if ({ let t12 = v_z.clone(); Py::Bool({ let t13 = Py::Int(1i64); if py::eq(&t12, &t13) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("h*lambda = 1 makes the implicit step singular"))));
        }
        return Ok(py::truediv(&Py::Int(1i64), &py::sub(&Py::Int(1i64), &v_z.clone())?)?);
    }
    if ({ let t14 = v_z.clone(); Py::Bool({ let t15 = Py::Int(2i64); if py::eq(&t14, &t15) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("h*lambda = 2 makes the trapezoidal step singular"))));
    }
    return Ok(py::truediv(&py::add(&Py::Int(1i64), &py::truediv(&v_z.clone(), &Py::Int(2i64))?)?, &py::sub(&Py::Int(1i64), &py::truediv(&v_z.clone(), &Py::Int(2i64))?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canon_c(mut v_zs: Py) -> R {
    let mut v_cplx: Py = Py::None;
    let mut v_real: Py = Py::None;
    v_zs = { let mut out = vec![]; for t2813 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t2813;
out.push((if ({ let t2814 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2815 = py::mul(&Py::Float(1e-25), &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)?; if py::le(&t2814, &t2815)? { true } else { false } }) }).truthy() { py::complex(&[py::real(&v_z.clone())?, Py::Float(0.0)])? } else { v_z.clone() })); } Py::list(out) };
    v_real = py::sorted(&{ let mut out = vec![]; for t2816 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t2816;
if ({ let t2817 = py::imag(&v_z.clone())?; Py::Bool({ let t2818 = Py::Int(0i64); if py::eq(&t2817, &t2818) { true } else { false } }) }).truthy() { out.push(py::real(&v_z.clone())?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_cplx = py::sorted_key(&{ let mut out = vec![]; for t2819 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t2819;
if ({ let t2820 = py::imag(&v_z.clone())?; Py::Bool({ let t2821 = Py::Int(0i64); if py::ne(&t2820, &t2821) { true } else { false } }) }).truthy() { out.push(v_z.clone()); }} Py::list(out) }, &{ let v_real = v_real.clone(); let v_zs = v_zs.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_z: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::round(&py::real(&v_z.clone())?, Some((Py::Int(9i64)).i()?))?, py::neg(&py::imag(&v_z.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    return Ok(py::add(&v_real.clone(), &v_cplx.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__charpoly(mut v_A: Py) -> R {
    let mut v_AM: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Mk: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t1894 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1894;
out.push({ let mut out = vec![]; for t1895 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t1895;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }); } Py::list(out) };
    v_I = { let mut out = vec![]; for t1896 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1896;
out.push({ let mut out = vec![]; for t1897 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1897;
out.push(py::fraction(&[py::int(&{ let t1898 = v_i.clone(); Py::Bool({ let t1899 = v_j.clone(); if py::eq(&t1898, &t1899) { true } else { false } }) })?])?); } Py::list(out) }); } Py::list(out) };
    v_c = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    v_Mk = { let mut out = vec![]; for t1900 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t1900;
out.push(py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?); } Py::list(out) };
    for t1901 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t1901;
        v_Mk = { let mut out = vec![]; for t1902 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1902;
out.push({ let mut out = vec![]; for t1903 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1903;
out.push(py::add(&py::sum(&{ let mut out = vec![]; for t1904 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_l: Py; 
v_l = t1904;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_l.clone())?, &py::getitem(&py::getitem(&v_Mk.clone(), &v_l.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?, &py::mul(&py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_I.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
        v_AM = { let mut out = vec![]; for t1905 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1905;
out.push({ let mut out = vec![]; for t1906 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1906;
out.push(py::sum(&{ let mut out = vec![]; for t1907 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_l: Py; 
v_l = t1907;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_l.clone())?, &py::getitem(&py::getitem(&v_Mk.clone(), &v_l.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t1908 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1908;
out.push(py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?)?, &v_k.clone())?])?;
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__classify(mut v_tr: Py, mut v_det: Py, mut v_scalar: Py, mut v_zero: Py) -> R {
    let mut v_disc: Py = Py::None;
    let mut v_side: Py = Py::None;
    let mut v_stab: Py = Py::None;
    v_disc = py::sub(&py::mul(&v_tr.clone(), &v_tr.clone())?, &py::mul(&Py::Int(4i64), &v_det.clone())?)?;
    if ({ let t2875 = v_det.clone(); Py::Bool({ let t2876 = Py::Int(0i64); if py::lt(&t2875, &t2876)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("saddle"), Py::str("unstable")]));
    }
    if ({ let t2877 = v_det.clone(); Py::Bool({ let t2878 = Py::Int(0i64); if py::eq(&t2877, &t2878) { true } else { false } }) }).truthy() {
        if ({ let t2879 = v_tr.clone(); Py::Bool({ let t2880 = Py::Int(0i64); if py::lt(&t2879, &t2880)? { true } else { false } }) }).truthy() {
            return Ok(Py::tuple(vec![Py::str("degenerate (line of equilibria)"), Py::str("stable (not asymptotically)")]));
        }
        if ({ let t2881 = v_tr.clone(); Py::Bool({ let t2882 = Py::Int(0i64); if py::gt(&t2881, &t2882)? { true } else { false } }) }).truthy() {
            return Ok(Py::tuple(vec![Py::str("degenerate (line of equilibria)"), Py::str("unstable")]));
        }
        return Ok((if (v_zero.clone()).truthy() { Py::tuple(vec![Py::str("degenerate (every point an equilibrium)"), Py::str("stable (not asymptotically)")]) } else { Py::tuple(vec![Py::str("degenerate (shear)"), Py::str("unstable")]) }));
    }
    if ({ let t2883 = v_tr.clone(); Py::Bool({ let t2884 = Py::Int(0i64); if py::eq(&t2883, &t2884) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("center"), Py::str("stable (not asymptotically)")]));
    }
    let t2887 = ((if ({ let t2885 = v_tr.clone(); Py::Bool({ let t2886 = Py::Int(0i64); if py::lt(&t2885, &t2886)? { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("stable"), Py::str("asymptotically stable")]) } else { Py::tuple(vec![Py::str("unstable"), Py::str("unstable")]) })).items()?;
    if t2887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_side = t2887[0].clone();
    v_stab = t2887[1].clone();
    if ({ let t2888 = v_disc.clone(); Py::Bool({ let t2889 = Py::Int(0i64); if py::gt(&t2888, &t2889)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str(&[py::format(&v_side.clone(), "")?, " node".to_string()].concat()), v_stab.clone()]));
    }
    if ({ let t2890 = v_disc.clone(); Py::Bool({ let t2891 = Py::Int(0i64); if py::lt(&t2890, &t2891)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str(&[py::format(&v_side.clone(), "")?, " spiral".to_string()].concat()), v_stab.clone()]));
    }
    return Ok(Py::tuple(vec![(if (v_scalar.clone()).truthy() { Py::str(&[py::format(&v_side.clone(), "")?, " star (proper node)".to_string()].concat()) } else { Py::str(&[py::format(&v_side.clone(), "")?, " degenerate node".to_string()].concat()) }), v_stab.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cos_particular(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_F0: Py, mut v_w: Py) -> R {
    let mut v_A_: Py = Py::None;
    let mut v_B_: Py = Py::None;
    let mut v_C_: Py = Py::None;
    let mut v_W_: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let t1102 = ({ let mut out = vec![]; for t1101 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_w.clone()])))? { let mut v_v: Py; 
v_v = t1101;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }).items()?;
    if t1102.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A_ = t1102[0].clone();
    v_B_ = t1102[1].clone();
    v_C_ = t1102[2].clone();
    v_W_ = t1102[3].clone();
    if ({ let t1107 = { let t1103 = v_B_.clone(); Py::Bool({ let t1104 = Py::Int(0i64); if py::eq(&t1103, &t1104) { true } else { false } }) }; if !t1107.truthy() { t1107 } else { { let t1105 = py::abs(&py::sub(&v_C_.clone(), &py::mul(&py::mul(&v_A_.clone(), &v_W_.clone())?, &v_W_.clone())?)?)?; Py::Bool({ let t1106 = py::mul(&py::fraction(&[Py::Int(1i64), py::pow(&Py::Int(10i64), &Py::Int(13i64))?])?, &py::add(&py::abs(&v_C_.clone())?, &py::abs(&py::mul(&py::mul(&v_A_.clone(), &v_W_.clone())?, &v_W_.clone())?)?)?)?; if py::le(&t1105, &t1106)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::None);
    }
    v_Z = py::truediv(&v_F0.clone(), &py::complex(&[py::sub(&v_c.clone(), &py::mul(&py::mul(&v_a.clone(), &v_w.clone())?, &v_w.clone())?)?, py::mul(&v_b.clone(), &v_w.clone())?])?)?;
    return Ok(v_Z.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__damping(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_D: Py) -> R {
    if ({ let t3361 = v_a.clone(); Py::Bool({ let t3362 = Py::Int(0i64); if py::lt(&t3361, &t3362)? { true } else { false } }) }).truthy() {
        let t3363 = (Py::tuple(vec![py::neg(&v_a.clone())?, py::neg(&v_b.clone())?, py::neg(&v_c.clone())?])).items()?;
        if t3363.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3363[0].clone();
        v_b = t3363[1].clone();
        v_c = t3363[2].clone();
    }
    if (py::not(&{ let t3368 = { let t3364 = v_c.clone(); Py::Bool({ let t3365 = Py::Int(0i64); if py::gt(&t3364, &t3365)? { true } else { false } }) }; if !t3368.truthy() { t3368 } else { { let t3366 = v_b.clone(); Py::Bool({ let t3367 = Py::Int(0i64); if py::ge(&t3366, &t3367)? { true } else { false } }) } } })).truthy() {
        return Ok(Py::str("not a damped oscillator (needs a, c of one sign and b of that sign or 0)"));
    }
    if ({ let t3369 = v_b.clone(); Py::Bool({ let t3370 = Py::Int(0i64); if py::eq(&t3369, &t3370) { true } else { false } }) }).truthy() {
        return Ok(Py::str("undamped"));
    }
    return Ok((if ({ let t3371 = v_D.clone(); Py::Bool({ let t3372 = Py::Int(0i64); if py::gt(&t3371, &t3372)? { true } else { false } }) }).truthy() { Py::str("overdamped") } else { (if ({ let t3373 = v_D.clone(); Py::Bool({ let t3374 = Py::Int(0i64); if py::eq(&t3373, &t3374) { true } else { false } }) }).truthy() { Py::str("critically damped") } else { Py::str("underdamped") }) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t3900 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t3900.truthy() { t3900 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t3901 = v_a.clone(); Py::Bool({ let t3902 = v_b.clone(); if py::eq(&t3901, &t3902) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    if ({ let t3903 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t3903.truthy() { t3903 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t3907 = py::not(&{ let t3904 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t3904.truthy() { t3904 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t3907.truthy() { t3907 } else { { let t3905 = py::len(&v_a.clone())?; Py::Bool({ let t3906 = py::len(&v_b.clone())?; if py::ne(&t3905, &t3906) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t3908 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3909 = (t3908).items()?;
if t3909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3909[0].clone();
v_y = t3909[1].clone();
out.push(f__diff(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t3914 = { let t3910 = v_a.clone(); Py::Bool({ let t3911 = Py::None; if py::is(&t3910, &t3911) { true } else { false } }) }; if t3914.truthy() { t3914 } else { { let t3912 = v_b.clone(); Py::Bool({ let t3913 = Py::None; if py::is(&t3912, &t3913) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t3915 = v_a.clone(); Py::Bool({ let t3916 = v_b.clone(); if py::is(&t3915, &t3916) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    let t3921: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_d = py::abs(&py::sub(&py::complex(&[v_a.clone()])?, &py::complex(&[v_b.clone()])?)?)?;
            if ({ let t3917 = v_d.clone(); Py::Bool({ let t3918 = v_d.clone(); if py::ne(&t3917, &t3918) { true } else { false } }) }).truthy() {
                return Ok(Some(Py::Float(1.0)));
            }
            return Ok(Some((if ({ let t3919 = v_d.clone(); Py::Bool({ let t3920 = Py::Float(1e-15); if py::le(&t3919, &t3920)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&py::complex(&[v_a.clone()])?)?, py::abs(&py::complex(&[v_b.clone()])?)?], true)?)? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3921 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exp(mut v_x: Py) -> R {
    let t3811: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(py::math1("exp", &v_x.clone())?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3811 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Overflow)) => {
            return Err(py::value_error(py::to_str(&Py::str("the result overflows (exponent too large)"))));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t3861 in py::iter(&(v_vals.clone()))? {
        v_v = t3861;
        if (py::not(&{ let t3863 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3863.truthy() { t3863 } else { { let t3862 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3862.truthy() { t3862 } else { py::math1("isfinite", &v_v.clone())? } } } })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("every input must be a finite number"))));
        }
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__forced(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_y0: Py, mut v_v0: Py, mut v_t: Py, mut v_yp_t: Py, mut v_yp0: Py, mut v_ypd0: Py) -> R {
    let mut v_h: Py = Py::None;
    v_h = f_hom2(v_a.clone(), v_b.clone(), v_c.clone(), py::sub(&v_y0.clone(), &v_yp0.clone())?, py::sub(&v_v0.clone(), &v_ypd0.clone())?, v_t.clone())?;
    return Ok(f__guard(py::add(&v_yp_t.clone(), &v_h.clone())?, Py::tuple(vec![v_yp_t.clone(), v_h.clone()]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__guard(mut v_total: Py, mut v_parts: Py) -> R {
    let mut v_big: Py = Py::None;
    v_big = py::sum(&{ let mut out = vec![]; for t3888 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3888;
out.push(py::abs(&v_p.clone())?); } Py::list(out) }, None)?;
    if ({ let t3889 = v_big.clone(); Py::Bool({ let t3890 = py::mul(&Py::Float(100000.0), &py::minmax(&[Py::Float(1.0), py::abs(&v_total.clone())?], true)?)?; if py::gt(&t3889, &t3890)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result is lost to cancellation between large terms (ill-conditioned inputs)"))));
    }
    if ({ let t3891 = py::abs(&v_total.clone())?; Py::Bool({ let t3892 = py::mul(&Py::Float(1e-13), &v_big.clone())?; if py::lt(&t3891, &t3892)? { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(v_total.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__laplace_sum(mut v_terms: Py, mut v_s: Py, mut v_power_shift: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_vals = { let mut out = vec![]; for t1593 in py::iter(&(v_terms.clone()))? { let mut v_c: Py; let mut v_n: Py; let mut v_z: Py; 
let t1594 = (t1593).items()?;
if t1594.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_c = t1594[0].clone();
v_n = t1594[1].clone();
v_z = t1594[2].clone();
out.push(py::truediv(&py::mul(&v_c.clone(), &py::factorial(&v_n.clone())?)?, &py::pow(&py::sub(&v_s.clone(), &v_z.clone())?, &py::add(&v_n.clone(), &v_power_shift.clone())?)?)?); } Py::list(out) };
    v_tot = py::sum(&v_vals.clone(), None)?;
    v_big = py::sum(&{ let mut out = vec![]; for t1595 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1595;
out.push(py::abs(&v_v.clone())?); } Py::list(out) }, None)?;
    if ({ let t1596 = py::abs(&py::imag(&v_tot.clone())?)?; Py::Bool({ let t1597 = py::mul(&Py::Float(1e-09), &v_big.clone())?; if py::gt(&t1596, &t1597)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the function is not real"))));
    }
    return Ok(f__guard(py::real(&v_tot.clone())?, { let mut v = vec![]; v.extend(({ let t1604 = { let mut out = vec![]; for t1603 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t1603;
out.push(py::real(&v_v.clone())?); } Py::list(out) }; if t1604.truthy() { t1604 } else { Py::list(vec![Py::Float(0.0)]) } }).items()?); Py::tuple(v) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_F(mut v_P: Py, mut v_s: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = Py::Float(0.0);
    for t1779 in py::iter(&(v_P.clone()))? {
        v_r = t1779;
        if ({ let t1780 = py::getitem(&v_r.clone(), &Py::Int(1i64))?; Py::Bool({ let t1781 = py::neg(&Py::Int(1i64))?; if py::eq(&t1780, &t1781) { true } else { false } }) }).truthy() {
            v_v = py::iadd(&v_v, &py::mul(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::math1("log", &py::abs(&v_s.clone())?)?)?)?;
        } else {
            v_v = py::iadd(&v_v, &py::truediv(&py::mul(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::pow(&v_s.clone(), &py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?)?, &py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?)?;
        }
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_Q(mut v_Q: Py, mut v_s: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_v = Py::Float(0.0);
    for t1784 in py::iter(&(v_Q.clone()))? {
        v_r = t1784;
        v_v = py::iadd(&v_v, &py::mul(&py::mul(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::pow(&v_s.clone(), &py::getitem(&v_r.clone(), &Py::Int(1i64))?)?)?, &py::math1("exp", &py::mul(&py::getitem(&v_r.clone(), &Py::Int(2i64))?, &v_s.clone())?)?)?)?;
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_factor_text(mut v_P: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_logc: Py = Py::None;
    let mut v_mono: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_piece: Py = Py::None;
    let mut v_poly: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_terms: Py = Py::None;
    v_logc = Py::Float(0.0);
    v_poly = Py::dict(vec![]);
    for t1802 in py::iter(&(v_P.clone()))? {
        v_r = t1802;
        if ({ let t1803 = py::getitem(&v_r.clone(), &Py::Int(0i64))?; Py::Bool({ let t1804 = Py::Int(0i64); if py::eq(&t1803, &t1804) { true } else { false } }) }).truthy() {
            continue;
        }
        if ({ let t1805 = py::getitem(&v_r.clone(), &Py::Int(1i64))?; Py::Bool({ let t1806 = py::neg(&Py::Int(1i64))?; if py::eq(&t1805, &t1806) { true } else { false } }) }).truthy() {
            v_logc = py::iadd(&v_logc, &py::getitem(&v_r.clone(), &Py::Int(0i64))?)?;
        } else {
            py::setitem(&v_poly.clone(), &py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, py::add(&py::method(&v_poly.clone(), "get", &[py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?, Py::Float(0.0)])?, &py::truediv(&py::getitem(&v_r.clone(), &Py::Int(0i64))?, &py::add(&py::getitem(&v_r.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?)?)?;
        }
    }
    v_parts = Py::list(vec![]);
    if ({ let t1807 = v_logc.clone(); Py::Bool({ let t1808 = Py::Int(0i64); if py::ne(&t1807, &t1808) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_parts.clone(), "append", &[(if ({ let t1809 = v_logc.clone(); Py::Bool({ let t1810 = Py::Int(1i64); if py::eq(&t1809, &t1810) { true } else { false } }) }).truthy() { Py::str("|x|") } else { py::add(&Py::str("|x|^"), &f__lfo_num(v_logc.clone())?)? })])?;
    }
    v_terms = Py::str("");
    for t1811 in py::range_iter(&[(Py::Int(7i64)).i()?, (py::neg(&Py::Int(4i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_p = t1811;
        if ({ let t1816 = { let t1812 = v_p.clone(); Py::Bool({ let t1813 = v_poly.clone(); if !py::contains(&t1813, &t1812)? { true } else { false } }) }; if t1816.truthy() { t1816 } else { { let t1814 = py::getitem(&v_poly.clone(), &v_p.clone())?; Py::Bool({ let t1815 = Py::Int(0i64); if py::eq(&t1814, &t1815) { true } else { false } }) } } }).truthy() {
            continue;
        }
        v_d = py::getitem(&v_poly.clone(), &v_p.clone())?;
        v_mono = (if ({ let t1817 = v_p.clone(); Py::Bool({ let t1818 = Py::Int(1i64); if py::eq(&t1817, &t1818) { true } else { false } }) }).truthy() { Py::str("x") } else { py::add(&Py::str("x^"), &Py::str(&py::to_str(&v_p.clone())))? });
        v_piece = (if ({ let t1819 = py::abs(&v_d.clone())?; Py::Bool({ let t1820 = Py::Int(1i64); if py::eq(&t1819, &t1820) { true } else { false } }) }).truthy() { v_mono.clone() } else { py::add(&f__lfo_num(py::abs(&v_d.clone())?)?, &v_mono.clone())? });
        if ({ let t1821 = v_terms.clone(); Py::Bool({ let t1822 = Py::str(""); if py::eq(&t1821, &t1822) { true } else { false } }) }).truthy() {
            v_terms = py::add(&(if ({ let t1823 = v_d.clone(); Py::Bool({ let t1824 = Py::Int(0i64); if py::lt(&t1823, &t1824)? { true } else { false } }) }).truthy() { Py::str("-") } else { Py::str("") }), &v_piece.clone())?;
        } else {
            v_terms = py::add(&py::add(&v_terms.clone(), &(if ({ let t1825 = v_d.clone(); Py::Bool({ let t1826 = Py::Int(0i64); if py::lt(&t1825, &t1826)? { true } else { false } }) }).truthy() { Py::str(" - ") } else { Py::str(" + ") }))?, &v_piece.clone())?;
        }
    }
    if ({ let t1827 = v_terms.clone(); Py::Bool({ let t1828 = Py::str(""); if py::ne(&t1827, &t1828) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_parts.clone(), "append", &[py::add(&py::add(&Py::str("exp("), &v_terms.clone())?, &Py::str(")"))?])?;
    }
    if ({ let t1829 = py::len(&v_parts.clone())?; Py::Bool({ let t1830 = Py::Int(0i64); if py::eq(&t1829, &t1830) { true } else { false } }) }).truthy() {
        return Ok(Py::str("1"));
    }
    return Ok(py::method(&Py::str(" * "), "join", &[v_parts.clone()])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_finite(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t1752 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t1752.truthy() { t1752 } else { { let t1751 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t1751.truthy() { t1751 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&v_name.clone(), &Py::str(" must be a finite number"))?)));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_integral(mut v_P: Py, mut v_Q: Py, mut v_x0: Py, mut v_x1: Py, mut v_panels: Py) -> R {
    let mut v_F0: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_size: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_F0 = f__lfo_F(v_P.clone(), v_x0.clone())?;
    v_h = py::truediv(&py::sub(&v_x1.clone(), &v_x0.clone())?, &v_panels.clone())?;
    v_total = Py::Float(0.0);
    v_size = Py::Float(0.0);
    for t1782 in py::range_iter(&[(v_panels.clone()).i()?]) {
        v_m = t1782;
        v_mid = py::add(&v_x0.clone(), &py::mul(&py::add(&v_m.clone(), &Py::Float(0.5))?, &v_h.clone())?)?;
        for t1783 in py::range_iter(&[(Py::Int(5i64)).i()?]) {
            v_j = t1783;
            v_s = py::add(&v_mid.clone(), &py::mul(&py::mul(&Py::Float(0.5), &v_h.clone())?, &py::getitem(&c__GL5_X()?, &v_j.clone())?)?)?;
            v_g = py::mul(&py::math1("exp", &py::sub(&f__lfo_F(v_P.clone(), v_s.clone())?, &v_F0.clone())?)?, &f__lfo_Q(v_Q.clone(), v_s.clone())?)?;
            v_total = py::iadd(&v_total, &py::mul(&py::getitem(&c__GL5_W()?, &v_j.clone())?, &v_g.clone())?)?;
            v_size = py::iadd(&v_size, &py::mul(&py::getitem(&c__GL5_W()?, &v_j.clone())?, &py::abs(&v_g.clone())?)?)?;
        }
    }
    return Ok(Py::tuple(vec![py::mul(&py::mul(&Py::Float(0.5), &v_h.clone())?, &v_total.clone())?, py::mul(&py::mul(&Py::Float(0.5), &py::abs(&v_h.clone())?)?, &v_size.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_num(mut v_v: Py) -> R {
    return Ok(Py::str(&[py::format(&py::add(&v_v.clone(), &Py::Float(0.0))?, ".12g")?].concat()));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lfo_rows(mut v_rows: Py, mut v_width: Py, mut v_name: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    if ({ let t1757 = { let t1753 = py::len(&v_rows.clone())?; Py::Bool({ let t1754 = Py::Int(1i64); if py::lt(&t1753, &t1754)? { true } else { false } }) }; if t1757.truthy() { t1757 } else { { let t1755 = py::len(&v_rows.clone())?; Py::Bool({ let t1756 = Py::Int(8i64); if py::gt(&t1755, &t1756)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&py::add(&v_name.clone(), &Py::str(": 1 to 8 terms"))?)));
    }
    v_out = Py::list(vec![]);
    for t1758 in py::iter(&(v_rows.clone()))? {
        v_r = t1758;
        if ({ let t1759 = py::len(&v_r.clone())?; Py::Bool({ let t1760 = v_width.clone(); if py::ne(&t1759, &t1760) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&py::add(&py::add(&py::add(&v_name.clone(), &Py::str(": every term is a row of "))?, &Py::str(&py::to_str(&v_width.clone())))?, &Py::str(" numbers"))?)));
        }
        v_c = f__lfo_finite(py::getitem(&v_r.clone(), &Py::Int(0i64))?, v_name.clone())?;
        v_k = f__lfo_finite(py::getitem(&v_r.clone(), &Py::Int(1i64))?, v_name.clone())?;
        if ({ let t1768 = { let t1761 = v_k.clone(); Py::Bool({ let t1762 = py::math1("floor", &v_k.clone())?; if py::ne(&t1761, &t1762) { true } else { false } }) }; if t1768.truthy() { t1768 } else { { let t1767 = { let t1763 = v_k.clone(); Py::Bool({ let t1764 = py::neg(&Py::Int(4i64))?; if py::lt(&t1763, &t1764)? { true } else { false } }) }; if t1767.truthy() { t1767 } else { { let t1765 = v_k.clone(); Py::Bool({ let t1766 = Py::Int(6i64); if py::gt(&t1765, &t1766)? { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&py::add(&v_name.clone(), &Py::str(": the power of x is a whole number from -4 to 6"))?)));
        }
        if ({ let t1769 = v_width.clone(); Py::Bool({ let t1770 = Py::Int(3i64); if py::eq(&t1769, &t1770) { true } else { false } }) }).truthy() {
            v_a = f__lfo_finite(py::getitem(&v_r.clone(), &Py::Int(2i64))?, v_name.clone())?;
            if ({ let t1771 = py::abs(&v_a.clone())?; Py::Bool({ let t1772 = Py::Int(20i64); if py::gt(&t1771, &t1772)? { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&py::add(&v_name.clone(), &Py::str(": the exponent rate a is at most 20 in size"))?)));
            }
            let _ = py::method(&v_out.clone(), "append", &[Py::list(vec![v_c.clone(), py::int(&v_k.clone())?, v_a.clone()])])?;
        } else {
            let _ = py::method(&v_out.clone(), "append", &[Py::list(vec![v_c.clone(), py::int(&v_k.clone())?])])?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__one_step(mut v_method: Py, mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_k3: Py = Py::None;
    let mut v_k4: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t0.clone(), v_y0.clone(), v_h.clone()]))?;
    if ({ let t3231 = v_h.clone(); Py::Bool({ let t3232 = Py::Int(0i64); if py::le(&t3231, &t3232)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step h must be positive"))));
    }
    if (py::not(&{ let t3233 = Py::Int(1i64); Py::Bool({ let t3234 = v_n.clone(); if py::le(&t3233, &t3234)? { { let t3235 = Py::Int(10000i64); if py::le(&t3234, &t3235)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of steps n must be between 1 and 10000"))));
    }
    v_y = v_y0.clone();
    for t3236 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3236;
        v_t = py::add(&v_t0.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?;
        if ({ let t3237 = v_method.clone(); Py::Bool({ let t3238 = Py::str("euler"); if py::eq(&t3237, &t3238) { true } else { false } }) }).truthy() {
            v_y = py::add(&v_y.clone(), &py::mul(&v_h.clone(), &py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?)?)?;
        } else {
            if ({ let t3239 = v_method.clone(); Py::Bool({ let t3240 = Py::str("heun"); if py::eq(&t3239, &t3240) { true } else { false } }) }).truthy() {
                v_k1 = py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?;
                v_k2 = py::call(&v_f.clone(), &[py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_k1.clone())?)?])?;
                v_y = py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&v_k1.clone(), &v_k2.clone())?)?, &Py::Int(2i64))?)?;
            } else {
                if ({ let t3241 = v_method.clone(); Py::Bool({ let t3242 = Py::str("midpoint"); if py::eq(&t3241, &t3242) { true } else { false } }) }).truthy() {
                    v_k1 = py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?;
                    v_y = py::add(&v_y.clone(), &py::mul(&v_h.clone(), &py::call(&v_f.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_k1.clone())?, &Py::Int(2i64))?)?])?)?)?;
                } else {
                    if ({ let t3243 = v_method.clone(); Py::Bool({ let t3244 = Py::str("rk4"); if py::eq(&t3243, &t3244) { true } else { false } }) }).truthy() {
                        v_k1 = py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?;
                        v_k2 = py::call(&v_f.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_k1.clone())?, &Py::Int(2i64))?)?])?;
                        v_k3 = py::call(&v_f.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_k2.clone())?, &Py::Int(2i64))?)?])?;
                        v_k4 = py::call(&v_f.clone(), &[py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_k3.clone())?)?])?;
                        v_y = py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&py::add(&py::add(&v_k1.clone(), &py::mul(&Py::Int(2i64), &v_k2.clone())?)?, &py::mul(&Py::Int(2i64), &v_k3.clone())?)?, &v_k4.clone())?)?, &Py::Int(6i64))?)?;
                    }
                }
            }
        }
        if ({ let t3247 = py::not(&py::math1("isfinite", &v_y.clone())?); if t3247.truthy() { t3247 } else { { let t3245 = py::abs(&v_y.clone())?; Py::Bool({ let t3246 = Py::Float(1e+100); if py::gt(&t3245, &t3246)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the numerical solution overflows"))));
        }
    }
    return Ok(Py::dict(vec![(Py::str("y_n"), f__r(v_y.clone())?), (Py::str("t_n"), f__r(py::add(&v_t0.clone(), &py::mul(&v_n.clone(), &v_h.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poles(mut v_Q: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_mlt: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rs: Py = Py::None;
    v_out = Py::list(vec![]);
    for t2794 in py::iter(&(py::getitem(&py::method(&v_Q.clone(), "sqf_list", &[])?, &Py::Int(1i64))?))? {
        let t2799 = (t2794).items()?;
        if t2799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_q = t2799[0].clone();
        v_mlt = t2799[1].clone();
        v_co = { let mut out = vec![]; for t2795 in py::iter(&(py::method(&v_q.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2795;
out.push(py::truediv(&ax_engine::pymp::call("mpf", &[py::getattr(&v_c.clone(), "p")?], &[])?, &py::getattr(&v_c.clone(), "q")?)?); } Py::list(out) };
        v_rs = (if ({ let t2796 = py::method(&v_q.clone(), "degree", &[])?; Py::Bool({ let t2797 = Py::Int(1i64); if py::eq(&t2796, &t2797) { true } else { false } }) }).truthy() { Py::list(vec![py::truediv(&py::neg(&py::getitem(&v_co.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_co.clone(), &Py::Int(0i64))?)?]) } else { ax_engine::pymp::call("polyroots", &[v_co.clone()], &[(String::from("maxsteps"), Py::Int(500i64)), (String::from("extraprec"), Py::Int(300i64))])? });
        v_out = py::iadd(&v_out, &{ let mut out = vec![]; for t2798 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t2798;
out.push(Py::tuple(vec![ax_engine::pymp::call("mpc", &[v_r.clone()], &[])?, v_mlt.clone()])); } Py::list(out) })?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__poly_x(mut v_text: Py, mut v_max_deg: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_e: Py = Py::None;
    v_e = ax_engine::pysym::helper("differential_equations:parse_text", &[v_text.clone(), Py::tuple(vec![Py::str("x")])], &[])?;
    let t3877: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_P = ax_engine::pysym::call("Poly", &[ax_engine::pysym::call("expand", &[v_e.clone()], &[])?, c_XS()?], &[])?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3877 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "PolynomialError")) => {
            return Err(py::value_error(py::to_str(&Py::str("this capability needs f(x) to be a polynomial in x"))));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t3878 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t3878;
out.push(py::getattr(&v_c.clone(), "is_rational")?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial needs rational coefficients"))));
    }
    if ({ let t3879 = py::method(&v_P.clone(), "degree", &[])?; Py::Bool({ let t3880 = v_max_deg.clone(); if py::gt(&t3879, &t3880)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["polynomials up to degree ".to_string(), py::format(&v_max_deg.clone(), "")?, " are supported".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3881 in py::iter(&(py::reversed(&py::method(&v_P.clone(), "all_coeffs", &[])?)?))? { let mut v_c: Py; 
v_c = t3881;
out.push(py::float(&v_c.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_v: Py) -> R {
    if (Py::Bool(matches!(v_v.clone(), Py::Complex(..)))).truthy() {
        return Ok(py::complex(&[f__r(py::real(&v_v.clone())?)?, f__r(py::imag(&v_v.clone())?)?])?);
    }
    v_v = py::float(&v_v.clone())?;
    if (py::not(&py::math1("isfinite", &v_v.clone())?)).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result is not finite (it overflows)"))));
    }
    return Ok((if ({ let t3893 = v_v.clone(); Py::Bool({ let t3894 = Py::Int(0i64); if py::eq(&t3893, &t3894) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::float(&Py::str(&[py::format(&v_v.clone(), ".12g")?].concat()))? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ratfun(mut v_text: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_num: Py = Py::None;
    v_e = ax_engine::pysym::helper("differential_equations:parse_text", &[v_text.clone(), Py::tuple(vec![Py::str("s")])], &[])?;
    let t2785 = (ax_engine::pysym::call("fraction", &[ax_engine::pysym::call("cancel", &[ax_engine::pysym::call("together", &[v_e.clone()], &[])?], &[])?], &[])?).items()?;
    if t2785.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2785[0].clone();
    v_den = t2785[1].clone();
    let t2787: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2786 = (Py::tuple(vec![ax_engine::pysym::call("Poly", &[v_num.clone(), c_SS()?], &[])?, ax_engine::pysym::call("Poly", &[v_den.clone(), c_SS()?], &[])?])).items()?;
            if t2786.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_P = t2786[0].clone();
            v_Q = t2786[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2787 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "PolynomialError")) => {
            return Err(py::value_error(py::to_str(&Py::str("F(s) must be a rational function of s"))));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t2788 in py::iter(&(py::add(&py::method(&v_P.clone(), "all_coeffs", &[])?, &py::method(&v_Q.clone(), "all_coeffs", &[])?)?))? { let mut v_c: Py; 
v_c = t2788;
out.push(py::getattr(&v_c.clone(), "is_rational")?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F(s) needs rational coefficients"))));
    }
    if (py::getattr(&v_P.clone(), "is_zero")?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F(s) is zero"))));
    }
    if ({ let t2789 = py::method(&v_P.clone(), "degree", &[])?; Py::Bool({ let t2790 = py::method(&v_Q.clone(), "degree", &[])?; if py::ge(&t2789, &t2790)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F(s) must be strictly proper (degree of numerator < degree of denominator)"))));
    }
    if ({ let t2791 = py::method(&v_Q.clone(), "degree", &[])?; Py::Bool({ let t2792 = Py::Int(10i64); if py::gt(&t2791, &t2792)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("denominators up to degree 10 are supported"))));
    }
    return Ok(Py::tuple(vec![v_P.clone(), v_Q.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots2(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Cc: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_sq: Py = Py::None;
    let t3332 = (Py::tuple(vec![py::fraction(&[v_a.clone()])?, py::fraction(&[v_b.clone()])?, py::fraction(&[v_c.clone()])?])).items()?;
    if t3332.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3332[0].clone();
    v_B = t3332[1].clone();
    v_Cc = t3332[2].clone();
    v_D = py::sub(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_Cc.clone())?)?;
    if ({ let t3333 = v_D.clone(); Py::Bool({ let t3334 = Py::Int(0i64); if py::gt(&t3333, &t3334)? { true } else { false } }) }).truthy() {
        v_sq = py::math1("sqrt", &v_D.clone())?;
        v_q = py::truediv(&py::neg(&py::add(&v_b.clone(), &py::copysign(&v_sq.clone(), &v_b.clone())?)?)?, &Py::Int(2i64))?;
        v_r = py::sorted(&Py::list(vec![py::truediv(&v_q.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())?;
        return Ok(Py::tuple(vec![py::getitem(&v_r.clone(), &Py::Int(0i64))?, py::getitem(&v_r.clone(), &Py::Int(1i64))?, v_D.clone()]));
    }
    if ({ let t3335 = v_D.clone(); Py::Bool({ let t3336 = Py::Int(0i64); if py::eq(&t3335, &t3336) { true } else { false } }) }).truthy() {
        v_r = py::float(&py::truediv(&py::neg(&v_B.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?)?;
        return Ok(Py::tuple(vec![v_r.clone(), v_r.clone(), v_D.clone()]));
    }
    let t3337 = (Py::tuple(vec![py::add(&py::float(&py::truediv(&py::neg(&v_B.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?)?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::neg(&v_D.clone())?)?, &py::mul(&Py::Int(2i64), &py::abs(&v_a.clone())?)?)?])).items()?;
    if t3337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t3337[0].clone();
    v_im = t3337[1].clone();
    return Ok(Py::tuple(vec![py::complex(&[v_re_.clone(), v_im.clone()])?, py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?, v_D.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__routh(mut v_co: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_changes: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_new: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_co = { let mut out = vec![]; for t3167 in py::iter(&(v_co.clone()))? { let mut v_v: Py; 
v_v = t3167;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) };
    while ({ let t3170 = v_co.clone(); if !t3170.truthy() { t3170 } else { { let t3168 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t3169 = Py::Int(0i64); if py::eq(&t3168, &t3169) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_co.clone(), "pop", &[Py::Int(0i64)])?;
    }
    if ({ let t3171 = py::len(&v_co.clone())?; Py::Bool({ let t3172 = Py::Int(2i64); if py::lt(&t3171, &t3172)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial needs degree >= 1"))));
    }
    if ({ let t3173 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t3174 = Py::Int(0i64); if py::lt(&t3173, &t3174)? { true } else { false } }) }).truthy() {
        v_co = { let mut out = vec![]; for t3175 in py::iter(&(v_co.clone()))? { let mut v_v: Py; 
v_v = t3175;
out.push(py::neg(&v_v.clone())?); } Py::list(out) };
    }
    let t3176 = (Py::tuple(vec![py::slice(&v_co.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, py::slice(&v_co.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?])).items()?;
    if t3176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r0 = t3176[0].clone();
    v_r1 = t3176[1].clone();
    v_rows = Py::list(vec![v_r0.clone(), py::add(&v_r1.clone(), &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&py::len(&v_r0.clone())?, &py::len(&v_r1.clone())?)?)?)?]);
    v_n = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?;
    for t3177 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t3177;
        let t3178 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t3178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3178[0].clone();
        v_b = t3178[1].clone();
        if ({ let t3179 = py::getitem(&v_b.clone(), &Py::Int(0i64))?; Py::Bool({ let t3180 = Py::Int(0i64); if py::eq(&t3179, &t3180) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the Routh array has a zero in its first column (a root on or symmetric about the imaginary axis): this version decides regular cases only"))));
        }
        v_new = py::add(&{ let mut out = vec![]; for t3181 in py::range_iter(&[(py::sub(&py::len(&v_a.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t3181;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![py::fraction(&[Py::Int(0i64)])?]))?;
        let _ = py::method(&v_rows.clone(), "append", &[v_new.clone()])?;
    }
    v_first = { let mut out = vec![]; for t3182 in py::iter(&(py::slice(&v_rows.clone(), None, Some((py::add(&v_n.clone(), &Py::Int(1i64))?).i()?), None)?))? { let mut v_r: Py; 
v_r = t3182;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t3183 in py::iter(&(v_first.clone()))? { let mut v_v: Py; 
v_v = t3183;
out.push({ let t3184 = v_v.clone(); Py::Bool({ let t3185 = Py::Int(0i64); if py::eq(&t3184, &t3185) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Routh array has a zero in its first column: this version decides regular cases only"))));
    }
    v_changes = py::sum(&{ let mut out = vec![]; for t3186 in py::iter(&(py::zip(&[v_first.clone(), py::slice(&v_first.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3187 = (t3186).items()?;
if t3187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3187[0].clone();
v_y = t3187[1].clone();
if ({ let t3188 = { let t3190 = v_x.clone(); Py::Bool({ let t3191 = Py::Int(0i64); if py::gt(&t3190, &t3191)? { true } else { false } }) }; Py::Bool({ let t3189 = { let t3192 = v_y.clone(); Py::Bool({ let t3193 = Py::Int(0i64); if py::gt(&t3192, &t3193)? { true } else { false } }) }; if py::ne(&t3188, &t3189) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    return Ok(Py::tuple(vec![v_first.clone(), v_changes.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sine_coeff(mut v_co: Py, mut v_L: Py, mut v_n: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_sgn: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_w = py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())?;
    v_sgn = (if (py::modulo(&v_n.clone(), &Py::Int(2i64))?).truthy() { py::neg(&Py::Float(1.0))? } else { Py::Float(1.0) });
    let t3884 = (Py::tuple(vec![Py::list(vec![py::truediv(&py::sub(&Py::Int(1i64), &v_sgn.clone())?, &v_w.clone())?]), Py::list(vec![Py::Float(0.0)])])).items()?;
    if t3884.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t3884[0].clone();
    v_C = t3884[1].clone();
    for t3885 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_co.clone())?).i()?]) {
        v_k = t3885;
        let _ = py::method(&v_S.clone(), "append", &[py::add(&py::truediv(&py::mul(&py::neg(&py::pow(&v_L.clone(), &v_k.clone())?)?, &v_sgn.clone())?, &v_w.clone())?, &py::mul(&py::truediv(&v_k.clone(), &v_w.clone())?, &py::getitem(&v_C.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?])?;
        let _ = py::method(&v_C.clone(), "append", &[py::mul(&py::truediv(&py::neg(&v_k.clone())?, &v_w.clone())?, &py::getitem(&v_S.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?])?;
    }
    v_terms = { let mut out = vec![]; for t3886 in py::iter(&(py::zip(&[v_co.clone(), v_S.clone()])?))? { let mut v_a: Py; let mut v_s: Py; 
let t3887 = (t3886).items()?;
if t3887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3887[0].clone();
v_s = t3887[1].clone();
out.push(py::mul(&v_a.clone(), &v_s.clone())?); } Py::list(out) };
    return Ok((if (Py::Bool((v_terms.clone()).items()?.iter().any(|x| x.truthy()))).truthy() { py::mul(&py::truediv(&Py::Int(2i64), &v_L.clone())?, &f__guard(py::sum(&v_terms.clone(), None)?, { let mut v = vec![]; v.extend((v_terms.clone()).items()?); Py::tuple(v) })?)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stab_word(mut v_m: Py) -> R {
    return Ok((if ({ let t2180 = py::abs(&v_m.clone())?; Py::Bool({ let t2181 = Py::Int(1i64); if py::lt(&t2180, &t2181)? { true } else { false } }) }).truthy() { Py::str("stable") } else { (if ({ let t2182 = py::abs(&v_m.clone())?; Py::Bool({ let t2183 = Py::Int(1i64); if py::gt(&t2182, &t2183)? { true } else { false } }) }).truthy() { Py::str("unstable") } else { Py::str("neutral") }) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_char(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t470 = (Py::tuple(vec![py::getitem(&v_i.clone(), &Py::str("a"))?, py::getitem(&v_i.clone(), &Py::str("b"))?, py::getitem(&v_i.clone(), &Py::str("c"))?])).items()?;
    if t470.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t470[0].clone();
    v_b = t470[1].clone();
    v_c = t470[2].clone();
    let t500: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_worst = Py::Float(0.0);
            for t471 in py::iter(&(Py::tuple(vec![py::complex(&[py::getitem(&v_o.clone(), &Py::str("r1"))?])?, py::complex(&[py::getitem(&v_o.clone(), &Py::str("r2"))?])?])))? {
                v_r = t471;
                v_terms = Py::list(vec![py::mul(&py::mul(&v_a.clone(), &v_r.clone())?, &v_r.clone())?, py::mul(&v_b.clone(), &v_r.clone())?, v_c.clone()]);
                v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sum(&v_terms.clone(), None)?)?, &{ let t473 = py::sum(&{ let mut out = vec![]; for t472 in py::iter(&(v_terms.clone()))? { let mut v_x: Py; 
v_x = t472;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }, None)?; if t473.truthy() { t473 } else { Py::Float(1.0) } })?], true)?;
            }
            let t474 = (Py::tuple(vec![py::complex(&[py::getitem(&v_o.clone(), &Py::str("r1"))?])?, py::complex(&[py::getitem(&v_o.clone(), &Py::str("r2"))?])?])).items()?;
            if t474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_r1 = t474[0].clone();
            v_r2 = t474[1].clone();
            v_worst = py::minmax(&[v_worst.clone(), py::mul(&f__diff(py::add(&v_r1.clone(), &v_r2.clone())?, py::truediv(&py::neg(&v_b.clone())?, &v_a.clone())?)?, &(if ({ let t497 = py::abs(&py::truediv(&v_b.clone(), &v_a.clone())?)?; Py::Bool({ let t498 = Py::Float(1e-12); if py::gt(&t497, &t498)? { true } else { false } }) }).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }))?, py::mul(&f__diff(py::mul(&v_r1.clone(), &v_r2.clone())?, py::truediv(&v_c.clone(), &v_a.clone())?)?, &(if (v_c.clone()).truthy() { Py::Int(1i64) } else { Py::Int(0i64) }))?], true)?;
            return Ok(Some(py::minmax(&[v_worst.clone(), f__diff(py::method(&v_o.clone(), "get", &[Py::str("damping")])?, py::getitem(&{ let t499 = v_i.clone(); f_char_roots(py::getitem(&t499, &Py::str("a"))?, py::getitem(&t499, &Py::str("b"))?, py::getitem(&t499, &Py::str("c"))?)? }, &Py::str("damping"))?)?], true)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t500 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_lfo(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v__size: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_P = f__lfo_rows(py::getitem(&v_i.clone(), &Py::str("p_terms"))?, Py::Int(2i64), Py::str("P(x)"))?;
    v_Q = f__lfo_rows(py::getitem(&v_i.clone(), &Py::str("q_terms"))?, Py::Int(3i64), Py::str("Q(x)"))?;
    let t1831 = (f__lfo_integral(v_P.clone(), v_Q.clone(), py::getitem(&v_i.clone(), &Py::str("x0"))?, py::getitem(&v_i.clone(), &Py::str("x1"))?, Py::Int(384i64))?).items()?;
    if t1831.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_I = t1831[0].clone();
    v__size = t1831[1].clone();
    v_ratio = py::math1("exp", &py::sub(&f__lfo_F(v_P.clone(), py::getitem(&v_i.clone(), &Py::str("x1"))?)?, &f__lfo_F(v_P.clone(), py::getitem(&v_i.clone(), &Py::str("x0"))?)?)?)?;
    v_total = py::add(&py::getitem(&v_i.clone(), &Py::str("y0"))?, &v_I.clone())?;
    if ({ let t1832 = py::abs(&v_total.clone())?; Py::Bool({ let t1833 = py::mul(&Py::Float(1e-13), &py::add(&py::abs(&py::getitem(&v_i.clone(), &Py::str("y0"))?)?, &py::abs(&v_I.clone())?)?)?; if py::lt(&t1832, &t1833)? { true } else { false } }) }).truthy() {
        v_total = Py::Float(0.0);
    }
    if ({ let t1834 = py::getitem(&v_o.clone(), &Py::str("integrating_factor"))?; Py::Bool({ let t1835 = f__lfo_factor_text(v_P.clone())?; if py::ne(&t1834, &t1835) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    return Ok(py::minmax(&[f__diff(py::getitem(&v_o.clone(), &Py::str("y"))?, py::truediv(&v_total.clone(), &v_ratio.clone())?)?, f__diff(py::getitem(&v_o.clone(), &Py::str("mu_ratio"))?, v_ratio.clone())?, f__diff(py::getitem(&v_o.clone(), &Py::str("integral"))?, v_I.clone())?], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__verify_partial(mut v_i: Py, mut v_o: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Pc: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v_Qc: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_got: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t2866: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2833 = (f__ratfun(py::getitem(&v_i.clone(), &Py::str("F"))?)?).items()?;
            if t2833.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_P = t2833[0].clone();
            v_Q = t2833[1].clone();
            let t2836 = (Py::tuple(vec![{ let mut out = vec![]; for t2834 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2834;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }, { let mut out = vec![]; for t2835 in py::iter(&(py::method(&v_Q.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2835;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }])).items()?;
            if t2836.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Pc = t2836[0].clone();
            v_Qc = t2836[1].clone();
            v_ev = { let v_P = v_P.clone(); let v_Pc = v_Pc.clone(); let v_Q = v_Q.clone(); let v_Qc = v_Qc.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_co: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2837 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t2838 = (t2837).items()?;
if t2838.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2838[0].clone();
v_c = t2838[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_z.clone(), &py::sub(&py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?, &v_k.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            v_worst = Py::Float(0.0);
            for t2839 in py::iter(&(Py::tuple(vec![py::complex(&[Py::Float(0.37), Py::Float(1.91)])?, py::complex(&[py::neg(&Py::Float(2.3))?, Py::Float(0.7)])?, py::complex(&[Py::Float(5.1), py::neg(&Py::Float(0.4))?])?])))? {
                v_z = t2839;
                v_want = py::truediv(&py::call(&v_ev.clone(), &[v_Pc.clone(), v_z.clone()])?, &py::call(&v_ev.clone(), &[v_Qc.clone(), v_z.clone()])?)?;
                v_got = py::sum(&{ let mut out = vec![]; for t2840 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("poles"))?, py::getitem(&v_o.clone(), &Py::str("residues"))?])?))? { let mut v_p: Py; let mut v_r: Py; 
let t2841 = (t2840).items()?;
if t2841.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t2841[0].clone();
v_r = t2841[1].clone();
out.push(py::truediv(&py::complex(&[v_r.clone()])?, &py::sub(&v_z.clone(), &py::complex(&[v_p.clone()])?)?)?); } Py::list(out) }, None)?;
                v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_got.clone(), &v_want.clone())?)?, &py::minmax(&[py::abs(&v_want.clone())?, Py::Float(1e-300)], true)?)?], true)?;
            }
            return Ok(Some(py::minmax(&[(if ({ let t2842 = v_worst.clone(); Py::Bool({ let t2843 = Py::Float(5e-11); if py::gt(&t2842, &t2843)? { true } else { false } }) }).truthy() { v_worst.clone() } else { Py::Float(0.0) }), f__diff(py::method(&v_o.clone(), "get", &[Py::str("count")])?, py::len(&py::getitem(&v_o.clone(), &Py::str("poles"))?)?)?, f__diff(py::len(&py::getitem(&v_o.clone(), &Py::str("poles"))?)?, py::method(&v_Q.clone(), "degree", &[])?)?], true)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2866 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_amplification(mut v_method: Py, mut v_lam: Py, mut v_h: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_lam.clone(), v_h.clone()]))?;
    if ({ let t4 = v_h.clone(); Py::Bool({ let t5 = Py::Int(0i64); if py::le(&t4, &t5)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step h must be positive"))));
    }
    v_z = py::mul(&v_lam.clone(), &v_h.clone())?;
    v_R = f__R(v_method.clone(), v_z.clone())?;
    return Ok(Py::dict(vec![(Py::str("R"), f__r(v_R.clone())?), (Py::str("abs_R"), f__r(py::abs(&v_R.clone())?)?), (Py::str("stable"), { let t18 = py::abs(&v_R.clone())?; Py::Bool({ let t19 = Py::Int(1i64); if py::le(&t18, &t19)? { true } else { false } }) }), (Py::str("z"), f__r(v_z.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_backward_euler(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dz: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_gy: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t0.clone(), v_y0.clone(), v_h.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t106 = { let t101 = v_h.clone(); Py::Bool({ let t102 = Py::Int(0i64); if py::le(&t101, &t102)? { true } else { false } }) }; if t106.truthy() { t106 } else { py::not(&{ let t103 = Py::Int(1i64); Py::Bool({ let t104 = v_n.clone(); if py::le(&t103, &t104)? { { let t105 = Py::Int(10000i64); if py::le(&t104, &t105)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs h > 0 and 1 <= n <= 10000"))));
    }
    v_e = ax_engine::pysym::helper("differential_equations:parse_text", &[v_f.clone()], &[])?;
    let t107 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:compile_float", &[v_e.clone(), Py::tuple(vec![Py::str("t"), Py::str("y")])], &[])?, ax_engine::pysym::helper("differential_equations:compile_float", &[ax_engine::pysym::call("diff", &[v_e.clone(), c_YS()?], &[])?, Py::tuple(vec![Py::str("t"), Py::str("y")])], &[])?])).items()?;
    if t107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_g = t107[0].clone();
    v_gy = t107[1].clone();
    v_y = v_y0.clone();
    for t108 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t108;
        v_t1 = py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?;
        v_z = py::add(&v_y.clone(), &py::mul(&v_h.clone(), &py::call(&v_g.clone(), &[py::add(&v_t0.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?, v_y.clone()])?)?)?;
        let mut t109 = false;
        for t110 in py::range_iter(&[(Py::Int(100i64)).i()?]) {
            v__ = t110;
            let t111 = (Py::tuple(vec![py::sub(&py::sub(&v_z.clone(), &v_y.clone())?, &py::mul(&v_h.clone(), &py::call(&v_g.clone(), &[v_t1.clone(), v_z.clone()])?)?)?, py::sub(&Py::Int(1i64), &py::mul(&v_h.clone(), &py::call(&v_gy.clone(), &[v_t1.clone(), v_z.clone()])?)?)?])).items()?;
            if t111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_F = t111[0].clone();
            v_J = t111[1].clone();
            if ({ let t112 = v_J.clone(); Py::Bool({ let t113 = Py::Int(0i64); if py::eq(&t112, &t113) { true } else { false } }) }).truthy() {
                return Err(py::value_error(py::to_str(&Py::str("Newton's method met a zero derivative"))));
            }
            v_dz = py::truediv(&v_F.clone(), &v_J.clone())?;
            v_z = py::sub(&v_z, &v_dz.clone())?;
            if ({ let t114 = py::abs(&v_dz.clone())?; Py::Bool({ let t115 = py::minmax(&[Py::Float(1e-15), py::mul(&Py::Float(4e-16), &py::abs(&v_z.clone())?)?], true)?; if py::le(&t114, &t115)? { true } else { false } }) }).truthy() {
                { t109 = true; break; }
            }
        }
        if !t109 {
            return Err(py::value_error(py::to_str(&Py::str("Newton's method did not converge for the implicit step"))));
        }
        if ({ let t116 = py::mul(&v_h.clone(), &py::abs(&py::call(&v_gy.clone(), &[v_t1.clone(), v_z.clone()])?)?)?; Py::Bool({ let t117 = Py::Int(1i64); if py::ge(&t116, &t117)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the step is too large for a unique implicit solution (needs h*|df/dy| < 1)"))));
        }
        v_y = v_z.clone();
        if ({ let t120 = py::not(&py::math1("isfinite", &v_y.clone())?); if t120.truthy() { t120 } else { { let t118 = py::abs(&v_y.clone())?; Py::Bool({ let t119 = Py::Float(1e+100); if py::gt(&t118, &t119)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the numerical solution overflows"))));
        }
    }
    return Ok(Py::dict(vec![(Py::str("y_n"), f__r(v_y.clone())?), (Py::str("t_n"), f__r(py::add(&v_t0.clone(), &py::mul(&v_n.clone(), &v_h.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bernoulli(mut v_p: Py, mut v_q: Py, mut v_n: Py, mut v_y0: Py, mut v_t: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_v0: Py = Py::None;
    let mut v_vs: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_p.clone(), v_q.clone(), v_n.clone(), v_y0.clone(), v_t.clone()]))?;
    if ({ let t163 = { let t153 = v_p.clone(); Py::Bool({ let t154 = Py::Int(0i64); if py::eq(&t153, &t154) { true } else { false } }) }; if t163.truthy() { t163 } else { { let t162 = { let t155 = v_n.clone(); Py::Bool({ let t156 = Py::Int(1i64); if py::eq(&t155, &t156) { true } else { false } }) }; if t162.truthy() { t162 } else { { let t161 = { let t157 = v_y0.clone(); Py::Bool({ let t158 = Py::Int(0i64); if py::le(&t157, &t158)? { true } else { false } }) }; if t161.truthy() { t161 } else { { let t159 = v_t.clone(); Py::Bool({ let t160 = Py::Int(0i64); if py::lt(&t159, &t160)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs p != 0, n != 1, y0 > 0 and t >= 0"))));
    }
    v_m = py::sub(&Py::Int(1i64), &v_n.clone())?;
    v_vs = py::truediv(&v_q.clone(), &v_p.clone())?;
    v_v = py::add(&v_vs.clone(), &py::mul(&py::sub(&py::pow(&v_y0.clone(), &v_m.clone())?, &v_vs.clone())?, &f__exp(py::mul(&py::mul(&py::neg(&v_m.clone())?, &v_p.clone())?, &v_t.clone())?)?)?)?;
    v_v0 = py::pow(&v_y0.clone(), &v_m.clone())?;
    if ({ let t172 = { let t165 = v_v.clone(); Py::Bool({ let t166 = Py::Int(0i64); if py::le(&t165, &t166)? { true } else { false } }) }; if t172.truthy() { t172 } else { { let t171 = { let t167 = v_vs.clone(); Py::Bool({ let t168 = Py::Int(0i64); if py::le(&t167, &t168)? { true } else { false } }) }; if !t171.truthy() { t171 } else { { let t169 = v_v.clone(); Py::Bool({ let t170 = py::mul(&Py::Float(1e-12), &v_v0.clone())?; if py::le(&t169, &t170)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the solution reaches zero or becomes unbounded before t"))));
    }
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::pow(&v_v.clone(), &py::truediv(&Py::Int(1i64), &v_m.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_brusselator(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_stab: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone()]))?;
    if ({ let t279 = { let t275 = v_a.clone(); Py::Bool({ let t276 = Py::Int(0i64); if py::le(&t275, &t276)? { true } else { false } }) }; if t279.truthy() { t279 } else { { let t277 = v_b.clone(); Py::Bool({ let t278 = Py::Int(0i64); if py::le(&t277, &t278)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a > 0 and b > 0"))));
    }
    let t280 = (Py::tuple(vec![py::fraction(&[v_a.clone()])?, py::fraction(&[v_b.clone()])?])).items()?;
    if t280.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t280[0].clone();
    v_B = t280[1].clone();
    let t298 = (f__classify(py::sub(&py::sub(&v_B.clone(), &Py::Int(1i64))?, &py::mul(&v_A.clone(), &v_A.clone())?)?, py::mul(&v_A.clone(), &v_A.clone())?, Py::Bool(false), Py::Bool(false))?).items()?;
    if t298.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kind = t298[0].clone();
    v_stab = t298[1].clone();
    return Ok(Py::dict(vec![(Py::str("x_eq"), f__r(v_a.clone())?), (Py::str("y_eq"), f__r(py::truediv(&v_b.clone(), &v_a.clone())?)?), (Py::str("b_critical"), f__r(py::add(&Py::Int(1i64), &py::mul(&v_a.clone(), &v_a.clone())?)?)?), (Py::str("type"), v_kind.clone()), (Py::str("stability"), v_stab.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bvp2(mut v_p: Py, mut v_q: Py, mut v_r: Py, mut v_xa: Py, mut v_xb: Py, mut v_ya: Py, mut v_yb: Py, mut v_x: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_u1: Py = Py::None;
    let mut v_u2: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let mut v_ypd: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_p.clone(), v_q.clone(), v_r.clone(), v_xa.clone(), v_xb.clone(), v_ya.clone(), v_yb.clone(), v_x.clone()]))?;
    if ({ let t205 = v_xb.clone(); Py::Bool({ let t206 = v_xa.clone(); if py::le(&t205, &t206)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs xa < xb"))));
    }
    if ({ let t207 = v_q.clone(); Py::Bool({ let t208 = Py::Int(0i64); if py::ne(&t207, &t208) { true } else { false } }) }).truthy() {
        let t209 = (Py::tuple(vec![{ let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); let v_xa = v_xa.clone(); let v_xb = v_xb.clone(); let v_ya = v_ya.clone(); let v_yb = v_yb.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&v_r.clone(), &v_q.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); let v_xa = v_xa.clone(); let v_xb = v_xb.clone(); let v_ya = v_ya.clone(); let v_yb = v_yb.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::Float(0.0));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }])).items()?;
        if t209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_yp = t209[0].clone();
        v_ypd = t209[1].clone();
    } else {
        if ({ let t210 = v_p.clone(); Py::Bool({ let t211 = Py::Int(0i64); if py::ne(&t210, &t211) { true } else { false } }) }).truthy() {
            let t212 = (Py::tuple(vec![{ let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); let v_xa = v_xa.clone(); let v_xb = v_xb.clone(); let v_ya = v_ya.clone(); let v_yb = v_yb.clone(); let v_yp = v_yp.clone(); let v_ypd = v_ypd.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&v_r.clone(), &v_s.clone())?, &v_p.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); let v_xa = v_xa.clone(); let v_xb = v_xb.clone(); let v_ya = v_ya.clone(); let v_yb = v_yb.clone(); let v_yp = v_yp.clone(); let v_ypd = v_ypd.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&v_r.clone(), &v_p.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }])).items()?;
            if t212.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_yp = t212[0].clone();
            v_ypd = t212[1].clone();
        } else {
            let t213 = (Py::tuple(vec![{ let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); let v_xa = v_xa.clone(); let v_xb = v_xb.clone(); let v_ya = v_ya.clone(); let v_yb = v_yb.clone(); let v_yp = v_yp.clone(); let v_ypd = v_ypd.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&py::mul(&py::mul(&v_r.clone(), &v_s.clone())?, &v_s.clone())?, &Py::Int(2i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, { let v_p = v_p.clone(); let v_q = v_q.clone(); let v_r = v_r.clone(); let v_x = v_x.clone(); let v_xa = v_xa.clone(); let v_xb = v_xb.clone(); let v_ya = v_ya.clone(); let v_yb = v_yb.clone(); let v_yp = v_yp.clone(); let v_ypd = v_ypd.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&v_r.clone(), &v_s.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }])).items()?;
            if t213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_yp = t213[0].clone();
            v_ypd = t213[1].clone();
        }
    }
    v_L = py::sub(&v_xb.clone(), &v_xa.clone())?;
    let t234 = (Py::tuple(vec![f_hom2(Py::Int(1i64), v_p.clone(), v_q.clone(), Py::Float(1.0), Py::Float(0.0), v_L.clone())?, f_hom2(Py::Int(1i64), v_p.clone(), v_q.clone(), Py::Float(0.0), Py::Float(1.0), v_L.clone())?])).items()?;
    if t234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u1 = t234[0].clone();
    v_u2 = t234[1].clone();
    if ({ let t235 = py::abs(&v_u2.clone())?; Py::Bool({ let t236 = py::mul(&Py::Float(1e-12), &py::add(&py::abs(&v_u1.clone())?, &v_L.clone())?)?; if py::le(&t235, &t236)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the boundary value problem has no unique solution (an eigenvalue case)"))));
    }
    v_alpha = py::sub(&v_ya.clone(), &py::call(&v_yp.clone(), &[v_xa.clone()])?)?;
    v_beta = py::truediv(&py::sub(&py::sub(&v_yb.clone(), &py::call(&v_yp.clone(), &[v_xb.clone()])?)?, &py::mul(&v_alpha.clone(), &v_u1.clone())?)?, &v_u2.clone())?;
    v_h = (if ({ let t237 = v_x.clone(); Py::Bool({ let t238 = v_xa.clone(); if py::ge(&t237, &t238)? { true } else { false } }) }).truthy() { f_hom2(Py::Int(1i64), v_p.clone(), v_q.clone(), v_alpha.clone(), v_beta.clone(), py::sub(&v_x.clone(), &v_xa.clone())?)? } else { Py::None });
    if ({ let t239 = v_h.clone(); Py::Bool({ let t240 = Py::None; if py::is(&t239, &t240) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x must lie in [xa, xb]"))));
    }
    if ({ let t241 = v_x.clone(); Py::Bool({ let t242 = v_xb.clone(); if py::gt(&t241, &t242)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("x must lie in [xa, xb]"))));
    }
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__guard(py::add(&py::call(&v_yp.clone(), &[v_x.clone()])?, &v_h.clone())?, Py::tuple(vec![py::call(&v_yp.clone(), &[v_x.clone()])?, v_h.clone()]))?)?), (Py::str("slope_a"), f__r(py::add(&v_beta.clone(), &py::call(&v_ypd.clone(), &[v_xa.clone()])?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cauchy_euler(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_y1: Py, mut v_v1: Py, mut v_t: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_y1.clone(), v_v1.clone(), v_t.clone()]))?;
    if ({ let t335 = { let t331 = v_a.clone(); Py::Bool({ let t332 = Py::Int(0i64); if py::eq(&t331, &t332) { true } else { false } }) }; if t335.truthy() { t335 } else { { let t333 = v_t.clone(); Py::Bool({ let t334 = Py::Int(0i64); if py::le(&t333, &t334)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a != 0 and t > 0"))));
    }
    let t342 = (f__roots2(v_a.clone(), py::sub(&v_b.clone(), &v_a.clone())?, v_c.clone())?).items()?;
    if t342.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t342[0].clone();
    v_r2 = t342[1].clone();
    v_D = t342[2].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f_hom2(v_a.clone(), py::sub(&v_b.clone(), &v_a.clone())?, v_c.clone(), v_y1.clone(), v_v1.clone(), py::math1("log", &v_t.clone())?)?)?), (Py::str("m1"), f__r(v_r1.clone())?), (Py::str("m2"), f__r(v_r2.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cfl(mut v_c: Py, mut v_dt: Py, mut v_dx: Py) -> R {
    let mut v_C: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_c.clone(), v_dt.clone(), v_dx.clone()]))?;
    if ({ let t402 = { let t395 = v_c.clone(); Py::Bool({ let t396 = Py::Int(0i64); if py::eq(&t395, &t396) { true } else { false } }) }; if t402.truthy() { t402 } else { { let t401 = { let t397 = v_dt.clone(); Py::Bool({ let t398 = Py::Int(0i64); if py::le(&t397, &t398)? { true } else { false } }) }; if t401.truthy() { t401 } else { { let t399 = v_dx.clone(); Py::Bool({ let t400 = Py::Int(0i64); if py::le(&t399, &t400)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs c != 0, dt > 0 and dx > 0"))));
    }
    v_C = py::truediv(&py::mul(&py::abs(&v_c.clone())?, &v_dt.clone())?, &v_dx.clone())?;
    return Ok(Py::dict(vec![(Py::str("courant"), f__r(v_C.clone())?), (Py::str("stable"), { let t405 = v_C.clone(); Py::Bool({ let t406 = Py::Int(1i64); if py::le(&t405, &t406)? { true } else { false } }) }), (Py::str("dt_max"), f__r(py::truediv(&v_dx.clone(), &py::abs(&v_c.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_char_roots(mut v_a: Py, mut v_b: Py, mut v_c: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_form: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone()]))?;
    if ({ let t437 = v_a.clone(); Py::Bool({ let t438 = Py::Int(0i64); if py::eq(&t437, &t438) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (the equation would be first order)"))));
    }
    let t445 = (f__roots2(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t445.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t445[0].clone();
    v_r2 = t445[1].clone();
    v_D = t445[2].clone();
    if ({ let t446 = v_D.clone(); Py::Bool({ let t447 = Py::Int(0i64); if py::gt(&t446, &t447)? { true } else { false } }) }).truthy() {
        let t448 = (Py::tuple(vec![Py::str("y = C1 e^(r1 t) + C2 e^(r2 t)"), Py::str("two distinct real roots")])).items()?;
        if t448.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_form = t448[0].clone();
        v_kind = t448[1].clone();
    } else {
        if ({ let t449 = v_D.clone(); Py::Bool({ let t450 = Py::Int(0i64); if py::eq(&t449, &t450) { true } else { false } }) }).truthy() {
            let t451 = (Py::tuple(vec![Py::str("y = (C1 + C2 t) e^(r t)"), Py::str("one repeated real root")])).items()?;
            if t451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_form = t451[0].clone();
            v_kind = t451[1].clone();
        } else {
            let t452 = (Py::tuple(vec![Py::str("y = e^(alpha t) (C1 cos(beta t) + C2 sin(beta t)), alpha = Re r1, beta = Im r1"), Py::str("complex conjugate roots")])).items()?;
            if t452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_form = t452[0].clone();
            v_kind = t452[1].clone();
        }
    }
    return Ok(Py::dict(vec![(Py::str("r1"), f__r(v_r1.clone())?), (Py::str("r2"), f__r(v_r2.clone())?), (Py::str("discriminant"), f__r(py::float(&v_D.clone())?)?), (Py::str("roots_type"), v_kind.clone()), (Py::str("damping"), f__damping(v_a.clone(), v_b.clone(), v_c.clone(), v_D.clone())?), (Py::str("solution_form"), v_form.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_char_time(mut v_k: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_k.clone()]))?;
    if ({ let t757 = v_k.clone(); Py::Bool({ let t758 = Py::Int(0i64); if py::eq(&t757, &t758) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("k must not be 0 (no growth or decay)"))));
    }
    return Ok(Py::dict(vec![(Py::str("t_char"), f__r(py::truediv(&py::math1("log", &Py::Int(2i64))?, &py::abs(&v_k.clone())?)?)?), (Py::str("kind"), (if ({ let t761 = v_k.clone(); Py::Bool({ let t762 = Py::Int(0i64); if py::gt(&t761, &t762)? { true } else { false } }) }).truthy() { Py::str("doubling time") } else { Py::str("half-life") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_competing_species(mut v_a1: Py, mut v_b11: Py, mut v_b12: Py, mut v_a2: Py, mut v_b21: Py, mut v_b22: Py) -> R {
    let mut v_A1: Py = Py::None;
    let mut v_A2: Py = Py::None;
    let mut v_B11: Py = Py::None;
    let mut v_B12: Py = Py::None;
    let mut v_B21: Py = Py::None;
    let mut v_B22: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_stab: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a1.clone(), v_b11.clone(), v_b12.clone(), v_a2.clone(), v_b21.clone(), v_b22.clone()]))?;
    let t505 = ({ let mut out = vec![]; for t504 in py::iter(&(Py::tuple(vec![v_a1.clone(), v_b11.clone(), v_b12.clone(), v_a2.clone(), v_b21.clone(), v_b22.clone()])))? { let mut v_v: Py; 
v_v = t504;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }).items()?;
    if t505.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_A1 = t505[0].clone();
    v_B11 = t505[1].clone();
    v_B12 = t505[2].clone();
    v_A2 = t505[3].clone();
    v_B21 = t505[4].clone();
    v_B22 = t505[5].clone();
    v_det = py::sub(&py::mul(&v_B11.clone(), &v_B22.clone())?, &py::mul(&v_B12.clone(), &v_B21.clone())?)?;
    if ({ let t506 = v_det.clone(); Py::Bool({ let t507 = Py::Int(0i64); if py::eq(&t506, &t507) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the nullclines are parallel: no isolated coexistence equilibrium"))));
    }
    v_x = py::truediv(&py::sub(&py::mul(&v_A1.clone(), &v_B22.clone())?, &py::mul(&v_B12.clone(), &v_A2.clone())?)?, &v_det.clone())?;
    v_y = py::truediv(&py::sub(&py::mul(&v_B11.clone(), &v_A2.clone())?, &py::mul(&v_B21.clone(), &v_A1.clone())?)?, &v_det.clone())?;
    if ({ let t512 = { let t508 = v_x.clone(); Py::Bool({ let t509 = Py::Int(0i64); if py::le(&t508, &t509)? { true } else { false } }) }; if t512.truthy() { t512 } else { { let t510 = v_y.clone(); Py::Bool({ let t511 = Py::Int(0i64); if py::le(&t510, &t511)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("there is no coexistence equilibrium with both populations positive"))));
    }
    v_J = Py::list(vec![Py::list(vec![py::mul(&py::neg(&v_B11.clone())?, &v_x.clone())?, py::mul(&py::neg(&v_B12.clone())?, &v_x.clone())?]), Py::list(vec![py::mul(&py::neg(&v_B21.clone())?, &v_y.clone())?, py::mul(&py::neg(&v_B22.clone())?, &v_y.clone())?])]);
    let t530 = (f__classify(py::add(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, py::sub(&py::mul(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, Py::Bool(false), Py::Bool(false))?).items()?;
    if t530.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kind = t530[0].clone();
    v_stab = t530[1].clone();
    return Ok(Py::dict(vec![(Py::str("x_eq"), f__r(py::float(&v_x.clone())?)?), (Py::str("y_eq"), f__r(py::float(&v_y.clone())?)?), (Py::str("type"), v_kind.clone()), (Py::str("stability"), v_stab.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_convolution(mut v_f: Py, mut v_g: Py, mut v_t: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_err: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t.clone()]))?;
    if ({ let t563 = v_t.clone(); Py::Bool({ let t564 = Py::Int(0i64); if py::lt(&t563, &t564)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("t must be >= 0"))));
    }
    if ({ let t565 = v_t.clone(); Py::Bool({ let t566 = Py::Int(0i64); if py::eq(&t565, &t566) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("value"), Py::Float(0.0))]));
    }
    let t567 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone(), Py::tuple(vec![Py::str("t")])], &[])?, ax_engine::pysym::helper("differential_equations:_fn", &[v_g.clone(), Py::tuple(vec![Py::str("t")])], &[])?])).items()?;
    if t567.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t567[0].clone();
    v_G = t567[1].clone();
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
        let t568 = (ax_engine::pymp::call("quad", &[{ let v_F = v_F.clone(); let v_G = v_G.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_t = v_t.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_u: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&py::call(&v_F.clone(), &[py::float(&v_u.clone())?])?, &py::call(&v_G.clone(), &[py::sub(&v_t.clone(), &py::float(&v_u.clone())?)?])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![Py::Int(0i64), py::truediv(&v_t.clone(), &Py::Int(2i64))?, v_t.clone()])], &[(String::from("error"), Py::Bool(true))])?).items()?;
        if t568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_v = t568[0].clone();
        v_err = t568[1].clone();
    }
    if ({ let t569 = v_err.clone(); Py::Bool({ let t570 = py::mul(&Py::Float(1e-13), &py::minmax(&[Py::Float(1.0), py::abs(&v_v.clone())?], true)?)?; if py::gt(&t569, &t570)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the convolution integral did not converge"))));
    }
    return Ok(Py::dict(vec![(Py::str("value"), f__r(py::float(&v_v.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cooling_T(mut v_T0: Py, mut v_Ta: Py, mut v_k: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_T0.clone(), v_Ta.clone(), v_k.clone(), v_t.clone()]))?;
    if ({ let t2511 = v_k.clone(); Py::Bool({ let t2512 = Py::Int(0i64); if py::le(&t2511, &t2512)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cooling constant k must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("T"), f__r(py::add(&v_Ta.clone(), &py::mul(&py::sub(&v_T0.clone(), &v_Ta.clone())?, &f__exp(py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cooling_k(mut v_T0: Py, mut v_Ta: Py, mut v_T1: Py, mut v_t1: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_T0.clone(), v_Ta.clone(), v_T1.clone(), v_t1.clone()]))?;
    if ({ let t2478 = { let t2470 = v_t1.clone(); Py::Bool({ let t2471 = Py::Int(0i64); if py::le(&t2470, &t2471)? { true } else { false } }) }; if t2478.truthy() { t2478 } else { { let t2477 = { let t2472 = v_T0.clone(); Py::Bool({ let t2473 = v_Ta.clone(); if py::eq(&t2472, &t2473) { true } else { false } }) }; if t2477.truthy() { t2477 } else { py::not(&{ let t2474 = py::minmax(&[v_T0.clone(), v_Ta.clone()], false)?; Py::Bool({ let t2475 = v_T1.clone(); if py::lt(&t2474, &t2475)? { { let t2476 = py::minmax(&[v_T0.clone(), v_Ta.clone()], true)?; if py::lt(&t2475, &t2476)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs t1 > 0 and T1 strictly between Ta and T0"))));
    }
    return Ok(Py::dict(vec![(Py::str("k"), f__r(py::truediv(&py::math1("log", &py::truediv(&py::sub(&v_T0.clone(), &v_Ta.clone())?, &py::sub(&v_T1.clone(), &v_Ta.clone())?)?)?, &v_t1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cooling_time(mut v_T0: Py, mut v_Ta: Py, mut v_k: Py, mut v_T1: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_T0.clone(), v_Ta.clone(), v_k.clone(), v_T1.clone()]))?;
    if ({ let t2546 = v_k.clone(); Py::Bool({ let t2547 = Py::Int(0i64); if py::le(&t2546, &t2547)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cooling constant k must be positive"))));
    }
    if ({ let t2556 = { let t2548 = v_T0.clone(); Py::Bool({ let t2549 = v_Ta.clone(); if py::eq(&t2548, &t2549) { true } else { false } }) }; if t2556.truthy() { t2556 } else { { let t2555 = py::not(&{ let t2550 = py::minmax(&[v_T0.clone(), v_Ta.clone()], false)?; Py::Bool({ let t2551 = v_T1.clone(); if py::lt(&t2550, &t2551)? { { let t2552 = py::minmax(&[v_T0.clone(), v_Ta.clone()], true)?; if py::le(&t2551, &t2552)? { true } else { false } } } else { false } }) }); if t2555.truthy() { t2555 } else { { let t2553 = py::truediv(&py::sub(&v_T1.clone(), &v_Ta.clone())?, &py::sub(&v_T0.clone(), &v_Ta.clone())?)?; Py::Bool({ let t2554 = Py::Int(0i64); if py::le(&t2553, &t2554)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("T1 must lie between the ambient temperature and T0"))));
    }
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::truediv(&py::math1("log", &py::truediv(&py::sub(&v_T0.clone(), &v_Ta.clone())?, &py::sub(&v_T1.clone(), &v_Ta.clone())?)?)?, &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_crank_nicolson(mut v_u: Py, mut v_r: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cp: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if ({ let t600 = py::len(&v_u.clone())?; Py::Bool({ let t601 = Py::Int(3i64); if py::lt(&t600, &t601)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least 3 grid values"))));
    }
    if ({ let t607 = { let t602 = v_r.clone(); Py::Bool({ let t603 = Py::Int(0i64); if py::le(&t602, &t603)? { true } else { false } }) }; if t607.truthy() { t607 } else { py::not(&{ let t604 = Py::Int(1i64); Py::Bool({ let t605 = v_n.clone(); if py::le(&t604, &t605)? { { let t606 = Py::Int(1000i64); if py::le(&t605, &t606)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs r > 0 and 1 <= n <= 1000"))));
    }
    v_v = Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_u.clone())?).items()?);
    v_m = py::sub(&py::len(&v_v.clone())?, &Py::Int(2i64))?;
    for t608 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t608;
        v_d = { let mut out = vec![]; for t609 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_m.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t609;
out.push(py::add(&py::add(&py::mul(&py::truediv(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_v.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::sub(&Py::Int(1i64), &v_r.clone())?, &py::getitem(&v_v.clone(), &v_i.clone())?)?)?, &py::mul(&py::truediv(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_v.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?); } Py::list(out) };
        let t610 = v_d.clone(); let t611 = Py::Int(0i64);
        py::setitem(&t610, &t611, py::iadd(&py::getitem(&t610, &t611)?, &py::mul(&py::truediv(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_v.clone(), &Py::Int(0i64))?)?)?)?;
        let t612 = v_d.clone(); let t613 = py::neg(&Py::Int(1i64))?;
        py::setitem(&t612, &t613, py::iadd(&py::getitem(&t612, &t613)?, &py::mul(&py::truediv(&v_r.clone(), &Py::Int(2i64))?, &py::getitem(&v_v.clone(), &py::neg(&Py::Int(1i64))?)?)?)?)?;
        let t614 = (Py::tuple(vec![py::truediv(&py::neg(&v_r.clone())?, &Py::Int(2i64))?, py::add(&Py::Int(1i64), &v_r.clone())?, py::truediv(&py::neg(&v_r.clone())?, &Py::Int(2i64))?])).items()?;
        if t614.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t614[0].clone();
        v_b = t614[1].clone();
        v_c = t614[2].clone();
        let t615 = (Py::tuple(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?, py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?])).items()?;
        if t615.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_cp = t615[0].clone();
        v_dp = t615[1].clone();
        let t616 = (Py::tuple(vec![py::truediv(&v_c.clone(), &v_b.clone())?, py::truediv(&py::getitem(&v_d.clone(), &Py::Int(0i64))?, &v_b.clone())?])).items()?;
        if t616.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_cp.clone(), &Py::Int(0i64), t616[0].clone())?;
        py::setitem(&v_dp.clone(), &Py::Int(0i64), t616[1].clone())?;
        for t617 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_m.clone()).i()?]) {
            v_i = t617;
            v_den = py::sub(&v_b.clone(), &py::mul(&v_a.clone(), &py::getitem(&v_cp.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?)?;
            let t618 = (Py::tuple(vec![py::truediv(&v_c.clone(), &v_den.clone())?, py::truediv(&py::sub(&py::getitem(&v_d.clone(), &v_i.clone())?, &py::mul(&v_a.clone(), &py::getitem(&v_dp.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &v_den.clone())?])).items()?;
            if t618.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_cp.clone(), &v_i.clone(), t618[0].clone())?;
            py::setitem(&v_dp.clone(), &v_i.clone(), t618[1].clone())?;
        }
        v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_m.clone())?;
        py::setitem(&v_x.clone(), &py::neg(&Py::Int(1i64))?, py::getitem(&v_dp.clone(), &py::neg(&Py::Int(1i64))?)?)?;
        for t619 in py::range_iter(&[(py::sub(&v_m.clone(), &Py::Int(2i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
            v_i = t619;
            py::setitem(&v_x.clone(), &v_i.clone(), py::sub(&py::getitem(&v_dp.clone(), &v_i.clone())?, &py::mul(&py::getitem(&v_cp.clone(), &v_i.clone())?, &py::getitem(&v_x.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?)?;
        }
        v_v = py::add(&py::add(&Py::list(vec![py::getitem(&v_v.clone(), &Py::Int(0i64))?]), &v_x.clone())?, &Py::list(vec![py::getitem(&v_v.clone(), &py::neg(&Py::Int(1i64))?)?]))?;
    }
    return Ok(Py::dict(vec![(Py::str("u"), { let mut out = vec![]; for t620 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t620;
out.push(f__r(v_x.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dalembert(mut v_f: Py, mut v_g: Py, mut v_c: Py, mut v_x: Py, mut v_t: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_err: Py = Py::None;
    let mut v_integ: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    let mut v_p3: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_c.clone(), v_x.clone(), v_t.clone()]))?;
    if ({ let t653 = v_c.clone(); Py::Bool({ let t654 = Py::Int(0i64); if py::le(&t653, &t654)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the wave speed c must be positive"))));
    }
    let t655 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone(), Py::tuple(vec![Py::str("x")])], &[])?, ax_engine::pysym::helper("differential_equations:_fn", &[v_g.clone(), Py::tuple(vec![Py::str("x")])], &[])?])).items()?;
    if t655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t655[0].clone();
    v_G = t655[1].clone();
    let t656 = (Py::tuple(vec![py::sub(&v_x.clone(), &py::mul(&v_c.clone(), &v_t.clone())?)?, py::add(&v_x.clone(), &py::mul(&v_c.clone(), &v_t.clone())?)?])).items()?;
    if t656.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a = t656[0].clone();
    v_b = t656[1].clone();
    if ({ let t657 = v_a.clone(); Py::Bool({ let t658 = v_b.clone(); if py::eq(&t657, &t658) { true } else { false } }) }).truthy() {
        let t659 = (Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)])).items()?;
        if t659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_integ = t659[0].clone();
        v_err = t659[1].clone();
    } else {
        {
            let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(20i64))?;
            let t660 = (ax_engine::pymp::call("quad", &[{ let v_F = v_F.clone(); let v_G = v_G.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_err = v_err.clone(); let v_f = v_f.clone(); let v_g = v_g.clone(); let v_integ = v_integ.clone(); let v_t = v_t.clone(); let v_x = v_x.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::call(&v_G.clone(), &[py::float(&v_s.clone())?])?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::list(vec![v_a.clone(), py::truediv(&py::add(&v_a.clone(), &v_b.clone())?, &Py::Int(2i64))?, v_b.clone()])], &[(String::from("error"), Py::Bool(true))])?).items()?;
            if t660.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_v = t660[0].clone();
            v_err = t660[1].clone();
        }
        v_integ = py::float(&v_v.clone())?;
        if ({ let t661 = v_err.clone(); Py::Bool({ let t662 = py::mul(&Py::Float(1e-13), &py::minmax(&[Py::Float(1.0), py::abs(&v_integ.clone())?], true)?)?; if py::gt(&t661, &t662)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the velocity integral did not converge"))));
        }
    }
    let t663 = (Py::tuple(vec![py::truediv(&py::call(&v_F.clone(), &[v_a.clone()])?, &Py::Int(2i64))?, py::truediv(&py::call(&v_F.clone(), &[v_b.clone()])?, &Py::Int(2i64))?])).items()?;
    if t663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t663[0].clone();
    v_p2 = t663[1].clone();
    v_p3 = py::truediv(&v_integ.clone(), &py::mul(&Py::Int(2i64), &v_c.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("u"), f__r(f__guard(py::add(&py::add(&v_p1.clone(), &v_p2.clone())?, &v_p3.clone())?, Py::tuple(vec![v_p1.clone(), v_p2.clone(), v_p3.clone()]))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_decay_age(mut v_T: Py, mut v_p: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_T.clone(), v_p.clone()]))?;
    if ({ let t3080 = { let t3075 = v_T.clone(); Py::Bool({ let t3076 = Py::Int(0i64); if py::le(&t3075, &t3076)? { true } else { false } }) }; if t3080.truthy() { t3080 } else { py::not(&{ let t3077 = Py::Int(0i64); Py::Bool({ let t3078 = v_p.clone(); if py::lt(&t3077, &t3078)? { { let t3079 = Py::Int(1i64); if py::le(&t3078, &t3079)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a positive half-life T and a remaining fraction p in (0, 1]"))));
    }
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::truediv(&py::mul(&v_T.clone(), &py::math1("log", &py::truediv(&Py::Int(1i64), &v_p.clone())?)?)?, &py::math1("log", &Py::Int(2i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_drag_linear(mut v_m: Py, mut v_c: Py, mut v_g: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_vt: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_c.clone(), v_g.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t1639 = { let t1635 = v_m.clone(); Py::Bool({ let t1636 = Py::Int(0i64); if py::le(&t1635, &t1636)? { true } else { false } }) }; if t1639.truthy() { t1639 } else { { let t1637 = v_c.clone(); Py::Bool({ let t1638 = Py::Int(0i64); if py::le(&t1637, &t1638)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m > 0 and c > 0"))));
    }
    v_vt = py::truediv(&py::mul(&v_m.clone(), &v_g.clone())?, &v_c.clone())?;
    return Ok(Py::dict(vec![(Py::str("v"), f__r(py::add(&v_vt.clone(), &py::mul(&py::sub(&v_v0.clone(), &v_vt.clone())?, &f__exp(py::truediv(&py::mul(&py::neg(&v_c.clone())?, &v_t.clone())?, &v_m.clone())?)?)?)?)?), (Py::str("v_terminal"), f__r(v_vt.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_drag_quadratic(mut v_m: Py, mut v_c: Py, mut v_g: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_s: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_vt: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_c.clone(), v_g.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t3000 = { let t2987 = v_m.clone(); Py::Bool({ let t2988 = Py::Int(0i64); if py::le(&t2987, &t2988)? { true } else { false } }) }; if t3000.truthy() { t3000 } else { { let t2999 = { let t2989 = v_c.clone(); Py::Bool({ let t2990 = Py::Int(0i64); if py::le(&t2989, &t2990)? { true } else { false } }) }; if t2999.truthy() { t2999 } else { { let t2998 = { let t2991 = v_g.clone(); Py::Bool({ let t2992 = Py::Int(0i64); if py::le(&t2991, &t2992)? { true } else { false } }) }; if t2998.truthy() { t2998 } else { { let t2997 = { let t2993 = v_v0.clone(); Py::Bool({ let t2994 = Py::Int(0i64); if py::lt(&t2993, &t2994)? { true } else { false } }) }; if t2997.truthy() { t2997 } else { { let t2995 = v_t.clone(); Py::Bool({ let t2996 = Py::Int(0i64); if py::lt(&t2995, &t2996)? { true } else { false } }) } } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m, c, g > 0, v0 >= 0 and t >= 0"))));
    }
    v_vt = py::math1("sqrt", &py::truediv(&py::mul(&v_m.clone(), &v_g.clone())?, &v_c.clone())?)?;
    v_s = py::truediv(&py::mul(&v_g.clone(), &v_t.clone())?, &v_vt.clone())?;
    if ({ let t3001 = v_v0.clone(); Py::Bool({ let t3002 = v_vt.clone(); if py::eq(&t3001, &t3002) { true } else { false } }) }).truthy() {
        v_v = v_vt.clone();
    } else {
        if ({ let t3003 = v_v0.clone(); Py::Bool({ let t3004 = v_vt.clone(); if py::lt(&t3003, &t3004)? { true } else { false } }) }).truthy() {
            v_v = py::mul(&v_vt.clone(), &py::math1("tanh", &py::add(&v_s.clone(), &py::math1("atanh", &py::truediv(&v_v0.clone(), &v_vt.clone())?)?)?)?)?;
        } else {
            v_v = py::truediv(&v_vt.clone(), &py::math1("tanh", &py::add(&v_s.clone(), &py::math1("atanh", &py::truediv(&v_vt.clone(), &v_v0.clone())?)?)?)?)?;
        }
    }
    return Ok(Py::dict(vec![(Py::str("v"), f__r(v_v.clone())?), (Py::str("v_terminal"), f__r(v_vt.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_equilibria_1d(mut v_f: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_left: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_right: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let mut v_sign: Py = Py::None;
    let mut v_stab: Py = Py::None;
    let mut v_tests: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_e = ax_engine::pysym::helper("differential_equations:parse_text", &[v_f.clone(), Py::tuple(vec![Py::str("y")])], &[])?;
    let t47: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_P = ax_engine::pysym::call("Poly", &[ax_engine::pysym::call("expand", &[v_e.clone()], &[])?, c_YS()?], &[])?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t47 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "PolynomialError")) => {
            return Err(py::value_error(py::to_str(&Py::str("f(y) must be a polynomial in y for this capability"))));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t48 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t48;
out.push(py::getattr(&v_c.clone(), "is_rational")?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial needs rational coefficients"))));
    }
    if ({ let t53 = { let t49 = py::method(&v_P.clone(), "degree", &[])?; Py::Bool({ let t50 = Py::Int(1i64); if py::lt(&t49, &t50)? { true } else { false } }) }; if t53.truthy() { t53 } else { { let t51 = py::method(&v_P.clone(), "degree", &[])?; Py::Bool({ let t52 = Py::Int(8i64); if py::gt(&t51, &t52)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("polynomials of degree 1 to 8 are supported"))));
    }
    v_roots = py::sorted_key(&py::set(&ax_engine::pysym::call("real_roots", &[v_P.clone()], &[])?)?, &{ let v_P = v_P.clone(); let v_e = v_e.clone(); let v_f = v_f.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::float(&ax_engine::pysym::call("N", &[v_r.clone(), Py::Int(30i64)], &[])?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    v_vals = { let mut out = vec![]; for t54 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t54;
out.push(py::float(&ax_engine::pysym::call("N", &[v_r.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) };
    v_tests = (if (v_vals.clone()).truthy() { Py::list(vec![ax_engine::pysym::call("Rational", &[py::sub(&py::math1("floor", &py::getitem(&v_vals.clone(), &Py::Int(0i64))?)?, &Py::Int(1i64))?], &[])?]) } else { Py::list(vec![]) });
    for t55 in py::range_iter(&[(py::sub(&py::len(&v_vals.clone())?, &Py::Int(1i64))?).i()?]) {
        v_i = t55;
        v_m = ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::truediv(&py::add(&py::getitem(&v_vals.clone(), &v_i.clone())?, &py::getitem(&v_vals.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?))], &[])?;
        if (py::not(&{ let t56 = py::getitem(&v_roots.clone(), &v_i.clone())?; Py::Bool({ let t57 = v_m.clone(); if py::lt(&t56, &t57)? { { let t58 = py::getitem(&v_roots.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?; if py::lt(&t57, &t58)? { true } else { false } } } else { false } }) })).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("two equilibria are too close to separate"))));
        }
        let _ = py::method(&v_tests.clone(), "append", &[v_m.clone()])?;
    }
    if (v_vals.clone()).truthy() {
        let _ = py::method(&v_tests.clone(), "append", &[ax_engine::pysym::call("Rational", &[py::add(&py::math1("ceil", &py::getitem(&v_vals.clone(), &py::neg(&Py::Int(1i64))?)?)?, &Py::Int(1i64))?], &[])?])?;
    }
    v_sign = { let mut out = vec![]; for t59 in py::iter(&(v_tests.clone()))? { let mut v_x: Py; 
v_x = t59;
out.push(py::int(&ax_engine::pysym::call("sign", &[py::method(&v_P.clone(), "eval", &[v_x.clone()])?], &[])?)?); } Py::list(out) };
    v_stab = Py::list(vec![]);
    for t60 in py::range_iter(&[(py::len(&v_vals.clone())?).i()?]) {
        v_i = t60;
        let t61 = (Py::tuple(vec![py::getitem(&v_sign.clone(), &v_i.clone())?, py::getitem(&v_sign.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?])).items()?;
        if t61.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_left = t61[0].clone();
        v_right = t61[1].clone();
        let _ = py::method(&v_stab.clone(), "append", &[(if ({ let t62 = v_left.clone(); Py::Bool({ let t63 = Py::Int(0i64); if py::gt(&t62, &t63)? { { let t64 = v_right.clone(); if py::gt(&t63, &t64)? { true } else { false } } } else { false } }) }).truthy() { Py::str("stable") } else { (if ({ let t65 = v_left.clone(); Py::Bool({ let t66 = Py::Int(0i64); if py::lt(&t65, &t66)? { { let t67 = v_right.clone(); if py::lt(&t66, &t67)? { true } else { false } } } else { false } }) }).truthy() { Py::str("unstable") } else { Py::str("semi-stable") }) })])?;
    }
    return Ok(Py::dict(vec![(Py::str("equilibria"), { let mut out = vec![]; for t68 in py::iter(&(v_vals.clone()))? { let mut v_v: Py; 
v_v = t68;
out.push(f__r(v_v.clone())?); } Py::list(out) }), (Py::str("stability"), v_stab.clone()), (Py::str("count"), py::len(&v_vals.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_euler_error(mut v_k: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    let mut v_ye: Py = Py::None;
    let mut v_yx: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_k.clone(), v_y0.clone(), v_h.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t801 = { let t796 = v_h.clone(); Py::Bool({ let t797 = Py::Int(0i64); if py::le(&t796, &t797)? { true } else { false } }) }; if t801.truthy() { t801 } else { py::not(&{ let t798 = Py::Int(1i64); Py::Bool({ let t799 = v_n.clone(); if py::le(&t798, &t799)? { { let t800 = Py::Int(10000i64); if py::le(&t799, &t800)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs h > 0 and 1 <= n <= 10000"))));
    }
    let t803 = (Py::tuple(vec![py::mul(&v_y0.clone(), &py::pow(&py::add(&Py::Int(1i64), &py::mul(&v_h.clone(), &v_k.clone())?)?, &v_n.clone())?)?, py::mul(&v_y0.clone(), &f__exp(py::mul(&py::mul(&v_k.clone(), &v_n.clone())?, &v_h.clone())?)?)?])).items()?;
    if t803.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ye = t803[0].clone();
    v_yx = t803[1].clone();
    return Ok(Py::dict(vec![(Py::str("y_euler"), f__r(v_ye.clone())?), (Py::str("y_exact"), f__r(v_yx.clone())?), (Py::str("error"), f__r(f__guard(py::sub(&v_yx.clone(), &v_ye.clone())?, Py::tuple(vec![v_yx.clone(), v_ye.clone()]))?)?), (Py::str("relative_error"), (if ({ let t811 = v_yx.clone(); Py::Bool({ let t812 = Py::Int(0i64); if py::ne(&t811, &t812) { true } else { false } }) }).truthy() { f__r(py::truediv(&f__guard(py::sub(&v_yx.clone(), &v_ye.clone())?, Py::tuple(vec![v_yx.clone(), v_ye.clone()]))?, &v_yx.clone())?)? } else { Py::Float(0.0) }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_exp_growth(mut v_y0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_y0.clone(), v_k.clone(), v_t.clone()]))?;
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::mul(&v_y0.clone(), &f__exp(py::mul(&v_k.clone(), &v_t.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forced_cos(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_F0: Py, mut v_w: Py, mut v_y0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_y: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_F0.clone(), v_w.clone(), v_y0.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t1100 = { let t1096 = v_a.clone(); Py::Bool({ let t1097 = Py::Int(0i64); if py::eq(&t1096, &t1097) { true } else { false } }) }; if t1100.truthy() { t1100 } else { { let t1098 = v_w.clone(); Py::Bool({ let t1099 = Py::Int(0i64); if py::le(&t1098, &t1099)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a != 0 and w > 0"))));
    }
    v_Z = f__cos_particular(v_a.clone(), v_b.clone(), v_c.clone(), v_F0.clone(), v_w.clone())?;
    if ({ let t1108 = v_Z.clone(); Py::Bool({ let t1109 = Py::None; if py::is(&t1108, &t1109) { true } else { false } }) }).truthy() {
        v_K = py::truediv(&v_F0.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_a.clone())?, &v_w.clone())?)?;
        v_y = f__forced(v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone(), py::mul(&py::mul(&v_K.clone(), &v_t.clone())?, &py::math1("sin", &py::mul(&v_w.clone(), &v_t.clone())?)?)?, Py::Float(0.0), Py::Float(0.0))?;
        return Ok(Py::dict(vec![(Py::str("y"), f__r(v_y.clone())?), (Py::str("resonant"), Py::Bool(true))]));
    }
    v_yp = py::sub(&py::mul(&py::real(&v_Z.clone())?, &py::math1("cos", &py::mul(&v_w.clone(), &v_t.clone())?)?)?, &py::mul(&py::imag(&v_Z.clone())?, &py::math1("sin", &py::mul(&v_w.clone(), &v_t.clone())?)?)?)?;
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__forced(v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone(), v_yp.clone(), py::real(&v_Z.clone())?, py::mul(&py::neg(&py::imag(&v_Z.clone())?)?, &v_w.clone())?)?)?), (Py::str("resonant"), Py::Bool(false))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ftcs(mut v_u: Py, mut v_r: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::int(&v_n.clone())?;
    if ({ let t1346 = py::len(&v_u.clone())?; Py::Bool({ let t1347 = Py::Int(3i64); if py::lt(&t1346, &t1347)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give at least 3 grid values"))));
    }
    if ({ let t1353 = { let t1348 = v_r.clone(); Py::Bool({ let t1349 = Py::Int(0i64); if py::le(&t1348, &t1349)? { true } else { false } }) }; if t1353.truthy() { t1353 } else { py::not(&{ let t1350 = Py::Int(1i64); Py::Bool({ let t1351 = v_n.clone(); if py::le(&t1350, &t1351)? { { let t1352 = Py::Int(1000i64); if py::le(&t1351, &t1352)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs r > 0 and 1 <= n <= 1000"))));
    }
    v_v = Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_u.clone())?).items()?);
    for t1354 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t1354;
        v_v = py::add(&py::add(&Py::list(vec![py::getitem(&v_v.clone(), &Py::Int(0i64))?]), &{ let mut out = vec![]; for t1355 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::sub(&py::len(&v_v.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t1355;
out.push(py::add(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::mul(&v_r.clone(), &py::add(&py::sub(&py::getitem(&v_v.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?, &py::mul(&Py::Int(2i64), &py::getitem(&v_v.clone(), &v_i.clone())?)?)?, &py::getitem(&v_v.clone(), &py::sub(&v_i.clone(), &Py::Int(1i64))?)?)?)?)?); } Py::list(out) })?, &Py::list(vec![py::getitem(&v_v.clone(), &py::neg(&Py::Int(1i64))?)?]))?;
        if ({ let t1356 = py::minmax(&[{ let mut out = vec![]; for t1358 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t1358;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?; Py::Bool({ let t1357 = Py::Float(1e+100); if py::gt(&t1356, &t1357)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the scheme blew up (unstable for r > 1/2)"))));
        }
    }
    return Ok(Py::dict(vec![(Py::str("u"), { let mut out = vec![]; for t1359 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t1359;
out.push(f__r(v_x.clone())?); } Py::list(out) }), (Py::str("stable_ratio"), { let t1362 = v_r.clone(); Py::Bool({ let t1363 = Py::Float(0.5); if py::le(&t1362, &t1363)? { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_gompertz(mut v_y0: Py, mut v_r: Py, mut v_K: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_y0.clone(), v_r.clone(), v_K.clone(), v_t.clone()]))?;
    if ({ let t1217 = { let t1213 = v_y0.clone(); Py::Bool({ let t1214 = Py::Int(0i64); if py::le(&t1213, &t1214)? { true } else { false } }) }; if t1217.truthy() { t1217 } else { { let t1215 = v_K.clone(); Py::Bool({ let t1216 = Py::Int(0i64); if py::le(&t1215, &t1216)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs y0 > 0 and K > 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::mul(&v_K.clone(), &f__exp(py::mul(&py::math1("log", &py::truediv(&v_y0.clone(), &v_K.clone())?)?, &f__exp(py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?)?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_half_life_decay(mut v_N0: Py, mut v_T: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_N0.clone(), v_T.clone(), v_t.clone()]))?;
    if ({ let t1251 = v_T.clone(); Py::Bool({ let t1252 = Py::Int(0i64); if py::le(&t1251, &t1252)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the half-life T must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("N"), f__r(py::mul(&v_N0.clone(), &py::pow(&Py::Float(2.0), &py::truediv(&py::neg(&v_t.clone())?, &v_T.clone())?)?)?)?), (Py::str("k"), f__r(py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_harvesting(mut v_r: Py, mut v_K: Py, mut v_H: Py) -> R {
    let mut v_Hc: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_r.clone(), v_K.clone(), v_H.clone()]))?;
    if ({ let t1988 = { let t1981 = v_r.clone(); Py::Bool({ let t1982 = Py::Int(0i64); if py::le(&t1981, &t1982)? { true } else { false } }) }; if t1988.truthy() { t1988 } else { { let t1987 = { let t1983 = v_K.clone(); Py::Bool({ let t1984 = Py::Int(0i64); if py::le(&t1983, &t1984)? { true } else { false } }) }; if t1987.truthy() { t1987 } else { { let t1985 = v_H.clone(); Py::Bool({ let t1986 = Py::Int(0i64); if py::lt(&t1985, &t1986)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs r > 0, K > 0 and H >= 0"))));
    }
    v_Hc = py::truediv(&py::mul(&v_r.clone(), &v_K.clone())?, &Py::Int(4i64))?;
    if ({ let t1989 = v_H.clone(); Py::Bool({ let t1990 = v_Hc.clone(); if py::gt(&t1989, &t1990)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["no equilibrium: the harvest exceeds the maximum sustainable yield rK/4 = ".to_string(), py::format(&v_Hc.clone(), "g")?, " (collapse)".to_string()].concat()))));
    }
    if ({ let t1991 = v_H.clone(); Py::Bool({ let t1992 = v_Hc.clone(); if py::eq(&t1991, &t1992) { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("y_high"), f__r(py::truediv(&v_K.clone(), &Py::Int(2i64))?)?), (Py::str("y_low"), f__r(py::truediv(&v_K.clone(), &Py::Int(2i64))?)?), (Py::str("H_critical"), f__r(v_Hc.clone())?), (Py::str("stability_high"), Py::str("semi-stable")), (Py::str("stability_low"), Py::str("semi-stable"))]));
    }
    v_hi = py::mul(&py::truediv(&v_K.clone(), &Py::Int(2i64))?, &py::add(&Py::Int(1i64), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::truediv(&py::mul(&Py::Int(4i64), &v_H.clone())?, &py::mul(&v_r.clone(), &v_K.clone())?)?)?)?)?)?;
    v_lo = py::truediv(&py::mul(&v_H.clone(), &v_K.clone())?, &py::mul(&v_r.clone(), &v_hi.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("y_high"), f__r(v_hi.clone())?), (Py::str("y_low"), f__r(v_lo.clone())?), (Py::str("H_critical"), f__r(v_Hc.clone())?), (Py::str("stability_high"), Py::str("stable")), (Py::str("stability_low"), Py::str("unstable"))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heat_series(mut v_f: Py, mut v_L: Py, mut v_alpha: Py, mut v_N: Py, mut v_x: Py, mut v_t: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_L.clone(), v_alpha.clone(), v_x.clone(), v_t.clone()]))?;
    v_N = py::int(&v_N.clone())?;
    if ({ let t1301 = { let t1286 = v_L.clone(); Py::Bool({ let t1287 = Py::Int(0i64); if py::le(&t1286, &t1287)? { true } else { false } }) }; if t1301.truthy() { t1301 } else { { let t1300 = { let t1288 = v_alpha.clone(); Py::Bool({ let t1289 = Py::Int(0i64); if py::le(&t1288, &t1289)? { true } else { false } }) }; if t1300.truthy() { t1300 } else { { let t1299 = { let t1290 = v_t.clone(); Py::Bool({ let t1291 = Py::Int(0i64); if py::lt(&t1290, &t1291)? { true } else { false } }) }; if t1299.truthy() { t1299 } else { { let t1298 = py::not(&{ let t1292 = Py::Int(0i64); Py::Bool({ let t1293 = v_x.clone(); if py::le(&t1292, &t1293)? { { let t1294 = v_L.clone(); if py::le(&t1293, &t1294)? { true } else { false } } } else { false } }) }); if t1298.truthy() { t1298 } else { py::not(&{ let t1295 = Py::Int(1i64); Py::Bool({ let t1296 = v_N.clone(); if py::le(&t1295, &t1296)? { { let t1297 = Py::Int(500i64); if py::le(&t1296, &t1297)? { true } else { false } } } else { false } }) }) } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0, alpha > 0, t >= 0, 0 <= x <= L and 1 <= N <= 500"))));
    }
    v_co = f__poly_x(v_f.clone(), Py::Int(6i64))?;
    v_terms = { let mut out = vec![]; for t1307 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) { let mut v_n: Py; 
v_n = t1307;
out.push(py::mul(&py::mul(&f__sine_coeff(v_co.clone(), v_L.clone(), v_n.clone())?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&v_alpha.clone())?, &py::pow(&py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())?, &Py::Int(2i64))?)?, &v_t.clone())?)?)?, &py::math1("sin", &py::truediv(&py::mul(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_x.clone())?, &v_L.clone())?)?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("u"), f__r(f__guard(py::fsum(&v_terms.clone())?, { let mut v = vec![]; v.extend((v_terms.clone()).items()?); Py::tuple(v) })?)?), (Py::str("b_1"), f__r(f__sine_coeff(v_co.clone(), v_L.clone(), Py::Int(1i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_heat_stability(mut v_alpha: Py, mut v_dt: Py, mut v_dx: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_alpha.clone(), v_dt.clone(), v_dx.clone()]))?;
    if ({ let t900 = { let t893 = v_alpha.clone(); Py::Bool({ let t894 = Py::Int(0i64); if py::le(&t893, &t894)? { true } else { false } }) }; if t900.truthy() { t900 } else { { let t899 = { let t895 = v_dt.clone(); Py::Bool({ let t896 = Py::Int(0i64); if py::le(&t895, &t896)? { true } else { false } }) }; if t899.truthy() { t899 } else { { let t897 = v_dx.clone(); Py::Bool({ let t898 = Py::Int(0i64); if py::le(&t897, &t898)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs alpha, dt, dx > 0"))));
    }
    v_r = py::truediv(&py::mul(&v_alpha.clone(), &v_dt.clone())?, &py::pow(&v_dx.clone(), &Py::Int(2i64))?)?;
    return Ok(Py::dict(vec![(Py::str("r"), f__r(v_r.clone())?), (Py::str("stable"), { let t903 = v_r.clone(); Py::Bool({ let t904 = Py::Float(0.5); if py::le(&t903, &t904)? { true } else { false } }) }), (Py::str("dt_max"), f__r(py::truediv(&py::mul(&v_dx.clone(), &v_dx.clone())?, &py::mul(&Py::Int(2i64), &v_alpha.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hom2(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_y0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p1: Py = Py::None;
    let mut v_p2: Py = Py::None;
    v_m = py::truediv(&py::neg(&v_b.clone())?, &py::mul(&Py::Int(2i64), &v_a.clone())?)?;
    v_d2 = py::sub(&py::mul(&v_m.clone(), &v_m.clone())?, &py::truediv(&v_c.clone(), &v_a.clone())?)?;
    v_E = f__exp(py::mul(&v_m.clone(), &v_t.clone())?)?;
    let t3824 = (f__CS(v_d2.clone(), v_t.clone())?).items()?;
    if t3824.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3824[0].clone();
    v_S = t3824[1].clone();
    let t3825 = (Py::tuple(vec![py::mul(&v_y0.clone(), &v_C.clone())?, py::mul(&py::sub(&v_v0.clone(), &py::mul(&v_m.clone(), &v_y0.clone())?)?, &v_S.clone())?])).items()?;
    if t3825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t3825[0].clone();
    v_p2 = t3825[1].clone();
    return Ok(py::mul(&v_E.clone(), &f__guard(py::add(&v_p1.clone(), &v_p2.clone())?, Py::tuple(vec![v_p1.clone(), v_p2.clone()]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_impulse2(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_t.clone()]))?;
    if ({ let t2670 = { let t2666 = v_a.clone(); Py::Bool({ let t2667 = Py::Int(0i64); if py::eq(&t2666, &t2667) { true } else { false } }) }; if t2670.truthy() { t2670 } else { { let t2668 = v_t.clone(); Py::Bool({ let t2669 = Py::Int(0i64); if py::lt(&t2668, &t2669)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a != 0 and t >= 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("h"), f__r(f_hom2(v_a.clone(), v_b.clone(), v_c.clone(), Py::Float(0.0), py::truediv(&Py::Float(1.0), &v_a.clone())?, v_t.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f_inverse_laplace(v_F: Py, v_t: Py) -> R {
    ax_engine::pysym::helper("differential_equations:inverse_laplace", &[v_F, v_t], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ivp2(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_y0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t3330 = v_a.clone(); Py::Bool({ let t3331 = Py::Int(0i64); if py::eq(&t3330, &t3331) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0"))));
    }
    let t3338 = (f__roots2(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t3338.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t3338[0].clone();
    v_r2 = t3338[1].clone();
    v_D = t3338[2].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f_hom2(v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone())?)?), (Py::str("damping"), f__damping(v_a.clone(), v_b.clone(), v_c.clone(), v_D.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jacobian_classify(mut v_f: Py, mut v_g: Py, mut v_x0: Py, mut v_y0: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_Jf: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_detF: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_pt: Py = Py::None;
    let mut v_stab: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_trF: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_x0.clone(), v_y0.clone()]))?;
    let t2589 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:parse_text", &[v_f.clone(), Py::tuple(vec![Py::str("x"), Py::str("y")])], &[])?, ax_engine::pysym::helper("differential_equations:parse_text", &[v_g.clone(), Py::tuple(vec![Py::str("x"), Py::str("y")])], &[])?])).items()?;
    if t2589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2589[0].clone();
    v_G = t2589[1].clone();
    v_pt = Py::dict(vec![(c_XS()?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_x0.clone())?))], &[])?), (c_YS()?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_y0.clone())?))], &[])?)]);
    for t2590 in py::iter(&(Py::tuple(vec![v_F.clone(), v_G.clone()])))? {
        v_h = t2590;
        v_v = ax_engine::pysym::call("N", &[py::method(&v_h.clone(), "subs", &[v_pt.clone()])?, Py::Int(40i64)], &[])?;
        if ({ let t2591 = py::abs(&py::complex(&[v_v.clone()])?)?; Py::Bool({ let t2592 = Py::Float(1e-12); if py::gt(&t2591, &t2592)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the point is not an equilibrium (f and g do not both vanish there)"))));
        }
    }
    v_J = { let mut out = vec![]; for t2593 in py::iter(&(Py::tuple(vec![v_F.clone(), v_G.clone()])))? { let mut v_h: Py; 
v_h = t2593;
out.push({ let mut out = vec![]; for t2594 in py::iter(&(Py::tuple(vec![c_XS()?, c_YS()?])))? { let mut v_v: Py; 
v_v = t2594;
out.push(py::method(&ax_engine::pysym::call("diff", &[v_h.clone(), v_v.clone()], &[])?, "subs", &[v_pt.clone()])?); } Py::list(out) }); } Py::list(out) };
    let t2595 = (Py::tuple(vec![ax_engine::pysym::call("nsimplify", &[ax_engine::pysym::call("N", &[py::add(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, Py::Int(50i64)], &[])?], &[(String::from("rational"), Py::Bool(true)), (String::from("tolerance"), Py::Float(1e-40))])?, ax_engine::pysym::call("nsimplify", &[ax_engine::pysym::call("N", &[py::sub(&py::mul(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, Py::Int(50i64)], &[])?], &[(String::from("rational"), Py::Bool(true)), (String::from("tolerance"), Py::Float(1e-40))])?])).items()?;
    if t2595.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t2595[0].clone();
    v_det = t2595[1].clone();
    let t2596 = (Py::tuple(vec![py::fraction(&[py::int(&py::getattr(&v_tr.clone(), "p")?)?, py::int(&py::getattr(&v_tr.clone(), "q")?)?])?, py::fraction(&[py::int(&py::getattr(&v_det.clone(), "p")?)?, py::int(&py::getattr(&v_det.clone(), "q")?)?])?])).items()?;
    if t2596.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_trF = t2596[0].clone();
    v_detF = t2596[1].clone();
    v_Jf = { let mut out = vec![]; for t2597 in py::iter(&(v_J.clone()))? { let mut v_row: Py; 
v_row = t2597;
out.push({ let mut out = vec![]; for t2598 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t2598;
out.push(py::float(&ax_engine::pysym::call("N", &[v_v.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) }); } Py::list(out) };
    let t2628 = (f__classify(v_trF.clone(), v_detF.clone(), { let t2623 = { let t2616 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?; Py::Bool({ let t2617 = Py::Int(0i64); if py::eq(&t2616, &t2617) { true } else { false } }) }; if !t2623.truthy() { t2623 } else { { let t2622 = { let t2618 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?; Py::Bool({ let t2619 = Py::Int(0i64); if py::eq(&t2618, &t2619) { true } else { false } }) }; if !t2622.truthy() { t2622 } else { { let t2620 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?; Py::Bool({ let t2621 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?; if py::eq(&t2620, &t2621) { true } else { false } }) } } } } }, Py::Bool(({ let mut out = vec![]; for t2624 in py::iter(&(v_Jf.clone()))? { let mut v_row: Py; 
v_row = t2624;
for t2625 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t2625;
out.push({ let t2626 = v_v.clone(); Py::Bool({ let t2627 = Py::Int(0i64); if py::eq(&t2626, &t2627) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())))?).items()?;
    if t2628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kind = t2628[0].clone();
    v_stab = t2628[1].clone();
    if ({ let t2631 = { let t2629 = v_kind.clone(); Py::Bool({ let t2630 = Py::str("center"); if py::eq(&t2629, &t2630) { true } else { false } }) }; if t2631.truthy() { t2631 } else { py::method(&v_kind.clone(), "startswith", &[Py::str("degenerate")])? } }).truthy() {
        v_stab = Py::str("linearization inconclusive");
    }
    return Ok(Py::dict(vec![(Py::str("type"), v_kind.clone()), (Py::str("stability"), v_stab.clone()), (Py::str("jacobian"), { let mut out = vec![]; for t2632 in py::iter(&(v_Jf.clone()))? { let mut v_row: Py; 
v_row = t2632;
out.push({ let mut out = vec![]; for t2633 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t2633;
out.push(f__r(v_v.clone())?); } Py::list(out) }); } Py::list(out) }), (Py::str("trace"), f__r(py::float(&v_trF.clone())?)?), (Py::str("det"), f__r(py::float(&v_detF.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_rect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_N: Py, mut v_x: Py, mut v_y: Py) -> R {
    let mut v_co: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_x.clone(), v_y.clone()]))?;
    v_N = py::int(&v_N.clone())?;
    if ({ let t1487 = { let t1471 = v_a.clone(); Py::Bool({ let t1472 = Py::Int(0i64); if py::le(&t1471, &t1472)? { true } else { false } }) }; if t1487.truthy() { t1487 } else { { let t1486 = { let t1473 = v_b.clone(); Py::Bool({ let t1474 = Py::Int(0i64); if py::le(&t1473, &t1474)? { true } else { false } }) }; if t1486.truthy() { t1486 } else { { let t1485 = py::not(&{ let t1475 = Py::Int(0i64); Py::Bool({ let t1476 = v_x.clone(); if py::le(&t1475, &t1476)? { { let t1477 = v_a.clone(); if py::le(&t1476, &t1477)? { true } else { false } } } else { false } }) }); if t1485.truthy() { t1485 } else { { let t1484 = py::not(&{ let t1478 = Py::Int(0i64); Py::Bool({ let t1479 = v_y.clone(); if py::le(&t1478, &t1479)? { { let t1480 = v_b.clone(); if py::le(&t1479, &t1480)? { true } else { false } } } else { false } }) }); if t1484.truthy() { t1484 } else { py::not(&{ let t1481 = Py::Int(1i64); Py::Bool({ let t1482 = v_N.clone(); if py::le(&t1481, &t1482)? { { let t1483 = Py::Int(500i64); if py::le(&t1482, &t1483)? { true } else { false } } } else { false } }) }) } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a, b > 0, (x, y) in the rectangle and 1 <= N <= 500"))));
    }
    v_co = f__poly_x(v_f.clone(), Py::Int(6i64))?;
    v_terms = Py::list(vec![]);
    for t1493 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t1493;
        v_k = py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_a.clone())?;
        if ({ let t1494 = py::mul(&v_k.clone(), &v_b.clone())?; Py::Bool({ let t1495 = Py::Int(20i64); if py::gt(&t1494, &t1495)? { true } else { false } }) }).truthy() {
            v_ratio = py::truediv(&py::mul(&py::math1("exp", &py::mul(&v_k.clone(), &py::sub(&v_y.clone(), &v_b.clone())?)?)?, &py::sub(&Py::Int(1i64), &py::math1("exp", &py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_k.clone())?, &v_y.clone())?)?)?)?, &py::sub(&Py::Int(1i64), &py::math1("exp", &py::mul(&py::mul(&py::neg(&Py::Int(2i64))?, &v_k.clone())?, &v_b.clone())?)?)?)?;
        } else {
            v_ratio = py::truediv(&py::math1("sinh", &py::mul(&v_k.clone(), &v_y.clone())?)?, &py::math1("sinh", &py::mul(&v_k.clone(), &v_b.clone())?)?)?;
        }
        let _ = py::method(&v_terms.clone(), "append", &[py::mul(&py::mul(&f__sine_coeff(v_co.clone(), v_a.clone(), v_n.clone())?, &v_ratio.clone())?, &py::math1("sin", &py::mul(&v_k.clone(), &v_x.clone())?)?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("u"), f__r((if (Py::Bool((v_terms.clone()).items()?.iter().any(|x| x.truthy()))).truthy() { f__guard(py::fsum(&v_terms.clone())?, { let mut v = vec![]; v.extend((v_terms.clone()).items()?); Py::tuple(v) })? } else { Py::Float(0.0) }))?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_shifted(mut v_f: Py, mut v_c: Py, mut v_s: Py) -> R {
    let mut v_G: Py = Py::None;
    let mut v_sigma: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_c.clone(), v_s.clone()]))?;
    if ({ let t1537 = v_c.clone(); Py::Bool({ let t1538 = Py::Int(0i64); if py::lt(&t1537, &t1538)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the delay c must be >= 0"))));
    }
    v_terms = ax_engine::pysym::helper("differential_equations:_expo_terms", &[v_f.clone()], &[])?;
    v_sigma = py::minmax(&[{ let mut out = vec![]; for t1539 in py::iter(&(v_terms.clone()))? { let mut v__: Py; let mut v_z: Py; 
let t1540 = (t1539).items()?;
if t1540.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t1540[0].clone();
v__ = t1540[1].clone();
v_z = t1540[2].clone();
out.push(py::real(&v_z.clone())?); } Py::list(out) }], true)?;
    if ({ let t1541 = v_s.clone(); Py::Bool({ let t1542 = v_sigma.clone(); if py::le(&t1541, &t1542)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the transform exists only for s > ".to_string(), py::format(&v_sigma.clone(), "g")?].concat()))));
    }
    v_G = f__laplace_sum(v_terms.clone(), v_s.clone(), Py::Int(1i64))?;
    return Ok(Py::dict(vec![(Py::str("F"), f__r(py::mul(&py::math1("exp", &py::mul(&py::neg(&v_c.clone())?, &v_s.clone())?)?, &v_G.clone())?)?), (Py::str("G"), f__r(v_G.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_laplace_value(mut v_f: Py, mut v_s: Py) -> R {
    let mut v_sigma: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_s.clone()]))?;
    v_terms = ax_engine::pysym::helper("differential_equations:_expo_terms", &[v_f.clone()], &[])?;
    v_sigma = py::minmax(&[{ let mut out = vec![]; for t1587 in py::iter(&(v_terms.clone()))? { let mut v__: Py; let mut v_z: Py; 
let t1588 = (t1587).items()?;
if t1588.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v__ = t1588[0].clone();
v__ = t1588[1].clone();
v_z = t1588[2].clone();
out.push(py::real(&v_z.clone())?); } Py::list(out) }], true)?;
    if ({ let t1589 = v_s.clone(); Py::Bool({ let t1590 = v_sigma.clone(); if py::le(&t1589, &t1590)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the transform exists only for s > ".to_string(), py::format(&v_sigma.clone(), "g")?].concat()))));
    }
    return Ok(Py::dict(vec![(Py::str("F"), f__r(f__laplace_sum(v_terms.clone(), v_s.clone(), Py::Int(1i64))?)?), (Py::str("sigma"), f__r(v_sigma.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lin1(mut v_a: Py, mut v_b: Py, mut v_y0: Py, mut v_t: Py) -> R {
    let mut v_tr: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_y0.clone(), v_t.clone()]))?;
    if ({ let t1673 = v_a.clone(); Py::Bool({ let t1674 = Py::Int(0i64); if py::eq(&t1673, &t1674) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0 (then y = y0 + b t directly)"))));
    }
    v_ys = py::truediv(&v_b.clone(), &v_a.clone())?;
    v_tr = py::mul(&py::sub(&v_y0.clone(), &v_ys.clone())?, &f__exp(py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?;
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__guard(py::add(&v_ys.clone(), &v_tr.clone())?, Py::tuple(vec![v_ys.clone(), v_tr.clone()]))?)?), (Py::str("y_steady"), f__r(v_ys.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lin1_exp(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_y0: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_t.clone()]))?;
    if ({ let t1713 = py::add(&v_a.clone(), &v_c.clone())?; Py::Bool({ let t1714 = Py::Int(0i64); if py::eq(&t1713, &t1714) { true } else { false } }) }).truthy() {
        v_y = py::mul(&py::add(&v_y0.clone(), &py::mul(&v_b.clone(), &v_t.clone())?)?, &f__exp(py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?;
        return Ok(Py::dict(vec![(Py::str("y"), f__r(v_y.clone())?), (Py::str("resonant"), Py::Bool(true))]));
    }
    v_A = py::truediv(&v_b.clone(), &py::add(&v_a.clone(), &v_c.clone())?)?;
    let t1718 = (Py::tuple(vec![py::mul(&v_A.clone(), &f__exp(py::mul(&v_c.clone(), &v_t.clone())?)?)?, py::mul(&py::sub(&v_y0.clone(), &v_A.clone())?, &f__exp(py::mul(&py::neg(&v_a.clone())?, &v_t.clone())?)?)?])).items()?;
    if t1718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t1718[0].clone();
    v_h = t1718[1].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__guard(py::add(&v_p.clone(), &v_h.clone())?, Py::tuple(vec![v_p.clone(), v_h.clone()]))?)?), (Py::str("resonant"), Py::Bool(false))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_first_order_variable(mut v_p_terms: Py, mut v_q_terms: Py, mut v_x0: Py, mut v_y0: Py, mut v_x1: Py) -> R {
    let mut v_I1: Py = Py::None;
    let mut v_I2: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v__s1: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_negative: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_size: Py = Py::None;
    let mut v_total: Py = Py::None;
    v_x0 = f__lfo_finite(v_x0.clone(), Py::str("x0"))?;
    v_y0 = f__lfo_finite(v_y0.clone(), Py::str("y0"))?;
    v_x1 = f__lfo_finite(v_x1.clone(), Py::str("x1"))?;
    v_P = f__lfo_rows(v_p_terms.clone(), Py::Int(2i64), Py::str("P(x)"))?;
    v_Q = f__lfo_rows(v_q_terms.clone(), Py::Int(3i64), Py::str("Q(x)"))?;
    v_negative = Py::Bool(false);
    for t1773 in py::iter(&(py::add(&v_P.clone(), &v_Q.clone())?))? {
        v_r = t1773;
        if ({ let t1774 = py::getitem(&v_r.clone(), &Py::Int(1i64))?; Py::Bool({ let t1775 = Py::Int(0i64); if py::lt(&t1774, &t1775)? { true } else { false } }) }).truthy() {
            v_negative = Py::Bool(true);
        }
    }
    if ({ let t1778 = v_negative.clone(); if !t1778.truthy() { t1778 } else { { let t1776 = py::mul(&v_x0.clone(), &v_x1.clone())?; Py::Bool({ let t1777 = Py::Int(0i64); if py::le(&t1776, &t1777)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a negative power of x is undefined at x = 0: x0 and x must be nonzero and on the same side of 0"))));
    }
    let t1787: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t1785 = (f__lfo_integral(v_P.clone(), v_Q.clone(), v_x0.clone(), v_x1.clone(), Py::Int(128i64))?).items()?;
            if t1785.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_I1 = t1785[0].clone();
            v__s1 = t1785[1].clone();
            let t1786 = (f__lfo_integral(v_P.clone(), v_Q.clone(), v_x0.clone(), v_x1.clone(), Py::Int(256i64))?).items()?;
            if t1786.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_I2 = t1786[0].clone();
            v_size = t1786[1].clone();
            v_ratio = py::math1("exp", &py::sub(&f__lfo_F(v_P.clone(), v_x1.clone())?, &f__lfo_F(v_P.clone(), v_x0.clone())?)?)?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1787 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Overflow)) || (matches!(e, PyErr::ZeroDivision)) => {
            return Err(py::value_error(py::to_str(&Py::str("the integrating factor overflows on this interval"))));
        }
        Err(e) => return Err(e),
    }
    if ({ let t1793 = py::not(&{ let t1790 = py::math1("isfinite", &v_I1.clone())?; if !t1790.truthy() { t1790 } else { { let t1789 = py::math1("isfinite", &v_I2.clone())?; if !t1789.truthy() { t1789 } else { { let t1788 = py::math1("isfinite", &v_size.clone())?; if !t1788.truthy() { t1788 } else { py::math1("isfinite", &v_ratio.clone())? } } } } } }); if t1793.truthy() { t1793 } else { { let t1791 = v_ratio.clone(); Py::Bool({ let t1792 = Py::Int(0i64); if py::eq(&t1791, &t1792) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the integrating factor overflows on this interval"))));
    }
    if ({ let t1794 = py::abs(&py::sub(&v_I1.clone(), &v_I2.clone())?)?; Py::Bool({ let t1795 = py::mul(&Py::Float(1e-11), &v_size.clone())?; if py::gt(&t1794, &t1795)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the integral does not settle: the solution changes too fast on this interval"))));
    }
    v_total = py::add(&v_y0.clone(), &v_I2.clone())?;
    v_big = py::add(&py::abs(&v_y0.clone())?, &py::abs(&v_I2.clone())?)?;
    if ({ let t1796 = v_big.clone(); Py::Bool({ let t1797 = py::mul(&Py::Float(100000.0), &py::minmax(&[Py::Float(1.0), py::abs(&v_total.clone())?], true)?)?; if py::gt(&t1796, &t1797)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result is lost to cancellation between large terms (ill-conditioned inputs)"))));
    }
    if ({ let t1798 = py::abs(&v_total.clone())?; Py::Bool({ let t1799 = py::mul(&Py::Float(1e-13), &v_big.clone())?; if py::lt(&t1798, &t1799)? { true } else { false } }) }).truthy() {
        v_total = Py::Float(0.0);
    }
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::truediv(&v_total.clone(), &v_ratio.clone())?)?), (Py::str("mu_ratio"), f__r(v_ratio.clone())?), (Py::str("integral"), f__r(v_I2.clone())?), (Py::str("integrating_factor"), f__lfo_factor_text(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_linear_stability(mut v_A: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if ({ let t1893 = { let t1885 = v_n.clone(); Py::Bool({ let t1886 = Py::Int(1i64); if py::lt(&t1885, &t1886)? { true } else { false } }) }; if t1893.truthy() { t1893 } else { { let t1892 = { let t1887 = v_n.clone(); Py::Bool({ let t1888 = Py::Int(8i64); if py::gt(&t1887, &t1888)? { true } else { false } }) }; if t1892.truthy() { t1892 } else { Py::Bool(({ let mut out = vec![]; for t1889 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1889;
out.push({ let t1890 = py::len(&v_r.clone())?; Py::Bool({ let t1891 = v_n.clone(); if py::ne(&t1890, &t1891) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be square, of size 1 to 8"))));
    }
    v_c = f__charpoly(v_A.clone())?;
    if ({ let t1909 = v_n.clone(); Py::Bool({ let t1910 = Py::Int(1i64); if py::eq(&t1909, &t1910) { true } else { false } }) }).truthy() {
        v_lam = py::neg(&py::getitem(&v_c.clone(), &Py::Int(1i64))?)?;
        if ({ let t1911 = v_lam.clone(); Py::Bool({ let t1912 = Py::Int(0i64); if py::eq(&t1911, &t1912) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("an eigenvalue lies on the imaginary axis: linear stability is not decided by eigenvalue signs"))));
        }
        v_k = py::int(&{ let t1913 = v_lam.clone(); Py::Bool({ let t1914 = Py::Int(0i64); if py::gt(&t1913, &t1914)? { true } else { false } }) })?;
        return Ok(Py::dict(vec![(Py::str("stable"), { let t1915 = v_k.clone(); Py::Bool({ let t1916 = Py::Int(0i64); if py::eq(&t1915, &t1916) { true } else { false } }) }), (Py::str("unstable_eigenvalues"), v_k.clone()), (Py::str("char_poly"), { let mut out = vec![]; for t1917 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t1917;
out.push(f__r(py::float(&v_v.clone())?)?); } Py::list(out) })]));
    }
    let t1947 = (f__routh(v_c.clone())?).items()?;
    if t1947.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t1947[0].clone();
    v_k = t1947[1].clone();
    return Ok(Py::dict(vec![(Py::str("stable"), { let t1948 = v_k.clone(); Py::Bool({ let t1949 = Py::Int(0i64); if py::eq(&t1948, &t1949) { true } else { false } }) }), (Py::str("unstable_eigenvalues"), v_k.clone()), (Py::str("char_poly"), { let mut out = vec![]; for t1950 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t1950;
out.push(f__r(py::float(&v_v.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(non_snake_case)]
fn f_linear_system_solution(v_A: Py, v_x0: Py, v_t: Py) -> R {
    ax_engine::pysym::helper("differential_equations:linear_system_solution", &[v_A, v_x0, v_t], &[])
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lm_cycle(mut v_r: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_r.clone()]))?;
    if (py::not(&{ let t2174 = Py::Int(3i64); Py::Bool({ let t2175 = v_r.clone(); if py::lt(&t2174, &t2175)? { { let t2176 = Py::Int(4i64); if py::le(&t2175, &t2176)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a 2-cycle exists for 3 < r <= 4"))));
    }
    v_s = py::math1("sqrt", &py::mul(&py::sub(&v_r.clone(), &Py::Int(3i64))?, &py::add(&v_r.clone(), &Py::Int(1i64))?)?)?;
    let t2177 = (Py::tuple(vec![py::truediv(&py::sub(&py::add(&v_r.clone(), &Py::Int(1i64))?, &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_r.clone())?)?, py::truediv(&py::add(&py::add(&v_r.clone(), &Py::Int(1i64))?, &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_r.clone())?)?])).items()?;
    if t2177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2177[0].clone();
    v_q = t2177[1].clone();
    v_m = py::sub(&py::add(&Py::Int(4i64), &py::mul(&Py::Int(2i64), &v_r.clone())?)?, &py::mul(&v_r.clone(), &v_r.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("p"), f__r(v_p.clone())?), (Py::str("q"), f__r(v_q.clone())?), (Py::str("multiplier"), f__r(v_m.clone())?), (Py::str("stability"), f__stab_word(v_m.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lm_fixed(mut v_r: Py) -> R {
    let mut v_mult: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_r.clone()]))?;
    if (py::not(&{ let t2066 = Py::Int(0i64); Py::Bool({ let t2067 = v_r.clone(); if py::lt(&t2066, &t2067)? { { let t2068 = Py::Int(4i64); if py::le(&t2067, &t2068)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the logistic map x -> r x (1 - x) needs 0 < r <= 4"))));
    }
    let t2069 = (Py::tuple(vec![Py::list(vec![Py::Float(0.0)]), Py::list(vec![v_r.clone()])])).items()?;
    if t2069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t2069[0].clone();
    v_mult = t2069[1].clone();
    if ({ let t2070 = v_r.clone(); Py::Bool({ let t2071 = Py::Int(1i64); if py::gt(&t2070, &t2071)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_pts.clone(), "append", &[py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_r.clone())?)?])?;
        let _ = py::method(&v_mult.clone(), "append", &[py::sub(&Py::Int(2i64), &v_r.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("fixed_points"), { let mut out = vec![]; for t2072 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t2072;
out.push(f__r(v_p.clone())?); } Py::list(out) }), (Py::str("multipliers"), { let mut out = vec![]; for t2075 in py::iter(&(v_mult.clone()))? { let mut v_m: Py; 
v_m = t2075;
out.push(f__r(v_m.clone())?); } Py::list(out) }), (Py::str("stability"), { let mut out = vec![]; for t2076 in py::iter(&(v_mult.clone()))? { let mut v_m: Py; 
v_m = t2076;
out.push(f__stab_word(v_m.clone())?); } Py::list(out) }), (Py::str("count"), py::len(&v_pts.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lm_lyapunov(mut v_r: Py, mut v_x0: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_attractor: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_hist: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_tot: Py = Py::None;
    let mut v_x: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_r.clone(), v_x0.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t2117 = py::not(&{ let t2111 = Py::Int(0i64); Py::Bool({ let t2112 = v_r.clone(); if py::lt(&t2111, &t2112)? { { let t2113 = Py::Int(4i64); if py::le(&t2112, &t2113)? { true } else { false } } } else { false } }) }); if t2117.truthy() { t2117 } else { py::not(&{ let t2114 = Py::Int(0i64); Py::Bool({ let t2115 = v_x0.clone(); if py::lt(&t2114, &t2115)? { { let t2116 = Py::Int(1i64); if py::lt(&t2115, &t2116)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs 0 < r <= 4 and 0 < x0 < 1"))));
    }
    if ({ let t2123 = { let t2118 = v_n.clone(); Py::Bool({ let t2119 = Py::Int(2i64); if py::lt(&t2118, &t2119)? { true } else { false } }) }; if t2123.truthy() { t2123 } else { { let t2122 = { let t2120 = v_n.clone(); Py::Bool({ let t2121 = Py::Int(1000000i64); if py::gt(&t2120, &t2121)? { true } else { false } }) }; if t2122.truthy() { t2122 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be an even number of iterations between 2 and 1000000 (whole cycles are averaged)"))));
    }
    let t2124 = (Py::tuple(vec![v_x0.clone(), Py::list(vec![])])).items()?;
    if t2124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2124[0].clone();
    v_hist = t2124[1].clone();
    let mut t2125 = false;
    for t2126 in py::range_iter(&[(Py::Int(1000000i64)).i()?]) {
        v_k = t2126;
        v_x = py::mul(&py::mul(&v_r.clone(), &v_x.clone())?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?;
        let _ = py::method(&v_hist.clone(), "append", &[v_x.clone()])?;
        if ({ let t2131 = { let t2127 = py::len(&v_hist.clone())?; Py::Bool({ let t2128 = Py::Int(2i64); if py::gt(&t2127, &t2128)? { true } else { false } }) }; if !t2131.truthy() { t2131 } else { { let t2129 = py::abs(&py::sub(&py::getitem(&v_hist.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_hist.clone(), &py::neg(&Py::Int(3i64))?)?)?)?; Py::Bool({ let t2130 = Py::Float(1e-14); if py::lt(&t2129, &t2130)? { true } else { false } }) } } }).truthy() {
            { t2125 = true; break; }
        }
    }
    if !t2125 {
        return Err(py::value_error(py::to_str(&Py::str("the orbit does not settle on a fixed point or a 2-cycle (chaotic or slowly converging regime): a finite-time exponent there depends on rounding, so it is not reported"))));
    }
    v_tot = Py::Float(0.0);
    for t2132 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t2132;
        v_d = py::abs(&py::mul(&v_r.clone(), &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_x.clone())?)?)?)?;
        if ({ let t2133 = v_d.clone(); Py::Bool({ let t2134 = Py::Int(0i64); if py::eq(&t2133, &t2134) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the orbit is superstable (f'(x) = 0): the exponent is minus infinity"))));
        }
        v_tot = py::iadd(&v_tot, &py::math1("log", &v_d.clone())?)?;
        v_x = py::mul(&py::mul(&v_r.clone(), &v_x.clone())?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?;
    }
    v_attractor = (if ({ let t2135 = py::abs(&py::sub(&py::mul(&py::mul(&v_r.clone(), &v_x.clone())?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?, &v_x.clone())?)?; Py::Bool({ let t2136 = Py::Float(1e-12); if py::lt(&t2135, &t2136)? { true } else { false } }) }).truthy() { Py::str("fixed point") } else { Py::str("2-cycle") });
    if ({ let t2141 = { let t2137 = v_attractor.clone(); Py::Bool({ let t2138 = Py::str("fixed point"); if py::eq(&t2137, &t2138) { true } else { false } }) }; if !t2141.truthy() { t2141 } else { { let t2139 = py::abs(&v_x.clone())?; Py::Bool({ let t2140 = Py::Float(1e-12); if py::lt(&t2139, &t2140)? { true } else { false } }) } } }).truthy() {
        v_attractor = Py::str("fixed point 0");
    }
    return Ok(Py::dict(vec![(Py::str("lyapunov"), f__r(py::truediv(&v_tot.clone(), &v_n.clone())?)?), (Py::str("attractor"), v_attractor.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logistic(mut v_P0: Py, mut v_r: Py, mut v_K: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_P0.clone(), v_r.clone(), v_K.clone(), v_t.clone()]))?;
    if ({ let t2221 = { let t2214 = v_P0.clone(); Py::Bool({ let t2215 = Py::Int(0i64); if py::le(&t2214, &t2215)? { true } else { false } }) }; if t2221.truthy() { t2221 } else { { let t2220 = { let t2216 = v_K.clone(); Py::Bool({ let t2217 = Py::Int(0i64); if py::le(&t2216, &t2217)? { true } else { false } }) }; if t2220.truthy() { t2220 } else { { let t2218 = v_r.clone(); Py::Bool({ let t2219 = Py::Int(0i64); if py::le(&t2218, &t2219)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs P0 > 0, K > 0 and r > 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("P"), f__r(py::truediv(&py::mul(&v_K.clone(), &v_P0.clone())?, &py::add(&v_P0.clone(), &py::mul(&py::sub(&v_K.clone(), &v_P0.clone())?, &f__exp(py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?)?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logistic_inflection(mut v_P0: Py, mut v_r: Py, mut v_K: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_P0.clone(), v_r.clone(), v_K.clone()]))?;
    if (py::not(&{ let t2033 = { let t2025 = v_r.clone(); Py::Bool({ let t2026 = Py::Int(0i64); if py::gt(&t2025, &t2026)? { true } else { false } }) }; if !t2033.truthy() { t2033 } else { { let t2032 = { let t2027 = v_K.clone(); Py::Bool({ let t2028 = Py::Int(0i64); if py::gt(&t2027, &t2028)? { true } else { false } }) }; if !t2032.truthy() { t2032 } else { { let t2029 = Py::Int(0i64); Py::Bool({ let t2030 = v_P0.clone(); if py::lt(&t2029, &t2030)? { { let t2031 = py::truediv(&v_K.clone(), &Py::Int(2i64))?; if py::lt(&t2030, &t2031)? { true } else { false } } } else { false } }) } } } } })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs r > 0, K > 0 and 0 < P0 < K/2 (the inflection lies ahead)"))));
    }
    return Ok(Py::dict(vec![(Py::str("t_inflection"), f__r(py::truediv(&py::math1("log", &py::sub(&py::truediv(&v_K.clone(), &v_P0.clone())?, &Py::Int(1i64))?)?, &v_r.clone())?)?), (Py::str("P_inflection"), f__r(py::truediv(&v_K.clone(), &Py::Int(2i64))?)?), (Py::str("max_rate"), f__r(py::truediv(&py::mul(&v_r.clone(), &v_K.clone())?, &Py::Int(4i64))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lorenz_eq(mut v_sigma: Py, mut v_rho: Py, mut v_beta: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_sigma.clone(), v_rho.clone(), v_beta.clone()]))?;
    if ({ let t2263 = { let t2256 = v_sigma.clone(); Py::Bool({ let t2257 = Py::Int(0i64); if py::le(&t2256, &t2257)? { true } else { false } }) }; if t2263.truthy() { t2263 } else { { let t2262 = { let t2258 = v_beta.clone(); Py::Bool({ let t2259 = Py::Int(0i64); if py::le(&t2258, &t2259)? { true } else { false } }) }; if t2262.truthy() { t2262 } else { { let t2260 = v_rho.clone(); Py::Bool({ let t2261 = Py::Int(0i64); if py::le(&t2260, &t2261)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs sigma, rho, beta > 0"))));
    }
    v_out = Py::dict(vec![(Py::str("origin_stability"), (if ({ let t2264 = v_rho.clone(); Py::Bool({ let t2265 = Py::Int(1i64); if py::lt(&t2264, &t2265)? { true } else { false } }) }).truthy() { Py::str("asymptotically stable") } else { (if ({ let t2266 = v_rho.clone(); Py::Bool({ let t2267 = Py::Int(1i64); if py::gt(&t2266, &t2267)? { true } else { false } }) }).truthy() { Py::str("unstable") } else { Py::str("marginal (rho = 1)") }) }))]);
    if ({ let t2268 = v_rho.clone(); Py::Bool({ let t2269 = Py::Int(1i64); if py::gt(&t2268, &t2269)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_out.clone(), "update", &[Py::dict(vec![(Py::str("count"), Py::Int(3i64)), (Py::str("x_c"), f__r(py::math1("sqrt", &py::mul(&v_beta.clone(), &py::sub(&v_rho.clone(), &Py::Int(1i64))?)?)?)?), (Py::str("z_c"), f__r(py::sub(&v_rho.clone(), &Py::Int(1i64))?)?)])])?;
    } else {
        py::setitem(&v_out.clone(), &Py::str("count"), Py::Int(1i64))?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lorenz_hopf(mut v_sigma: Py, mut v_beta: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_sigma.clone(), v_beta.clone()]))?;
    if ({ let t2309 = { let t2302 = v_sigma.clone(); Py::Bool({ let t2303 = Py::Int(0i64); if py::le(&t2302, &t2303)? { true } else { false } }) }; if t2309.truthy() { t2309 } else { { let t2308 = { let t2304 = v_beta.clone(); Py::Bool({ let t2305 = Py::Int(0i64); if py::le(&t2304, &t2305)? { true } else { false } }) }; if t2308.truthy() { t2308 } else { { let t2306 = v_sigma.clone(); Py::Bool({ let t2307 = py::add(&v_beta.clone(), &Py::Int(1i64))?; if py::le(&t2306, &t2307)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs sigma > beta + 1 (otherwise C+- never lose stability through a Hopf bifurcation)"))));
    }
    return Ok(Py::dict(vec![(Py::str("rho_H"), f__r(py::truediv(&py::mul(&v_sigma.clone(), &py::add(&py::add(&v_sigma.clone(), &v_beta.clone())?, &Py::Int(3i64))?)?, &py::sub(&py::sub(&v_sigma.clone(), &v_beta.clone())?, &Py::Int(1i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lotka_volterra(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()]))?;
    if ({ let t2342 = py::minmax(&[v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()], false)?; Py::Bool({ let t2343 = Py::Int(0i64); if py::le(&t2342, &t2343)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("all four parameters must be positive"))));
    }
    v_w = py::math1("sqrt", &py::mul(&v_a.clone(), &v_c.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("x_eq"), f__r(py::truediv(&v_c.clone(), &v_d.clone())?)?), (Py::str("y_eq"), f__r(py::truediv(&v_a.clone(), &v_b.clone())?)?), (Py::str("omega"), f__r(v_w.clone())?), (Py::str("period"), f__r(py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mixing(mut v_V: Py, mut v_r: Py, mut v_c_in: Py, mut v_A0: Py, mut v_t: Py) -> R {
    let mut v_Ainf: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_V.clone(), v_r.clone(), v_c_in.clone(), v_A0.clone(), v_t.clone()]))?;
    if ({ let t2436 = { let t2426 = v_V.clone(); Py::Bool({ let t2427 = Py::Int(0i64); if py::le(&t2426, &t2427)? { true } else { false } }) }; if t2436.truthy() { t2436 } else { { let t2435 = { let t2428 = v_r.clone(); Py::Bool({ let t2429 = Py::Int(0i64); if py::le(&t2428, &t2429)? { true } else { false } }) }; if t2435.truthy() { t2435 } else { { let t2434 = { let t2430 = v_c_in.clone(); Py::Bool({ let t2431 = Py::Int(0i64); if py::lt(&t2430, &t2431)? { true } else { false } }) }; if t2434.truthy() { t2434 } else { { let t2432 = v_A0.clone(); Py::Bool({ let t2433 = Py::Int(0i64); if py::lt(&t2432, &t2433)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs V > 0, r > 0, c_in >= 0, A0 >= 0"))));
    }
    v_Ainf = py::mul(&v_c_in.clone(), &v_V.clone())?;
    return Ok(Py::dict(vec![(Py::str("A"), f__r(py::add(&v_Ainf.clone(), &py::mul(&py::sub(&v_A0.clone(), &v_Ainf.clone())?, &f__exp(py::truediv(&py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?, &v_V.clone())?)?)?)?)?), (Py::str("A_limit"), f__r(v_Ainf.clone())?), (Py::str("concentration"), f__r(py::truediv(&py::add(&v_Ainf.clone(), &py::mul(&py::sub(&v_A0.clone(), &v_Ainf.clone())?, &f__exp(py::truediv(&py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?, &v_V.clone())?)?)?)?, &v_V.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_oscillator_params(mut v_m: Py, mut v_c: Py, mut v_k: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_w0: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_c.clone(), v_k.clone()]))?;
    if ({ let t708 = { let t701 = v_m.clone(); Py::Bool({ let t702 = Py::Int(0i64); if py::le(&t701, &t702)? { true } else { false } }) }; if t708.truthy() { t708 } else { { let t707 = { let t703 = v_c.clone(); Py::Bool({ let t704 = Py::Int(0i64); if py::lt(&t703, &t704)? { true } else { false } }) }; if t707.truthy() { t707 } else { { let t705 = v_k.clone(); Py::Bool({ let t706 = Py::Int(0i64); if py::le(&t705, &t706)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m > 0, c >= 0, k > 0"))));
    }
    v_w0 = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    v_zeta = py::truediv(&v_c.clone(), &py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&v_m.clone(), &v_k.clone())?)?)?)?;
    v_D = py::sub(&py::pow(&py::fraction(&[v_c.clone()])?, &Py::Int(2i64))?, &py::mul(&py::mul(&Py::Int(4i64), &py::fraction(&[v_m.clone()])?)?, &py::fraction(&[v_k.clone()])?)?)?;
    v_out = Py::dict(vec![(Py::str("omega0"), f__r(v_w0.clone())?), (Py::str("zeta"), f__r(v_zeta.clone())?), (Py::str("damping"), f__damping(v_m.clone(), v_c.clone(), v_k.clone(), v_D.clone())?)]);
    if ({ let t725 = v_D.clone(); Py::Bool({ let t726 = Py::Int(0i64); if py::lt(&t725, &t726)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("omega_d"), f__r(py::truediv(&py::math1("sqrt", &py::sub(&py::mul(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_k.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_m.clone())?)?)?)?;
        py::setitem(&v_out.clone(), &Py::str("period_d"), f__r(py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &py::truediv(&py::math1("sqrt", &py::sub(&py::mul(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_k.clone())?, &py::mul(&v_c.clone(), &v_c.clone())?)?)?, &py::mul(&Py::Int(2i64), &v_m.clone())?)?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_partial_fractions(mut v_F: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Pc: Py = Py::None;
    let mut v_Q: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dQ: Py = Py::None;
    let mut v_dQc: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_order: Py = Py::None;
    let mut v_out_p: Py = Py::None;
    let mut v_out_r: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_pairs: Py = Py::None;
    let mut v_ps: Py = Py::None;
    let mut v_pz: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zz: Py = Py::None;
    let t2793 = (f__ratfun(v_F.clone())?).items()?;
    if t2793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2793[0].clone();
    v_Q = t2793[1].clone();
    v_ps = f__poles(v_Q.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t2800 in py::iter(&(v_ps.clone()))? { let mut v__: Py; let mut v_m: Py; 
let t2801 = (t2800).items()?;
if t2801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t2801[0].clone();
v_m = t2801[1].clone();
out.push({ let t2802 = v_m.clone(); Py::Bool({ let t2803 = Py::Int(1i64); if py::gt(&t2802, &t2803)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("repeated poles: this capability lists simple poles only (use the inverse transform)"))));
    }
    v_dQ = py::method(&v_Q.clone(), "diff", &[c_SS()?])?;
    let t2806 = (Py::tuple(vec![{ let mut out = vec![]; for t2804 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2804;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }, { let mut out = vec![]; for t2805 in py::iter(&(py::method(&v_dQ.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2805;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }])).items()?;
    if t2806.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t2806[0].clone();
    v_dQc = t2806[1].clone();
    v_ev = { let v_F = v_F.clone(); let v_P = v_P.clone(); let v_Pc = v_Pc.clone(); let v_Q = v_Q.clone(); let v_dQ = v_dQ.clone(); let v_dQc = v_dQc.clone(); let v_ps = v_ps.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_co: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2807 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_i: Py; 
let t2808 = (t2807).items()?;
if t2808.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2808[0].clone();
v_c = t2808[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_z.clone(), &py::sub(&py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?, &v_i.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_pairs = Py::list(vec![]);
    for t2809 in py::iter(&(v_ps.clone()))? {
        let t2812 = (t2809).items()?;
        if t2812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t2812[0].clone();
        v__ = t2812[1].clone();
        v_z = py::complex(&[v_p.clone()])?;
        if ({ let t2810 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2811 = py::mul(&Py::Float(1e-25), &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)?; if py::le(&t2810, &t2811)? { true } else { false } }) }).truthy() {
            v_z = py::complex(&[py::real(&v_z.clone())?, Py::Float(0.0)])?;
        }
        v_r = py::truediv(&py::call(&v_ev.clone(), &[v_Pc.clone(), v_z.clone()])?, &py::call(&v_ev.clone(), &[v_dQc.clone(), v_z.clone()])?)?;
        let _ = py::method(&v_pairs.clone(), "append", &[Py::tuple(vec![v_z.clone(), v_r.clone()])])?;
    }
    v_order = f__canon_c({ let mut out = vec![]; for t2822 in py::iter(&(v_pairs.clone()))? { let mut v__: Py; let mut v_z: Py; 
let t2823 = (t2822).items()?;
if t2823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_z = t2823[0].clone();
v__ = t2823[1].clone();
out.push(v_z.clone()); } Py::list(out) })?;
    v_pz = Py::dict(vec![]);
    for t2824 in py::iter(&(v_pairs.clone()))? {
        let t2825 = (t2824).items()?;
        if t2825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_z = t2825[0].clone();
        v_r = t2825[1].clone();
        py::setitem(&v_pz.clone(), &v_z.clone(), v_r.clone())?;
    }
    let t2826 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2826.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_out_p = t2826[0].clone();
    v_out_r = t2826[1].clone();
    for t2827 in py::iter(&(v_order.clone()))? {
        v_z = t2827;
        v_zz = (if (Py::Bool(matches!(v_z.clone(), Py::Complex(..)))).truthy() { v_z.clone() } else { py::complex(&[v_z.clone(), Py::Float(0.0)])? });
        v_r = py::getitem(&v_pz.clone(), &v_zz.clone())?;
        let _ = py::method(&v_out_p.clone(), "append", &[f__r((if (Py::Bool(matches!(v_z.clone(), Py::Complex(..)))).truthy() { v_z.clone() } else { py::float(&v_z.clone())? }))?])?;
        let _ = py::method(&v_out_r.clone(), "append", &[(if ({ let t2830 = py::imag(&v_zz.clone())?; Py::Bool({ let t2831 = Py::Int(0i64); if py::eq(&t2830, &t2831) { true } else { false } }) }).truthy() { f__r(py::real(&py::complex(&[py::real(&v_r.clone())?, Py::Float(0.0)])?)?)? } else { f__r(v_r.clone())? })])?;
    }
    return Ok(Py::dict(vec![(Py::str("poles"), v_out_p.clone()), (Py::str("residues"), v_out_r.clone()), (Py::str("count"), py::len(&v_out_p.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_phase_portrait(mut v_A: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_kind: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_r2: Py = Py::None;
    let mut v_stab: Py = Py::None;
    let mut v_tr: Py = Py::None;
    if ({ let t2871 = { let t2867 = py::len(&v_A.clone())?; Py::Bool({ let t2868 = Py::Int(2i64); if py::ne(&t2867, &t2868) { true } else { false } }) }; if t2871.truthy() { t2871 } else { { let t2869 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2870 = Py::Int(2i64); if py::ne(&t2869, &t2870) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be a 2x2 matrix"))));
    }
    let t2873 = ({ let mut out = vec![]; for t2872 in py::iter(&(Py::tuple(vec![py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?])))? { let mut v_v: Py; 
v_v = t2872;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }).items()?;
    if t2873.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2873[0].clone();
    v_b = t2873[1].clone();
    v_c = t2873[2].clone();
    v_d = t2873[3].clone();
    let t2874 = (Py::tuple(vec![py::add(&v_a.clone(), &v_d.clone())?, py::sub(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_b.clone(), &v_c.clone())?)?])).items()?;
    if t2874.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t2874[0].clone();
    v_det = t2874[1].clone();
    let t2905 = (f__classify(v_tr.clone(), v_det.clone(), { let t2899 = { let t2892 = v_b.clone(); Py::Bool({ let t2893 = Py::Int(0i64); if py::eq(&t2892, &t2893) { true } else { false } }) }; if !t2899.truthy() { t2899 } else { { let t2898 = { let t2894 = v_c.clone(); Py::Bool({ let t2895 = Py::Int(0i64); if py::eq(&t2894, &t2895) { true } else { false } }) }; if !t2898.truthy() { t2898 } else { { let t2896 = v_a.clone(); Py::Bool({ let t2897 = v_d.clone(); if py::eq(&t2896, &t2897) { true } else { false } }) } } } } }, { let t2900 = v_a.clone(); Py::Bool({ let t2901 = v_b.clone(); if py::eq(&t2900, &t2901) { { let t2902 = v_c.clone(); if py::eq(&t2901, &t2902) { { let t2903 = v_d.clone(); if py::eq(&t2902, &t2903) { { let t2904 = Py::Int(0i64); if py::eq(&t2903, &t2904) { true } else { false } } } else { false } } } else { false } } } else { false } }) })?).items()?;
    if t2905.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kind = t2905[0].clone();
    v_stab = t2905[1].clone();
    let t2912 = (f__roots2(Py::Float(1.0), py::neg(&py::float(&v_tr.clone())?)?, py::float(&v_det.clone())?)?).items()?;
    if t2912.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t2912[0].clone();
    v_r2 = t2912[1].clone();
    v__ = t2912[2].clone();
    return Ok(Py::dict(vec![(Py::str("type"), v_kind.clone()), (Py::str("stability"), v_stab.clone()), (Py::str("lambda1"), f__r(v_r1.clone())?), (Py::str("lambda2"), f__r(v_r2.clone())?), (Py::str("trace"), f__r(py::float(&v_tr.clone())?)?), (Py::str("det"), f__r(py::float(&v_det.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_practical_resonance(mut v_m: Py, mut v_c: Py, mut v_k: Py, mut v_F0: Py) -> R {
    let mut v_w2: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_c.clone(), v_k.clone(), v_F0.clone()]))?;
    if ({ let t2952 = { let t2945 = v_m.clone(); Py::Bool({ let t2946 = Py::Int(0i64); if py::le(&t2945, &t2946)? { true } else { false } }) }; if t2952.truthy() { t2952 } else { { let t2951 = { let t2947 = v_c.clone(); Py::Bool({ let t2948 = Py::Int(0i64); if py::le(&t2947, &t2948)? { true } else { false } }) }; if t2951.truthy() { t2951 } else { { let t2949 = v_k.clone(); Py::Bool({ let t2950 = Py::Int(0i64); if py::le(&t2949, &t2950)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m, c, k > 0"))));
    }
    v_w2 = py::sub(&py::truediv(&v_k.clone(), &v_m.clone())?, &py::truediv(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_m.clone())?)?)?;
    if ({ let t2953 = v_w2.clone(); Py::Bool({ let t2954 = Py::Int(0i64); if py::le(&t2953, &t2954)? { true } else { false } }) }).truthy() {
        return Ok(Py::dict(vec![(Py::str("omega_r"), Py::Float(0.0)), (Py::str("amplitude_max"), f__r(py::truediv(&py::abs(&v_F0.clone())?, &v_k.clone())?)?), (Py::str("has_peak"), Py::Bool(false))]));
    }
    return Ok(Py::dict(vec![(Py::str("omega_r"), f__r(py::math1("sqrt", &v_w2.clone())?)?), (Py::str("amplitude_max"), f__r(py::truediv(&py::abs(&v_F0.clone())?, &py::mul(&v_c.clone(), &py::math1("sqrt", &py::sub(&py::truediv(&v_k.clone(), &v_m.clone())?, &py::truediv(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_m.clone())?, &v_m.clone())?)?)?)?)?)?)?), (Py::str("has_peak"), Py::Bool(true))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pure_resonance(mut v_m: Py, mut v_k: Py, mut v_F0: Py, mut v_t: Py) -> R {
    let mut v_w0: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_k.clone(), v_F0.clone(), v_t.clone()]))?;
    if ({ let t3697 = { let t3693 = v_m.clone(); Py::Bool({ let t3694 = Py::Int(0i64); if py::le(&t3693, &t3694)? { true } else { false } }) }; if t3697.truthy() { t3697 } else { { let t3695 = v_k.clone(); Py::Bool({ let t3696 = Py::Int(0i64); if py::le(&t3695, &t3696)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m > 0 and k > 0"))));
    }
    v_w0 = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::mul(&py::mul(&py::truediv(&v_F0.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_w0.clone())?)?, &v_t.clone())?, &py::math1("sin", &py::mul(&v_w0.clone(), &v_t.clone())?)?)?)?), (Py::str("envelope"), f__r(py::truediv(&py::mul(&py::abs(&v_F0.clone())?, &v_t.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_w0.clone())?)?)?), (Py::str("omega0"), f__r(v_w0.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_riccati_sq(mut v_k: Py, mut v_y0: Py, mut v_t: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_k.clone(), v_y0.clone(), v_t.clone()]))?;
    if ({ let t3037 = v_t.clone(); Py::Bool({ let t3038 = Py::Int(0i64); if py::lt(&t3037, &t3038)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs t >= 0"))));
    }
    v_d = py::sub(&Py::Int(1i64), &py::mul(&py::mul(&v_k.clone(), &v_y0.clone())?, &v_t.clone())?)?;
    if ({ let t3039 = v_d.clone(); Py::Bool({ let t3040 = Py::Int(0i64); if py::le(&t3039, &t3040)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the solution blows up at t = ".to_string(), py::format(&py::truediv(&Py::Int(1i64), &py::mul(&v_k.clone(), &v_y0.clone())?)?, "g")?, ", before the requested time".to_string()].concat()))));
    }
    v_out = Py::dict(vec![(Py::str("y"), f__r(py::truediv(&v_y0.clone(), &v_d.clone())?)?)]);
    if ({ let t3043 = py::mul(&v_k.clone(), &v_y0.clone())?; Py::Bool({ let t3044 = Py::Int(0i64); if py::gt(&t3043, &t3044)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("blowup_time"), f__r(py::truediv(&Py::Int(1i64), &py::mul(&v_k.clone(), &v_y0.clone())?)?)?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rk4_system(mut v_f: Py, mut v_g: Py, mut v_t0: Py, mut v_x0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_a3: Py = Py::None;
    let mut v_a4: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_b2: Py = Py::None;
    let mut v_b3: Py = Py::None;
    let mut v_b4: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t0.clone(), v_x0.clone(), v_y0.clone(), v_h.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t3285 = { let t3280 = v_h.clone(); Py::Bool({ let t3281 = Py::Int(0i64); if py::le(&t3280, &t3281)? { true } else { false } }) }; if t3285.truthy() { t3285 } else { py::not(&{ let t3282 = Py::Int(1i64); Py::Bool({ let t3283 = v_n.clone(); if py::le(&t3282, &t3283)? { { let t3284 = Py::Int(10000i64); if py::le(&t3283, &t3284)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs h > 0 and 1 <= n <= 10000"))));
    }
    let t3286 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone(), Py::tuple(vec![Py::str("t"), Py::str("x"), Py::str("y")])], &[])?, ax_engine::pysym::helper("differential_equations:_fn", &[v_g.clone(), Py::tuple(vec![Py::str("t"), Py::str("x"), Py::str("y")])], &[])?])).items()?;
    if t3286.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3286[0].clone();
    v_G = t3286[1].clone();
    let t3287 = (Py::tuple(vec![v_x0.clone(), v_y0.clone()])).items()?;
    if t3287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3287[0].clone();
    v_y = t3287[1].clone();
    for t3288 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3288;
        v_t = py::add(&v_t0.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?;
        let t3289 = (Py::tuple(vec![py::call(&v_F.clone(), &[v_t.clone(), v_x.clone(), v_y.clone()])?, py::call(&v_G.clone(), &[v_t.clone(), v_x.clone(), v_y.clone()])?])).items()?;
        if t3289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a1 = t3289[0].clone();
        v_b1 = t3289[1].clone();
        let t3290 = (Py::tuple(vec![py::call(&v_F.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a1.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b1.clone())?, &Py::Int(2i64))?)?])?, py::call(&v_G.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a1.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b1.clone())?, &Py::Int(2i64))?)?])?])).items()?;
        if t3290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a2 = t3290[0].clone();
        v_b2 = t3290[1].clone();
        let t3291 = (Py::tuple(vec![py::call(&v_F.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a2.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b2.clone())?, &Py::Int(2i64))?)?])?, py::call(&v_G.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a2.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b2.clone())?, &Py::Int(2i64))?)?])?])).items()?;
        if t3291.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a3 = t3291[0].clone();
        v_b3 = t3291[1].clone();
        v_t1 = py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?;
        let t3292 = (Py::tuple(vec![py::call(&v_F.clone(), &[v_t1.clone(), py::add(&v_x.clone(), &py::mul(&v_h.clone(), &v_a3.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_b3.clone())?)?])?, py::call(&v_G.clone(), &[v_t1.clone(), py::add(&v_x.clone(), &py::mul(&v_h.clone(), &v_a3.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_b3.clone())?)?])?])).items()?;
        if t3292.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a4 = t3292[0].clone();
        v_b4 = t3292[1].clone();
        let t3293 = (Py::tuple(vec![py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&py::add(&py::add(&v_a1.clone(), &py::mul(&Py::Int(2i64), &v_a2.clone())?)?, &py::mul(&Py::Int(2i64), &v_a3.clone())?)?, &v_a4.clone())?)?, &Py::Int(6i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&py::add(&py::add(&v_b1.clone(), &py::mul(&Py::Int(2i64), &v_b2.clone())?)?, &py::mul(&Py::Int(2i64), &v_b3.clone())?)?, &v_b4.clone())?)?, &Py::Int(6i64))?)?])).items()?;
        if t3293.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t3293[0].clone();
        v_y = t3293[1].clone();
        if ({ let t3297 = py::not(&{ let t3294 = py::math1("isfinite", &v_x.clone())?; if !t3294.truthy() { t3294 } else { py::math1("isfinite", &v_y.clone())? } }); if t3297.truthy() { t3297 } else { { let t3295 = py::minmax(&[py::abs(&v_x.clone())?, py::abs(&v_y.clone())?], true)?; Py::Bool({ let t3296 = Py::Float(1e+100); if py::gt(&t3295, &t3296)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the numerical solution overflows"))));
        }
    }
    return Ok(Py::dict(vec![(Py::str("x_n"), f__r(v_x.clone())?), (Py::str("y_n"), f__r(v_y.clone())?), (Py::str("t_n"), f__r(py::add(&v_t0.clone(), &py::mul(&v_n.clone(), &v_h.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_robin_eigen(mut v_L: Py, mut v_h: Py, mut v_n: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let mut v_mu: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_L.clone(), v_h.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t3121 = { let t3114 = v_L.clone(); Py::Bool({ let t3115 = Py::Int(0i64); if py::le(&t3114, &t3115)? { true } else { false } }) }; if t3121.truthy() { t3121 } else { { let t3120 = { let t3116 = v_h.clone(); Py::Bool({ let t3117 = Py::Int(0i64); if py::le(&t3116, &t3117)? { true } else { false } }) }; if t3120.truthy() { t3120 } else { { let t3118 = v_n.clone(); Py::Bool({ let t3119 = Py::Int(1i64); if py::lt(&t3118, &t3119)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0, h > 0 and n >= 1"))));
    }
    v_F = { let v_L = v_L.clone(); let v_h = v_h.clone(); let v_n = v_n.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_mu: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::mul(&v_mu.clone(), &py::math1("cos", &py::mul(&v_mu.clone(), &v_L.clone())?)?)?, &py::mul(&v_h.clone(), &py::math1("sin", &py::mul(&v_mu.clone(), &v_L.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t3122 = (Py::tuple(vec![py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Float(0.5))?, &Py::Float(std::f64::consts::PI))?, &v_L.clone())?, py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())?])).items()?;
    if t3122.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3122[0].clone();
    v_hi = t3122[1].clone();
    v_flo = py::call(&v_F.clone(), &[v_lo.clone()])?;
    for t3123 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t3123;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t3124 = { let t3126 = py::call(&v_F.clone(), &[v_mid.clone()])?; Py::Bool({ let t3127 = Py::Int(0i64); if py::gt(&t3126, &t3127)? { true } else { false } }) }; Py::Bool({ let t3125 = { let t3128 = v_flo.clone(); Py::Bool({ let t3129 = Py::Int(0i64); if py::gt(&t3128, &t3129)? { true } else { false } }) }; if py::eq(&t3124, &t3125) { true } else { false } }) }).truthy() {
            let t3130 = (Py::tuple(vec![v_mid.clone(), py::call(&v_F.clone(), &[v_mid.clone()])?])).items()?;
            if t3130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t3130[0].clone();
            v_flo = t3130[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
        if ({ let t3131 = py::sub(&v_hi.clone(), &v_lo.clone())?; Py::Bool({ let t3132 = py::mul(&Py::Float(4e-16), &v_hi.clone())?; if py::le(&t3131, &t3132)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_mu = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
    return Ok(Py::dict(vec![(Py::str("lambda_n"), f__r(py::mul(&v_mu.clone(), &v_mu.clone())?)?), (Py::str("sqrt_lambda"), f__r(v_mu.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_routh_hurwitz(mut v_coeffs: Py) -> R {
    let mut v_ch: Py = Py::None;
    let mut v_first: Py = Py::None;
    if (py::not(&{ let t3162 = Py::Int(2i64); Py::Bool({ let t3163 = py::len(&v_coeffs.clone())?; if py::le(&t3162, &t3163)? { { let t3164 = Py::Int(9i64); if py::le(&t3163, &t3164)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 9 coefficients (degree 1 to 8), highest power first"))));
    }
    if ({ let t3165 = py::getitem(&v_coeffs.clone(), &Py::Int(0i64))?; Py::Bool({ let t3166 = Py::Int(0i64); if py::eq(&t3165, &t3166) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the leading coefficient must not be 0"))));
    }
    let t3194 = (f__routh(v_coeffs.clone())?).items()?;
    if t3194.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t3194[0].clone();
    v_ch = t3194[1].clone();
    return Ok(Py::dict(vec![(Py::str("stable"), { let t3195 = v_ch.clone(); Py::Bool({ let t3196 = Py::Int(0i64); if py::eq(&t3195, &t3196) { true } else { false } }) }), (Py::str("rhp_roots"), v_ch.clone()), (Py::str("first_column"), { let mut out = vec![]; for t3197 in py::iter(&(v_first.clone()))? { let mut v_v: Py; 
v_v = t3197;
out.push(f__r(py::float(&v_v.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_separable_power(mut v_a: Py, mut v_n: Py, mut v_y0: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_n.clone(), v_y0.clone(), v_t.clone()]))?;
    if ({ let t3409 = { let t3405 = v_n.clone(); Py::Bool({ let t3406 = Py::Int(0i64); if py::lt(&t3405, &t3406)? { true } else { false } }) }; if t3409.truthy() { t3409 } else { { let t3407 = v_t.clone(); Py::Bool({ let t3408 = Py::Int(0i64); if py::lt(&t3407, &t3408)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs n >= 0 and t >= 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::mul(&v_y0.clone(), &f__exp(py::truediv(&py::mul(&v_a.clone(), &py::pow(&v_t.clone(), &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?, &py::add(&v_n.clone(), &Py::Int(1i64))?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_shm(mut v_m: Py, mut v_k: Py, mut v_x0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_w: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_k.clone(), v_x0.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t3447 = { let t3443 = v_m.clone(); Py::Bool({ let t3444 = Py::Int(0i64); if py::le(&t3443, &t3444)? { true } else { false } }) }; if t3447.truthy() { t3447 } else { { let t3445 = v_k.clone(); Py::Bool({ let t3446 = Py::Int(0i64); if py::le(&t3445, &t3446)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m > 0 and k > 0"))));
    }
    v_w = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("x"), f__r(py::add(&py::mul(&v_x0.clone(), &py::math1("cos", &py::mul(&v_w.clone(), &v_t.clone())?)?)?, &py::mul(&py::truediv(&v_v0.clone(), &v_w.clone())?, &py::math1("sin", &py::mul(&v_w.clone(), &v_t.clone())?)?)?)?)?), (Py::str("amplitude"), f__r(py::hypot(&[v_x0.clone(), py::truediv(&v_v0.clone(), &v_w.clone())?])?)?), (Py::str("omega"), f__r(v_w.clone())?), (Py::str("period"), f__r(py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_w.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sine_coefficient(mut v_f: Py, mut v_L: Py, mut v_n: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_L.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t1166 = { let t1162 = v_L.clone(); Py::Bool({ let t1163 = Py::Int(0i64); if py::le(&t1162, &t1163)? { true } else { false } }) }; if t1166.truthy() { t1166 } else { { let t1164 = v_n.clone(); Py::Bool({ let t1165 = Py::Int(1i64); if py::lt(&t1164, &t1165)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0 and n >= 1"))));
    }
    return Ok(Py::dict(vec![(Py::str("b_n"), f__r(f__sine_coeff(f__poly_x(v_f.clone(), Py::Int(6i64))?, v_L.clone(), v_n.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sir(mut v_beta: Py, mut v_gamma: Py, mut v_S0: Py, mut v_I0: Py) -> R {
    let mut v_Imax: Py = Py::None;
    let mut v_R0: Py = Py::None;
    let mut v_Sinf: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_beta.clone(), v_gamma.clone(), v_S0.clone(), v_I0.clone()]))?;
    if ({ let t3493 = { let t3480 = v_beta.clone(); Py::Bool({ let t3481 = Py::Int(0i64); if py::le(&t3480, &t3481)? { true } else { false } }) }; if t3493.truthy() { t3493 } else { { let t3492 = { let t3482 = v_gamma.clone(); Py::Bool({ let t3483 = Py::Int(0i64); if py::le(&t3482, &t3483)? { true } else { false } }) }; if t3492.truthy() { t3492 } else { py::not(&{ let t3491 = { let t3484 = Py::Int(0i64); Py::Bool({ let t3485 = v_S0.clone(); if py::lt(&t3484, &t3485)? { true } else { false } }) }; if !t3491.truthy() { t3491 } else { { let t3490 = { let t3486 = Py::Int(0i64); Py::Bool({ let t3487 = v_I0.clone(); if py::lt(&t3486, &t3487)? { true } else { false } }) }; if !t3490.truthy() { t3490 } else { { let t3488 = py::add(&v_S0.clone(), &v_I0.clone())?; Py::Bool({ let t3489 = Py::Int(1i64); if py::le(&t3488, &t3489)? { true } else { false } }) } } } } }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs beta, gamma > 0 and fractions S0, I0 > 0 with S0 + I0 <= 1"))));
    }
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    if ({ let t3494 = py::mul(&v_R0.clone(), &v_S0.clone())?; Py::Bool({ let t3495 = Py::Int(1i64); if py::gt(&t3494, &t3495)? { true } else { false } }) }).truthy() {
        v_Imax = py::sub(&py::add(&v_S0.clone(), &v_I0.clone())?, &py::truediv(&py::add(&Py::Int(1i64), &py::math1("log", &py::mul(&v_R0.clone(), &v_S0.clone())?)?)?, &v_R0.clone())?)?;
    } else {
        v_Imax = v_I0.clone();
    }
    v_h = { let v_I0 = v_I0.clone(); let v_Imax = v_Imax.clone(); let v_R0 = v_R0.clone(); let v_S0 = v_S0.clone(); let v_beta = v_beta.clone(); let v_gamma = v_gamma.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_S: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("log", &py::truediv(&v_S.clone(), &v_S0.clone())?)?, &py::mul(&v_R0.clone(), &py::sub(&py::sub(&v_S.clone(), &v_S0.clone())?, &v_I0.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t3496 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&Py::Int(1i64), &v_R0.clone())?])).items()?;
    if t3496.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3496[0].clone();
    v_hi = t3496[1].clone();
    for t3497 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t3497;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t3502 = { let t3498 = v_mid.clone(); Py::Bool({ let t3499 = Py::Int(0i64); if py::eq(&t3498, &t3499) { true } else { false } }) }; if t3502.truthy() { t3502 } else { { let t3500 = py::call(&v_h.clone(), &[v_mid.clone()])?; Py::Bool({ let t3501 = Py::Int(0i64); if py::lt(&t3500, &t3501)? { true } else { false } }) } } }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
        if ({ let t3503 = py::sub(&v_hi.clone(), &v_lo.clone())?; Py::Bool({ let t3504 = py::mul(&Py::Float(4e-16), &v_hi.clone())?; if py::le(&t3503, &t3504)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_Sinf = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
    return Ok(Py::dict(vec![(Py::str("R0"), f__r(v_R0.clone())?), (Py::str("I_max"), f__r(v_Imax.clone())?), (Py::str("S_inf"), f__r(v_Sinf.clone())?), (Py::str("attack_rate"), f__r(py::sub(&v_S0.clone(), &v_Sinf.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_slope_field(mut v_f: Py, mut v_t: Py, mut v_y: Py) -> R {
    let mut v_m: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t.clone(), v_y.clone()]))?;
    v_m = py::call(&ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, &[v_t.clone(), v_y.clone()])?;
    return Ok(Py::dict(vec![(Py::str("slope"), f__r(v_m.clone())?), (Py::str("angle_deg"), f__r(py::math1("degrees", &py::math1("atan", &v_m.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_steady_state(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_F0: Py, mut v_w: Py) -> R {
    let mut v_den: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_F0.clone(), v_w.clone()]))?;
    if ({ let t3579 = { let t3569 = v_a.clone(); Py::Bool({ let t3570 = Py::Int(0i64); if py::le(&t3569, &t3570)? { true } else { false } }) }; if t3579.truthy() { t3579 } else { { let t3578 = { let t3571 = v_b.clone(); Py::Bool({ let t3572 = Py::Int(0i64); if py::le(&t3571, &t3572)? { true } else { false } }) }; if t3578.truthy() { t3578 } else { { let t3577 = { let t3573 = v_c.clone(); Py::Bool({ let t3574 = Py::Int(0i64); if py::le(&t3573, &t3574)? { true } else { false } }) }; if t3577.truthy() { t3577 } else { { let t3575 = v_w.clone(); Py::Bool({ let t3576 = Py::Int(0i64); if py::le(&t3575, &t3576)? { true } else { false } }) } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a, b, c > 0 (damped oscillator) and w > 0"))));
    }
    v_den = py::hypot(&[py::sub(&v_c.clone(), &py::mul(&py::mul(&v_a.clone(), &v_w.clone())?, &v_w.clone())?)?, py::mul(&v_b.clone(), &v_w.clone())?])?;
    return Ok(Py::dict(vec![(Py::str("amplitude"), f__r(py::truediv(&py::abs(&v_F0.clone())?, &v_den.clone())?)?), (Py::str("phase_lag"), f__r(py::atan2(&py::mul(&v_b.clone(), &v_w.clone())?, &py::sub(&v_c.clone(), &py::mul(&py::mul(&v_a.clone(), &v_w.clone())?, &v_w.clone())?)?)?)?), (Py::str("gain"), f__r(py::truediv(&Py::Int(1i64), &v_den.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_step2(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_t: Py) -> R {
    let mut v_h: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_ys: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_t.clone()]))?;
    if ({ let t2730 = { let t2723 = v_a.clone(); Py::Bool({ let t2724 = Py::Int(0i64); if py::eq(&t2723, &t2724) { true } else { false } }) }; if t2730.truthy() { t2730 } else { { let t2729 = { let t2725 = v_c.clone(); Py::Bool({ let t2726 = Py::Int(0i64); if py::eq(&t2725, &t2726) { true } else { false } }) }; if t2729.truthy() { t2729 } else { { let t2727 = v_t.clone(); Py::Bool({ let t2728 = Py::Int(0i64); if py::lt(&t2727, &t2728)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a != 0, c != 0 and t >= 0"))));
    }
    v_ys = py::truediv(&Py::Float(1.0), &v_c.clone())?;
    v_h = f_hom2(v_a.clone(), v_b.clone(), v_c.clone(), py::neg(&v_ys.clone())?, Py::Float(0.0), v_t.clone())?;
    v_out = Py::dict(vec![(Py::str("y"), f__r(f__guard(py::add(&v_ys.clone(), &v_h.clone())?, Py::tuple(vec![v_ys.clone(), v_h.clone()]))?)?)]);
    if ({ let t2757 = { let t2753 = py::truediv(&v_b.clone(), &v_a.clone())?; Py::Bool({ let t2754 = Py::Int(0i64); if py::gt(&t2753, &t2754)? { true } else { false } }) }; if !t2757.truthy() { t2757 } else { { let t2755 = py::truediv(&v_c.clone(), &v_a.clone())?; Py::Bool({ let t2756 = Py::Int(0i64); if py::gt(&t2755, &t2756)? { true } else { false } }) } } }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("final_value"), f__r(v_ys.clone())?)?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_step_limit(mut v_method: Py, mut v_lam: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_co: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_mid: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_lam.clone()]))?;
    if ({ let t935 = v_lam.clone(); Py::Bool({ let t936 = Py::Int(0i64); if py::ge(&t935, &t936)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs lambda < 0 (a decaying mode)"))));
    }
    v_co = py::getitem(&c__STAB_POLY()?, &v_method.clone())?;
    v_g = { let v_co = v_co.clone(); let v_lam = v_lam.clone(); let v_method = v_method.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::abs(&py::sum(&{ let mut out = vec![]; for t937 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t938 = (t937).items()?;
if t938.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t938[0].clone();
v_c = t938[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&py::neg(&v_x.clone())?, &v_k.clone())?)?); } Py::list(out) }, None)?)?, &Py::Int(1i64))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t939 = (Py::tuple(vec![Py::Float(1.0), Py::Float(4.0)])).items()?;
    if t939.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t939[0].clone();
    v_hi = t939[1].clone();
    for t940 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t940;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t941 = py::call(&v_g.clone(), &[v_mid.clone()])?; Py::Bool({ let t942 = Py::Int(0i64); if py::le(&t941, &t942)? { true } else { false } }) }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
        if ({ let t943 = py::sub(&v_hi.clone(), &v_lo.clone())?; Py::Bool({ let t944 = py::mul(&Py::Float(4e-16), &v_hi.clone())?; if py::le(&t943, &t944)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(Py::dict(vec![(Py::str("h_max"), f__r(py::truediv(&v_lo.clone(), &py::abs(&v_lam.clone())?)?)?), (Py::str("z_boundary"), f__r(py::neg(&v_lo.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sturm_liouville(mut v_bc: Py, mut v_L: Py, mut v_n: Py) -> R {
    let mut v_fn: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_L.clone()]))?;
    v_n = py::int(&v_n.clone())?;
    if ({ let t3616 = { let t3612 = v_L.clone(); Py::Bool({ let t3613 = Py::Int(0i64); if py::le(&t3612, &t3613)? { true } else { false } }) }; if t3616.truthy() { t3616 } else { { let t3614 = v_n.clone(); Py::Bool({ let t3615 = Py::Int(1i64); if py::lt(&t3614, &t3615)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0 and n >= 1"))));
    }
    v_k = (if ({ let t3617 = v_bc.clone(); Py::Bool({ let t3618 = Py::tuple(vec![Py::str("dirichlet-neumann"), Py::str("neumann-dirichlet")]); if py::contains(&t3618, &t3617)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Float(0.5))?, &Py::Float(std::f64::consts::PI))?, &v_L.clone())? } else { py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())? });
    v_fn = py::getitem(&Py::dict(vec![(Py::str("dirichlet-dirichlet"), Py::str("sin")), (Py::str("dirichlet-neumann"), Py::str("sin")), (Py::str("neumann-dirichlet"), Py::str("cos")), (Py::str("neumann-neumann"), Py::str("cos"))]), &v_bc.clone())?;
    return Ok(Py::dict(vec![(Py::str("lambda_n"), f__r(py::mul(&v_k.clone(), &v_k.clone())?)?), (Py::str("sqrt_lambda"), f__r(v_k.clone())?), (Py::str("eigenfunction"), Py::str(&[py::format(&v_fn.clone(), "")?, "(".to_string(), py::format(&Py::str(&py::repr(&f__r(v_k.clone())?)), "")?, " x)".to_string()].concat()))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_time_to_reach(mut v_y0: Py, mut v_y1: Py, mut v_k: Py) -> R {
    let mut v_t: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_y0.clone(), v_y1.clone(), v_k.clone()]))?;
    if ({ let t1060 = { let t1053 = v_k.clone(); Py::Bool({ let t1054 = Py::Int(0i64); if py::eq(&t1053, &t1054) { true } else { false } }) }; if t1060.truthy() { t1060 } else { { let t1059 = { let t1055 = v_y0.clone(); Py::Bool({ let t1056 = Py::Int(0i64); if py::eq(&t1055, &t1056) { true } else { false } }) }; if t1059.truthy() { t1059 } else { { let t1057 = py::truediv(&v_y1.clone(), &v_y0.clone())?; Py::Bool({ let t1058 = Py::Int(0i64); if py::le(&t1057, &t1058)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs k != 0 and y1, y0 of the same sign"))));
    }
    v_t = py::truediv(&py::math1("log", &py::truediv(&v_y1.clone(), &v_y0.clone())?)?, &v_k.clone())?;
    if ({ let t1061 = v_t.clone(); Py::Bool({ let t1062 = Py::Int(0i64); if py::lt(&t1061, &t1062)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the target is not reached for t >= 0 with this rate"))));
    }
    return Ok(Py::dict(vec![(Py::str("t"), f__r(v_t.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_torricelli(mut v_h0: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_h: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_h0.clone(), v_k.clone(), v_t.clone()]))?;
    if ({ let t3658 = { let t3651 = v_h0.clone(); Py::Bool({ let t3652 = Py::Int(0i64); if py::le(&t3651, &t3652)? { true } else { false } }) }; if t3658.truthy() { t3658 } else { { let t3657 = { let t3653 = v_k.clone(); Py::Bool({ let t3654 = Py::Int(0i64); if py::le(&t3653, &t3654)? { true } else { false } }) }; if t3657.truthy() { t3657 } else { { let t3655 = v_t.clone(); Py::Bool({ let t3656 = Py::Int(0i64); if py::lt(&t3655, &t3656)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs h0 > 0, k > 0 and t >= 0"))));
    }
    v_T = py::truediv(&py::mul(&Py::Int(2i64), &py::math1("sqrt", &v_h0.clone())?)?, &v_k.clone())?;
    v_h = (if ({ let t3659 = v_t.clone(); Py::Bool({ let t3660 = v_T.clone(); if py::lt(&t3659, &t3660)? { true } else { false } }) }).truthy() { py::pow(&py::sub(&py::math1("sqrt", &v_h0.clone())?, &py::truediv(&py::mul(&v_k.clone(), &v_t.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))? } else { Py::Float(0.0) });
    return Ok(Py::dict(vec![(Py::str("h"), f__r(v_h.clone())?), (Py::str("t_empty"), f__r(v_T.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_two_points(mut v_t1: Py, mut v_y1: Py, mut v_t2: Py, mut v_y2: Py) -> R {
    let mut v_k: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t1.clone(), v_y1.clone(), v_t2.clone(), v_y2.clone()]))?;
    if ({ let t984 = { let t977 = v_t1.clone(); Py::Bool({ let t978 = v_t2.clone(); if py::eq(&t977, &t978) { true } else { false } }) }; if t984.truthy() { t984 } else { { let t983 = { let t979 = v_y1.clone(); Py::Bool({ let t980 = Py::Int(0i64); if py::eq(&t979, &t980) { true } else { false } }) }; if t983.truthy() { t983 } else { { let t981 = py::truediv(&v_y2.clone(), &v_y1.clone())?; Py::Bool({ let t982 = Py::Int(0i64); if py::le(&t981, &t982)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs two different times and two nonzero values of the same sign"))));
    }
    v_k = py::truediv(&py::math1("log", &py::truediv(&v_y2.clone(), &v_y1.clone())?)?, &py::sub(&v_t2.clone(), &v_t1.clone())?)?;
    return Ok(Py::dict(vec![(Py::str("k"), f__r(v_k.clone())?), (Py::str("y0"), f__r(py::mul(&v_y1.clone(), &f__exp(py::mul(&py::neg(&v_k.clone())?, &v_t1.clone())?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_uc_exp(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_F: Py, mut v_s: Py, mut v_y0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_A_: Py = Py::None;
    let mut v_B_: Py = Py::None;
    let mut v_C_: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_S_: Py = Py::None;
    let mut v_dP: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_mlt: Py = Py::None;
    let mut v_yp0: Py = Py::None;
    let mut v_yp_t: Py = Py::None;
    let mut v_ypd0: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_F.clone(), v_s.clone(), v_y0.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t3730 = v_a.clone(); Py::Bool({ let t3731 = Py::Int(0i64); if py::eq(&t3730, &t3731) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0"))));
    }
    let t3733 = ({ let mut out = vec![]; for t3732 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_s.clone()])))? { let mut v_v: Py; 
v_v = t3732;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }).items()?;
    if t3733.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A_ = t3733[0].clone();
    v_B_ = t3733[1].clone();
    v_C_ = t3733[2].clone();
    v_S_ = t3733[3].clone();
    let t3734 = (Py::tuple(vec![py::add(&py::add(&py::mul(&py::mul(&v_A_.clone(), &v_S_.clone())?, &v_S_.clone())?, &py::mul(&v_B_.clone(), &v_S_.clone())?)?, &v_C_.clone())?, py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_A_.clone())?, &v_S_.clone())?, &v_B_.clone())?])).items()?;
    if t3734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t3734[0].clone();
    v_dP = t3734[1].clone();
    if ({ let t3735 = v_P.clone(); Py::Bool({ let t3736 = Py::Int(0i64); if py::ne(&t3735, &t3736) { true } else { false } }) }).truthy() {
        let t3737 = (Py::tuple(vec![Py::Int(0i64), py::truediv(&v_F.clone(), &py::float(&v_P.clone())?)?])).items()?;
        if t3737.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mlt = t3737[0].clone();
        v_A = t3737[1].clone();
    } else {
        if ({ let t3738 = v_dP.clone(); Py::Bool({ let t3739 = Py::Int(0i64); if py::ne(&t3738, &t3739) { true } else { false } }) }).truthy() {
            let t3740 = (Py::tuple(vec![Py::Int(1i64), py::truediv(&v_F.clone(), &py::float(&v_dP.clone())?)?])).items()?;
            if t3740.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_mlt = t3740[0].clone();
            v_A = t3740[1].clone();
        } else {
            let t3741 = (Py::tuple(vec![Py::Int(2i64), py::truediv(&v_F.clone(), &py::mul(&Py::Int(2i64), &v_a.clone())?)?])).items()?;
            if t3741.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_mlt = t3741[0].clone();
            v_A = t3741[1].clone();
        }
    }
    v_e = f__exp(py::mul(&v_s.clone(), &v_t.clone())?)?;
    v_yp_t = py::mul(&py::mul(&v_A.clone(), &py::pow(&v_t.clone(), &v_mlt.clone())?)?, &v_e.clone())?;
    let t3747 = ((if ({ let t3743 = v_mlt.clone(); Py::Bool({ let t3744 = Py::Int(0i64); if py::eq(&t3743, &t3744) { true } else { false } }) }).truthy() { Py::tuple(vec![v_A.clone(), py::mul(&v_A.clone(), &v_s.clone())?]) } else { (if ({ let t3745 = v_mlt.clone(); Py::Bool({ let t3746 = Py::Int(1i64); if py::eq(&t3745, &t3746) { true } else { false } }) }).truthy() { Py::tuple(vec![Py::Float(0.0), v_A.clone()]) } else { Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)]) }) })).items()?;
    if t3747.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_yp0 = t3747[0].clone();
    v_ypd0 = t3747[1].clone();
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__forced(v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone(), v_yp_t.clone(), v_yp0.clone(), v_ypd0.clone())?)?), (Py::str("A"), f__r(v_A.clone())?), (Py::str("resonance_multiplicity"), v_mlt.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_uc_poly(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_p: Py, mut v_y0: Py, mut v_v0: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_yp_t: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone()]))?;
    if ({ let t3799 = v_a.clone(); Py::Bool({ let t3800 = Py::Int(0i64); if py::eq(&t3799, &t3800) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0"))));
    }
    if ({ let t3801 = v_c.clone(); Py::Bool({ let t3802 = Py::Int(0i64); if py::eq(&t3801, &t3802) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("c must not be 0 (integrate once: the equation is first order in y')"))));
    }
    if (py::not(&{ let t3803 = Py::Int(1i64); Py::Bool({ let t3804 = py::len(&v_p.clone())?; if py::le(&t3803, &t3804)? { { let t3805 = Py::Int(5i64); if py::le(&t3804, &t3805)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the forcing polynomial has degree at most 4 (1 to 5 coefficients p0, p1, ...)"))));
    }
    v_m = py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?;
    v_q = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_m.clone(), &Py::Int(3i64))?)?;
    for t3806 in py::range_iter(&[(v_m.clone()).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_k = t3806;
        py::setitem(&v_q.clone(), &v_k.clone(), py::truediv(&py::sub(&py::sub(&py::getitem(&v_p.clone(), &v_k.clone())?, &py::mul(&py::mul(&v_b.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_q.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::mul(&py::mul(&v_a.clone(), &py::add(&v_k.clone(), &Py::Int(2i64))?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_q.clone(), &py::add(&v_k.clone(), &Py::Int(2i64))?)?)?)?, &v_c.clone())?)?;
    }
    v_q = py::slice(&v_q.clone(), None, Some((py::add(&v_m.clone(), &Py::Int(1i64))?).i()?), None)?;
    v_yp_t = py::sum(&{ let mut out = vec![]; for t3807 in py::iter(&(py::enumerate(&v_q.clone(), 0)?))? { let mut v_k: Py; let mut v_qk: Py; 
let t3808 = (t3807).items()?;
if t3808.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3808[0].clone();
v_qk = t3808[1].clone();
out.push(py::mul(&v_qk.clone(), &py::pow(&v_t.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__forced(v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone(), v_yp_t.clone(), py::getitem(&v_q.clone(), &Py::Int(0i64))?, (if ({ let t3831 = v_m.clone(); Py::Bool({ let t3832 = Py::Int(1i64); if py::ge(&t3831, &t3832)? { true } else { false } }) }).truthy() { py::getitem(&v_q.clone(), &Py::Int(1i64))? } else { Py::Float(0.0) }))?)?), (Py::str("particular"), { let mut out = vec![]; for t3833 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t3833;
out.push(f__r(v_v.clone())?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_wave_series(mut v_f: Py, mut v_L: Py, mut v_c: Py, mut v_N: Py, mut v_x: Py, mut v_t: Py, mut v_g: Py) -> R {
    let mut v_cf: Py = Py::None;
    let mut v_cg: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_terms: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_L.clone(), v_c.clone(), v_x.clone(), v_t.clone()]))?;
    v_N = py::int(&v_N.clone())?;
    if ({ let t3876 = { let t3864 = v_L.clone(); Py::Bool({ let t3865 = Py::Int(0i64); if py::le(&t3864, &t3865)? { true } else { false } }) }; if t3876.truthy() { t3876 } else { { let t3875 = { let t3866 = v_c.clone(); Py::Bool({ let t3867 = Py::Int(0i64); if py::le(&t3866, &t3867)? { true } else { false } }) }; if t3875.truthy() { t3875 } else { { let t3874 = py::not(&{ let t3868 = Py::Int(0i64); Py::Bool({ let t3869 = v_x.clone(); if py::le(&t3868, &t3869)? { { let t3870 = v_L.clone(); if py::le(&t3869, &t3870)? { true } else { false } } } else { false } }) }); if t3874.truthy() { t3874 } else { py::not(&{ let t3871 = Py::Int(1i64); Py::Bool({ let t3872 = v_N.clone(); if py::le(&t3871, &t3872)? { { let t3873 = Py::Int(500i64); if py::le(&t3872, &t3873)? { true } else { false } } } else { false } }) }) } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0, c > 0, 0 <= x <= L and 1 <= N <= 500"))));
    }
    let t3882 = (Py::tuple(vec![f__poly_x(v_f.clone(), Py::Int(6i64))?, f__poly_x(v_g.clone(), Py::Int(6i64))?])).items()?;
    if t3882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cf = t3882[0].clone();
    v_cg = t3882[1].clone();
    v_terms = Py::list(vec![]);
    for t3883 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t3883;
        v_w = py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())?;
        let _ = py::method(&v_terms.clone(), "append", &[py::mul(&py::add(&py::mul(&f__sine_coeff(v_cf.clone(), v_L.clone(), v_n.clone())?, &py::math1("cos", &py::mul(&py::mul(&v_w.clone(), &v_c.clone())?, &v_t.clone())?)?)?, &py::mul(&py::truediv(&f__sine_coeff(v_cg.clone(), v_L.clone(), v_n.clone())?, &py::mul(&v_w.clone(), &v_c.clone())?)?, &py::math1("sin", &py::mul(&py::mul(&v_w.clone(), &v_c.clone())?, &v_t.clone())?)?)?)?, &py::math1("sin", &py::mul(&v_w.clone(), &v_x.clone())?)?)?])?;
    }
    return Ok(Py::dict(vec![(Py::str("u"), f__r(f__guard(py::fsum(&v_terms.clone())?, { let mut v = vec![]; v.extend((v_terms.clone()).items()?); Py::tuple(v) })?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1022(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1025: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1024 = v_inputs.clone(); s_1023(py::getitem(&t1024, &Py::str("y0"))?, py::getitem(&t1024, &Py::str("k"))?, py::getitem(&t1024, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1025 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1026 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1027 = (t1026).items()?;
if t1027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1027[0].clone();
v_v = t1027[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1023(mut v_y0: Py, mut v_k: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("y"), f__r(py::mul(&v_y0.clone(), &py::pow(&Py::Int(2i64), &py::truediv(&py::mul(&v_k.clone(), &v_t.clone())?, &py::math1("log", &Py::Int(2i64))?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1065(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1068: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1067 = v_inputs.clone(); s_1066(py::getitem(&t1067, &Py::str("y0"))?, py::getitem(&t1067, &Py::str("y1"))?, py::getitem(&t1067, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1068 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1069 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1070 = (t1069).items()?;
if t1070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1070[0].clone();
v_v = t1070[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1066(mut v_y0: Py, mut v_y1: Py, mut v_k: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::truediv(&py::sub(&py::math1("log", &py::abs(&v_y1.clone())?)?, &py::math1("log", &py::abs(&v_y0.clone())?)?)?, &v_k.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1132(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1134: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1133 = v_inputs.clone(); f_forced_cos(py::getitem(&t1133, &Py::str("a"))?, py::getitem(&t1133, &Py::str("b"))?, py::getitem(&t1133, &Py::str("c"))?, py::getitem(&t1133, &Py::str("F0"))?, py::getitem(&t1133, &Py::str("w"))?, py::getitem(&t1133, &Py::str("y0"))?, py::getitem(&t1133, &Py::str("v0"))?, py::getitem(&t1133, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1134 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1135 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1136 = (t1135).items()?;
if t1136.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1136[0].clone();
v_v = t1136[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1183(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1185: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1184 = v_inputs.clone(); f_sine_coefficient(py::getitem(&t1184, &Py::str("f"))?, py::getitem(&t1184, &Py::str("L"))?, py::getitem(&t1184, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1185 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1186 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1187 = (t1186).items()?;
if t1187.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1187[0].clone();
v_v = t1187[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1221(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1223: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1222 = v_inputs.clone(); f_gompertz(py::getitem(&t1222, &Py::str("y0"))?, py::getitem(&t1222, &Py::str("r"))?, py::getitem(&t1222, &Py::str("K"))?, py::getitem(&t1222, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1223 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1224 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1225 = (t1224).items()?;
if t1225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1225[0].clone();
v_v = t1225[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_123(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t125: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t124 = v_inputs.clone(); f_backward_euler(py::getitem(&t124, &Py::str("f"))?, py::getitem(&t124, &Py::str("t0"))?, py::getitem(&t124, &Py::str("y0"))?, py::getitem(&t124, &Py::str("h"))?, py::getitem(&t124, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t125 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t126 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t127 = (t126).items()?;
if t127.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t127[0].clone();
v_v = t127[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1255(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1258: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1257 = v_inputs.clone(); s_1256(py::getitem(&t1257, &Py::str("N0"))?, py::getitem(&t1257, &Py::str("T"))?, py::getitem(&t1257, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1258 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1259 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1260 = (t1259).items()?;
if t1260.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1260[0].clone();
v_v = t1260[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1256(mut v_N0: Py, mut v_T: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("N"), f__r(py::mul(&v_N0.clone(), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&py::math1("log", &Py::Int(2i64))?)?, &v_t.clone())?, &v_T.clone())?)?)?)?), (Py::str("k"), f__r(py::truediv(&py::math1("log", &Py::Int(2i64))?, &v_T.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1319(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1321: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1320 = v_inputs.clone(); f_heat_series(py::getitem(&t1320, &Py::str("f"))?, py::getitem(&t1320, &Py::str("L"))?, py::getitem(&t1320, &Py::str("alpha"))?, py::getitem(&t1320, &Py::str("N"))?, py::getitem(&t1320, &Py::str("x"))?, py::getitem(&t1320, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1321 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1322 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1323 = (t1322).items()?;
if t1323.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1323[0].clone();
v_v = t1323[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1364(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1366: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1365 = v_inputs.clone(); f_ftcs(py::getitem(&t1365, &Py::str("u"))?, py::getitem(&t1365, &Py::str("r"))?, py::getitem(&t1365, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1366 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1367 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1368 = (t1367).items()?;
if t1368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1368[0].clone();
v_v = t1368[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1391(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    return Ok(f__one_step(Py::str("heun"), ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, v_t0.clone(), v_y0.clone(), v_h.clone(), py::int(&v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1414(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1416: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1415 = v_inputs.clone(); s_1391(py::getitem(&t1415, &Py::str("f"))?, py::getitem(&t1415, &Py::str("t0"))?, py::getitem(&t1415, &Py::str("y0"))?, py::getitem(&t1415, &Py::str("h"))?, py::getitem(&t1415, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1416 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1417 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1418 = (t1417).items()?;
if t1418.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1418[0].clone();
v_v = t1418[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1441(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1443: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1442 = v_inputs.clone(); f_inverse_laplace(py::getitem(&t1442, &Py::str("F"))?, py::getitem(&t1442, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1443 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1444 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1445 = (t1444).items()?;
if t1445.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1445[0].clone();
v_v = t1445[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1507(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1509: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1508 = v_inputs.clone(); f_laplace_rect(py::getitem(&t1508, &Py::str("f"))?, py::getitem(&t1508, &Py::str("a"))?, py::getitem(&t1508, &Py::str("b"))?, py::getitem(&t1508, &Py::str("N"))?, py::getitem(&t1508, &Py::str("x"))?, py::getitem(&t1508, &Py::str("y"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1509 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1510 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1511 = (t1510).items()?;
if t1511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1511[0].clone();
v_v = t1511[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1557(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1559: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1558 = v_inputs.clone(); f_laplace_shifted(py::getitem(&t1558, &Py::str("f"))?, py::getitem(&t1558, &Py::str("c"))?, py::getitem(&t1558, &Py::str("s"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1559 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1560 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1561 = (t1560).items()?;
if t1561.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1561[0].clone();
v_v = t1561[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1605(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1607: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1606 = v_inputs.clone(); f_laplace_value(py::getitem(&t1606, &Py::str("f"))?, py::getitem(&t1606, &Py::str("s"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1607 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1608 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1609 = (t1608).items()?;
if t1609.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1609[0].clone();
v_v = t1609[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1643(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1645: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1644 = v_inputs.clone(); f_drag_linear(py::getitem(&t1644, &Py::str("m"))?, py::getitem(&t1644, &Py::str("c"))?, py::getitem(&t1644, &Py::str("g"))?, py::getitem(&t1644, &Py::str("v0"))?, py::getitem(&t1644, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1645 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1646 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1647 = (t1646).items()?;
if t1647.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1647[0].clone();
v_v = t1647[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1683(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1685: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1684 = v_inputs.clone(); f_lin1(py::getitem(&t1684, &Py::str("a"))?, py::getitem(&t1684, &Py::str("b"))?, py::getitem(&t1684, &Py::str("y0"))?, py::getitem(&t1684, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1685 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1686 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1687 = (t1686).items()?;
if t1687.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1687[0].clone();
v_v = t1687[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1724(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1726: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1725 = v_inputs.clone(); f_lin1_exp(py::getitem(&t1725, &Py::str("a"))?, py::getitem(&t1725, &Py::str("b"))?, py::getitem(&t1725, &Py::str("c"))?, py::getitem(&t1725, &Py::str("y0"))?, py::getitem(&t1725, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1726 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1727 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1728 = (t1727).items()?;
if t1728.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1728[0].clone();
v_v = t1728[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_175(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t176 = v_inputs.clone(); f_bernoulli(py::getitem(&t176, &Py::str("p"))?, py::getitem(&t176, &Py::str("q"))?, py::getitem(&t176, &Py::str("n"))?, py::getitem(&t176, &Py::str("y0"))?, py::getitem(&t176, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t177 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t178 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t179 = (t178).items()?;
if t179.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t179[0].clone();
v_v = t179[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1859 = v_inputs.clone(); f_linear_system_solution(py::getitem(&t1859, &Py::str("A"))?, py::getitem(&t1859, &Py::str("x0"))?, py::getitem(&t1859, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1860 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1861 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1862 = (t1861).items()?;
if t1862.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1862[0].clone();
v_v = t1862[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1951(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1953: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1952 = v_inputs.clone(); f_linear_stability(py::getitem(&t1952, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1953 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1954 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1955 = (t1954).items()?;
if t1955.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1955[0].clone();
v_v = t1955[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1995(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1997: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1996 = v_inputs.clone(); f_harvesting(py::getitem(&t1996, &Py::str("r"))?, py::getitem(&t1996, &Py::str("K"))?, py::getitem(&t1996, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1997 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1998 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1999 = (t1998).items()?;
if t1999.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1999[0].clone();
v_v = t1999[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_20(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t22: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t21 = v_inputs.clone(); f_amplification(py::getitem(&t21, &Py::str("method"))?, py::getitem(&t21, &Py::str("lam"))?, py::getitem(&t21, &Py::str("h"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t22 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t23 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t24 = (t23).items()?;
if t24.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t24[0].clone();
v_v = t24[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2036(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2038: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2037 = v_inputs.clone(); f_logistic_inflection(py::getitem(&t2037, &Py::str("P0"))?, py::getitem(&t2037, &Py::str("r"))?, py::getitem(&t2037, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2038 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2039 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2040 = (t2039).items()?;
if t2040.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2040[0].clone();
v_v = t2040[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2081(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2082 = v_inputs.clone(); f_lm_fixed(py::getitem(&t2082, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2083 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2084 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2085 = (t2084).items()?;
if t2085.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2085[0].clone();
v_v = t2085[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2146: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2145 = v_inputs.clone(); f_lm_lyapunov(py::getitem(&t2145, &Py::str("r"))?, py::getitem(&t2145, &Py::str("x0"))?, py::getitem(&t2145, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2146 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2147 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2148 = (t2147).items()?;
if t2148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2148[0].clone();
v_v = t2148[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2184(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2186: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2185 = v_inputs.clone(); f_lm_cycle(py::getitem(&t2185, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2186 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2187 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2188 = (t2187).items()?;
if t2188.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2188[0].clone();
v_v = t2188[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2225(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2228: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2227 = v_inputs.clone(); s_2226(py::getitem(&t2227, &Py::str("P0"))?, py::getitem(&t2227, &Py::str("r"))?, py::getitem(&t2227, &Py::str("K"))?, py::getitem(&t2227, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2228 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2229 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2230 = (t2229).items()?;
if t2230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2230[0].clone();
v_v = t2230[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2226(mut v_P0: Py, mut v_r: Py, mut v_K: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("P"), f__r(py::truediv(&v_K.clone(), &py::add(&Py::Int(1i64), &py::mul(&py::sub(&py::truediv(&v_K.clone(), &v_P0.clone())?, &Py::Int(1i64))?, &f__exp(py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?)?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2272(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2274: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2273 = v_inputs.clone(); f_lorenz_eq(py::getitem(&t2273, &Py::str("sigma"))?, py::getitem(&t2273, &Py::str("rho"))?, py::getitem(&t2273, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2274 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2275 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2276 = (t2275).items()?;
if t2276.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2276[0].clone();
v_v = t2276[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2312(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2314: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2313 = v_inputs.clone(); f_lorenz_hopf(py::getitem(&t2313, &Py::str("sigma"))?, py::getitem(&t2313, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2314 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2315 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2316 = (t2315).items()?;
if t2316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2316[0].clone();
v_v = t2316[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2346(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2348: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2347 = v_inputs.clone(); f_lotka_volterra(py::getitem(&t2347, &Py::str("a"))?, py::getitem(&t2347, &Py::str("b"))?, py::getitem(&t2347, &Py::str("c"))?, py::getitem(&t2347, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2348 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2349 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2350 = (t2349).items()?;
if t2350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2350[0].clone();
v_v = t2350[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2373(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    return Ok(f__one_step(Py::str("midpoint"), ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, v_t0.clone(), v_y0.clone(), v_h.clone(), py::int(&v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2396(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2398: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2397 = v_inputs.clone(); s_2373(py::getitem(&t2397, &Py::str("f"))?, py::getitem(&t2397, &Py::str("t0"))?, py::getitem(&t2397, &Py::str("y0"))?, py::getitem(&t2397, &Py::str("h"))?, py::getitem(&t2397, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2398 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2399 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2400 = (t2399).items()?;
if t2400.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2400[0].clone();
v_v = t2400[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2440(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2442: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2441 = v_inputs.clone(); f_mixing(py::getitem(&t2441, &Py::str("V"))?, py::getitem(&t2441, &Py::str("r"))?, py::getitem(&t2441, &Py::str("c_in"))?, py::getitem(&t2441, &Py::str("A0"))?, py::getitem(&t2441, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2442 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2443 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2444 = (t2443).items()?;
if t2444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2444[0].clone();
v_v = t2444[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_245(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t247: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t246 = v_inputs.clone(); f_bvp2(py::getitem(&t246, &Py::str("p"))?, py::getitem(&t246, &Py::str("q"))?, py::getitem(&t246, &Py::str("r"))?, py::getitem(&t246, &Py::str("xa"))?, py::getitem(&t246, &Py::str("xb"))?, py::getitem(&t246, &Py::str("ya"))?, py::getitem(&t246, &Py::str("yb"))?, py::getitem(&t246, &Py::str("x"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t247 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t248 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t249 = (t248).items()?;
if t249.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t249[0].clone();
v_v = t249[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2481(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2483: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2482 = v_inputs.clone(); f_cooling_k(py::getitem(&t2482, &Py::str("T0"))?, py::getitem(&t2482, &Py::str("Ta"))?, py::getitem(&t2482, &Py::str("T1"))?, py::getitem(&t2482, &Py::str("t1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2483 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2484 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2485 = (t2484).items()?;
if t2485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2485[0].clone();
v_v = t2485[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2516(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2518: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2517 = v_inputs.clone(); f_cooling_T(py::getitem(&t2517, &Py::str("T0"))?, py::getitem(&t2517, &Py::str("Ta"))?, py::getitem(&t2517, &Py::str("k"))?, py::getitem(&t2517, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2518 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2519 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2520 = (t2519).items()?;
if t2520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2520[0].clone();
v_v = t2520[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2559(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2561: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2560 = v_inputs.clone(); f_cooling_time(py::getitem(&t2560, &Py::str("T0"))?, py::getitem(&t2560, &Py::str("Ta"))?, py::getitem(&t2560, &Py::str("k"))?, py::getitem(&t2560, &Py::str("T1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2561 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2562 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2563 = (t2562).items()?;
if t2563.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2563[0].clone();
v_v = t2563[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2636(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2638: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2637 = v_inputs.clone(); f_jacobian_classify(py::getitem(&t2637, &Py::str("f"))?, py::getitem(&t2637, &Py::str("g"))?, py::getitem(&t2637, &Py::str("x0"))?, py::getitem(&t2637, &Py::str("y0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2638 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2639 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2640 = (t2639).items()?;
if t2640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2640[0].clone();
v_v = t2640[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2693(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2695: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2694 = v_inputs.clone(); f_impulse2(py::getitem(&t2694, &Py::str("a"))?, py::getitem(&t2694, &Py::str("b"))?, py::getitem(&t2694, &Py::str("c"))?, py::getitem(&t2694, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2695 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2696 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2697 = (t2696).items()?;
if t2697.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2697[0].clone();
v_v = t2697[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2758(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2760: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2759 = v_inputs.clone(); f_step2(py::getitem(&t2759, &Py::str("a"))?, py::getitem(&t2759, &Py::str("b"))?, py::getitem(&t2759, &Py::str("c"))?, py::getitem(&t2759, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2760 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2761 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2762 = (t2761).items()?;
if t2762.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2762[0].clone();
v_v = t2762[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2832(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_partial(v_i.clone(), v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2915(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2917: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2916 = v_inputs.clone(); f_phase_portrait(py::getitem(&t2916, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2917 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2918 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2919 = (t2918).items()?;
if t2919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2919[0].clone();
v_v = t2919[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2957(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2959: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2958 = v_inputs.clone(); f_practical_resonance(py::getitem(&t2958, &Py::str("m"))?, py::getitem(&t2958, &Py::str("c"))?, py::getitem(&t2958, &Py::str("k"))?, py::getitem(&t2958, &Py::str("F0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2959 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2960 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2961 = (t2960).items()?;
if t2961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2961[0].clone();
v_v = t2961[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3007(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3009: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3008 = v_inputs.clone(); f_drag_quadratic(py::getitem(&t3008, &Py::str("m"))?, py::getitem(&t3008, &Py::str("c"))?, py::getitem(&t3008, &Py::str("g"))?, py::getitem(&t3008, &Py::str("v0"))?, py::getitem(&t3008, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3009 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3011 = (t3010).items()?;
if t3011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3011[0].clone();
v_v = t3011[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_301(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t303: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t302 = v_inputs.clone(); f_brusselator(py::getitem(&t302, &Py::str("a"))?, py::getitem(&t302, &Py::str("b"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t303 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t304 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t305 = (t304).items()?;
if t305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t305[0].clone();
v_v = t305[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3045(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3047: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3046 = v_inputs.clone(); f_riccati_sq(py::getitem(&t3046, &Py::str("k"))?, py::getitem(&t3046, &Py::str("y0"))?, py::getitem(&t3046, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3047 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3048 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3049 = (t3048).items()?;
if t3049.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3049[0].clone();
v_v = t3049[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3083(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3086: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3085 = v_inputs.clone(); s_3084(py::getitem(&t3085, &Py::str("T"))?, py::getitem(&t3085, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3086 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3087 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3088 = (t3087).items()?;
if t3088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3088[0].clone();
v_v = t3088[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3084(mut v_T: Py, mut v_p: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::mul(&py::neg(&v_T.clone())?, &py::math1("log2", &v_p.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3135(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3137: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3136 = v_inputs.clone(); f_robin_eigen(py::getitem(&t3136, &Py::str("L"))?, py::getitem(&t3136, &Py::str("h"))?, py::getitem(&t3136, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3137 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3138 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3139 = (t3138).items()?;
if t3139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3139[0].clone();
v_v = t3139[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3200(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3202: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3201 = v_inputs.clone(); f_routh_hurwitz(py::getitem(&t3201, &Py::str("coeffs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3202 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3203 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3204 = (t3203).items()?;
if t3204.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3204[0].clone();
v_v = t3204[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3227(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    return Ok(f__one_step(Py::str("rk4"), ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, v_t0.clone(), v_y0.clone(), v_h.clone(), py::int(&v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3250(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3252: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3251 = v_inputs.clone(); s_3227(py::getitem(&t3251, &Py::str("f"))?, py::getitem(&t3251, &Py::str("t0"))?, py::getitem(&t3251, &Py::str("y0"))?, py::getitem(&t3251, &Py::str("h"))?, py::getitem(&t3251, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3252 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3253 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3254 = (t3253).items()?;
if t3254.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3254[0].clone();
v_v = t3254[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3300(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3302: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3301 = v_inputs.clone(); f_rk4_system(py::getitem(&t3301, &Py::str("f"))?, py::getitem(&t3301, &Py::str("g"))?, py::getitem(&t3301, &Py::str("t0"))?, py::getitem(&t3301, &Py::str("x0"))?, py::getitem(&t3301, &Py::str("y0"))?, py::getitem(&t3301, &Py::str("h"))?, py::getitem(&t3301, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3302 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3303 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3304 = (t3303).items()?;
if t3304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3304[0].clone();
v_v = t3304[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3375(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3377: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3376 = v_inputs.clone(); f_ivp2(py::getitem(&t3376, &Py::str("a"))?, py::getitem(&t3376, &Py::str("b"))?, py::getitem(&t3376, &Py::str("c"))?, py::getitem(&t3376, &Py::str("y0"))?, py::getitem(&t3376, &Py::str("v0"))?, py::getitem(&t3376, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3377 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3378 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3379 = (t3378).items()?;
if t3379.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3379[0].clone();
v_v = t3379[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3414 = v_inputs.clone(); f_separable_power(py::getitem(&t3414, &Py::str("a"))?, py::getitem(&t3414, &Py::str("n"))?, py::getitem(&t3414, &Py::str("y0"))?, py::getitem(&t3414, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3416 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3417 = (t3416).items()?;
if t3417.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3417[0].clone();
v_v = t3417[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3450(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3452: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3451 = v_inputs.clone(); f_shm(py::getitem(&t3451, &Py::str("m"))?, py::getitem(&t3451, &Py::str("k"))?, py::getitem(&t3451, &Py::str("x0"))?, py::getitem(&t3451, &Py::str("v0"))?, py::getitem(&t3451, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3452 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3453 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3454 = (t3453).items()?;
if t3454.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3454[0].clone();
v_v = t3454[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3507(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3509: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3508 = v_inputs.clone(); f_sir(py::getitem(&t3508, &Py::str("beta"))?, py::getitem(&t3508, &Py::str("gamma"))?, py::getitem(&t3508, &Py::str("S0"))?, py::getitem(&t3508, &Py::str("I0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3509 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3510 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3511 = (t3510).items()?;
if t3511.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3511[0].clone();
v_v = t3511[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3539(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3541: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3540 = v_inputs.clone(); f_slope_field(py::getitem(&t3540, &Py::str("f"))?, py::getitem(&t3540, &Py::str("t"))?, py::getitem(&t3540, &Py::str("y"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3541 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3542 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3543 = (t3542).items()?;
if t3543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3543[0].clone();
v_v = t3543[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3582(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3584: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3583 = v_inputs.clone(); f_steady_state(py::getitem(&t3583, &Py::str("a"))?, py::getitem(&t3583, &Py::str("b"))?, py::getitem(&t3583, &Py::str("c"))?, py::getitem(&t3583, &Py::str("F0"))?, py::getitem(&t3583, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3584 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3585 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3586 = (t3585).items()?;
if t3586.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3586[0].clone();
v_v = t3586[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3621(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3623: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3622 = v_inputs.clone(); f_sturm_liouville(py::getitem(&t3622, &Py::str("bc"))?, py::getitem(&t3622, &Py::str("L"))?, py::getitem(&t3622, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3623 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3624 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3625 = (t3624).items()?;
if t3625.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3625[0].clone();
v_v = t3625[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_365(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t367: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t366 = v_inputs.clone(); f_cauchy_euler(py::getitem(&t366, &Py::str("a"))?, py::getitem(&t366, &Py::str("b"))?, py::getitem(&t366, &Py::str("c"))?, py::getitem(&t366, &Py::str("y1"))?, py::getitem(&t366, &Py::str("v1"))?, py::getitem(&t366, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t367 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t368 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t369 = (t368).items()?;
if t369.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t369[0].clone();
v_v = t369[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3663(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3665: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3664 = v_inputs.clone(); f_torricelli(py::getitem(&t3664, &Py::str("h0"))?, py::getitem(&t3664, &Py::str("k"))?, py::getitem(&t3664, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3665 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3666 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3667 = (t3666).items()?;
if t3667.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3667[0].clone();
v_v = t3667[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3700(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3702: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3701 = v_inputs.clone(); f_pure_resonance(py::getitem(&t3701, &Py::str("m"))?, py::getitem(&t3701, &Py::str("k"))?, py::getitem(&t3701, &Py::str("F0"))?, py::getitem(&t3701, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3702 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3703 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3704 = (t3703).items()?;
if t3704.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3704[0].clone();
v_v = t3704[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3769(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3771: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3770 = v_inputs.clone(); f_uc_exp(py::getitem(&t3770, &Py::str("a"))?, py::getitem(&t3770, &Py::str("b"))?, py::getitem(&t3770, &Py::str("c"))?, py::getitem(&t3770, &Py::str("F"))?, py::getitem(&t3770, &Py::str("s"))?, py::getitem(&t3770, &Py::str("y0"))?, py::getitem(&t3770, &Py::str("v0"))?, py::getitem(&t3770, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3771 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3772 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3773 = (t3772).items()?;
if t3773.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3773[0].clone();
v_v = t3773[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3834(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3836: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3835 = v_inputs.clone(); f_uc_poly(py::getitem(&t3835, &Py::str("a"))?, py::getitem(&t3835, &Py::str("b"))?, py::getitem(&t3835, &Py::str("c"))?, py::getitem(&t3835, &Py::str("p"))?, py::getitem(&t3835, &Py::str("y0"))?, py::getitem(&t3835, &Py::str("v0"))?, py::getitem(&t3835, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3836 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3837 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3838 = (t3837).items()?;
if t3838.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3838[0].clone();
v_v = t3838[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3895(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3897: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3896 = v_inputs.clone(); f_wave_series(py::getitem(&t3896, &Py::str("f"))?, py::getitem(&t3896, &Py::str("L"))?, py::getitem(&t3896, &Py::str("c"))?, py::getitem(&t3896, &Py::str("N"))?, py::getitem(&t3896, &Py::str("x"))?, py::getitem(&t3896, &Py::str("t"))?, py::method(&t3896, "get", &[Py::str("g"), Py::str("0")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3897 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3898 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3899 = (t3898).items()?;
if t3899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3899[0].clone();
v_v = t3899[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_407(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t409: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t408 = v_inputs.clone(); f_cfl(py::getitem(&t408, &Py::str("c"))?, py::getitem(&t408, &Py::str("dt"))?, py::getitem(&t408, &Py::str("dx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t409 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t410 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t411 = (t410).items()?;
if t411.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t411[0].clone();
v_v = t411[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_469(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_char(v_i.clone(), v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_533(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t534 = v_inputs.clone(); f_competing_species(py::getitem(&t534, &Py::str("a1"))?, py::getitem(&t534, &Py::str("b11"))?, py::getitem(&t534, &Py::str("b12"))?, py::getitem(&t534, &Py::str("a2"))?, py::getitem(&t534, &Py::str("b21"))?, py::getitem(&t534, &Py::str("b22"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t535 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t536 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t537 = (t536).items()?;
if t537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t537[0].clone();
v_v = t537[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_573(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t575: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t574 = v_inputs.clone(); f_convolution(py::getitem(&t574, &Py::str("f"))?, py::getitem(&t574, &Py::str("g"))?, py::getitem(&t574, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t575 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t576 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t577 = (t576).items()?;
if t577.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t577[0].clone();
v_v = t577[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_623(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t625: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t624 = v_inputs.clone(); f_crank_nicolson(py::getitem(&t624, &Py::str("u"))?, py::getitem(&t624, &Py::str("r"))?, py::getitem(&t624, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t625 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t626 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t627 = (t626).items()?;
if t627.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t627[0].clone();
v_v = t627[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_671(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t673: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t672 = v_inputs.clone(); f_dalembert(py::getitem(&t672, &Py::str("f"))?, py::getitem(&t672, &Py::str("g"))?, py::getitem(&t672, &Py::str("c"))?, py::getitem(&t672, &Py::str("x"))?, py::getitem(&t672, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t673 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t674 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t675 = (t674).items()?;
if t675.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t675[0].clone();
v_v = t675[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_71(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t73: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t72 = v_inputs.clone(); f_equilibria_1d(py::getitem(&t72, &Py::str("f"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t73 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t74 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t75 = (t74).items()?;
if t75.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t75[0].clone();
v_v = t75[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_727(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t729: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t728 = v_inputs.clone(); f_oscillator_params(py::getitem(&t728, &Py::str("m"))?, py::getitem(&t728, &Py::str("c"))?, py::getitem(&t728, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t729 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t730 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t731 = (t730).items()?;
if t731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t731[0].clone();
v_v = t731[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_763(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t768: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t767 = v_inputs.clone(); s_764(py::getitem(&t767, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t768 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t769 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t770 = (t769).items()?;
if t770.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t770[0].clone();
v_v = t770[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_764(mut v_k: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("t_char"), f__r(py::truediv(&py::math1("log1p", &Py::Int(1i64))?, &py::abs(&v_k.clone())?)?)?), (Py::str("kind"), (if ({ let t765 = v_k.clone(); Py::Bool({ let t766 = Py::Int(0i64); if py::gt(&t765, &t766)? { true } else { false } }) }).truthy() { Py::str("doubling time") } else { Py::str("half-life") }))]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_813(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t815: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t814 = v_inputs.clone(); f_euler_error(py::getitem(&t814, &Py::str("k"))?, py::getitem(&t814, &Py::str("y0"))?, py::getitem(&t814, &Py::str("h"))?, py::getitem(&t814, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t815 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t816 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t817 = (t816).items()?;
if t817.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t817[0].clone();
v_v = t817[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_840(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    return Ok(f__one_step(Py::str("euler"), ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, v_t0.clone(), v_y0.clone(), v_h.clone(), py::int(&v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_863(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t865: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t864 = v_inputs.clone(); s_840(py::getitem(&t864, &Py::str("f"))?, py::getitem(&t864, &Py::str("t0"))?, py::getitem(&t864, &Py::str("y0"))?, py::getitem(&t864, &Py::str("h"))?, py::getitem(&t864, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t865 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t866 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t867 = (t866).items()?;
if t867.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t867[0].clone();
v_v = t867[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_905(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t906 = v_inputs.clone(); f_heat_stability(py::getitem(&t906, &Py::str("alpha"))?, py::getitem(&t906, &Py::str("dt"))?, py::getitem(&t906, &Py::str("dx"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t907 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t908 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t909 = (t908).items()?;
if t909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t909[0].clone();
v_v = t909[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_947(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t949: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t948 = v_inputs.clone(); f_step_limit(py::getitem(&t948, &Py::str("method"))?, py::getitem(&t948, &Py::str("lam"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t949 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t950 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t951 = (t950).items()?;
if t951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t951[0].clone();
v_v = t951[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_988(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t991: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t990 = v_inputs.clone(); s_989(py::getitem(&t990, &Py::str("t1"))?, py::getitem(&t990, &Py::str("y1"))?, py::getitem(&t990, &Py::str("t2"))?, py::getitem(&t990, &Py::str("y2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t991 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t992 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t993 = (t992).items()?;
if t993.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t993[0].clone();
v_v = t993[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_989(mut v_t1: Py, mut v_y1: Py, mut v_t2: Py, mut v_y2: Py) -> R {
    return Ok(py::call(&{ let v_t1 = v_t1.clone(); let v_t2 = v_t2.clone(); let v_y1 = v_y1.clone(); let v_y2 = v_y2.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_k: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::dict(vec![(Py::str("k"), f__r(v_k.clone())?), (Py::str("y0"), f__r(py::mul(&v_y2.clone(), &f__exp(py::mul(&py::neg(&v_k.clone())?, &v_t2.clone())?)?)?)?)]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, &[py::truediv(&py::sub(&py::math1("log", &py::abs(&v_y2.clone())?)?, &py::math1("log", &py::abs(&v_y1.clone())?)?)?, &py::sub(&v_t2.clone(), &v_t1.clone())?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}
