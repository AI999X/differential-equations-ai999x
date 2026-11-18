//! Differential Equations: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p differential-equations-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&differential_equations_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "sturm_liouville_eigenvalues", "inputs": {"L": "1", "n": "2", "bc": "dirichlet-dirichlet"}}));
    println!();
    show(json!({"key": "brusselator_hopf", "inputs": {"a": "1", "b": "3"}}));
    println!();
    show(json!({"key": "radiometric_age", "inputs": {"T": "5730", "p": "0.25"}}));
    println!();
    show(json!({"key": "convolution_integral", "inputs": {"f": "exp(-t)", "g": "sin(t)", "t": "2"}}));
    println!();
    show(json!({"key": "phase_portrait_2x2", "inputs": {"A": [[0, 1], [-2, -3]]}}));
    println!();
    show(json!({"key": "backward_euler_method", "inputs": {"f": "-2*y + t", "y0": "1", "h": "0.1", "n": "2", "t0": "0"}}));
    println!();
    show(json!({"key": "cauchy_euler_ivp", "inputs": {"a": "1", "b": "2", "c": "-6", "y1": "1", "v1": "0", "t": "2"}}));
    println!();
    show(json!({"key": "robin_eigenvalues", "inputs": {"L": "1", "h": "1", "n": "1"}}));
    println!();
    show(json!({"key": "competing_species_equilibrium", "inputs": {"a1": "3", "b11": "1", "b12": "1", "a2": "2", "b21": "0.5", "b22": "1"}}));
    println!();
    show(json!({"key": "bernoulli_equation", "inputs": {"p": "1", "q": "1", "n": "2", "y0": "0.5", "t": "1"}}));
    println!();
    show(json!({"key": "ode_impulse_response", "inputs": {"a": "1", "b": "2", "c": "5", "t": "1"}}));
    println!();
    show(json!({"key": "linear_system_solution", "inputs": {"A": [[0, 1], [-2, -3]], "x0": [1, 0], "t": "1"}}));
    println!();
    show(json!({"key": "cfl_condition", "inputs": {"c": "2", "dt": "0.01", "dx": "0.05"}}));
    println!();
    show(json!({"key": "practical_resonance", "inputs": {"m": "1", "c": "0.4", "k": "4", "F0": "2"}}));
    println!();
    show(json!({"key": "fourier_sine_coefficient", "inputs": {"f": "x*(1 - x)", "L": "1", "n": "1"}}));
}
