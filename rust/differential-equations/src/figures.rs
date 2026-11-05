//! Differential Equations' own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: every step, trajectory, profile, eigenvalue and verdict is a further solve of the model (its one-step
//! and system solvers, its series solutions, its classifiers). A figure the model cannot support for these inputs is
//! left out, never drawn wrong (docs/FIGURE_PARITY.md).

use ax_engine::engine::Answer;
use ax_engine::figkit::{self as k, num, num_of, DPlot, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use serde_json::Value;

#[path = "figures_phase.rs"]
mod phase;
#[path = "figures_pde.rs"]
mod pde;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let v: Vec<Option<Figure>> = match s.key {
        "euler_method" => vec![steps(s, "Euler's method")],
        "improved_euler_method" => vec![steps(s, "improved Euler method")],
        "midpoint_method" => vec![steps(s, "midpoint method")],
        "runge_kutta_4" => vec![steps(s, "fourth-order Runge-Kutta method")],
        "backward_euler_method" => vec![steps(s, "backward Euler method")],
        "euler_error_exponential" => vec![euler_error(s)],
        "fourier_sine_coefficient" => vec![fourier_stems(s)],
        "robin_eigenvalues" => vec![robin_stems(s)],
        "sturm_liouville_eigenvalues" => vec![sturm_stems(s)],
        "damped_oscillator_parameters" => vec![damping_decision(s)],
        "logistic_map_lyapunov" => vec![lyapunov_decision(s)],
        "lorenz_hopf_threshold" => vec![hopf_field(s)],
        "characteristic_roots" => vec![root_locus(s)],
        "partial_fraction_residues" => vec![poles(s)],
        "linear_system_stability" => vec![stability_matrix(s)],
        "practical_resonance" => vec![resonance(s)],
        "steady_state_amplitude" => vec![frequency_response(s)],
        "second_order_homogeneous_ivp" => vec![ivp_family(s)],
        "autonomous_equilibria" => vec![phase::phase_line(s)],
        "logistic_harvesting" => vec![phase::harvesting(s)],
        "logistic_map_fixed_points" => vec![phase::cobweb(s, false)],
        "logistic_map_two_cycle" => vec![phase::bifurcation(s), phase::cobweb(s, true)],
        "lorenz_equilibria" => vec![phase::lorenz(s)],
        "brusselator_hopf" => vec![phase::brusselator(s)],
        "competing_species_equilibrium" => vec![phase::competing(s)],
        "lotka_volterra_equilibrium" => vec![phase::predator_prey(s)],
        "nonlinear_equilibrium_type" => vec![phase::nonlinear(s)],
        "runge_kutta_4_system" => vec![phase::rk4_orbit(s)],
        "phase_portrait_2x2" => vec![phase::linear_portrait(s), phase::trace_det(s)],
        "slope_field" => vec![phase::slope_field(s)],
        "crank_nicolson_steps" => vec![pde::scheme_profiles(s, false)],
        "heat_ftcs_steps" => vec![pde::ftcs_decision(s), pde::ftcs_heatmap(s), pde::scheme_profiles(s, true)],
        "heat_equation_series" => vec![pde::series_profiles(s, "heat")],
        "wave_equation_string_series" => vec![pde::series_profiles(s, "wave")],
        "dalembert_solution" => vec![pde::series_profiles(s, "dalembert")],
        "laplace_equation_rectangle" => vec![pde::laplace_field(s)],
        _ => vec![],
    };
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ shared helpers
pub(crate) fn tx(t: &str) -> Input {
    Input::Text(t.to_string())
}
pub(crate) fn fl(x: f64) -> Input {
    Input::Float(x)
}
pub(crate) fn int(n: i64) -> Input {
    Input::Int(n)
}

/// A number written into an expression (a negative one in brackets).
pub(crate) fn lit(x: f64) -> String {
    if x < 0.0 { format!("({x})") } else { format!("{x}") }
}

/// A number or whole-number input as read.
pub(crate) fn nv(s: &Solved, sym: &str) -> Option<f64> {
    s.inp(sym).or_else(|| s.vec(sym).and_then(|v| (v.len() == 1).then(|| v[0])))
}

/// A complex value as returned: a number, or {"re", "im"}.
pub(crate) fn cxv(v: &Value) -> Option<(f64, f64)> {
    match v {
        Value::Number(n) => n.as_f64().map(|x| (x, 0.0)),
        Value::Object(m) => Some((m.get("re")?.as_f64()?, m.get("im")?.as_f64()?)),
        _ => None,
    }
    .filter(|z| z.0.is_finite() && z.1.is_finite())
}
pub(crate) fn cx_out(a: &Answer, sym: &str) -> Option<(f64, f64)> {
    cxv(&a.values.iter().find(|x| x.0 == sym)?.1)
}
pub(crate) fn cx_text(z: (f64, f64)) -> String {
    if z.1.abs() < 1e-12 * z.0.abs().max(1.0) {
        num(z.0)
    } else {
        format!("{} {} {}i", num(z.0), if z.1 < 0.0 { "-" } else { "+" }, num(z.1.abs()))
    }
}
pub(crate) fn text_of(a: &Answer, sym: &str) -> Option<String> {
    match &a.values.iter().find(|x| x.0 == sym)?.1 {
        Value::String(t) => Some(t.clone()),
        Value::Bool(b) => Some(if *b { "yes".into() } else { "no".into() }),
        _ => None,
    }
}

/// Vertical stems from 0 to each value (colour k), offset across by dx.
pub(crate) fn stems(p: &mut DPlot, pts: &[(f64, f64)], kk: usize, dx: f64) {
    for (x, y) in pts {
        p.seg((x + dx, 0.0), (x + dx, *y), kk, false);
        p.dot(x + dx, *y, kk, 4.0);
    }
}

pub(crate) fn range_of(v: impl Iterator<Item = f64>) -> Option<(f64, f64)> {
    let all: Vec<f64> = v.filter(|x| x.is_finite()).collect();
    if all.is_empty() {
        return None;
    }
    let lo = all.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = all.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((lo, hi))
}

// ------------------------------------------------------------------------------------------------ one-step methods
/// The method's steps for n = 1, 2, ... (each one solve), beside Euler's method with the same step (for the other
/// methods) and a fourth-order solution with ten times smaller steps (also the model).
fn steps(s: &Solved, name: &str) -> Option<Figure> {
    let (f, t0, y0, h, n) = (s.txt("f")?, nv(s, "t0")?, nv(s, "y0")?, nv(s, "h")?, nv(s, "n")?);
    let (yn, tn) = (s.out("y_n")?, s.out("t_n")?);
    if !(h != 0.0) {
        return None;
    }
    let kmax = (n as i64).clamp(8, 20);
    let base = |key: &str, hh: f64, m: i64| -> Option<(f64, f64)> {
        let a = s.solve_other(key, &[("f", tx(&f)), ("t0", fl(t0)), ("y0", fl(y0)), ("h", fl(hh)), ("n", int(m))])?;
        Some((num_of(&a, "t_n")?, num_of(&a, "y_n")?))
    };
    let mut mine = vec![(t0, y0)];
    let mut euler = vec![(t0, y0)];
    let mut refr = vec![(t0, y0)];
    for m in 1..=kmax {
        if let Some(p) = base(s.key, h, m) {
            mine.push(p);
        } else {
            break;
        }
        if s.key != "euler_method" {
            if let Some(p) = base("euler_method", h, m) {
                euler.push(p);
            }
        }
        if let Some(p) = base("runge_kutta_4", h / 10.0, 10 * m) {
            refr.push(p);
        }
    }
    if mine.len() < 3 || refr.len() < 3 {
        return None;
    }
    let ys = mine.iter().chain(euler.iter()).chain(refr.iter()).map(|p| p.1);
    let (ylo, yhi) = range_of(ys)?;
    let (xlo, xhi) = range_of(mine.iter().map(|p| p.0))?;
    let mut p = DPlot::new((xlo.min(xhi), xhi.max(xlo)), (ylo, yhi), "t", "y", &format!("y' = {f}"));
    p.curve(&refr, 3);
    p.key("fourth order, h/10", 3);
    if euler.len() > 2 {
        p.curve(&euler, 2);
        for q in &euler {
            p.dot(q.0, q.1, 2, 3.0);
        }
        p.key("Euler's method", 2);
    }
    p.curve(&mine, 0);
    for q in &mine {
        p.dot(q.0, q.1, 0, 3.5);
    }
    p.key(&clip_name(name), 0);
    p.mark(tn, yn, &format!("y_n = {}", num(yn)));
    let last = *mine.last()?;
    let rlast = refr.iter().find(|q| (q.0 - last.0).abs() < 1e-9 * last.0.abs().max(1.0)).copied();
    let mut cap = format!("Steps of the {name} for y' = {f}, y({}) = {} with step h = {}: each point one run of the model \
                           with n = 1 ... {}", num(t0), num(y0), num(h), mine.len() - 1);
    if euler.len() > 2 {
        cap.push_str(", beside Euler's method with the same step");
    }
    cap.push_str(&format!(", and a fourth-order solution with steps h/10 (also the model). At t = {}: {} {}",
                          num(last.0), name, num(last.1)));
    if let Some(r) = rlast {
        cap.push_str(&format!(", fourth order with small steps {}", num(r.1)));
    }
    cap.push_str(&format!(". Marked: the request, n = {}, y_n = {} at t = {}.", n, num(yn), num(tn)));
    Some(s.chart("steps", &format!("Steps of the {name} against a fine solution"), &cap, &p.body()))
}

fn clip_name(n: &str) -> String {
    let mut c = n.chars();
    match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

fn euler_error(s: &Solved) -> Option<Figure> {
    let (kk, y0, h, n) = (nv(s, "k")?, nv(s, "y0")?, nv(s, "h")?, nv(s, "n")?);
    let (ye, yx, err) = (s.out("y_euler")?, s.out("y_exact")?, s.out("error")?);
    let m = (n as i64).clamp(4, 30);
    let mut e = vec![(0.0, y0)];
    let mut x = vec![(0.0, y0)];
    for j in 1..=m {
        let Some(a) = s.with_input(&[("n", int(j))]) else { break };
        let t = j as f64 * h;
        if let (Some(a1), Some(a2)) = (num_of(&a, "y_euler"), num_of(&a, "y_exact")) {
            e.push((t, a1));
            x.push((t, a2));
        }
    }
    if e.len() < 3 {
        return None;
    }
    let (ylo, yhi) = range_of(e.iter().chain(x.iter()).map(|p| p.1))?;
    let (xlo, xhi) = range_of(e.iter().map(|p| p.0))?;
    let mut p = DPlot::new((xlo, xhi), (ylo, yhi), "t = n h", "y", &format!("y' = {} y, y0 = {}", num(kk), num(y0)));
    p.curve(&x, 3);
    p.key("exact", 3);
    p.curve(&e, 0);
    for q in &e {
        p.dot(q.0, q.1, 0, 3.5);
    }
    p.key("Euler's method", 0);
    let tn = n * h;
    p.seg((tn, ye), (tn, yx), 2, false);
    p.mark(tn, ye, &format!("error {}", num(err)));
    let cap = format!("Euler's method on y' = k y with k = {}, y0 = {}, h = {}: the model's Euler value and exact value \
                       after n = 1 ... {} steps. Marked: after {} steps Euler gives {} against {} (error {}).",
                      num(kk), num(y0), num(h), e.len() - 1, n, num(ye), num(yx), num(err));
    Some(s.chart("steps", "Euler's error accumulating step by step", &cap, &p.body()))
}

// ------------------------------------------------------------------------------------------------ stems
fn fourier_stems(s: &Solved) -> Option<Figure> {
    let (f, l, n, bn) = (s.txt("f")?, nv(s, "L")?, nv(s, "n")?, s.out("b_n")?);
    let top = (n as i64).max(12).min(30);
    let mut pts = vec![];
    for j in 1..=top {
        if let Some(a) = s.with_input(&[("n", int(j))]) {
            if let Some(b) = num_of(&a, "b_n") {
                pts.push((j as f64, k::snap(b, bn.abs().max(1e-300))));
            }
        }
    }
    if pts.len() < 3 {
        return None;
    }
    let (lo, hi) = range_of(pts.iter().map(|p| p.1).chain([0.0]))?;
    let mut p = DPlot::new((0.5, top as f64 + 0.5), (lo, hi), "n", "b_n", &format!("f(x) = {f} on 0 <= x <= {}", num(l)));
    p.hline(0.0);
    stems(&mut p, &pts, 0, 0.0);
    p.mark(n, bn, &format!("b{} = {}", n, num(bn)));
    let cap = format!("Fourier sine coefficients b_n of f(x) = {f} on 0 <= x <= {}, n = 1 ... {}, each computed by the \
                       model: {}. Marked: b{} = {}.", num(l), top,
                      pts.iter().take(6).map(|q| num(q.1)).collect::<Vec<_>>().join(", ") + ", ...", n, num(bn));
    Some(s.chart("stems", "Fourier sine coefficients of the function", &cap, &p.body()))
}

fn robin_stems(s: &Solved) -> Option<Figure> {
    let (l, h, n, lam) = (nv(s, "L")?, nv(s, "h")?, nv(s, "n")?, s.out("lambda_n")?);
    let hs = [h / 10.0, h, h * 10.0];
    let top = (n as i64).max(6).min(12);
    let mut series: Vec<(f64, Vec<(f64, f64)>)> = vec![];
    for hh in hs {
        let mut pts = vec![];
        for j in 1..=top {
            if let Some(a) = s.with_input(&[("h", fl(hh)), ("n", int(j))]) {
                if let Some(v) = num_of(&a, "lambda_n") {
                    pts.push((j as f64, v));
                }
            }
        }
        if pts.len() >= 2 {
            series.push((hh, pts));
        }
    }
    if series.is_empty() {
        return None;
    }
    let (_, hi) = range_of(series.iter().flat_map(|x| x.1.iter().map(|p| p.1)))?;
    let mut p = DPlot::new((0.5, top as f64 + 0.5), (0.0, hi), "n", "lambda_n",
                          &format!("L = {}, h = {}: lambda{} = {}", num(l), num(h), n, num(lam)));
    let mut notes = vec![];
    for (i, (hh, pts)) in series.iter().enumerate() {
        let kk = [2, 0, 1][i % 3];
        stems(&mut p, pts, kk, (i as f64 - 1.0) * 0.22);
        p.key(&format!("h = {}", num(*hh)), kk);
        notes.push(format!("h = {}: lambda_1 = {}", num(*hh), num(pts[0].1)));
    }
    p.mark(n, lam, "");
    let cap = format!("Eigenvalues lambda_n of y'' + lambda y = 0 on 0 <= x <= {} with a fixed end and a Robin end, n = 1 \
                       ... {}, from the model for three end coefficients ({}); a larger h moves them up. Marked: the \
                       request, lambda{} = {}.", num(l), top, notes.join("; "), n, num(lam));
    Some(s.chart("stems", "Robin eigenvalues for three end coefficients", &cap, &p.body()))
}

fn sturm_stems(s: &Solved) -> Option<Figure> {
    let (bc, l, n, lam) = (s.txt("bc")?, nv(s, "L")?, nv(s, "n")?, s.out("lambda_n")?);
    let bcs = ["dirichlet-dirichlet", "dirichlet-neumann", "neumann-dirichlet", "neumann-neumann"];
    let top = (n as i64).max(6).min(12);
    let mut series = vec![];
    for b in bcs {
        let mut pts = vec![];
        for j in 1..=top {
            if let Some(a) = s.with_input(&[("bc", tx(b)), ("n", int(j))]) {
                if let Some(v) = num_of(&a, "lambda_n") {
                    pts.push((j as f64, v));
                }
            }
        }
        if pts.len() >= 2 {
            series.push((b, pts));
        }
    }
    if series.len() < 2 {
        return None;
    }
    let (_, hi) = range_of(series.iter().flat_map(|x| x.1.iter().map(|p| p.1)))?;
    let mut p = DPlot::new((0.5, top as f64 + 0.5), (0.0, hi), "n", "lambda_n",
                          &format!("L = {}, {bc}: lambda{} = {}", num(l), n, num(lam)));
    let mut notes = vec![];
    for (i, (b, pts)) in series.iter().enumerate() {
        stems(&mut p, pts, i, (i as f64 - 1.5) * 0.17);
        let short: String = b.split('-').map(|w| w.chars().next().unwrap_or('?').to_ascii_uppercase().to_string())
            .collect::<Vec<_>>().join("-");
        p.key(&short, i);
        notes.push(format!("{b}: lambda_1 = {}", num(pts[0].1)));
    }
    p.mark(n, lam, "");
    let cap = format!("Eigenvalues lambda_n of y'' + lambda y = 0 on 0 <= x <= {} for n = 1 ... {} under four pairs of end \
                       conditions (D Dirichlet, N Neumann), each from the model ({}). Marked: the request, bc = {bc}, \
                       n = {}, lambda = {}.", num(l), top, notes.join("; "), n, num(lam));
    Some(s.chart("stems", "Eigenvalue ladders for four end conditions", &cap, &p.body()))
}

// ------------------------------------------------------------------------------------------------ verdict maps, fields
/// A verdict over a grid of two number inputs (each cell one solve), the request marked.
#[allow(clippy::too_many_arguments)]
fn verdict_grid(s: &Solved, xsym: &str, xs: &[f64], ysym: &str, ys: &[f64], out: &str, int_y: bool)
                -> Option<(Vec<(usize, usize, String)>, Vec<String>)> {
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let yi = if int_y { int(y.round() as i64) } else { fl(*y) };
            let Some(a) = s.with_input(&[(xsym, fl(*x)), (ysym, yi)]) else { continue };
            let Some(v) = text_of(&a, out) else { continue };
            if !names.contains(&v) {
                names.push(v.clone());
            }
            cells.push((i, j, v));
        }
    }
    (cells.len() >= 4).then_some((cells, names))
}

/// A verdict map with up to eight verdicts (four strong and four light shades) and its key in rows below the plot:
/// the body and its height. Cells (i, j, verdict) at (xs[i], ys[j]); the request marked.
#[allow(clippy::too_many_arguments)]
pub(crate) fn verdict_map(xs: &[f64], ys: &[f64], cells: &[(usize, usize, String)], names: &[String], mine: (f64, f64),
                          xlab: &str, ylab: &str, header: &str) -> (String, f64) {
    const CLS: [&str; 8] = ["axp-b", "axp-b2", "axp-b3", "axp-b4", "axp-s", "axp-s2", "axp-s3", "axp-s4"];
    let half = |v: &[f64], i: usize| -> (f64, f64) {
        let l = if i == 0 { v[0] - (v.get(1).unwrap_or(&v[0]) - v[0]) / 2.0 } else { (v[i - 1] + v[i]) / 2.0 };
        let r = if i + 1 >= v.len() { v[i] + (v[i] - v[i.saturating_sub(1)]) / 2.0 } else { (v[i] + v[i + 1]) / 2.0 };
        (l, r)
    };
    let (x0, x1) = (half(xs, 0).0, half(xs, xs.len() - 1).1);
    let (y0, y1) = (half(ys, 0).0, half(ys, ys.len() - 1).1);
    let mut p = DPlot::new((x0, x1), (y0, y1), xlab, ylab, header);
    for (i, j, c) in cells {
        let kk = names.iter().position(|n| n == c).unwrap_or(0) % 8;
        let (l, r) = half(xs, *i);
        let (b, t) = half(ys, *j);
        let (a, z) = (p.px(l, t), p.px(r, b));
        p.raw(&format!("<rect class=\"{}\" x=\"{:.1}\" y=\"{:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>", CLS[kk], a.0, a.1,
                       (z.0 - a.0).abs() + 0.5, (z.1 - a.1).abs() + 0.5));
    }
    p.mark(mine.0, mine.1, "");
    let mut out = p.body();
    let mut y = 352.0;
    let mut x = 96.0;
    for (kk, n) in names.iter().enumerate().take(8) {
        let w = 7.0 * n.chars().count() as f64 + 30.0;
        if x + w > 512.0 {
            x = 96.0;
            y += 20.0;
        }
        out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{:.1}\" width=\"12\" height=\"10\"/>\
                               <text class=\"axp-lt\" x=\"{:.1}\" y=\"{y:.1}\">{}</text>", CLS[kk], y - 9.0, x + 16.0,
                              n.replace('&', "&amp;").replace('<', "&lt;")));
        x += w;
    }
    (out, y + 12.0)
}

fn count_note(cells: &[(usize, usize, String)], names: &[String]) -> String {
    names.iter().map(|n| format!("{n}: {} cells", cells.iter().filter(|c| &c.2 == n).count())).collect::<Vec<_>>().join(", ")
}

fn damping_decision(s: &Solved) -> Option<Figure> {
    let (m, c, kk, zeta) = (nv(s, "m")?, nv(s, "c")?, nv(s, "k")?, s.out("zeta")?);
    let verdict = s.out_text("damping")?;
    if !(m > 0.0) {
        return None;
    }
    let chi = if zeta > 0.0 { 2.2 * c / zeta } else { 2.0 * c.max(1.0) };
    let xs = k::span(0.0, chi, 15, false);
    let ys = k::span(m * 0.25, m * 3.0, 11, false);
    let (cells, names) = verdict_grid(s, "c", &xs, "m", &ys, "damping", false)?;
    let body = k::decision(&xs, &ys, &cells, &names, (c, m), "Damping coefficient c", "Mass m",
                           &format!("k = {}: {}", num(kk), verdict));
    let cap = format!("The damping case over c from 0 to {} and m from {} to {}: each of the {} x {} cells is one run of \
                       the model, coloured by the verdict it returned ({}). Other input: k = {}. Marked: the request, \
                       c = {}, m = {}: {} (damping ratio {}).", num(chi), num(m * 0.25), num(m * 3.0), xs.len(), ys.len(),
                      count_note(&cells, &names), num(kk), num(c), num(m), verdict, num(zeta));
    Some(s.chart("decision", "Where the damping case changes", &cap, &body))
}

fn lyapunov_decision(s: &Solved) -> Option<Figure> {
    let (r, x0) = (nv(s, "r")?, nv(s, "x0")?);
    let att = s.out_text("attractor")?;
    let lam = s.out("lyapunov")?;
    // each cell iterates the map: at most 150 iterations a cell keeps the redraw quick
    let n = nv(s, "n")?;
    let nn = n.min(150.0);
    // r values clear of the slowly settling points r = 1 and r = 3 and of the chaotic range, which the model refuses
    // only after long iteration
    let xs = [0.4, 0.75, 1.3, 1.7, 2.1, 2.5, 2.8, 3.2, 3.4];
    let ys = k::span(0.05, 0.95, 5, false);
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let Some(a) = s.with_input(&[("r", fl(*x)), ("x0", fl(*y)), ("n", int(nn as i64))]) else { continue };
            let Some(v) = text_of(&a, "attractor") else { continue };
            if !names.contains(&v) {
                names.push(v.clone());
            }
            cells.push((i, j, v));
        }
    }
    if cells.len() < 4 {
        return None;
    }
    let body = k::decision(&xs, &ys, &cells, &names, (r, x0), "Parameter r", "Initial value x0",
                           &format!("{} (Lyapunov exponent {})", att, num(lam)));
    let cap = format!("The attractor of the logistic map for r = 0.4 ... 3.4 (clear of r = 1 and r = 3, where the orbit \
                       settles slowly) and x0 from 0.05 to 0.95: each of the \
                       {} x {} cells is one run of the model with n = {} iterations, coloured by the verdict it returned \
                       ({}); a blank cell is one the model refuses (no settled orbit). Marked: the request, r = {}, x0 = \
                       {}: {}, Lyapunov exponent {}.", xs.len(), ys.len(), nn, count_note(&cells, &names), num(r),
                      num(x0), att, num(lam));
    Some(s.chart("decision", "Where the logistic map's attractor changes", &cap, &body))
}

fn hopf_field(s: &Solved) -> Option<Figure> {
    let (sigma, beta, rh) = (nv(s, "sigma")?, nv(s, "beta")?, s.out("rho_H")?);
    if !(beta > 0.0 && sigma > 0.0) {
        return None;
    }
    let xs = k::span(beta / 6.0, beta * 1.8, 13, true);
    let ys = k::span(sigma * 0.8, sigma * 1.6, 9, false);
    let mut vals = vec![];
    let mut got = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with(&[("beta", *x), ("sigma", *y)]).and_then(|a| num_of(&a, "rho_H"));
            got += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if got < 4 {
        return None;
    }
    let body = k::field(&xs, &ys, &vals, (beta, sigma), "Geometric factor beta", "Prandtl number sigma",
                        &format!("rho_H = {}", num(rh)), (true, false));
    let cap = format!("The Hopf threshold rho_H of the Lorenz system over beta from {} to {} (logarithmic axis) and sigma \
                       from {} to {}: each of the {} x {} cells is one run of the model, shaded by its value; blank cells \
                       are inputs the model refuses (sigma must exceed beta + 1). Marked: the request, beta = {}, sigma = \
                       {}, rho_H = {}.", num(xs[0]), num(xs[12]), num(ys[0]), num(ys[8]), xs.len(), ys.len(), num(beta),
                      num(sigma), num(rh));
    Some(s.chart("field", "Hopf threshold over the two parameters", &cap, &body))
}

// ------------------------------------------------------------------------------------------------ complex plane, matrix
fn root_locus(s: &Solved) -> Option<Figure> {
    let (a, b, c) = (nv(s, "a")?, nv(s, "b")?, nv(s, "c")?);
    let (r1, r2) = (cx_out(s.a, "r1")?, cx_out(s.a, "r2")?);
    let kind = s.out_text("roots_type").unwrap_or_default();
    let hi = (2.0 * b.abs()).max(4.0 * (a * c).abs().sqrt()).max(1.0);
    let bs = if b >= 0.0 { k::span(0.0, hi, 41, false) } else { k::span(-hi, 0.0, 41, false) };
    let mut p1 = vec![];
    let mut p2 = vec![];
    for bv in &bs {
        if let Some(an) = s.with(&[("b", *bv)]) {
            if let (Some(z1), Some(z2)) = (cx_out(&an, "r1"), cx_out(&an, "r2")) {
                p1.push(z1);
                p2.push(z2);
            }
        }
    }
    if p1.len() < 3 {
        return None;
    }
    let mine = vec![(String::new(), r1), (String::new(), r2)];
    let body = cplane(&[p1, p2], &mine, &format!("b = {}: r = {}, {}", num(b), cx_text(r1), cx_text(r2)));
    let cap = format!("Roots of the characteristic equation a r^2 + b r + c = 0 with a = {}, c = {} for {} values of b \
                       from 0 to {}, all from the model: complex pairs while b is small, meeting on the real axis and \
                       splitting. Marked: the request b = {}, roots {} and {} ({}).", num(a), num(c), bs.len(), num(hi),
                      num(b), cx_text(r1), cx_text(r2), kind);
    Some(s.chart("complex", "Root locus of the characteristic equation as b varies", &cap, &body))
}

/// The complex plane to scale (real across, imaginary up, axes dashed): paths as curves, `mine` marked and labelled.
fn cplane(paths: &[Vec<(f64, f64)>], mine: &[(String, (f64, f64))], header: &str) -> String {
    let mut xs: Vec<f64> = paths.iter().flat_map(|p| p.iter().map(|z| z.0)).chain(mine.iter().map(|m| m.1 .0)).collect();
    let mut ys: Vec<f64> = paths.iter().flat_map(|p| p.iter().map(|z| z.1)).chain(mine.iter().map(|m| m.1 .1)).collect();
    xs.push(0.0);
    ys.push(0.0);
    let span = range_of(xs.iter().copied()).map(|r| r.1 - r.0).unwrap_or(1.0)
        .max(range_of(ys.iter().copied()).map(|r| r.1 - r.0).unwrap_or(1.0)).max(1e-9);
    ys.push(span * 0.15);
    ys.push(-span * 0.15);
    let mut p = DPlot::equal(&xs, &ys, "Real part", "Imaginary part", header);
    p.hline(0.0);
    p.vline(0.0);
    for (i, path) in paths.iter().enumerate() {
        p.curve(path, [0, 2][i % 2]);
    }
    for (i, (lab, z)) in mine.iter().enumerate() {
        if i == 0 {
            p.mark(z.0, z.1, lab);
        } else {
            p.dot(z.0, z.1, 1, 6.0);
            let anchor = if p.px(z.0, z.1).0 > 330.0 { "end" } else { "start" };
            let dx = if anchor == "end" { -0.04 * span } else { 0.04 * span };
            p.label(z.0 + dx, z.1 - 0.06 * span * (i as f64 % 2.0 * 2.0 - 1.0), anchor, "axp-vt", lab);
        }
    }
    p.body()
}

fn poles(s: &Solved) -> Option<Figure> {
    let f = s.txt("F")?;
    let ps: Vec<(f64, f64)> = s.outj("poles")?.as_array()?.iter().filter_map(cxv).collect();
    let rs: Vec<(f64, f64)> = s.outj("residues")?.as_array()?.iter().filter_map(cxv).collect();
    if ps.is_empty() || ps.len() != rs.len() {
        return None;
    }
    let mine: Vec<(String, (f64, f64))> = ps.iter().zip(&rs)
        .map(|(p, r)| (format!("{} (residue {})", cx_text(*p), cx_text(*r)), *p)).collect();
    let body = cplane(&[], &mine, &format!("F(s) = {}", f));
    let cap = format!("The poles of F(s) = {f} returned by the model, drawn in the complex plane with the residue of each: \
                       {}.", mine.iter().map(|m| m.0.clone()).collect::<Vec<_>>().join("; "));
    Some(s.chart("complex", "Poles of the rational function in the complex plane", &cap, &body))
}

fn stability_matrix(s: &Solved) -> Option<Figure> {
    let m = s.mat("A")?;
    if m.len() > 12 || m.iter().any(|r| r.len() != m.len()) {
        return None;
    }
    let stable = s.out_text("stable")?;
    let unst = s.out("unstable_eigenvalues")?;
    let cp = s.out_list("char_poly")?;
    let names: Vec<String> = (1..=m.len()).map(|i| i.to_string()).collect();
    let body = k::heat(&m, &names, &names, &format!("stable: {stable}, unstable eigenvalues: {}", num(unst)));
    let cap = format!("The matrix A of x' = A x as a heat map (rows and columns numbered; each cell shaded by its size, \
                       negative entries in the second colour). Returned by the model: characteristic polynomial \
                       coefficients {}, stable: {stable}, eigenvalues with non-negative real part: {}.",
                      cp.iter().map(|x| num(*x)).collect::<Vec<_>>().join(", "), num(unst));
    Some(s.chart("matrix", "The system matrix and its stability", &cap, &body))
}

// ------------------------------------------------------------------------------------------------ responses, families
/// The natural frequency the model gives for mass m, damping c, stiffness k.
fn omega0(s: &Solved, m: f64, c: f64, kk: f64) -> Option<f64> {
    let a = s.solve_other("damped_oscillator_parameters", &[("m", fl(m)), ("c", fl(c)), ("k", fl(kk))])?;
    num_of(&a, "omega0").filter(|w| *w > 0.0)
}

fn resonance(s: &Solved) -> Option<Figure> {
    let (m, c, kk, f0) = (nv(s, "m")?, nv(s, "c")?, nv(s, "k")?, nv(s, "F0")?);
    let (wr, amax) = (s.out("omega_r")?, s.out("amplitude_max")?);
    let peak = s.out_text("has_peak").map(|t| t == "yes").unwrap_or(false);
    let w0 = omega0(s, m, c, kk)?;
    let ws = k::span(w0 * 0.02, w0 * 2.5, 61, false);
    let mut pts = vec![];
    for w in &ws {
        if let Some(a) = s.solve_other("steady_state_amplitude", &[("a", fl(m)), ("b", fl(c)), ("c", fl(kk)), ("F0", fl(f0)),
                                                                  ("w", fl(*w))]) {
            if let Some(v) = num_of(&a, "amplitude") {
                pts.push((*w, v));
            }
        }
    }
    if pts.len() < 5 {
        return None;
    }
    let (lo, hi) = range_of(pts.iter().map(|p| p.1).chain([0.0, amax]))?;
    let mut p = DPlot::new((0.0, ws[60]), (lo, hi), "Forcing frequency w (rad/s)", "Steady-state amplitude",
                          &format!("m = {}, c = {}, k = {}, F0 = {}", num(m), num(c), num(kk), num(f0)));
    p.curve(&pts, 0);
    if peak {
        p.vline(wr);
        p.mark(wr, amax, &format!("w_r = {}, peak {}", num(wr), num(amax)));
    } else {
        p.mark(pts[0].0, pts[0].1, &format!("no peak: largest {}", num(amax)));
    }
    let cap = format!("Steady-state amplitude of m y'' + c y' + k y = F0 cos(w t) with m = {}, c = {}, k = {}, F0 = {}: \
                       amplitudes from the model's steady-state capability at {} frequencies from {} to {} (natural \
                       frequency {} from the model). {}", num(m), num(c), num(kk), num(f0), pts.len(), num(ws[0]),
                      num(ws[60]), num(w0),
                      if peak { format!("Marked: the practical-resonance frequency w_r = {} and peak amplitude {}.", num(wr), num(amax)) }
                      else { format!("The damping is too large for a peak: the amplitude only falls (largest {}).", num(amax)) });
    Some(s.chart("response", "Practical resonance: the amplitude against the forcing frequency", &cap, &p.body()))
}

fn frequency_response(s: &Solved) -> Option<Figure> {
    let (a, b, c, f0, w) = (nv(s, "a")?, nv(s, "b")?, nv(s, "c")?, nv(s, "F0")?, nv(s, "w")?);
    let (amp, lag, gain) = (s.out("amplitude")?, s.out("phase_lag")?, s.out("gain")?);
    let w0 = omega0(s, a, b.max(0.0), c)?;
    let bs: Vec<f64> = if b > 0.0 { vec![b * 0.5, b, b * 2.0, b * 4.0] } else { vec![b] };
    let ws = k::span(w0 * 0.05, (w0 * 2.5).max(w * 1.2), 41, false);
    let mut amps = vec![];
    let mut lags = vec![];
    for bv in &bs {
        let mut pa = vec![];
        let mut pl = vec![];
        for wv in &ws {
            if let Some(an) = s.with(&[("b", *bv), ("w", *wv)]) {
                if let (Some(x), Some(y)) = (num_of(&an, "amplitude"), num_of(&an, "phase_lag")) {
                    pa.push((*wv, x));
                    pl.push((*wv, y));
                }
            }
        }
        if pa.len() >= 3 {
            amps.push((format!("b = {}", num(*bv)), pa));
            lags.push((format!("b = {}", num(*bv)), pl));
        }
    }
    if amps.is_empty() {
        return None;
    }
    let p1 = k::lines_y(&amps, "Forcing frequency w (rad/s)", "Amplitude", &format!("amplitude {}", num(amp)), Some((w, amp)),
                        &format!("w = {}", num(w)), false, true);
    let p2 = k::lines_y(&lags, "Forcing frequency w (rad/s)", "Phase lag (rad)", &format!("phase lag {} rad", num(lag)),
                        Some((w, lag)), &format!("w = {}", num(w)), false, true);
    let (body, h) = k::stack(&[p1, p2]);
    let cap = format!("Steady-state response of a y'' + b y' + c y = F0 cos(w t) with a = {}, c = {}, F0 = {}: amplitude \
                       (top) and phase lag (bottom) from the model at {} forcing frequencies for {} damping coefficients. \
                       Marked: the request w = {}, amplitude {}, phase lag {} rad, gain {}.", num(a), num(c), num(f0),
                      ws.len(), amps.len(), num(w), num(amp), num(lag), num(gain));
    Some(s.figure("response", "Frequency response of the forced oscillator", &cap, &body, h))
}

fn ivp_family(s: &Solved) -> Option<Figure> {
    let (c, y0, y) = (nv(s, "c")?, nv(s, "y0")?, s.out("y")?);
    let log = c > 0.0;
    let cs = if log { k::span(c / 10.0, c * 10.0, 31, true) } else { k::span(c - 5.0, c + 5.0, 31, false) };
    let vals: Vec<f64> = if y0 != 0.0 { vec![y0, y0 * 0.5, y0 * 2.0] } else { vec![0.0, 0.5, 1.0] };
    let fam = s.family("c", &cs, "y", "y0", &vals, "");
    if fam.is_empty() {
        return None;
    }
    let body = k::lines(&fam, "Coefficient of y, c", "y(t)", &format!("y = {}", num(y)), Some((c, y)),
                        &format!("c = {}", num(c)), log);
    let cap = format!("y(t) against c from {} to {}{}, one curve for each initial value y0 ({}), {} points per curve \
                       computed by the model, other inputs as entered. Marked: the request, c = {}, y = {}.", num(cs[0]),
                      num(cs[30]), if log { " on a logarithmic axis" } else { "" },
                      vals.iter().map(|v| num(*v)).collect::<Vec<_>>().join(", "), cs.len(), num(c), num(y));
    Some(s.chart("curve-2", "Value at time t against the coefficient c, for three initial values", &cap, &body))
}
