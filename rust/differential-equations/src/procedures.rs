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
        "linear_system_solution" => {
            let r = f_linear_system_solution(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1751(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "linear_system_stability" => {
            let r = f_linear_stability(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1844(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_harvesting" => {
            let r = f_harvesting(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1888(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_inflection" => {
            let r = f_logistic_inflection(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1929(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_map_fixed_points" => {
            let r = f_lm_fixed(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1974(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_map_lyapunov" => {
            let r = f_lm_lyapunov(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2037(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_map_two_cycle" => {
            let r = f_lm_cycle(match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2077(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "logistic_value" => {
            let r = f_logistic(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2118(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lorenz_equilibria" => {
            let r = f_lorenz_eq(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2165(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lorenz_hopf_threshold" => {
            let r = f_lorenz_hopf(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2205(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "lotka_volterra_equilibrium" => {
            let r = f_lotka_volterra(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2239(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "midpoint_method" => {
            let r = s_2266(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2289(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "mixing_tank" => {
            let r = f_mixing(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2333(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_cooling_constant" => {
            let r = f_cooling_k(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2374(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_cooling_temperature" => {
            let r = f_cooling_T(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2409(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "newton_cooling_time" => {
            let r = f_cooling_time(match v.get("T0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2452(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "nonlinear_equilibrium_type" => {
            let r = f_jacobian_classify(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ode_impulse_response" => {
            let r = f_impulse2(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2586(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ode_step_response" => {
            let r = f_step2(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2651(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "partial_fraction_residues" => {
            let r = f_partial_fractions(match v.get("F") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2725(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "phase_portrait_2x2" => {
            let r = f_phase_portrait(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2808(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "practical_resonance" => {
            let r = f_practical_resonance(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2850(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_drag_velocity" => {
            let r = f_drag_quadratic(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2900(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "quadratic_ode_blowup" => {
            let r = f_riccati_sq(match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2938(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "radiometric_age" => {
            let r = f_decay_age(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2976(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "robin_eigenvalues" => {
            let r = f_robin_eigen(match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3028(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "routh_hurwitz" => {
            let r = f_routh_hurwitz(match v.get("coeffs") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3093(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "runge_kutta_4" => {
            let r = s_3120(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3143(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "runge_kutta_4_system" => {
            let r = f_rk4_system(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3193(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "second_order_homogeneous_ivp" => {
            let r = f_ivp2(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3268(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "separable_power" => {
            let r = f_separable_power(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3306(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "simple_harmonic_motion" => {
            let r = f_shm(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3343(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sir_epidemic" => {
            let r = f_sir(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3400(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "slope_field" => {
            let r = f_slope_field(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3432(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "steady_state_amplitude" => {
            let r = f_steady_state(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3475(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "sturm_liouville_eigenvalues" => {
            let r = f_sturm_liouville(match v.get("bc") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3514(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "torricelli_draining" => {
            let r = f_torricelli(match v.get("h0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3556(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "undamped_resonance" => {
            let r = f_pure_resonance(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "undetermined_coefficients_exponential" => {
            let r = f_uc_exp(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3662(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "undetermined_coefficients_polynomial" => {
            let r = f_uc_poly(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("v0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3727(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "wave_equation_string_series" => {
            let r = f_wave_series(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("g") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("0")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3788(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    if ({ let t3705 = py::abs(&v_z.clone())?; Py::Bool({ let t3706 = Py::Int(1i64); if py::lt(&t3705, &t3706)? { true } else { false } }) }).truthy() {
        let t3707 = (Py::tuple(vec![Py::Float(1.0), v_t.clone(), Py::Float(1.0), v_t.clone(), Py::Int(1i64)])).items()?;
        if t3707.len() != 5 { return Err(PyErr::Value("unpack".into())); }
        v_C = t3707[0].clone();
        v_S = t3707[1].clone();
        v_tc = t3707[2].clone();
        v_ts = t3707[3].clone();
        v_k = t3707[4].clone();
        while (Py::Bool(true)).truthy() {
            v_tc = py::mul(&v_tc, &py::truediv(&v_z.clone(), &py::mul(&py::sub(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &py::mul(&Py::Int(2i64), &v_k.clone())?)?)?)?;
            v_ts = py::mul(&v_ts, &py::truediv(&v_z.clone(), &py::mul(&py::mul(&Py::Int(2i64), &v_k.clone())?, &py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?)?)?)?;
            let t3708 = (Py::tuple(vec![py::add(&v_C.clone(), &v_tc.clone())?, py::add(&v_S.clone(), &v_ts.clone())?, py::add(&v_k.clone(), &Py::Int(1i64))?])).items()?;
            if t3708.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_C = t3708[0].clone();
            v_S = t3708[1].clone();
            v_k = t3708[2].clone();
            if ({ let t3713 = { let t3709 = py::abs(&v_tc.clone())?; Py::Bool({ let t3710 = Py::Float(1e-18); if py::lt(&t3709, &t3710)? { true } else { false } }) }; if !t3713.truthy() { t3713 } else { { let t3711 = py::abs(&v_ts.clone())?; Py::Bool({ let t3712 = py::mul(&Py::Float(1e-18), &py::minmax(&[Py::Float(1.0), py::abs(&v_t.clone())?], true)?)?; if py::lt(&t3711, &t3712)? { true } else { false } }) } } }).truthy() {
                return Ok(Py::tuple(vec![v_C.clone(), v_S.clone()]));
            }
        }
    }
    let t3716: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            if ({ let t3714 = v_d2.clone(); Py::Bool({ let t3715 = Py::Int(0i64); if py::gt(&t3714, &t3715)? { true } else { false } }) }).truthy() {
                v_d = py::math1("sqrt", &v_d2.clone())?;
                return Ok(Some(Py::tuple(vec![py::math1("cosh", &py::mul(&v_d.clone(), &v_t.clone())?)?, py::truediv(&py::math1("sinh", &py::mul(&v_d.clone(), &v_t.clone())?)?, &v_d.clone())?])));
            }
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3716 {
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
    v_zs = { let mut out = vec![]; for t2706 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t2706;
out.push((if ({ let t2707 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2708 = py::mul(&Py::Float(1e-25), &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)?; if py::le(&t2707, &t2708)? { true } else { false } }) }).truthy() { py::complex(&[py::real(&v_z.clone())?, Py::Float(0.0)])? } else { v_z.clone() })); } Py::list(out) };
    v_real = py::sorted(&{ let mut out = vec![]; for t2709 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t2709;
if ({ let t2710 = py::imag(&v_z.clone())?; Py::Bool({ let t2711 = Py::Int(0i64); if py::eq(&t2710, &t2711) { true } else { false } }) }).truthy() { out.push(py::real(&v_z.clone())?); }} Py::list(out) }, (Py::Bool(false)).truthy())?;
    v_cplx = py::sorted_key(&{ let mut out = vec![]; for t2712 in py::iter(&(v_zs.clone()))? { let mut v_z: Py; 
v_z = t2712;
if ({ let t2713 = py::imag(&v_z.clone())?; Py::Bool({ let t2714 = Py::Int(0i64); if py::ne(&t2713, &t2714) { true } else { false } }) }).truthy() { out.push(v_z.clone()); }} Py::list(out) }, &{ let v_real = v_real.clone(); let v_zs = v_zs.clone(); py::func(move |a: &[Py]| -> R {
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
    v_M = { let mut out = vec![]; for t1787 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1787;
out.push({ let mut out = vec![]; for t1788 in py::iter(&(v_r.clone()))? { let mut v_v: Py; 
v_v = t1788;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }); } Py::list(out) };
    v_I = { let mut out = vec![]; for t1789 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1789;
out.push({ let mut out = vec![]; for t1790 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1790;
out.push(py::fraction(&[py::int(&{ let t1791 = v_i.clone(); Py::Bool({ let t1792 = v_j.clone(); if py::eq(&t1791, &t1792) { true } else { false } }) })?])?); } Py::list(out) }); } Py::list(out) };
    v_c = Py::list(vec![py::fraction(&[Py::Int(1i64)])?]);
    v_Mk = { let mut out = vec![]; for t1793 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t1793;
out.push(py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &v_n.clone())?); } Py::list(out) };
    for t1794 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t1794;
        v_Mk = { let mut out = vec![]; for t1795 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1795;
out.push({ let mut out = vec![]; for t1796 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1796;
out.push(py::add(&py::sum(&{ let mut out = vec![]; for t1797 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_l: Py; 
v_l = t1797;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_l.clone())?, &py::getitem(&py::getitem(&v_Mk.clone(), &v_l.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?, &py::mul(&py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&py::getitem(&v_I.clone(), &v_i.clone())?, &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
        v_AM = { let mut out = vec![]; for t1798 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1798;
out.push({ let mut out = vec![]; for t1799 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1799;
out.push(py::sum(&{ let mut out = vec![]; for t1800 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_l: Py; 
v_l = t1800;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_l.clone())?, &py::getitem(&py::getitem(&v_Mk.clone(), &v_l.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t1801 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1801;
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
    if ({ let t2768 = v_det.clone(); Py::Bool({ let t2769 = Py::Int(0i64); if py::lt(&t2768, &t2769)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("saddle"), Py::str("unstable")]));
    }
    if ({ let t2770 = v_det.clone(); Py::Bool({ let t2771 = Py::Int(0i64); if py::eq(&t2770, &t2771) { true } else { false } }) }).truthy() {
        if ({ let t2772 = v_tr.clone(); Py::Bool({ let t2773 = Py::Int(0i64); if py::lt(&t2772, &t2773)? { true } else { false } }) }).truthy() {
            return Ok(Py::tuple(vec![Py::str("degenerate (line of equilibria)"), Py::str("stable (not asymptotically)")]));
        }
        if ({ let t2774 = v_tr.clone(); Py::Bool({ let t2775 = Py::Int(0i64); if py::gt(&t2774, &t2775)? { true } else { false } }) }).truthy() {
            return Ok(Py::tuple(vec![Py::str("degenerate (line of equilibria)"), Py::str("unstable")]));
        }
        return Ok((if (v_zero.clone()).truthy() { Py::tuple(vec![Py::str("degenerate (every point an equilibrium)"), Py::str("stable (not asymptotically)")]) } else { Py::tuple(vec![Py::str("degenerate (shear)"), Py::str("unstable")]) }));
    }
    if ({ let t2776 = v_tr.clone(); Py::Bool({ let t2777 = Py::Int(0i64); if py::eq(&t2776, &t2777) { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str("center"), Py::str("stable (not asymptotically)")]));
    }
    let t2780 = ((if ({ let t2778 = v_tr.clone(); Py::Bool({ let t2779 = Py::Int(0i64); if py::lt(&t2778, &t2779)? { true } else { false } }) }).truthy() { Py::tuple(vec![Py::str("stable"), Py::str("asymptotically stable")]) } else { Py::tuple(vec![Py::str("unstable"), Py::str("unstable")]) })).items()?;
    if t2780.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_side = t2780[0].clone();
    v_stab = t2780[1].clone();
    if ({ let t2781 = v_disc.clone(); Py::Bool({ let t2782 = Py::Int(0i64); if py::gt(&t2781, &t2782)? { true } else { false } }) }).truthy() {
        return Ok(Py::tuple(vec![Py::str(&[py::format(&v_side.clone(), "")?, " node".to_string()].concat()), v_stab.clone()]));
    }
    if ({ let t2783 = v_disc.clone(); Py::Bool({ let t2784 = Py::Int(0i64); if py::lt(&t2783, &t2784)? { true } else { false } }) }).truthy() {
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
    if ({ let t3254 = v_a.clone(); Py::Bool({ let t3255 = Py::Int(0i64); if py::lt(&t3254, &t3255)? { true } else { false } }) }).truthy() {
        let t3256 = (Py::tuple(vec![py::neg(&v_a.clone())?, py::neg(&v_b.clone())?, py::neg(&v_c.clone())?])).items()?;
        if t3256.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3256[0].clone();
        v_b = t3256[1].clone();
        v_c = t3256[2].clone();
    }
    if (py::not(&{ let t3261 = { let t3257 = v_c.clone(); Py::Bool({ let t3258 = Py::Int(0i64); if py::gt(&t3257, &t3258)? { true } else { false } }) }; if !t3261.truthy() { t3261 } else { { let t3259 = v_b.clone(); Py::Bool({ let t3260 = Py::Int(0i64); if py::ge(&t3259, &t3260)? { true } else { false } }) } } })).truthy() {
        return Ok(Py::str("not a damped oscillator (needs a, c of one sign and b of that sign or 0)"));
    }
    if ({ let t3262 = v_b.clone(); Py::Bool({ let t3263 = Py::Int(0i64); if py::eq(&t3262, &t3263) { true } else { false } }) }).truthy() {
        return Ok(Py::str("undamped"));
    }
    return Ok((if ({ let t3264 = v_D.clone(); Py::Bool({ let t3265 = Py::Int(0i64); if py::gt(&t3264, &t3265)? { true } else { false } }) }).truthy() { Py::str("overdamped") } else { (if ({ let t3266 = v_D.clone(); Py::Bool({ let t3267 = Py::Int(0i64); if py::eq(&t3266, &t3267) { true } else { false } }) }).truthy() { Py::str("critically damped") } else { Py::str("underdamped") }) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t3793 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t3793.truthy() { t3793 } else { Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))) } }).truthy() {
        return Ok((if ({ let t3794 = v_a.clone(); Py::Bool({ let t3795 = v_b.clone(); if py::eq(&t3794, &t3795) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    if ({ let t3796 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t3796.truthy() { t3796 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t3800 = py::not(&{ let t3797 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t3797.truthy() { t3797 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t3800.truthy() { t3800 } else { { let t3798 = py::len(&v_a.clone())?; Py::Bool({ let t3799 = py::len(&v_b.clone())?; if py::ne(&t3798, &t3799) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(1.0));
        }
        return Ok({ let t = { let mut out = vec![]; for t3801 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3802 = (t3801).items()?;
if t3802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3802[0].clone();
v_y = t3802[1].clone();
out.push(f__diff(v_x.clone(), v_y.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    }
    if ({ let t3807 = { let t3803 = v_a.clone(); Py::Bool({ let t3804 = Py::None; if py::is(&t3803, &t3804) { true } else { false } }) }; if t3807.truthy() { t3807 } else { { let t3805 = v_b.clone(); Py::Bool({ let t3806 = Py::None; if py::is(&t3805, &t3806) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t3808 = v_a.clone(); Py::Bool({ let t3809 = v_b.clone(); if py::is(&t3808, &t3809) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(1.0) }));
    }
    let t3814: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_d = py::abs(&py::sub(&py::complex(&[v_a.clone()])?, &py::complex(&[v_b.clone()])?)?)?;
            if ({ let t3810 = v_d.clone(); Py::Bool({ let t3811 = v_d.clone(); if py::ne(&t3810, &t3811) { true } else { false } }) }).truthy() {
                return Ok(Some(Py::Float(1.0)));
            }
            return Ok(Some((if ({ let t3812 = v_d.clone(); Py::Bool({ let t3813 = Py::Float(1e-15); if py::le(&t3812, &t3813)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&py::complex(&[v_a.clone()])?)?, py::abs(&py::complex(&[v_b.clone()])?)?], true)?)? })));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3814 {
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
    let t3704: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            return Ok(Some(py::math1("exp", &v_x.clone())?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3704 {
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
    for t3754 in py::iter(&(v_vals.clone()))? {
        v_v = t3754;
        if (py::not(&{ let t3756 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3756.truthy() { t3756 } else { { let t3755 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3755.truthy() { t3755 } else { py::math1("isfinite", &v_v.clone())? } } } })).truthy() {
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
    v_big = py::sum(&{ let mut out = vec![]; for t3781 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3781;
out.push(py::abs(&v_p.clone())?); } Py::list(out) }, None)?;
    if ({ let t3782 = v_big.clone(); Py::Bool({ let t3783 = py::mul(&Py::Float(100000.0), &py::minmax(&[Py::Float(1.0), py::abs(&v_total.clone())?], true)?)?; if py::gt(&t3782, &t3783)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the result is lost to cancellation between large terms (ill-conditioned inputs)"))));
    }
    if ({ let t3784 = py::abs(&v_total.clone())?; Py::Bool({ let t3785 = py::mul(&Py::Float(1e-13), &v_big.clone())?; if py::lt(&t3784, &t3785)? { true } else { false } }) }).truthy() {
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
fn f__one_step(mut v_method: Py, mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_k1: Py = Py::None;
    let mut v_k2: Py = Py::None;
    let mut v_k3: Py = Py::None;
    let mut v_k4: Py = Py::None;
    let mut v_t: Py = Py::None;
    let mut v_y: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_t0.clone(), v_y0.clone(), v_h.clone()]))?;
    if ({ let t3124 = v_h.clone(); Py::Bool({ let t3125 = Py::Int(0i64); if py::le(&t3124, &t3125)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the step h must be positive"))));
    }
    if (py::not(&{ let t3126 = Py::Int(1i64); Py::Bool({ let t3127 = v_n.clone(); if py::le(&t3126, &t3127)? { { let t3128 = Py::Int(10000i64); if py::le(&t3127, &t3128)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the number of steps n must be between 1 and 10000"))));
    }
    v_y = v_y0.clone();
    for t3129 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3129;
        v_t = py::add(&v_t0.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?;
        if ({ let t3130 = v_method.clone(); Py::Bool({ let t3131 = Py::str("euler"); if py::eq(&t3130, &t3131) { true } else { false } }) }).truthy() {
            v_y = py::add(&v_y.clone(), &py::mul(&v_h.clone(), &py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?)?)?;
        } else {
            if ({ let t3132 = v_method.clone(); Py::Bool({ let t3133 = Py::str("heun"); if py::eq(&t3132, &t3133) { true } else { false } }) }).truthy() {
                v_k1 = py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?;
                v_k2 = py::call(&v_f.clone(), &[py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_k1.clone())?)?])?;
                v_y = py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&v_k1.clone(), &v_k2.clone())?)?, &Py::Int(2i64))?)?;
            } else {
                if ({ let t3134 = v_method.clone(); Py::Bool({ let t3135 = Py::str("midpoint"); if py::eq(&t3134, &t3135) { true } else { false } }) }).truthy() {
                    v_k1 = py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?;
                    v_y = py::add(&v_y.clone(), &py::mul(&v_h.clone(), &py::call(&v_f.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_k1.clone())?, &Py::Int(2i64))?)?])?)?)?;
                } else {
                    if ({ let t3136 = v_method.clone(); Py::Bool({ let t3137 = Py::str("rk4"); if py::eq(&t3136, &t3137) { true } else { false } }) }).truthy() {
                        v_k1 = py::call(&v_f.clone(), &[v_t.clone(), v_y.clone()])?;
                        v_k2 = py::call(&v_f.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_k1.clone())?, &Py::Int(2i64))?)?])?;
                        v_k3 = py::call(&v_f.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_k2.clone())?, &Py::Int(2i64))?)?])?;
                        v_k4 = py::call(&v_f.clone(), &[py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_k3.clone())?)?])?;
                        v_y = py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&py::add(&py::add(&v_k1.clone(), &py::mul(&Py::Int(2i64), &v_k2.clone())?)?, &py::mul(&Py::Int(2i64), &v_k3.clone())?)?, &v_k4.clone())?)?, &Py::Int(6i64))?)?;
                    }
                }
            }
        }
        if ({ let t3140 = py::not(&py::math1("isfinite", &v_y.clone())?); if t3140.truthy() { t3140 } else { { let t3138 = py::abs(&v_y.clone())?; Py::Bool({ let t3139 = Py::Float(1e+100); if py::gt(&t3138, &t3139)? { true } else { false } }) } } }).truthy() {
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
    for t2687 in py::iter(&(py::getitem(&py::method(&v_Q.clone(), "sqf_list", &[])?, &Py::Int(1i64))?))? {
        let t2692 = (t2687).items()?;
        if t2692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_q = t2692[0].clone();
        v_mlt = t2692[1].clone();
        v_co = { let mut out = vec![]; for t2688 in py::iter(&(py::method(&v_q.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2688;
out.push(py::truediv(&ax_engine::pymp::call("mpf", &[py::getattr(&v_c.clone(), "p")?], &[])?, &py::getattr(&v_c.clone(), "q")?)?); } Py::list(out) };
        v_rs = (if ({ let t2689 = py::method(&v_q.clone(), "degree", &[])?; Py::Bool({ let t2690 = Py::Int(1i64); if py::eq(&t2689, &t2690) { true } else { false } }) }).truthy() { Py::list(vec![py::truediv(&py::neg(&py::getitem(&v_co.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_co.clone(), &Py::Int(0i64))?)?]) } else { ax_engine::pymp::call("polyroots", &[v_co.clone()], &[(String::from("maxsteps"), Py::Int(500i64)), (String::from("extraprec"), Py::Int(300i64))])? });
        v_out = py::iadd(&v_out, &{ let mut out = vec![]; for t2691 in py::iter(&(v_rs.clone()))? { let mut v_r: Py; 
v_r = t2691;
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
    let t3770: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_P = ax_engine::pysym::call("Poly", &[ax_engine::pysym::call("expand", &[v_e.clone()], &[])?, c_XS()?], &[])?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3770 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "PolynomialError")) => {
            return Err(py::value_error(py::to_str(&Py::str("this capability needs f(x) to be a polynomial in x"))));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t3771 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t3771;
out.push(py::getattr(&v_c.clone(), "is_rational")?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial needs rational coefficients"))));
    }
    if ({ let t3772 = py::method(&v_P.clone(), "degree", &[])?; Py::Bool({ let t3773 = v_max_deg.clone(); if py::gt(&t3772, &t3773)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["polynomials up to degree ".to_string(), py::format(&v_max_deg.clone(), "")?, " are supported".to_string()].concat()))));
    }
    return Ok({ let mut out = vec![]; for t3774 in py::iter(&(py::reversed(&py::method(&v_P.clone(), "all_coeffs", &[])?)?))? { let mut v_c: Py; 
v_c = t3774;
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
    return Ok((if ({ let t3786 = v_v.clone(); Py::Bool({ let t3787 = Py::Int(0i64); if py::eq(&t3786, &t3787) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::float(&Py::str(&[py::format(&v_v.clone(), ".12g")?].concat()))? }));
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
    let t2678 = (ax_engine::pysym::call("fraction", &[ax_engine::pysym::call("cancel", &[ax_engine::pysym::call("together", &[v_e.clone()], &[])?], &[])?], &[])?).items()?;
    if t2678.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2678[0].clone();
    v_den = t2678[1].clone();
    let t2680: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2679 = (Py::tuple(vec![ax_engine::pysym::call("Poly", &[v_num.clone(), c_SS()?], &[])?, ax_engine::pysym::call("Poly", &[v_den.clone(), c_SS()?], &[])?])).items()?;
            if t2679.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_P = t2679[0].clone();
            v_Q = t2679[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2680 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(&e, PyErr::Other(c) if c.as_str() == "PolynomialError")) => {
            return Err(py::value_error(py::to_str(&Py::str("F(s) must be a rational function of s"))));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&Py::Bool(({ let mut out = vec![]; for t2681 in py::iter(&(py::add(&py::method(&v_P.clone(), "all_coeffs", &[])?, &py::method(&v_Q.clone(), "all_coeffs", &[])?)?))? { let mut v_c: Py; 
v_c = t2681;
out.push(py::getattr(&v_c.clone(), "is_rational")?); } Py::list(out) }).items()?.iter().all(|x| x.truthy())))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F(s) needs rational coefficients"))));
    }
    if (py::getattr(&v_P.clone(), "is_zero")?).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F(s) is zero"))));
    }
    if ({ let t2682 = py::method(&v_P.clone(), "degree", &[])?; Py::Bool({ let t2683 = py::method(&v_Q.clone(), "degree", &[])?; if py::ge(&t2682, &t2683)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("F(s) must be strictly proper (degree of numerator < degree of denominator)"))));
    }
    if ({ let t2684 = py::method(&v_Q.clone(), "degree", &[])?; Py::Bool({ let t2685 = Py::Int(10i64); if py::gt(&t2684, &t2685)? { true } else { false } }) }).truthy() {
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
    let t3225 = (Py::tuple(vec![py::fraction(&[v_a.clone()])?, py::fraction(&[v_b.clone()])?, py::fraction(&[v_c.clone()])?])).items()?;
    if t3225.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3225[0].clone();
    v_B = t3225[1].clone();
    v_Cc = t3225[2].clone();
    v_D = py::sub(&py::mul(&v_B.clone(), &v_B.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_A.clone())?, &v_Cc.clone())?)?;
    if ({ let t3226 = v_D.clone(); Py::Bool({ let t3227 = Py::Int(0i64); if py::gt(&t3226, &t3227)? { true } else { false } }) }).truthy() {
        v_sq = py::math1("sqrt", &v_D.clone())?;
        v_q = py::truediv(&py::neg(&py::add(&v_b.clone(), &py::copysign(&v_sq.clone(), &v_b.clone())?)?)?, &Py::Int(2i64))?;
        v_r = py::sorted(&Py::list(vec![py::truediv(&v_q.clone(), &v_a.clone())?, py::truediv(&v_c.clone(), &v_q.clone())?]), (Py::Bool(false)).truthy())?;
        return Ok(Py::tuple(vec![py::getitem(&v_r.clone(), &Py::Int(0i64))?, py::getitem(&v_r.clone(), &Py::Int(1i64))?, v_D.clone()]));
    }
    if ({ let t3228 = v_D.clone(); Py::Bool({ let t3229 = Py::Int(0i64); if py::eq(&t3228, &t3229) { true } else { false } }) }).truthy() {
        v_r = py::float(&py::truediv(&py::neg(&v_B.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?)?;
        return Ok(Py::tuple(vec![v_r.clone(), v_r.clone(), v_D.clone()]));
    }
    let t3230 = (Py::tuple(vec![py::add(&py::float(&py::truediv(&py::neg(&v_B.clone())?, &py::mul(&Py::Int(2i64), &v_A.clone())?)?)?, &Py::Float(0.0))?, py::truediv(&py::math1("sqrt", &py::neg(&v_D.clone())?)?, &py::mul(&Py::Int(2i64), &py::abs(&v_a.clone())?)?)?])).items()?;
    if t3230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t3230[0].clone();
    v_im = t3230[1].clone();
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
    v_co = { let mut out = vec![]; for t3060 in py::iter(&(v_co.clone()))? { let mut v_v: Py; 
v_v = t3060;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) };
    while ({ let t3063 = v_co.clone(); if !t3063.truthy() { t3063 } else { { let t3061 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t3062 = Py::Int(0i64); if py::eq(&t3061, &t3062) { true } else { false } }) } } }).truthy() {
        let _ = py::method(&v_co.clone(), "pop", &[Py::Int(0i64)])?;
    }
    if ({ let t3064 = py::len(&v_co.clone())?; Py::Bool({ let t3065 = Py::Int(2i64); if py::lt(&t3064, &t3065)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the polynomial needs degree >= 1"))));
    }
    if ({ let t3066 = py::getitem(&v_co.clone(), &Py::Int(0i64))?; Py::Bool({ let t3067 = Py::Int(0i64); if py::lt(&t3066, &t3067)? { true } else { false } }) }).truthy() {
        v_co = { let mut out = vec![]; for t3068 in py::iter(&(v_co.clone()))? { let mut v_v: Py; 
v_v = t3068;
out.push(py::neg(&v_v.clone())?); } Py::list(out) };
    }
    let t3069 = (Py::tuple(vec![py::slice(&v_co.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, py::slice(&v_co.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?])).items()?;
    if t3069.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r0 = t3069[0].clone();
    v_r1 = t3069[1].clone();
    v_rows = Py::list(vec![v_r0.clone(), py::add(&v_r1.clone(), &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&py::len(&v_r0.clone())?, &py::len(&v_r1.clone())?)?)?)?]);
    v_n = py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?;
    for t3070 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t3070;
        let t3071 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t3071.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3071[0].clone();
        v_b = t3071[1].clone();
        if ({ let t3072 = py::getitem(&v_b.clone(), &Py::Int(0i64))?; Py::Bool({ let t3073 = Py::Int(0i64); if py::eq(&t3072, &t3073) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the Routh array has a zero in its first column (a root on or symmetric about the imaginary axis): this version decides regular cases only"))));
        }
        v_new = py::add(&{ let mut out = vec![]; for t3074 in py::range_iter(&[(py::sub(&py::len(&v_a.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t3074;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![py::fraction(&[Py::Int(0i64)])?]))?;
        let _ = py::method(&v_rows.clone(), "append", &[v_new.clone()])?;
    }
    v_first = { let mut out = vec![]; for t3075 in py::iter(&(py::slice(&v_rows.clone(), None, Some((py::add(&v_n.clone(), &Py::Int(1i64))?).i()?), None)?))? { let mut v_r: Py; 
v_r = t3075;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    if (Py::Bool(({ let mut out = vec![]; for t3076 in py::iter(&(v_first.clone()))? { let mut v_v: Py; 
v_v = t3076;
out.push({ let t3077 = v_v.clone(); Py::Bool({ let t3078 = Py::Int(0i64); if py::eq(&t3077, &t3078) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the Routh array has a zero in its first column: this version decides regular cases only"))));
    }
    v_changes = py::sum(&{ let mut out = vec![]; for t3079 in py::iter(&(py::zip(&[v_first.clone(), py::slice(&v_first.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t3080 = (t3079).items()?;
if t3080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t3080[0].clone();
v_y = t3080[1].clone();
if ({ let t3081 = { let t3083 = v_x.clone(); Py::Bool({ let t3084 = Py::Int(0i64); if py::gt(&t3083, &t3084)? { true } else { false } }) }; Py::Bool({ let t3082 = { let t3085 = v_y.clone(); Py::Bool({ let t3086 = Py::Int(0i64); if py::gt(&t3085, &t3086)? { true } else { false } }) }; if py::ne(&t3081, &t3082) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
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
    let t3777 = (Py::tuple(vec![Py::list(vec![py::truediv(&py::sub(&Py::Int(1i64), &v_sgn.clone())?, &v_w.clone())?]), Py::list(vec![Py::Float(0.0)])])).items()?;
    if t3777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_S = t3777[0].clone();
    v_C = t3777[1].clone();
    for t3778 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::len(&v_co.clone())?).i()?]) {
        v_k = t3778;
        let _ = py::method(&v_S.clone(), "append", &[py::add(&py::truediv(&py::mul(&py::neg(&py::pow(&v_L.clone(), &v_k.clone())?)?, &v_sgn.clone())?, &v_w.clone())?, &py::mul(&py::truediv(&v_k.clone(), &v_w.clone())?, &py::getitem(&v_C.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?)?])?;
        let _ = py::method(&v_C.clone(), "append", &[py::mul(&py::truediv(&py::neg(&v_k.clone())?, &v_w.clone())?, &py::getitem(&v_S.clone(), &py::sub(&v_k.clone(), &Py::Int(1i64))?)?)?])?;
    }
    v_terms = { let mut out = vec![]; for t3779 in py::iter(&(py::zip(&[v_co.clone(), v_S.clone()])?))? { let mut v_a: Py; let mut v_s: Py; 
let t3780 = (t3779).items()?;
if t3780.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3780[0].clone();
v_s = t3780[1].clone();
out.push(py::mul(&v_a.clone(), &v_s.clone())?); } Py::list(out) };
    return Ok((if (Py::Bool((v_terms.clone()).items()?.iter().any(|x| x.truthy()))).truthy() { py::mul(&py::truediv(&Py::Int(2i64), &v_L.clone())?, &f__guard(py::sum(&v_terms.clone(), None)?, { let mut v = vec![]; v.extend((v_terms.clone()).items()?); Py::tuple(v) })?)? } else { Py::Float(0.0) }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__stab_word(mut v_m: Py) -> R {
    return Ok((if ({ let t2073 = py::abs(&v_m.clone())?; Py::Bool({ let t2074 = Py::Int(1i64); if py::lt(&t2073, &t2074)? { true } else { false } }) }).truthy() { Py::str("stable") } else { (if ({ let t2075 = py::abs(&v_m.clone())?; Py::Bool({ let t2076 = Py::Int(1i64); if py::gt(&t2075, &t2076)? { true } else { false } }) }).truthy() { Py::str("unstable") } else { Py::str("neutral") }) }));
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
    let t2759: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t2726 = (f__ratfun(py::getitem(&v_i.clone(), &Py::str("F"))?)?).items()?;
            if t2726.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_P = t2726[0].clone();
            v_Q = t2726[1].clone();
            let t2729 = (Py::tuple(vec![{ let mut out = vec![]; for t2727 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2727;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }, { let mut out = vec![]; for t2728 in py::iter(&(py::method(&v_Q.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2728;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }])).items()?;
            if t2729.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_Pc = t2729[0].clone();
            v_Qc = t2729[1].clone();
            v_ev = { let v_P = v_P.clone(); let v_Pc = v_Pc.clone(); let v_Q = v_Q.clone(); let v_Qc = v_Qc.clone(); let v_i = v_i.clone(); let v_o = v_o.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_co: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2730 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_k: Py; 
let t2731 = (t2730).items()?;
if t2731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2731[0].clone();
v_c = t2731[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_z.clone(), &py::sub(&py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?, &v_k.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
            v_worst = Py::Float(0.0);
            for t2732 in py::iter(&(Py::tuple(vec![py::complex(&[Py::Float(0.37), Py::Float(1.91)])?, py::complex(&[py::neg(&Py::Float(2.3))?, Py::Float(0.7)])?, py::complex(&[Py::Float(5.1), py::neg(&Py::Float(0.4))?])?])))? {
                v_z = t2732;
                v_want = py::truediv(&py::call(&v_ev.clone(), &[v_Pc.clone(), v_z.clone()])?, &py::call(&v_ev.clone(), &[v_Qc.clone(), v_z.clone()])?)?;
                v_got = py::sum(&{ let mut out = vec![]; for t2733 in py::iter(&(py::zip(&[py::getitem(&v_o.clone(), &Py::str("poles"))?, py::getitem(&v_o.clone(), &Py::str("residues"))?])?))? { let mut v_p: Py; let mut v_r: Py; 
let t2734 = (t2733).items()?;
if t2734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t2734[0].clone();
v_r = t2734[1].clone();
out.push(py::truediv(&py::complex(&[v_r.clone()])?, &py::sub(&v_z.clone(), &py::complex(&[v_p.clone()])?)?)?); } Py::list(out) }, None)?;
                v_worst = py::minmax(&[v_worst.clone(), py::truediv(&py::abs(&py::sub(&v_got.clone(), &v_want.clone())?)?, &py::minmax(&[py::abs(&v_want.clone())?, Py::Float(1e-300)], true)?)?], true)?;
            }
            return Ok(Some(py::minmax(&[(if ({ let t2735 = v_worst.clone(); Py::Bool({ let t2736 = Py::Float(5e-11); if py::gt(&t2735, &t2736)? { true } else { false } }) }).truthy() { v_worst.clone() } else { Py::Float(0.0) }), f__diff(py::method(&v_o.clone(), "get", &[Py::str("count")])?, py::len(&py::getitem(&v_o.clone(), &Py::str("poles"))?)?)?, f__diff(py::len(&py::getitem(&v_o.clone(), &Py::str("poles"))?)?, py::method(&v_Q.clone(), "degree", &[])?)?], true)?));
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2759 {
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
    if ({ let t2404 = v_k.clone(); Py::Bool({ let t2405 = Py::Int(0i64); if py::le(&t2404, &t2405)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cooling constant k must be positive"))));
    }
    return Ok(Py::dict(vec![(Py::str("T"), f__r(py::add(&v_Ta.clone(), &py::mul(&py::sub(&v_T0.clone(), &v_Ta.clone())?, &f__exp(py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cooling_k(mut v_T0: Py, mut v_Ta: Py, mut v_T1: Py, mut v_t1: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_T0.clone(), v_Ta.clone(), v_T1.clone(), v_t1.clone()]))?;
    if ({ let t2371 = { let t2363 = v_t1.clone(); Py::Bool({ let t2364 = Py::Int(0i64); if py::le(&t2363, &t2364)? { true } else { false } }) }; if t2371.truthy() { t2371 } else { { let t2370 = { let t2365 = v_T0.clone(); Py::Bool({ let t2366 = v_Ta.clone(); if py::eq(&t2365, &t2366) { true } else { false } }) }; if t2370.truthy() { t2370 } else { py::not(&{ let t2367 = py::minmax(&[v_T0.clone(), v_Ta.clone()], false)?; Py::Bool({ let t2368 = v_T1.clone(); if py::lt(&t2367, &t2368)? { { let t2369 = py::minmax(&[v_T0.clone(), v_Ta.clone()], true)?; if py::lt(&t2368, &t2369)? { true } else { false } } } else { false } }) }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs t1 > 0 and T1 strictly between Ta and T0"))));
    }
    return Ok(Py::dict(vec![(Py::str("k"), f__r(py::truediv(&py::math1("log", &py::truediv(&py::sub(&v_T0.clone(), &v_Ta.clone())?, &py::sub(&v_T1.clone(), &v_Ta.clone())?)?)?, &v_t1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cooling_time(mut v_T0: Py, mut v_Ta: Py, mut v_k: Py, mut v_T1: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_T0.clone(), v_Ta.clone(), v_k.clone(), v_T1.clone()]))?;
    if ({ let t2439 = v_k.clone(); Py::Bool({ let t2440 = Py::Int(0i64); if py::le(&t2439, &t2440)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the cooling constant k must be positive"))));
    }
    if ({ let t2449 = { let t2441 = v_T0.clone(); Py::Bool({ let t2442 = v_Ta.clone(); if py::eq(&t2441, &t2442) { true } else { false } }) }; if t2449.truthy() { t2449 } else { { let t2448 = py::not(&{ let t2443 = py::minmax(&[v_T0.clone(), v_Ta.clone()], false)?; Py::Bool({ let t2444 = v_T1.clone(); if py::lt(&t2443, &t2444)? { { let t2445 = py::minmax(&[v_T0.clone(), v_Ta.clone()], true)?; if py::le(&t2444, &t2445)? { true } else { false } } } else { false } }) }); if t2448.truthy() { t2448 } else { { let t2446 = py::truediv(&py::sub(&v_T1.clone(), &v_Ta.clone())?, &py::sub(&v_T0.clone(), &v_Ta.clone())?)?; Py::Bool({ let t2447 = Py::Int(0i64); if py::le(&t2446, &t2447)? { true } else { false } }) } } } } }).truthy() {
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
    if ({ let t2973 = { let t2968 = v_T.clone(); Py::Bool({ let t2969 = Py::Int(0i64); if py::le(&t2968, &t2969)? { true } else { false } }) }; if t2973.truthy() { t2973 } else { py::not(&{ let t2970 = Py::Int(0i64); Py::Bool({ let t2971 = v_p.clone(); if py::lt(&t2970, &t2971)? { { let t2972 = Py::Int(1i64); if py::le(&t2971, &t2972)? { true } else { false } } } else { false } }) }) } }).truthy() {
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
    if ({ let t2893 = { let t2880 = v_m.clone(); Py::Bool({ let t2881 = Py::Int(0i64); if py::le(&t2880, &t2881)? { true } else { false } }) }; if t2893.truthy() { t2893 } else { { let t2892 = { let t2882 = v_c.clone(); Py::Bool({ let t2883 = Py::Int(0i64); if py::le(&t2882, &t2883)? { true } else { false } }) }; if t2892.truthy() { t2892 } else { { let t2891 = { let t2884 = v_g.clone(); Py::Bool({ let t2885 = Py::Int(0i64); if py::le(&t2884, &t2885)? { true } else { false } }) }; if t2891.truthy() { t2891 } else { { let t2890 = { let t2886 = v_v0.clone(); Py::Bool({ let t2887 = Py::Int(0i64); if py::lt(&t2886, &t2887)? { true } else { false } }) }; if t2890.truthy() { t2890 } else { { let t2888 = v_t.clone(); Py::Bool({ let t2889 = Py::Int(0i64); if py::lt(&t2888, &t2889)? { true } else { false } }) } } } } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m, c, g > 0, v0 >= 0 and t >= 0"))));
    }
    v_vt = py::math1("sqrt", &py::truediv(&py::mul(&v_m.clone(), &v_g.clone())?, &v_c.clone())?)?;
    v_s = py::truediv(&py::mul(&v_g.clone(), &v_t.clone())?, &v_vt.clone())?;
    if ({ let t2894 = v_v0.clone(); Py::Bool({ let t2895 = v_vt.clone(); if py::eq(&t2894, &t2895) { true } else { false } }) }).truthy() {
        v_v = v_vt.clone();
    } else {
        if ({ let t2896 = v_v0.clone(); Py::Bool({ let t2897 = v_vt.clone(); if py::lt(&t2896, &t2897)? { true } else { false } }) }).truthy() {
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
    if ({ let t1881 = { let t1874 = v_r.clone(); Py::Bool({ let t1875 = Py::Int(0i64); if py::le(&t1874, &t1875)? { true } else { false } }) }; if t1881.truthy() { t1881 } else { { let t1880 = { let t1876 = v_K.clone(); Py::Bool({ let t1877 = Py::Int(0i64); if py::le(&t1876, &t1877)? { true } else { false } }) }; if t1880.truthy() { t1880 } else { { let t1878 = v_H.clone(); Py::Bool({ let t1879 = Py::Int(0i64); if py::lt(&t1878, &t1879)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs r > 0, K > 0 and H >= 0"))));
    }
    v_Hc = py::truediv(&py::mul(&v_r.clone(), &v_K.clone())?, &Py::Int(4i64))?;
    if ({ let t1882 = v_H.clone(); Py::Bool({ let t1883 = v_Hc.clone(); if py::gt(&t1882, &t1883)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["no equilibrium: the harvest exceeds the maximum sustainable yield rK/4 = ".to_string(), py::format(&v_Hc.clone(), "g")?, " (collapse)".to_string()].concat()))));
    }
    if ({ let t1884 = v_H.clone(); Py::Bool({ let t1885 = v_Hc.clone(); if py::eq(&t1884, &t1885) { true } else { false } }) }).truthy() {
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
    let t3717 = (f__CS(v_d2.clone(), v_t.clone())?).items()?;
    if t3717.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_C = t3717[0].clone();
    v_S = t3717[1].clone();
    let t3718 = (Py::tuple(vec![py::mul(&v_y0.clone(), &v_C.clone())?, py::mul(&py::sub(&v_v0.clone(), &py::mul(&v_m.clone(), &v_y0.clone())?)?, &v_S.clone())?])).items()?;
    if t3718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p1 = t3718[0].clone();
    v_p2 = t3718[1].clone();
    return Ok(py::mul(&v_E.clone(), &f__guard(py::add(&v_p1.clone(), &v_p2.clone())?, Py::tuple(vec![v_p1.clone(), v_p2.clone()]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_impulse2(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_t.clone()]))?;
    if ({ let t2563 = { let t2559 = v_a.clone(); Py::Bool({ let t2560 = Py::Int(0i64); if py::eq(&t2559, &t2560) { true } else { false } }) }; if t2563.truthy() { t2563 } else { { let t2561 = v_t.clone(); Py::Bool({ let t2562 = Py::Int(0i64); if py::lt(&t2561, &t2562)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3223 = v_a.clone(); Py::Bool({ let t3224 = Py::Int(0i64); if py::eq(&t3223, &t3224) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0"))));
    }
    let t3231 = (f__roots2(v_a.clone(), v_b.clone(), v_c.clone())?).items()?;
    if t3231.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t3231[0].clone();
    v_r2 = t3231[1].clone();
    v_D = t3231[2].clone();
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
    let t2482 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:parse_text", &[v_f.clone(), Py::tuple(vec![Py::str("x"), Py::str("y")])], &[])?, ax_engine::pysym::helper("differential_equations:parse_text", &[v_g.clone(), Py::tuple(vec![Py::str("x"), Py::str("y")])], &[])?])).items()?;
    if t2482.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t2482[0].clone();
    v_G = t2482[1].clone();
    v_pt = Py::dict(vec![(c_XS()?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_x0.clone())?))], &[])?), (c_YS()?, ax_engine::pysym::call("Rational", &[Py::str(&py::repr(&py::float(&v_y0.clone())?))], &[])?)]);
    for t2483 in py::iter(&(Py::tuple(vec![v_F.clone(), v_G.clone()])))? {
        v_h = t2483;
        v_v = ax_engine::pysym::call("N", &[py::method(&v_h.clone(), "subs", &[v_pt.clone()])?, Py::Int(40i64)], &[])?;
        if ({ let t2484 = py::abs(&py::complex(&[v_v.clone()])?)?; Py::Bool({ let t2485 = Py::Float(1e-12); if py::gt(&t2484, &t2485)? { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the point is not an equilibrium (f and g do not both vanish there)"))));
        }
    }
    v_J = { let mut out = vec![]; for t2486 in py::iter(&(Py::tuple(vec![v_F.clone(), v_G.clone()])))? { let mut v_h: Py; 
v_h = t2486;
out.push({ let mut out = vec![]; for t2487 in py::iter(&(Py::tuple(vec![c_XS()?, c_YS()?])))? { let mut v_v: Py; 
v_v = t2487;
out.push(py::method(&ax_engine::pysym::call("diff", &[v_h.clone(), v_v.clone()], &[])?, "subs", &[v_pt.clone()])?); } Py::list(out) }); } Py::list(out) };
    let t2488 = (Py::tuple(vec![ax_engine::pysym::call("nsimplify", &[ax_engine::pysym::call("N", &[py::add(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, Py::Int(50i64)], &[])?], &[(String::from("rational"), Py::Bool(true)), (String::from("tolerance"), Py::Float(1e-40))])?, ax_engine::pysym::call("nsimplify", &[ax_engine::pysym::call("N", &[py::sub(&py::mul(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?)?, &py::mul(&py::getitem(&py::getitem(&v_J.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, &py::getitem(&py::getitem(&v_J.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?)?)?, Py::Int(50i64)], &[])?], &[(String::from("rational"), Py::Bool(true)), (String::from("tolerance"), Py::Float(1e-40))])?])).items()?;
    if t2488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t2488[0].clone();
    v_det = t2488[1].clone();
    let t2489 = (Py::tuple(vec![py::fraction(&[py::int(&py::getattr(&v_tr.clone(), "p")?)?, py::int(&py::getattr(&v_tr.clone(), "q")?)?])?, py::fraction(&[py::int(&py::getattr(&v_det.clone(), "p")?)?, py::int(&py::getattr(&v_det.clone(), "q")?)?])?])).items()?;
    if t2489.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_trF = t2489[0].clone();
    v_detF = t2489[1].clone();
    v_Jf = { let mut out = vec![]; for t2490 in py::iter(&(v_J.clone()))? { let mut v_row: Py; 
v_row = t2490;
out.push({ let mut out = vec![]; for t2491 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t2491;
out.push(py::float(&ax_engine::pysym::call("N", &[v_v.clone(), Py::Int(30i64)], &[])?)?); } Py::list(out) }); } Py::list(out) };
    let t2521 = (f__classify(v_trF.clone(), v_detF.clone(), { let t2516 = { let t2509 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?; Py::Bool({ let t2510 = Py::Int(0i64); if py::eq(&t2509, &t2510) { true } else { false } }) }; if !t2516.truthy() { t2516 } else { { let t2515 = { let t2511 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?; Py::Bool({ let t2512 = Py::Int(0i64); if py::eq(&t2511, &t2512) { true } else { false } }) }; if !t2515.truthy() { t2515 } else { { let t2513 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?; Py::Bool({ let t2514 = py::getitem(&py::getitem(&v_Jf.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?; if py::eq(&t2513, &t2514) { true } else { false } }) } } } } }, Py::Bool(({ let mut out = vec![]; for t2517 in py::iter(&(v_Jf.clone()))? { let mut v_row: Py; 
v_row = t2517;
for t2518 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t2518;
out.push({ let t2519 = v_v.clone(); Py::Bool({ let t2520 = Py::Int(0i64); if py::eq(&t2519, &t2520) { true } else { false } }) }); }} Py::list(out) }).items()?.iter().all(|x| x.truthy())))?).items()?;
    if t2521.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kind = t2521[0].clone();
    v_stab = t2521[1].clone();
    if ({ let t2524 = { let t2522 = v_kind.clone(); Py::Bool({ let t2523 = Py::str("center"); if py::eq(&t2522, &t2523) { true } else { false } }) }; if t2524.truthy() { t2524 } else { py::method(&v_kind.clone(), "startswith", &[Py::str("degenerate")])? } }).truthy() {
        v_stab = Py::str("linearization inconclusive");
    }
    return Ok(Py::dict(vec![(Py::str("type"), v_kind.clone()), (Py::str("stability"), v_stab.clone()), (Py::str("jacobian"), { let mut out = vec![]; for t2525 in py::iter(&(v_Jf.clone()))? { let mut v_row: Py; 
v_row = t2525;
out.push({ let mut out = vec![]; for t2526 in py::iter(&(v_row.clone()))? { let mut v_v: Py; 
v_v = t2526;
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
fn f_linear_stability(mut v_A: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    if ({ let t1786 = { let t1778 = v_n.clone(); Py::Bool({ let t1779 = Py::Int(1i64); if py::lt(&t1778, &t1779)? { true } else { false } }) }; if t1786.truthy() { t1786 } else { { let t1785 = { let t1780 = v_n.clone(); Py::Bool({ let t1781 = Py::Int(8i64); if py::gt(&t1780, &t1781)? { true } else { false } }) }; if t1785.truthy() { t1785 } else { Py::Bool(({ let mut out = vec![]; for t1782 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t1782;
out.push({ let t1783 = py::len(&v_r.clone())?; Py::Bool({ let t1784 = v_n.clone(); if py::ne(&t1783, &t1784) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be square, of size 1 to 8"))));
    }
    v_c = f__charpoly(v_A.clone())?;
    if ({ let t1802 = v_n.clone(); Py::Bool({ let t1803 = Py::Int(1i64); if py::eq(&t1802, &t1803) { true } else { false } }) }).truthy() {
        v_lam = py::neg(&py::getitem(&v_c.clone(), &Py::Int(1i64))?)?;
        if ({ let t1804 = v_lam.clone(); Py::Bool({ let t1805 = Py::Int(0i64); if py::eq(&t1804, &t1805) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("an eigenvalue lies on the imaginary axis: linear stability is not decided by eigenvalue signs"))));
        }
        v_k = py::int(&{ let t1806 = v_lam.clone(); Py::Bool({ let t1807 = Py::Int(0i64); if py::gt(&t1806, &t1807)? { true } else { false } }) })?;
        return Ok(Py::dict(vec![(Py::str("stable"), { let t1808 = v_k.clone(); Py::Bool({ let t1809 = Py::Int(0i64); if py::eq(&t1808, &t1809) { true } else { false } }) }), (Py::str("unstable_eigenvalues"), v_k.clone()), (Py::str("char_poly"), { let mut out = vec![]; for t1810 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t1810;
out.push(f__r(py::float(&v_v.clone())?)?); } Py::list(out) })]));
    }
    let t1840 = (f__routh(v_c.clone())?).items()?;
    if t1840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t1840[0].clone();
    v_k = t1840[1].clone();
    return Ok(Py::dict(vec![(Py::str("stable"), { let t1841 = v_k.clone(); Py::Bool({ let t1842 = Py::Int(0i64); if py::eq(&t1841, &t1842) { true } else { false } }) }), (Py::str("unstable_eigenvalues"), v_k.clone()), (Py::str("char_poly"), { let mut out = vec![]; for t1843 in py::iter(&(v_c.clone()))? { let mut v_v: Py; 
v_v = t1843;
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
    if (py::not(&{ let t2067 = Py::Int(3i64); Py::Bool({ let t2068 = v_r.clone(); if py::lt(&t2067, &t2068)? { { let t2069 = Py::Int(4i64); if py::le(&t2068, &t2069)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a 2-cycle exists for 3 < r <= 4"))));
    }
    v_s = py::math1("sqrt", &py::mul(&py::sub(&v_r.clone(), &Py::Int(3i64))?, &py::add(&v_r.clone(), &Py::Int(1i64))?)?)?;
    let t2070 = (Py::tuple(vec![py::truediv(&py::sub(&py::add(&v_r.clone(), &Py::Int(1i64))?, &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_r.clone())?)?, py::truediv(&py::add(&py::add(&v_r.clone(), &Py::Int(1i64))?, &v_s.clone())?, &py::mul(&Py::Int(2i64), &v_r.clone())?)?])).items()?;
    if t2070.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_p = t2070[0].clone();
    v_q = t2070[1].clone();
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
    if (py::not(&{ let t1959 = Py::Int(0i64); Py::Bool({ let t1960 = v_r.clone(); if py::lt(&t1959, &t1960)? { { let t1961 = Py::Int(4i64); if py::le(&t1960, &t1961)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the logistic map x -> r x (1 - x) needs 0 < r <= 4"))));
    }
    let t1962 = (Py::tuple(vec![Py::list(vec![Py::Float(0.0)]), Py::list(vec![v_r.clone()])])).items()?;
    if t1962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pts = t1962[0].clone();
    v_mult = t1962[1].clone();
    if ({ let t1963 = v_r.clone(); Py::Bool({ let t1964 = Py::Int(1i64); if py::gt(&t1963, &t1964)? { true } else { false } }) }).truthy() {
        let _ = py::method(&v_pts.clone(), "append", &[py::sub(&Py::Int(1i64), &py::truediv(&Py::Int(1i64), &v_r.clone())?)?])?;
        let _ = py::method(&v_mult.clone(), "append", &[py::sub(&Py::Int(2i64), &v_r.clone())?])?;
    }
    return Ok(Py::dict(vec![(Py::str("fixed_points"), { let mut out = vec![]; for t1965 in py::iter(&(v_pts.clone()))? { let mut v_p: Py; 
v_p = t1965;
out.push(f__r(v_p.clone())?); } Py::list(out) }), (Py::str("multipliers"), { let mut out = vec![]; for t1968 in py::iter(&(v_mult.clone()))? { let mut v_m: Py; 
v_m = t1968;
out.push(f__r(v_m.clone())?); } Py::list(out) }), (Py::str("stability"), { let mut out = vec![]; for t1969 in py::iter(&(v_mult.clone()))? { let mut v_m: Py; 
v_m = t1969;
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
    if ({ let t2010 = py::not(&{ let t2004 = Py::Int(0i64); Py::Bool({ let t2005 = v_r.clone(); if py::lt(&t2004, &t2005)? { { let t2006 = Py::Int(4i64); if py::le(&t2005, &t2006)? { true } else { false } } } else { false } }) }); if t2010.truthy() { t2010 } else { py::not(&{ let t2007 = Py::Int(0i64); Py::Bool({ let t2008 = v_x0.clone(); if py::lt(&t2007, &t2008)? { { let t2009 = Py::Int(1i64); if py::lt(&t2008, &t2009)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs 0 < r <= 4 and 0 < x0 < 1"))));
    }
    if ({ let t2016 = { let t2011 = v_n.clone(); Py::Bool({ let t2012 = Py::Int(2i64); if py::lt(&t2011, &t2012)? { true } else { false } }) }; if t2016.truthy() { t2016 } else { { let t2015 = { let t2013 = v_n.clone(); Py::Bool({ let t2014 = Py::Int(1000000i64); if py::gt(&t2013, &t2014)? { true } else { false } }) }; if t2015.truthy() { t2015 } else { py::modulo(&v_n.clone(), &Py::Int(2i64))? } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("n must be an even number of iterations between 2 and 1000000 (whole cycles are averaged)"))));
    }
    let t2017 = (Py::tuple(vec![v_x0.clone(), Py::list(vec![])])).items()?;
    if t2017.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t2017[0].clone();
    v_hist = t2017[1].clone();
    let mut t2018 = false;
    for t2019 in py::range_iter(&[(Py::Int(1000000i64)).i()?]) {
        v_k = t2019;
        v_x = py::mul(&py::mul(&v_r.clone(), &v_x.clone())?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?;
        let _ = py::method(&v_hist.clone(), "append", &[v_x.clone()])?;
        if ({ let t2024 = { let t2020 = py::len(&v_hist.clone())?; Py::Bool({ let t2021 = Py::Int(2i64); if py::gt(&t2020, &t2021)? { true } else { false } }) }; if !t2024.truthy() { t2024 } else { { let t2022 = py::abs(&py::sub(&py::getitem(&v_hist.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_hist.clone(), &py::neg(&Py::Int(3i64))?)?)?)?; Py::Bool({ let t2023 = Py::Float(1e-14); if py::lt(&t2022, &t2023)? { true } else { false } }) } } }).truthy() {
            { t2018 = true; break; }
        }
    }
    if !t2018 {
        return Err(py::value_error(py::to_str(&Py::str("the orbit does not settle on a fixed point or a 2-cycle (chaotic or slowly converging regime): a finite-time exponent there depends on rounding, so it is not reported"))));
    }
    v_tot = Py::Float(0.0);
    for t2025 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t2025;
        v_d = py::abs(&py::mul(&v_r.clone(), &py::sub(&Py::Int(1i64), &py::mul(&Py::Int(2i64), &v_x.clone())?)?)?)?;
        if ({ let t2026 = v_d.clone(); Py::Bool({ let t2027 = Py::Int(0i64); if py::eq(&t2026, &t2027) { true } else { false } }) }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the orbit is superstable (f'(x) = 0): the exponent is minus infinity"))));
        }
        v_tot = py::iadd(&v_tot, &py::math1("log", &v_d.clone())?)?;
        v_x = py::mul(&py::mul(&v_r.clone(), &v_x.clone())?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?;
    }
    v_attractor = (if ({ let t2028 = py::abs(&py::sub(&py::mul(&py::mul(&v_r.clone(), &v_x.clone())?, &py::sub(&Py::Int(1i64), &v_x.clone())?)?, &v_x.clone())?)?; Py::Bool({ let t2029 = Py::Float(1e-12); if py::lt(&t2028, &t2029)? { true } else { false } }) }).truthy() { Py::str("fixed point") } else { Py::str("2-cycle") });
    if ({ let t2034 = { let t2030 = v_attractor.clone(); Py::Bool({ let t2031 = Py::str("fixed point"); if py::eq(&t2030, &t2031) { true } else { false } }) }; if !t2034.truthy() { t2034 } else { { let t2032 = py::abs(&v_x.clone())?; Py::Bool({ let t2033 = Py::Float(1e-12); if py::lt(&t2032, &t2033)? { true } else { false } }) } } }).truthy() {
        v_attractor = Py::str("fixed point 0");
    }
    return Ok(Py::dict(vec![(Py::str("lyapunov"), f__r(py::truediv(&v_tot.clone(), &v_n.clone())?)?), (Py::str("attractor"), v_attractor.clone())]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logistic(mut v_P0: Py, mut v_r: Py, mut v_K: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_P0.clone(), v_r.clone(), v_K.clone(), v_t.clone()]))?;
    if ({ let t2114 = { let t2107 = v_P0.clone(); Py::Bool({ let t2108 = Py::Int(0i64); if py::le(&t2107, &t2108)? { true } else { false } }) }; if t2114.truthy() { t2114 } else { { let t2113 = { let t2109 = v_K.clone(); Py::Bool({ let t2110 = Py::Int(0i64); if py::le(&t2109, &t2110)? { true } else { false } }) }; if t2113.truthy() { t2113 } else { { let t2111 = v_r.clone(); Py::Bool({ let t2112 = Py::Int(0i64); if py::le(&t2111, &t2112)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs P0 > 0, K > 0 and r > 0"))));
    }
    return Ok(Py::dict(vec![(Py::str("P"), f__r(py::truediv(&py::mul(&v_K.clone(), &v_P0.clone())?, &py::add(&v_P0.clone(), &py::mul(&py::sub(&v_K.clone(), &v_P0.clone())?, &f__exp(py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?)?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_logistic_inflection(mut v_P0: Py, mut v_r: Py, mut v_K: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_P0.clone(), v_r.clone(), v_K.clone()]))?;
    if (py::not(&{ let t1926 = { let t1918 = v_r.clone(); Py::Bool({ let t1919 = Py::Int(0i64); if py::gt(&t1918, &t1919)? { true } else { false } }) }; if !t1926.truthy() { t1926 } else { { let t1925 = { let t1920 = v_K.clone(); Py::Bool({ let t1921 = Py::Int(0i64); if py::gt(&t1920, &t1921)? { true } else { false } }) }; if !t1925.truthy() { t1925 } else { { let t1922 = Py::Int(0i64); Py::Bool({ let t1923 = v_P0.clone(); if py::lt(&t1922, &t1923)? { { let t1924 = py::truediv(&v_K.clone(), &Py::Int(2i64))?; if py::lt(&t1923, &t1924)? { true } else { false } } } else { false } }) } } } } })).truthy() {
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
    if ({ let t2156 = { let t2149 = v_sigma.clone(); Py::Bool({ let t2150 = Py::Int(0i64); if py::le(&t2149, &t2150)? { true } else { false } }) }; if t2156.truthy() { t2156 } else { { let t2155 = { let t2151 = v_beta.clone(); Py::Bool({ let t2152 = Py::Int(0i64); if py::le(&t2151, &t2152)? { true } else { false } }) }; if t2155.truthy() { t2155 } else { { let t2153 = v_rho.clone(); Py::Bool({ let t2154 = Py::Int(0i64); if py::le(&t2153, &t2154)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs sigma, rho, beta > 0"))));
    }
    v_out = Py::dict(vec![(Py::str("origin_stability"), (if ({ let t2157 = v_rho.clone(); Py::Bool({ let t2158 = Py::Int(1i64); if py::lt(&t2157, &t2158)? { true } else { false } }) }).truthy() { Py::str("asymptotically stable") } else { (if ({ let t2159 = v_rho.clone(); Py::Bool({ let t2160 = Py::Int(1i64); if py::gt(&t2159, &t2160)? { true } else { false } }) }).truthy() { Py::str("unstable") } else { Py::str("marginal (rho = 1)") }) }))]);
    if ({ let t2161 = v_rho.clone(); Py::Bool({ let t2162 = Py::Int(1i64); if py::gt(&t2161, &t2162)? { true } else { false } }) }).truthy() {
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
    if ({ let t2202 = { let t2195 = v_sigma.clone(); Py::Bool({ let t2196 = Py::Int(0i64); if py::le(&t2195, &t2196)? { true } else { false } }) }; if t2202.truthy() { t2202 } else { { let t2201 = { let t2197 = v_beta.clone(); Py::Bool({ let t2198 = Py::Int(0i64); if py::le(&t2197, &t2198)? { true } else { false } }) }; if t2201.truthy() { t2201 } else { { let t2199 = v_sigma.clone(); Py::Bool({ let t2200 = py::add(&v_beta.clone(), &Py::Int(1i64))?; if py::le(&t2199, &t2200)? { true } else { false } }) } } } } }).truthy() {
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
    if ({ let t2235 = py::minmax(&[v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone()], false)?; Py::Bool({ let t2236 = Py::Int(0i64); if py::le(&t2235, &t2236)? { true } else { false } }) }).truthy() {
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
    if ({ let t2329 = { let t2319 = v_V.clone(); Py::Bool({ let t2320 = Py::Int(0i64); if py::le(&t2319, &t2320)? { true } else { false } }) }; if t2329.truthy() { t2329 } else { { let t2328 = { let t2321 = v_r.clone(); Py::Bool({ let t2322 = Py::Int(0i64); if py::le(&t2321, &t2322)? { true } else { false } }) }; if t2328.truthy() { t2328 } else { { let t2327 = { let t2323 = v_c_in.clone(); Py::Bool({ let t2324 = Py::Int(0i64); if py::lt(&t2323, &t2324)? { true } else { false } }) }; if t2327.truthy() { t2327 } else { { let t2325 = v_A0.clone(); Py::Bool({ let t2326 = Py::Int(0i64); if py::lt(&t2325, &t2326)? { true } else { false } }) } } } } } } }).truthy() {
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
    let t2686 = (f__ratfun(v_F.clone())?).items()?;
    if t2686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2686[0].clone();
    v_Q = t2686[1].clone();
    v_ps = f__poles(v_Q.clone())?;
    if (Py::Bool(({ let mut out = vec![]; for t2693 in py::iter(&(v_ps.clone()))? { let mut v__: Py; let mut v_m: Py; 
let t2694 = (t2693).items()?;
if t2694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t2694[0].clone();
v_m = t2694[1].clone();
out.push({ let t2695 = v_m.clone(); Py::Bool({ let t2696 = Py::Int(1i64); if py::gt(&t2695, &t2696)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy()))).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("repeated poles: this capability lists simple poles only (use the inverse transform)"))));
    }
    v_dQ = py::method(&v_Q.clone(), "diff", &[c_SS()?])?;
    let t2699 = (Py::tuple(vec![{ let mut out = vec![]; for t2697 in py::iter(&(py::method(&v_P.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2697;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }, { let mut out = vec![]; for t2698 in py::iter(&(py::method(&v_dQ.clone(), "all_coeffs", &[])?))? { let mut v_c: Py; 
v_c = t2698;
out.push(py::complex(&[v_c.clone()])?); } Py::list(out) }])).items()?;
    if t2699.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Pc = t2699[0].clone();
    v_dQc = t2699[1].clone();
    v_ev = { let v_F = v_F.clone(); let v_P = v_P.clone(); let v_Pc = v_Pc.clone(); let v_Q = v_Q.clone(); let v_dQ = v_dQ.clone(); let v_dQc = v_dQc.clone(); let v_ps = v_ps.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_co: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_z: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t2700 in py::iter(&(py::enumerate(&v_co.clone(), 0)?))? { let mut v_c: Py; let mut v_i: Py; 
let t2701 = (t2700).items()?;
if t2701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t2701[0].clone();
v_c = t2701[1].clone();
out.push(py::mul(&v_c.clone(), &py::pow(&v_z.clone(), &py::sub(&py::sub(&py::len(&v_co.clone())?, &Py::Int(1i64))?, &v_i.clone())?)?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_pairs = Py::list(vec![]);
    for t2702 in py::iter(&(v_ps.clone()))? {
        let t2705 = (t2702).items()?;
        if t2705.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_p = t2705[0].clone();
        v__ = t2705[1].clone();
        v_z = py::complex(&[v_p.clone()])?;
        if ({ let t2703 = py::abs(&py::imag(&v_z.clone())?)?; Py::Bool({ let t2704 = py::mul(&Py::Float(1e-25), &py::minmax(&[Py::Float(1.0), py::abs(&v_z.clone())?], true)?)?; if py::le(&t2703, &t2704)? { true } else { false } }) }).truthy() {
            v_z = py::complex(&[py::real(&v_z.clone())?, Py::Float(0.0)])?;
        }
        v_r = py::truediv(&py::call(&v_ev.clone(), &[v_Pc.clone(), v_z.clone()])?, &py::call(&v_ev.clone(), &[v_dQc.clone(), v_z.clone()])?)?;
        let _ = py::method(&v_pairs.clone(), "append", &[Py::tuple(vec![v_z.clone(), v_r.clone()])])?;
    }
    v_order = f__canon_c({ let mut out = vec![]; for t2715 in py::iter(&(v_pairs.clone()))? { let mut v__: Py; let mut v_z: Py; 
let t2716 = (t2715).items()?;
if t2716.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_z = t2716[0].clone();
v__ = t2716[1].clone();
out.push(v_z.clone()); } Py::list(out) })?;
    v_pz = Py::dict(vec![]);
    for t2717 in py::iter(&(v_pairs.clone()))? {
        let t2718 = (t2717).items()?;
        if t2718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_z = t2718[0].clone();
        v_r = t2718[1].clone();
        py::setitem(&v_pz.clone(), &v_z.clone(), v_r.clone())?;
    }
    let t2719 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t2719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_out_p = t2719[0].clone();
    v_out_r = t2719[1].clone();
    for t2720 in py::iter(&(v_order.clone()))? {
        v_z = t2720;
        v_zz = (if (Py::Bool(matches!(v_z.clone(), Py::Complex(..)))).truthy() { v_z.clone() } else { py::complex(&[v_z.clone(), Py::Float(0.0)])? });
        v_r = py::getitem(&v_pz.clone(), &v_zz.clone())?;
        let _ = py::method(&v_out_p.clone(), "append", &[f__r((if (Py::Bool(matches!(v_z.clone(), Py::Complex(..)))).truthy() { v_z.clone() } else { py::float(&v_z.clone())? }))?])?;
        let _ = py::method(&v_out_r.clone(), "append", &[(if ({ let t2723 = py::imag(&v_zz.clone())?; Py::Bool({ let t2724 = Py::Int(0i64); if py::eq(&t2723, &t2724) { true } else { false } }) }).truthy() { f__r(py::real(&py::complex(&[py::real(&v_r.clone())?, Py::Float(0.0)])?)?)? } else { f__r(v_r.clone())? })])?;
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
    if ({ let t2764 = { let t2760 = py::len(&v_A.clone())?; Py::Bool({ let t2761 = Py::Int(2i64); if py::ne(&t2760, &t2761) { true } else { false } }) }; if t2764.truthy() { t2764 } else { { let t2762 = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2763 = Py::Int(2i64); if py::ne(&t2762, &t2763) { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("A must be a 2x2 matrix"))));
    }
    let t2766 = ({ let mut out = vec![]; for t2765 in py::iter(&(Py::tuple(vec![py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(1i64))?, py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(0i64))?, py::getitem(&py::getitem(&v_A.clone(), &Py::Int(1i64))?, &Py::Int(1i64))?])))? { let mut v_v: Py; 
v_v = t2765;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }).items()?;
    if t2766.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t2766[0].clone();
    v_b = t2766[1].clone();
    v_c = t2766[2].clone();
    v_d = t2766[3].clone();
    let t2767 = (Py::tuple(vec![py::add(&v_a.clone(), &v_d.clone())?, py::sub(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_b.clone(), &v_c.clone())?)?])).items()?;
    if t2767.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tr = t2767[0].clone();
    v_det = t2767[1].clone();
    let t2798 = (f__classify(v_tr.clone(), v_det.clone(), { let t2792 = { let t2785 = v_b.clone(); Py::Bool({ let t2786 = Py::Int(0i64); if py::eq(&t2785, &t2786) { true } else { false } }) }; if !t2792.truthy() { t2792 } else { { let t2791 = { let t2787 = v_c.clone(); Py::Bool({ let t2788 = Py::Int(0i64); if py::eq(&t2787, &t2788) { true } else { false } }) }; if !t2791.truthy() { t2791 } else { { let t2789 = v_a.clone(); Py::Bool({ let t2790 = v_d.clone(); if py::eq(&t2789, &t2790) { true } else { false } }) } } } } }, { let t2793 = v_a.clone(); Py::Bool({ let t2794 = v_b.clone(); if py::eq(&t2793, &t2794) { { let t2795 = v_c.clone(); if py::eq(&t2794, &t2795) { { let t2796 = v_d.clone(); if py::eq(&t2795, &t2796) { { let t2797 = Py::Int(0i64); if py::eq(&t2796, &t2797) { true } else { false } } } else { false } } } else { false } } } else { false } }) })?).items()?;
    if t2798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_kind = t2798[0].clone();
    v_stab = t2798[1].clone();
    let t2805 = (f__roots2(Py::Float(1.0), py::neg(&py::float(&v_tr.clone())?)?, py::float(&v_det.clone())?)?).items()?;
    if t2805.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_r1 = t2805[0].clone();
    v_r2 = t2805[1].clone();
    v__ = t2805[2].clone();
    return Ok(Py::dict(vec![(Py::str("type"), v_kind.clone()), (Py::str("stability"), v_stab.clone()), (Py::str("lambda1"), f__r(v_r1.clone())?), (Py::str("lambda2"), f__r(v_r2.clone())?), (Py::str("trace"), f__r(py::float(&v_tr.clone())?)?), (Py::str("det"), f__r(py::float(&v_det.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_practical_resonance(mut v_m: Py, mut v_c: Py, mut v_k: Py, mut v_F0: Py) -> R {
    let mut v_w2: Py = Py::None;
    let _ = f__fin(Py::tuple(vec![v_m.clone(), v_c.clone(), v_k.clone(), v_F0.clone()]))?;
    if ({ let t2845 = { let t2838 = v_m.clone(); Py::Bool({ let t2839 = Py::Int(0i64); if py::le(&t2838, &t2839)? { true } else { false } }) }; if t2845.truthy() { t2845 } else { { let t2844 = { let t2840 = v_c.clone(); Py::Bool({ let t2841 = Py::Int(0i64); if py::le(&t2840, &t2841)? { true } else { false } }) }; if t2844.truthy() { t2844 } else { { let t2842 = v_k.clone(); Py::Bool({ let t2843 = Py::Int(0i64); if py::le(&t2842, &t2843)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs m, c, k > 0"))));
    }
    v_w2 = py::sub(&py::truediv(&v_k.clone(), &v_m.clone())?, &py::truediv(&py::mul(&v_c.clone(), &v_c.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_m.clone())?, &v_m.clone())?)?)?;
    if ({ let t2846 = v_w2.clone(); Py::Bool({ let t2847 = Py::Int(0i64); if py::le(&t2846, &t2847)? { true } else { false } }) }).truthy() {
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
    if ({ let t3590 = { let t3586 = v_m.clone(); Py::Bool({ let t3587 = Py::Int(0i64); if py::le(&t3586, &t3587)? { true } else { false } }) }; if t3590.truthy() { t3590 } else { { let t3588 = v_k.clone(); Py::Bool({ let t3589 = Py::Int(0i64); if py::le(&t3588, &t3589)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t2930 = v_t.clone(); Py::Bool({ let t2931 = Py::Int(0i64); if py::lt(&t2930, &t2931)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs t >= 0"))));
    }
    v_d = py::sub(&Py::Int(1i64), &py::mul(&py::mul(&v_k.clone(), &v_y0.clone())?, &v_t.clone())?)?;
    if ({ let t2932 = v_d.clone(); Py::Bool({ let t2933 = Py::Int(0i64); if py::le(&t2932, &t2933)? { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&["the solution blows up at t = ".to_string(), py::format(&py::truediv(&Py::Int(1i64), &py::mul(&v_k.clone(), &v_y0.clone())?)?, "g")?, ", before the requested time".to_string()].concat()))));
    }
    v_out = Py::dict(vec![(Py::str("y"), f__r(py::truediv(&v_y0.clone(), &v_d.clone())?)?)]);
    if ({ let t2936 = py::mul(&v_k.clone(), &v_y0.clone())?; Py::Bool({ let t2937 = Py::Int(0i64); if py::gt(&t2936, &t2937)? { true } else { false } }) }).truthy() {
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
    if ({ let t3178 = { let t3173 = v_h.clone(); Py::Bool({ let t3174 = Py::Int(0i64); if py::le(&t3173, &t3174)? { true } else { false } }) }; if t3178.truthy() { t3178 } else { py::not(&{ let t3175 = Py::Int(1i64); Py::Bool({ let t3176 = v_n.clone(); if py::le(&t3175, &t3176)? { { let t3177 = Py::Int(10000i64); if py::le(&t3176, &t3177)? { true } else { false } } } else { false } }) }) } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs h > 0 and 1 <= n <= 10000"))));
    }
    let t3179 = (Py::tuple(vec![ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone(), Py::tuple(vec![Py::str("t"), Py::str("x"), Py::str("y")])], &[])?, ax_engine::pysym::helper("differential_equations:_fn", &[v_g.clone(), Py::tuple(vec![Py::str("t"), Py::str("x"), Py::str("y")])], &[])?])).items()?;
    if t3179.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_F = t3179[0].clone();
    v_G = t3179[1].clone();
    let t3180 = (Py::tuple(vec![v_x0.clone(), v_y0.clone()])).items()?;
    if t3180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x = t3180[0].clone();
    v_y = t3180[1].clone();
    for t3181 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t3181;
        v_t = py::add(&v_t0.clone(), &py::mul(&v_k.clone(), &v_h.clone())?)?;
        let t3182 = (Py::tuple(vec![py::call(&v_F.clone(), &[v_t.clone(), v_x.clone(), v_y.clone()])?, py::call(&v_G.clone(), &[v_t.clone(), v_x.clone(), v_y.clone()])?])).items()?;
        if t3182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a1 = t3182[0].clone();
        v_b1 = t3182[1].clone();
        let t3183 = (Py::tuple(vec![py::call(&v_F.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a1.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b1.clone())?, &Py::Int(2i64))?)?])?, py::call(&v_G.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a1.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b1.clone())?, &Py::Int(2i64))?)?])?])).items()?;
        if t3183.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a2 = t3183[0].clone();
        v_b2 = t3183[1].clone();
        let t3184 = (Py::tuple(vec![py::call(&v_F.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a2.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b2.clone())?, &Py::Int(2i64))?)?])?, py::call(&v_G.clone(), &[py::add(&v_t.clone(), &py::truediv(&v_h.clone(), &Py::Int(2i64))?)?, py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_a2.clone())?, &Py::Int(2i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &v_b2.clone())?, &Py::Int(2i64))?)?])?])).items()?;
        if t3184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a3 = t3184[0].clone();
        v_b3 = t3184[1].clone();
        v_t1 = py::add(&v_t0.clone(), &py::mul(&py::add(&v_k.clone(), &Py::Int(1i64))?, &v_h.clone())?)?;
        let t3185 = (Py::tuple(vec![py::call(&v_F.clone(), &[v_t1.clone(), py::add(&v_x.clone(), &py::mul(&v_h.clone(), &v_a3.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_b3.clone())?)?])?, py::call(&v_G.clone(), &[v_t1.clone(), py::add(&v_x.clone(), &py::mul(&v_h.clone(), &v_a3.clone())?)?, py::add(&v_y.clone(), &py::mul(&v_h.clone(), &v_b3.clone())?)?])?])).items()?;
        if t3185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a4 = t3185[0].clone();
        v_b4 = t3185[1].clone();
        let t3186 = (Py::tuple(vec![py::add(&v_x.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&py::add(&py::add(&v_a1.clone(), &py::mul(&Py::Int(2i64), &v_a2.clone())?)?, &py::mul(&Py::Int(2i64), &v_a3.clone())?)?, &v_a4.clone())?)?, &Py::Int(6i64))?)?, py::add(&v_y.clone(), &py::truediv(&py::mul(&v_h.clone(), &py::add(&py::add(&py::add(&v_b1.clone(), &py::mul(&Py::Int(2i64), &v_b2.clone())?)?, &py::mul(&Py::Int(2i64), &v_b3.clone())?)?, &v_b4.clone())?)?, &Py::Int(6i64))?)?])).items()?;
        if t3186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_x = t3186[0].clone();
        v_y = t3186[1].clone();
        if ({ let t3190 = py::not(&{ let t3187 = py::math1("isfinite", &v_x.clone())?; if !t3187.truthy() { t3187 } else { py::math1("isfinite", &v_y.clone())? } }); if t3190.truthy() { t3190 } else { { let t3188 = py::minmax(&[py::abs(&v_x.clone())?, py::abs(&v_y.clone())?], true)?; Py::Bool({ let t3189 = Py::Float(1e+100); if py::gt(&t3188, &t3189)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3014 = { let t3007 = v_L.clone(); Py::Bool({ let t3008 = Py::Int(0i64); if py::le(&t3007, &t3008)? { true } else { false } }) }; if t3014.truthy() { t3014 } else { { let t3013 = { let t3009 = v_h.clone(); Py::Bool({ let t3010 = Py::Int(0i64); if py::le(&t3009, &t3010)? { true } else { false } }) }; if t3013.truthy() { t3013 } else { { let t3011 = v_n.clone(); Py::Bool({ let t3012 = Py::Int(1i64); if py::lt(&t3011, &t3012)? { true } else { false } }) } } } } }).truthy() {
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
    let t3015 = (Py::tuple(vec![py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Float(0.5))?, &Py::Float(std::f64::consts::PI))?, &v_L.clone())?, py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())?])).items()?;
    if t3015.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3015[0].clone();
    v_hi = t3015[1].clone();
    v_flo = py::call(&v_F.clone(), &[v_lo.clone()])?;
    for t3016 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t3016;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t3017 = { let t3019 = py::call(&v_F.clone(), &[v_mid.clone()])?; Py::Bool({ let t3020 = Py::Int(0i64); if py::gt(&t3019, &t3020)? { true } else { false } }) }; Py::Bool({ let t3018 = { let t3021 = v_flo.clone(); Py::Bool({ let t3022 = Py::Int(0i64); if py::gt(&t3021, &t3022)? { true } else { false } }) }; if py::eq(&t3017, &t3018) { true } else { false } }) }).truthy() {
            let t3023 = (Py::tuple(vec![v_mid.clone(), py::call(&v_F.clone(), &[v_mid.clone()])?])).items()?;
            if t3023.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t3023[0].clone();
            v_flo = t3023[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
        if ({ let t3024 = py::sub(&v_hi.clone(), &v_lo.clone())?; Py::Bool({ let t3025 = py::mul(&Py::Float(4e-16), &v_hi.clone())?; if py::le(&t3024, &t3025)? { true } else { false } }) }).truthy() {
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
    if (py::not(&{ let t3055 = Py::Int(2i64); Py::Bool({ let t3056 = py::len(&v_coeffs.clone())?; if py::le(&t3055, &t3056)? { { let t3057 = Py::Int(9i64); if py::le(&t3056, &t3057)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("give 2 to 9 coefficients (degree 1 to 8), highest power first"))));
    }
    if ({ let t3058 = py::getitem(&v_coeffs.clone(), &Py::Int(0i64))?; Py::Bool({ let t3059 = Py::Int(0i64); if py::eq(&t3058, &t3059) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the leading coefficient must not be 0"))));
    }
    let t3087 = (f__routh(v_coeffs.clone())?).items()?;
    if t3087.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_first = t3087[0].clone();
    v_ch = t3087[1].clone();
    return Ok(Py::dict(vec![(Py::str("stable"), { let t3088 = v_ch.clone(); Py::Bool({ let t3089 = Py::Int(0i64); if py::eq(&t3088, &t3089) { true } else { false } }) }), (Py::str("rhp_roots"), v_ch.clone()), (Py::str("first_column"), { let mut out = vec![]; for t3090 in py::iter(&(v_first.clone()))? { let mut v_v: Py; 
v_v = t3090;
out.push(f__r(py::float(&v_v.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_separable_power(mut v_a: Py, mut v_n: Py, mut v_y0: Py, mut v_t: Py) -> R {
    let _ = f__fin(Py::tuple(vec![v_a.clone(), v_n.clone(), v_y0.clone(), v_t.clone()]))?;
    if ({ let t3302 = { let t3298 = v_n.clone(); Py::Bool({ let t3299 = Py::Int(0i64); if py::lt(&t3298, &t3299)? { true } else { false } }) }; if t3302.truthy() { t3302 } else { { let t3300 = v_t.clone(); Py::Bool({ let t3301 = Py::Int(0i64); if py::lt(&t3300, &t3301)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3340 = { let t3336 = v_m.clone(); Py::Bool({ let t3337 = Py::Int(0i64); if py::le(&t3336, &t3337)? { true } else { false } }) }; if t3340.truthy() { t3340 } else { { let t3338 = v_k.clone(); Py::Bool({ let t3339 = Py::Int(0i64); if py::le(&t3338, &t3339)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3386 = { let t3373 = v_beta.clone(); Py::Bool({ let t3374 = Py::Int(0i64); if py::le(&t3373, &t3374)? { true } else { false } }) }; if t3386.truthy() { t3386 } else { { let t3385 = { let t3375 = v_gamma.clone(); Py::Bool({ let t3376 = Py::Int(0i64); if py::le(&t3375, &t3376)? { true } else { false } }) }; if t3385.truthy() { t3385 } else { py::not(&{ let t3384 = { let t3377 = Py::Int(0i64); Py::Bool({ let t3378 = v_S0.clone(); if py::lt(&t3377, &t3378)? { true } else { false } }) }; if !t3384.truthy() { t3384 } else { { let t3383 = { let t3379 = Py::Int(0i64); Py::Bool({ let t3380 = v_I0.clone(); if py::lt(&t3379, &t3380)? { true } else { false } }) }; if !t3383.truthy() { t3383 } else { { let t3381 = py::add(&v_S0.clone(), &v_I0.clone())?; Py::Bool({ let t3382 = Py::Int(1i64); if py::le(&t3381, &t3382)? { true } else { false } }) } } } } }) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs beta, gamma > 0 and fractions S0, I0 > 0 with S0 + I0 <= 1"))));
    }
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    if ({ let t3387 = py::mul(&v_R0.clone(), &v_S0.clone())?; Py::Bool({ let t3388 = Py::Int(1i64); if py::gt(&t3387, &t3388)? { true } else { false } }) }).truthy() {
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
    let t3389 = (Py::tuple(vec![Py::Float(0.0), py::truediv(&Py::Int(1i64), &v_R0.clone())?])).items()?;
    if t3389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3389[0].clone();
    v_hi = t3389[1].clone();
    for t3390 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t3390;
        v_mid = py::truediv(&py::add(&v_lo.clone(), &v_hi.clone())?, &Py::Int(2i64))?;
        if ({ let t3395 = { let t3391 = v_mid.clone(); Py::Bool({ let t3392 = Py::Int(0i64); if py::eq(&t3391, &t3392) { true } else { false } }) }; if t3395.truthy() { t3395 } else { { let t3393 = py::call(&v_h.clone(), &[v_mid.clone()])?; Py::Bool({ let t3394 = Py::Int(0i64); if py::lt(&t3393, &t3394)? { true } else { false } }) } } }).truthy() {
            v_lo = v_mid.clone();
        } else {
            v_hi = v_mid.clone();
        }
        if ({ let t3396 = py::sub(&v_hi.clone(), &v_lo.clone())?; Py::Bool({ let t3397 = py::mul(&Py::Float(4e-16), &v_hi.clone())?; if py::le(&t3396, &t3397)? { true } else { false } }) }).truthy() {
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
    if ({ let t3472 = { let t3462 = v_a.clone(); Py::Bool({ let t3463 = Py::Int(0i64); if py::le(&t3462, &t3463)? { true } else { false } }) }; if t3472.truthy() { t3472 } else { { let t3471 = { let t3464 = v_b.clone(); Py::Bool({ let t3465 = Py::Int(0i64); if py::le(&t3464, &t3465)? { true } else { false } }) }; if t3471.truthy() { t3471 } else { { let t3470 = { let t3466 = v_c.clone(); Py::Bool({ let t3467 = Py::Int(0i64); if py::le(&t3466, &t3467)? { true } else { false } }) }; if t3470.truthy() { t3470 } else { { let t3468 = v_w.clone(); Py::Bool({ let t3469 = Py::Int(0i64); if py::le(&t3468, &t3469)? { true } else { false } }) } } } } } } }).truthy() {
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
    if ({ let t2623 = { let t2616 = v_a.clone(); Py::Bool({ let t2617 = Py::Int(0i64); if py::eq(&t2616, &t2617) { true } else { false } }) }; if t2623.truthy() { t2623 } else { { let t2622 = { let t2618 = v_c.clone(); Py::Bool({ let t2619 = Py::Int(0i64); if py::eq(&t2618, &t2619) { true } else { false } }) }; if t2622.truthy() { t2622 } else { { let t2620 = v_t.clone(); Py::Bool({ let t2621 = Py::Int(0i64); if py::lt(&t2620, &t2621)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs a != 0, c != 0 and t >= 0"))));
    }
    v_ys = py::truediv(&Py::Float(1.0), &v_c.clone())?;
    v_h = f_hom2(v_a.clone(), v_b.clone(), v_c.clone(), py::neg(&v_ys.clone())?, Py::Float(0.0), v_t.clone())?;
    v_out = Py::dict(vec![(Py::str("y"), f__r(f__guard(py::add(&v_ys.clone(), &v_h.clone())?, Py::tuple(vec![v_ys.clone(), v_h.clone()]))?)?)]);
    if ({ let t2650 = { let t2646 = py::truediv(&v_b.clone(), &v_a.clone())?; Py::Bool({ let t2647 = Py::Int(0i64); if py::gt(&t2646, &t2647)? { true } else { false } }) }; if !t2650.truthy() { t2650 } else { { let t2648 = py::truediv(&v_c.clone(), &v_a.clone())?; Py::Bool({ let t2649 = Py::Int(0i64); if py::gt(&t2648, &t2649)? { true } else { false } }) } } }).truthy() {
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
    if ({ let t3509 = { let t3505 = v_L.clone(); Py::Bool({ let t3506 = Py::Int(0i64); if py::le(&t3505, &t3506)? { true } else { false } }) }; if t3509.truthy() { t3509 } else { { let t3507 = v_n.clone(); Py::Bool({ let t3508 = Py::Int(1i64); if py::lt(&t3507, &t3508)? { true } else { false } }) } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0 and n >= 1"))));
    }
    v_k = (if ({ let t3510 = v_bc.clone(); Py::Bool({ let t3511 = Py::tuple(vec![Py::str("dirichlet-neumann"), Py::str("neumann-dirichlet")]); if py::contains(&t3511, &t3510)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::sub(&v_n.clone(), &Py::Float(0.5))?, &Py::Float(std::f64::consts::PI))?, &v_L.clone())? } else { py::truediv(&py::mul(&v_n.clone(), &Py::Float(std::f64::consts::PI))?, &v_L.clone())? });
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
    if ({ let t3551 = { let t3544 = v_h0.clone(); Py::Bool({ let t3545 = Py::Int(0i64); if py::le(&t3544, &t3545)? { true } else { false } }) }; if t3551.truthy() { t3551 } else { { let t3550 = { let t3546 = v_k.clone(); Py::Bool({ let t3547 = Py::Int(0i64); if py::le(&t3546, &t3547)? { true } else { false } }) }; if t3550.truthy() { t3550 } else { { let t3548 = v_t.clone(); Py::Bool({ let t3549 = Py::Int(0i64); if py::lt(&t3548, &t3549)? { true } else { false } }) } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs h0 > 0, k > 0 and t >= 0"))));
    }
    v_T = py::truediv(&py::mul(&Py::Int(2i64), &py::math1("sqrt", &v_h0.clone())?)?, &v_k.clone())?;
    v_h = (if ({ let t3552 = v_t.clone(); Py::Bool({ let t3553 = v_T.clone(); if py::lt(&t3552, &t3553)? { true } else { false } }) }).truthy() { py::pow(&py::sub(&py::math1("sqrt", &v_h0.clone())?, &py::truediv(&py::mul(&v_k.clone(), &v_t.clone())?, &Py::Int(2i64))?)?, &Py::Int(2i64))? } else { Py::Float(0.0) });
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
    if ({ let t3623 = v_a.clone(); Py::Bool({ let t3624 = Py::Int(0i64); if py::eq(&t3623, &t3624) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0"))));
    }
    let t3626 = ({ let mut out = vec![]; for t3625 in py::iter(&(Py::tuple(vec![v_a.clone(), v_b.clone(), v_c.clone(), v_s.clone()])))? { let mut v_v: Py; 
v_v = t3625;
out.push(py::fraction(&[v_v.clone()])?); } Py::list(out) }).items()?;
    if t3626.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A_ = t3626[0].clone();
    v_B_ = t3626[1].clone();
    v_C_ = t3626[2].clone();
    v_S_ = t3626[3].clone();
    let t3627 = (Py::tuple(vec![py::add(&py::add(&py::mul(&py::mul(&v_A_.clone(), &v_S_.clone())?, &v_S_.clone())?, &py::mul(&v_B_.clone(), &v_S_.clone())?)?, &v_C_.clone())?, py::add(&py::mul(&py::mul(&Py::Int(2i64), &v_A_.clone())?, &v_S_.clone())?, &v_B_.clone())?])).items()?;
    if t3627.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t3627[0].clone();
    v_dP = t3627[1].clone();
    if ({ let t3628 = v_P.clone(); Py::Bool({ let t3629 = Py::Int(0i64); if py::ne(&t3628, &t3629) { true } else { false } }) }).truthy() {
        let t3630 = (Py::tuple(vec![Py::Int(0i64), py::truediv(&v_F.clone(), &py::float(&v_P.clone())?)?])).items()?;
        if t3630.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_mlt = t3630[0].clone();
        v_A = t3630[1].clone();
    } else {
        if ({ let t3631 = v_dP.clone(); Py::Bool({ let t3632 = Py::Int(0i64); if py::ne(&t3631, &t3632) { true } else { false } }) }).truthy() {
            let t3633 = (Py::tuple(vec![Py::Int(1i64), py::truediv(&v_F.clone(), &py::float(&v_dP.clone())?)?])).items()?;
            if t3633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_mlt = t3633[0].clone();
            v_A = t3633[1].clone();
        } else {
            let t3634 = (Py::tuple(vec![Py::Int(2i64), py::truediv(&v_F.clone(), &py::mul(&Py::Int(2i64), &v_a.clone())?)?])).items()?;
            if t3634.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_mlt = t3634[0].clone();
            v_A = t3634[1].clone();
        }
    }
    v_e = f__exp(py::mul(&v_s.clone(), &v_t.clone())?)?;
    v_yp_t = py::mul(&py::mul(&v_A.clone(), &py::pow(&v_t.clone(), &v_mlt.clone())?)?, &v_e.clone())?;
    let t3640 = ((if ({ let t3636 = v_mlt.clone(); Py::Bool({ let t3637 = Py::Int(0i64); if py::eq(&t3636, &t3637) { true } else { false } }) }).truthy() { Py::tuple(vec![v_A.clone(), py::mul(&v_A.clone(), &v_s.clone())?]) } else { (if ({ let t3638 = v_mlt.clone(); Py::Bool({ let t3639 = Py::Int(1i64); if py::eq(&t3638, &t3639) { true } else { false } }) }).truthy() { Py::tuple(vec![Py::Float(0.0), v_A.clone()]) } else { Py::tuple(vec![Py::Float(0.0), Py::Float(0.0)]) }) })).items()?;
    if t3640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_yp0 = t3640[0].clone();
    v_ypd0 = t3640[1].clone();
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
    if ({ let t3692 = v_a.clone(); Py::Bool({ let t3693 = Py::Int(0i64); if py::eq(&t3692, &t3693) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("a must not be 0"))));
    }
    if ({ let t3694 = v_c.clone(); Py::Bool({ let t3695 = Py::Int(0i64); if py::eq(&t3694, &t3695) { true } else { false } }) }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("c must not be 0 (integrate once: the equation is first order in y')"))));
    }
    if (py::not(&{ let t3696 = Py::Int(1i64); Py::Bool({ let t3697 = py::len(&v_p.clone())?; if py::le(&t3696, &t3697)? { { let t3698 = Py::Int(5i64); if py::le(&t3697, &t3698)? { true } else { false } } } else { false } }) })).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("the forcing polynomial has degree at most 4 (1 to 5 coefficients p0, p1, ...)"))));
    }
    v_m = py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?;
    v_q = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_m.clone(), &Py::Int(3i64))?)?;
    for t3699 in py::range_iter(&[(v_m.clone()).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) {
        v_k = t3699;
        py::setitem(&v_q.clone(), &v_k.clone(), py::truediv(&py::sub(&py::sub(&py::getitem(&v_p.clone(), &v_k.clone())?, &py::mul(&py::mul(&v_b.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_q.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::mul(&py::mul(&v_a.clone(), &py::add(&v_k.clone(), &Py::Int(2i64))?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_q.clone(), &py::add(&v_k.clone(), &Py::Int(2i64))?)?)?)?, &v_c.clone())?)?;
    }
    v_q = py::slice(&v_q.clone(), None, Some((py::add(&v_m.clone(), &Py::Int(1i64))?).i()?), None)?;
    v_yp_t = py::sum(&{ let mut out = vec![]; for t3700 in py::iter(&(py::enumerate(&v_q.clone(), 0)?))? { let mut v_k: Py; let mut v_qk: Py; 
let t3701 = (t3700).items()?;
if t3701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3701[0].clone();
v_qk = t3701[1].clone();
out.push(py::mul(&v_qk.clone(), &py::pow(&v_t.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("y"), f__r(f__forced(v_a.clone(), v_b.clone(), v_c.clone(), v_y0.clone(), v_v0.clone(), v_t.clone(), v_yp_t.clone(), py::getitem(&v_q.clone(), &Py::Int(0i64))?, (if ({ let t3724 = v_m.clone(); Py::Bool({ let t3725 = Py::Int(1i64); if py::ge(&t3724, &t3725)? { true } else { false } }) }).truthy() { py::getitem(&v_q.clone(), &Py::Int(1i64))? } else { Py::Float(0.0) }))?)?), (Py::str("particular"), { let mut out = vec![]; for t3726 in py::iter(&(v_q.clone()))? { let mut v_v: Py; 
v_v = t3726;
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
    if ({ let t3769 = { let t3757 = v_L.clone(); Py::Bool({ let t3758 = Py::Int(0i64); if py::le(&t3757, &t3758)? { true } else { false } }) }; if t3769.truthy() { t3769 } else { { let t3768 = { let t3759 = v_c.clone(); Py::Bool({ let t3760 = Py::Int(0i64); if py::le(&t3759, &t3760)? { true } else { false } }) }; if t3768.truthy() { t3768 } else { { let t3767 = py::not(&{ let t3761 = Py::Int(0i64); Py::Bool({ let t3762 = v_x.clone(); if py::le(&t3761, &t3762)? { { let t3763 = v_L.clone(); if py::le(&t3762, &t3763)? { true } else { false } } } else { false } }) }); if t3767.truthy() { t3767 } else { py::not(&{ let t3764 = Py::Int(1i64); Py::Bool({ let t3765 = v_N.clone(); if py::le(&t3764, &t3765)? { { let t3766 = Py::Int(500i64); if py::le(&t3765, &t3766)? { true } else { false } } } else { false } }) }) } } } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str("needs L > 0, c > 0, 0 <= x <= L and 1 <= N <= 500"))));
    }
    let t3775 = (Py::tuple(vec![f__poly_x(v_f.clone(), Py::Int(6i64))?, f__poly_x(v_g.clone(), Py::Int(6i64))?])).items()?;
    if t3775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_cf = t3775[0].clone();
    v_cg = t3775[1].clone();
    v_terms = Py::list(vec![]);
    for t3776 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_N.clone(), &Py::Int(1i64))?).i()?]) {
        v_n = t3776;
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
fn s_1751(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1753: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1752 = v_inputs.clone(); f_linear_system_solution(py::getitem(&t1752, &Py::str("A"))?, py::getitem(&t1752, &Py::str("x0"))?, py::getitem(&t1752, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1753 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1754 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1755 = (t1754).items()?;
if t1755.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1755[0].clone();
v_v = t1755[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1844(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1846: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1845 = v_inputs.clone(); f_linear_stability(py::getitem(&t1845, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1846 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1847 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1848 = (t1847).items()?;
if t1848.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1848[0].clone();
v_v = t1848[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1888(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1890: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1889 = v_inputs.clone(); f_harvesting(py::getitem(&t1889, &Py::str("r"))?, py::getitem(&t1889, &Py::str("K"))?, py::getitem(&t1889, &Py::str("H"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1890 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1891 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1892 = (t1891).items()?;
if t1892.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1892[0].clone();
v_v = t1892[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1929(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1931: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1930 = v_inputs.clone(); f_logistic_inflection(py::getitem(&t1930, &Py::str("P0"))?, py::getitem(&t1930, &Py::str("r"))?, py::getitem(&t1930, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1931 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1932 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1933 = (t1932).items()?;
if t1933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1933[0].clone();
v_v = t1933[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1974(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t1976: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1975 = v_inputs.clone(); f_lm_fixed(py::getitem(&t1975, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1976 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t1977 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t1978 = (t1977).items()?;
if t1978.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1978[0].clone();
v_v = t1978[1].clone();
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
fn s_2037(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2039: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2038 = v_inputs.clone(); f_lm_lyapunov(py::getitem(&t2038, &Py::str("r"))?, py::getitem(&t2038, &Py::str("x0"))?, py::getitem(&t2038, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2039 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2040 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2041 = (t2040).items()?;
if t2041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2041[0].clone();
v_v = t2041[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2077(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2079: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2078 = v_inputs.clone(); f_lm_cycle(py::getitem(&t2078, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2079 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2080 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2081 = (t2080).items()?;
if t2081.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2081[0].clone();
v_v = t2081[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2118(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2121: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2120 = v_inputs.clone(); s_2119(py::getitem(&t2120, &Py::str("P0"))?, py::getitem(&t2120, &Py::str("r"))?, py::getitem(&t2120, &Py::str("K"))?, py::getitem(&t2120, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2121 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2122 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2123 = (t2122).items()?;
if t2123.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2123[0].clone();
v_v = t2123[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2119(mut v_P0: Py, mut v_r: Py, mut v_K: Py, mut v_t: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("P"), f__r(py::truediv(&v_K.clone(), &py::add(&Py::Int(1i64), &py::mul(&py::sub(&py::truediv(&v_K.clone(), &v_P0.clone())?, &Py::Int(1i64))?, &f__exp(py::mul(&py::neg(&v_r.clone())?, &v_t.clone())?)?)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2165(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2167: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2166 = v_inputs.clone(); f_lorenz_eq(py::getitem(&t2166, &Py::str("sigma"))?, py::getitem(&t2166, &Py::str("rho"))?, py::getitem(&t2166, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2167 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2168 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2169 = (t2168).items()?;
if t2169.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2169[0].clone();
v_v = t2169[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2205(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2207: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2206 = v_inputs.clone(); f_lorenz_hopf(py::getitem(&t2206, &Py::str("sigma"))?, py::getitem(&t2206, &Py::str("beta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2207 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2208 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2209 = (t2208).items()?;
if t2209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2209[0].clone();
v_v = t2209[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2239(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2241: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2240 = v_inputs.clone(); f_lotka_volterra(py::getitem(&t2240, &Py::str("a"))?, py::getitem(&t2240, &Py::str("b"))?, py::getitem(&t2240, &Py::str("c"))?, py::getitem(&t2240, &Py::str("d"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2241 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2242 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2243 = (t2242).items()?;
if t2243.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2243[0].clone();
v_v = t2243[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2266(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    return Ok(f__one_step(Py::str("midpoint"), ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, v_t0.clone(), v_y0.clone(), v_h.clone(), py::int(&v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2289(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2291: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2290 = v_inputs.clone(); s_2266(py::getitem(&t2290, &Py::str("f"))?, py::getitem(&t2290, &Py::str("t0"))?, py::getitem(&t2290, &Py::str("y0"))?, py::getitem(&t2290, &Py::str("h"))?, py::getitem(&t2290, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2291 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2292 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2293 = (t2292).items()?;
if t2293.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2293[0].clone();
v_v = t2293[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2333(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2335: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2334 = v_inputs.clone(); f_mixing(py::getitem(&t2334, &Py::str("V"))?, py::getitem(&t2334, &Py::str("r"))?, py::getitem(&t2334, &Py::str("c_in"))?, py::getitem(&t2334, &Py::str("A0"))?, py::getitem(&t2334, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2335 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2336 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2337 = (t2336).items()?;
if t2337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2337[0].clone();
v_v = t2337[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2374(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2376: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2375 = v_inputs.clone(); f_cooling_k(py::getitem(&t2375, &Py::str("T0"))?, py::getitem(&t2375, &Py::str("Ta"))?, py::getitem(&t2375, &Py::str("T1"))?, py::getitem(&t2375, &Py::str("t1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2376 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2377 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2378 = (t2377).items()?;
if t2378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2378[0].clone();
v_v = t2378[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2409(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2411: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2410 = v_inputs.clone(); f_cooling_T(py::getitem(&t2410, &Py::str("T0"))?, py::getitem(&t2410, &Py::str("Ta"))?, py::getitem(&t2410, &Py::str("k"))?, py::getitem(&t2410, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2411 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2412 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2413 = (t2412).items()?;
if t2413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2413[0].clone();
v_v = t2413[1].clone();
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
fn s_2452(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2454: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2453 = v_inputs.clone(); f_cooling_time(py::getitem(&t2453, &Py::str("T0"))?, py::getitem(&t2453, &Py::str("Ta"))?, py::getitem(&t2453, &Py::str("k"))?, py::getitem(&t2453, &Py::str("T1"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2454 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2455 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2456 = (t2455).items()?;
if t2456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2456[0].clone();
v_v = t2456[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2531: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2530 = v_inputs.clone(); f_jacobian_classify(py::getitem(&t2530, &Py::str("f"))?, py::getitem(&t2530, &Py::str("g"))?, py::getitem(&t2530, &Py::str("x0"))?, py::getitem(&t2530, &Py::str("y0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2531 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2532 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2533 = (t2532).items()?;
if t2533.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2533[0].clone();
v_v = t2533[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2586(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2588: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2587 = v_inputs.clone(); f_impulse2(py::getitem(&t2587, &Py::str("a"))?, py::getitem(&t2587, &Py::str("b"))?, py::getitem(&t2587, &Py::str("c"))?, py::getitem(&t2587, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2588 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2589 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2590 = (t2589).items()?;
if t2590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2590[0].clone();
v_v = t2590[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2651(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2653: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2652 = v_inputs.clone(); f_step2(py::getitem(&t2652, &Py::str("a"))?, py::getitem(&t2652, &Py::str("b"))?, py::getitem(&t2652, &Py::str("c"))?, py::getitem(&t2652, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2653 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2654 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2655 = (t2654).items()?;
if t2655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2655[0].clone();
v_v = t2655[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2725(mut v_i: Py, mut v_o: Py) -> R {
    return Ok(f__verify_partial(v_i.clone(), v_o.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2808(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2810: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2809 = v_inputs.clone(); f_phase_portrait(py::getitem(&t2809, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2810 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2811 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2812 = (t2811).items()?;
if t2812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2812[0].clone();
v_v = t2812[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2850(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2852: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2851 = v_inputs.clone(); f_practical_resonance(py::getitem(&t2851, &Py::str("m"))?, py::getitem(&t2851, &Py::str("c"))?, py::getitem(&t2851, &Py::str("k"))?, py::getitem(&t2851, &Py::str("F0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2852 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2853 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2854 = (t2853).items()?;
if t2854.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2854[0].clone();
v_v = t2854[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2900(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2902: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2901 = v_inputs.clone(); f_drag_quadratic(py::getitem(&t2901, &Py::str("m"))?, py::getitem(&t2901, &Py::str("c"))?, py::getitem(&t2901, &Py::str("g"))?, py::getitem(&t2901, &Py::str("v0"))?, py::getitem(&t2901, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2902 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2903 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2904 = (t2903).items()?;
if t2904.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2904[0].clone();
v_v = t2904[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2938(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2940: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2939 = v_inputs.clone(); f_riccati_sq(py::getitem(&t2939, &Py::str("k"))?, py::getitem(&t2939, &Py::str("y0"))?, py::getitem(&t2939, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2940 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2941 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2942 = (t2941).items()?;
if t2942.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2942[0].clone();
v_v = t2942[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2976(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t2979: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2978 = v_inputs.clone(); s_2977(py::getitem(&t2978, &Py::str("T"))?, py::getitem(&t2978, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2979 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t2980 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t2981 = (t2980).items()?;
if t2981.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t2981[0].clone();
v_v = t2981[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2977(mut v_T: Py, mut v_p: Py) -> R {
    return Ok(Py::dict(vec![(Py::str("t"), f__r(py::mul(&py::neg(&v_T.clone())?, &py::math1("log2", &v_p.clone())?)?)?)]));
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
fn s_3028(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3030: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3029 = v_inputs.clone(); f_robin_eigen(py::getitem(&t3029, &Py::str("L"))?, py::getitem(&t3029, &Py::str("h"))?, py::getitem(&t3029, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3030 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3031 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3032 = (t3031).items()?;
if t3032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3032[0].clone();
v_v = t3032[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3093(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3095: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3094 = v_inputs.clone(); f_routh_hurwitz(py::getitem(&t3094, &Py::str("coeffs"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3095 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3096 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3097 = (t3096).items()?;
if t3097.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3097[0].clone();
v_v = t3097[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3120(mut v_f: Py, mut v_t0: Py, mut v_y0: Py, mut v_h: Py, mut v_n: Py) -> R {
    return Ok(f__one_step(Py::str("rk4"), ax_engine::pysym::helper("differential_equations:_fn", &[v_f.clone()], &[])?, v_t0.clone(), v_y0.clone(), v_h.clone(), py::int(&v_n.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3143(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3145: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3144 = v_inputs.clone(); s_3120(py::getitem(&t3144, &Py::str("f"))?, py::getitem(&t3144, &Py::str("t0"))?, py::getitem(&t3144, &Py::str("y0"))?, py::getitem(&t3144, &Py::str("h"))?, py::getitem(&t3144, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3145 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3146 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3147 = (t3146).items()?;
if t3147.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3147[0].clone();
v_v = t3147[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3193(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3195: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3194 = v_inputs.clone(); f_rk4_system(py::getitem(&t3194, &Py::str("f"))?, py::getitem(&t3194, &Py::str("g"))?, py::getitem(&t3194, &Py::str("t0"))?, py::getitem(&t3194, &Py::str("x0"))?, py::getitem(&t3194, &Py::str("y0"))?, py::getitem(&t3194, &Py::str("h"))?, py::getitem(&t3194, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3195 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3196 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3197 = (t3196).items()?;
if t3197.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3197[0].clone();
v_v = t3197[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3268(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3270: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3269 = v_inputs.clone(); f_ivp2(py::getitem(&t3269, &Py::str("a"))?, py::getitem(&t3269, &Py::str("b"))?, py::getitem(&t3269, &Py::str("c"))?, py::getitem(&t3269, &Py::str("y0"))?, py::getitem(&t3269, &Py::str("v0"))?, py::getitem(&t3269, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3270 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3271 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3272 = (t3271).items()?;
if t3272.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3272[0].clone();
v_v = t3272[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3306(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3308: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3307 = v_inputs.clone(); f_separable_power(py::getitem(&t3307, &Py::str("a"))?, py::getitem(&t3307, &Py::str("n"))?, py::getitem(&t3307, &Py::str("y0"))?, py::getitem(&t3307, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3308 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3309 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3310 = (t3309).items()?;
if t3310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3310[0].clone();
v_v = t3310[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3343(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3345: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3344 = v_inputs.clone(); f_shm(py::getitem(&t3344, &Py::str("m"))?, py::getitem(&t3344, &Py::str("k"))?, py::getitem(&t3344, &Py::str("x0"))?, py::getitem(&t3344, &Py::str("v0"))?, py::getitem(&t3344, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3345 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3346 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3347 = (t3346).items()?;
if t3347.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3347[0].clone();
v_v = t3347[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3400(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3402: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3401 = v_inputs.clone(); f_sir(py::getitem(&t3401, &Py::str("beta"))?, py::getitem(&t3401, &Py::str("gamma"))?, py::getitem(&t3401, &Py::str("S0"))?, py::getitem(&t3401, &Py::str("I0"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3402 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3403 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3404 = (t3403).items()?;
if t3404.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3404[0].clone();
v_v = t3404[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3432(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3434: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3433 = v_inputs.clone(); f_slope_field(py::getitem(&t3433, &Py::str("f"))?, py::getitem(&t3433, &Py::str("t"))?, py::getitem(&t3433, &Py::str("y"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3434 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3436 = (t3435).items()?;
if t3436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3436[0].clone();
v_v = t3436[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3475(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3477: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3476 = v_inputs.clone(); f_steady_state(py::getitem(&t3476, &Py::str("a"))?, py::getitem(&t3476, &Py::str("b"))?, py::getitem(&t3476, &Py::str("c"))?, py::getitem(&t3476, &Py::str("F0"))?, py::getitem(&t3476, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3477 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3478 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3479 = (t3478).items()?;
if t3479.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3479[0].clone();
v_v = t3479[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3514(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3516: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3515 = v_inputs.clone(); f_sturm_liouville(py::getitem(&t3515, &Py::str("bc"))?, py::getitem(&t3515, &Py::str("L"))?, py::getitem(&t3515, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3516 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3517 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3518 = (t3517).items()?;
if t3518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3518[0].clone();
v_v = t3518[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3556(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3558: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3557 = v_inputs.clone(); f_torricelli(py::getitem(&t3557, &Py::str("h0"))?, py::getitem(&t3557, &Py::str("k"))?, py::getitem(&t3557, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3558 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3559 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3560 = (t3559).items()?;
if t3560.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3560[0].clone();
v_v = t3560[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3594 = v_inputs.clone(); f_pure_resonance(py::getitem(&t3594, &Py::str("m"))?, py::getitem(&t3594, &Py::str("k"))?, py::getitem(&t3594, &Py::str("F0"))?, py::getitem(&t3594, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3595 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3596 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3597 = (t3596).items()?;
if t3597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3597[0].clone();
v_v = t3597[1].clone();
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
fn s_3662(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3664: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3663 = v_inputs.clone(); f_uc_exp(py::getitem(&t3663, &Py::str("a"))?, py::getitem(&t3663, &Py::str("b"))?, py::getitem(&t3663, &Py::str("c"))?, py::getitem(&t3663, &Py::str("F"))?, py::getitem(&t3663, &Py::str("s"))?, py::getitem(&t3663, &Py::str("y0"))?, py::getitem(&t3663, &Py::str("v0"))?, py::getitem(&t3663, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3664 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3665 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3666 = (t3665).items()?;
if t3666.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3666[0].clone();
v_v = t3666[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3727(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3729: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3728 = v_inputs.clone(); f_uc_poly(py::getitem(&t3728, &Py::str("a"))?, py::getitem(&t3728, &Py::str("b"))?, py::getitem(&t3728, &Py::str("c"))?, py::getitem(&t3728, &Py::str("p"))?, py::getitem(&t3728, &Py::str("y0"))?, py::getitem(&t3728, &Py::str("v0"))?, py::getitem(&t3728, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3729 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3730 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3731 = (t3730).items()?;
if t3731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3731[0].clone();
v_v = t3731[1].clone();
out.push(f__diff(py::method(&v_outputs.clone(), "get", &[v_k.clone()])?, v_v.clone())?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3788(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_want: Py = Py::None;
    let t3790: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3789 = v_inputs.clone(); f_wave_series(py::getitem(&t3789, &Py::str("f"))?, py::getitem(&t3789, &Py::str("L"))?, py::getitem(&t3789, &Py::str("c"))?, py::getitem(&t3789, &Py::str("N"))?, py::getitem(&t3789, &Py::str("x"))?, py::getitem(&t3789, &Py::str("t"))?, py::method(&t3789, "get", &[Py::str("g"), Py::str("0")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3790 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(Py::Float(1.0));
        }
        Err(e) => return Err(e),
    }
    return Ok({ let t = { let mut out = vec![]; for t3791 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t3792 = (t3791).items()?;
if t3792.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t3792[0].clone();
v_v = t3792[1].clone();
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
