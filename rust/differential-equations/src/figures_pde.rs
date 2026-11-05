//! Partial differential equations: profiles along the rod or string at several times, the explicit scheme's steps
//! as a heat map, its stability verdict over a grid, and the steady plate temperature; every value one run of the
//! model (its series solutions and its finite-difference steps).

use super::{fl, int, nv, range_of, text_of};
use ax_engine::figkit::{self as k, num, num_of, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

fn vec_input(v: &[f64]) -> Input {
    Input::Text(v.iter().map(|x| format!("{x}")).collect::<Vec<_>>().join(", "))
}

/// The scheme's state after m steps from u (one run of the model), r as given.
fn after(s: &Solved, key: &str, u: &[f64], r: f64, m: i64) -> Option<Vec<f64>> {
    let a = s.solve_other(key, &[("u", vec_input(u)), ("r", fl(r)), ("n", int(m))])?;
    k::list_of(&a, "u").filter(|v| v.len() == u.len())
}

fn profile(v: &[f64]) -> Vec<(f64, f64)> {
    v.iter().enumerate().map(|(i, y)| (i as f64, *y)).collect()
}

/// Crank-Nicolson (`ftcs` false) or the explicit scheme (true): the profile after several step counts.
pub fn scheme_profiles(s: &Solved, ftcs: bool) -> Option<Figure> {
    let (u, r, n) = (s.vec("u")?, nv(s, "r")?, nv(s, "n")? as i64);
    let got = s.out_list("u")?;
    if u.len() < 3 || u.len() > 200 || n < 1 {
        return None;
    }
    let mid = u.len() / 2;
    let mut series = vec![("initial".to_string(), profile(&u))];
    let cap;
    let title;
    if ftcs {
        series.push((format!("r = {}, n = {}", num(r), n), profile(&got)));
        let r2 = if r <= 0.5 { 0.6 } else { 0.25 };
        let other = after(s, s.key, &u, r2, n)?;
        series.push((format!("r = {}, n = {}", num(r2), n), profile(&other)));
        cap = format!("The explicit heat-equation scheme on {} nodes (ends held) from the profile {}, {} step(s) each \
                       computed by the model: with r = {} (the request) and with r = {} across the stability limit 1/2; \
                       below it the peak spreads smoothly, above it a sawtooth grows. Middle node after the last step: {} \
                       and {}.", u.len(), short(&u), n, num(r), num(r2), num(got[mid]), num(other[mid]));
        title = "Stable and unstable explicit heat steps";
    } else {
        series.push((if n == 1 { "1 step".to_string() } else { format!("{n} steps") }, profile(&got)));
        for m in [2 * n, 4 * n] {
            if m <= 60 {
                if let Some(v) = after(s, s.key, &u, r, m) {
                    series.push((format!("{m} steps"), profile(&v)));
                }
            }
        }
        let last = series.last().map(|x| x.1[mid].1).unwrap_or(f64::NAN);
        cap = format!("Crank-Nicolson steps of the heat equation on {} nodes (ends held) from {} with r = {}: the profile \
                       after {} step(s) (the request) and after more steps, each computed by the model; the peak spreads \
                       without oscillation (middle node {} after the last). Request: u = {}.", u.len(), short(&u), num(r),
                      n, num(last), short(&got));
        title = "Crank-Nicolson steps spreading heat";
    }
    let body = k::lines(&series, "Node", "u", &format!("middle node {}", num(got[mid])), Some((mid as f64, got[mid])),
                        &format!("node {mid}: {}", num(got[mid])), false);
    Some(s.chart("profiles", title, &cap, &body))
}

fn short(v: &[f64]) -> String {
    let t: Vec<String> = v.iter().take(9).map(|x| num(*x)).collect();
    format!("{}{}", t.join(", "), if v.len() > 9 { ", ..." } else { "" })
}

pub fn ftcs_heatmap(s: &Solved) -> Option<Figure> {
    let (u, r) = (s.vec("u")?, nv(s, "r")?);
    if u.len() < 3 || u.len() > 40 {
        return None;
    }
    let mut rows = vec![u.clone()];
    for m in 1..=7 {
        match after(s, s.key, &u, r, m) {
            Some(v) => rows.push(v),
            None => break,
        }
    }
    if rows.len() < 3 {
        return None;
    }
    let mid = u.len() / 2;
    let rl: Vec<String> = (0..rows.len()).map(|i| format!("step {i}")).collect();
    let cl: Vec<String> = (0..u.len()).map(|i| i.to_string()).collect();
    let body = k::heat(&rows, &rl, &cl, &format!("r = {}", num(r)));
    let cap = format!("Temperatures along a rod ({} nodes, ends held) from the explicit heat-equation scheme with r = {}, \
                       starting from {}: row k is the state after k steps, each computed by the model (middle node {} -> {} \
                       after {} steps).", u.len(), num(r), short(&u), num(u[mid]), num(rows[rows.len() - 1][mid]),
                      rows.len() - 1);
    Some(s.chart("heatmap", "Heat spreading along the rod, step by step", &cap, &body))
}

pub fn ftcs_decision(s: &Solved) -> Option<Figure> {
    let (r, n) = (nv(s, "r")?, nv(s, "n")?);
    let verdict = s.out_text("stable_ratio")?;
    let xs = k::span(0.04, (2.0 * r).max(1.0), 13, false);
    let ys: Vec<f64> = (1..=9).map(|i| i as f64).collect();
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, x) in xs.iter().enumerate() {
        for (j, y) in ys.iter().enumerate() {
            let Some(a) = s.with_input(&[("r", fl(*x)), ("n", int(*y as i64))]) else { continue };
            let Some(v) = text_of(&a, "stable_ratio") else { continue };
            if !names.contains(&v) {
                names.push(v.clone());
            }
            cells.push((i, j, v));
        }
    }
    if cells.len() < 4 {
        return None;
    }
    let body = k::decision(&xs, &ys, &cells, &names, (r, n), "Mesh ratio r", "Number of steps n",
                           &format!("r <= 1/2: {verdict}"));
    let counts: Vec<String> = names.iter().map(|v| format!("{v}: {} cells", cells.iter().filter(|c| &c.2 == v).count())).collect();
    let cap = format!("Whether the mesh ratio is within the stability limit (r <= 1/2) over r from {} to {} and n from 1 \
                       to 9: each of the {} x {} cells is one run of the model, coloured by the verdict it returned ({}). \
                       Marked: the request, r = {}, n = {}: {verdict}.", num(xs[0]), num(xs[12]), xs.len(), ys.len(),
                      counts.join(", "), num(r), n);
    Some(s.chart("decision", "Where the explicit scheme's stability verdict changes", &cap, &body))
}

/// Profiles u(x) at four times: the heat series, the vibrating string, or d'Alembert's solution.
pub fn series_profiles(s: &Solved, which: &str) -> Option<Figure> {
    let (x, t, u) = (nv(s, "x")?, nv(s, "t")?, s.out("u")?);
    let times: Vec<f64> = if t > 0.0 { vec![0.0, t / 2.0, t, 2.0 * t] } else { vec![0.0, 0.5, 1.0, 2.0] };
    let xs = match which {
        "dalembert" => {
            let c = nv(s, "c")?;
            let w = 3.0f64.max(2.5 * c.abs() * times[3]);
            k::span(x - w, x + w, 41, false)
        }
        _ => {
            let l = nv(s, "L")?;
            if !(l > 0.0) {
                return None;
            }
            k::span(0.0, l, 31, false)
        }
    };
    let mut series = vec![];
    for tv in &times {
        let pts: Vec<(f64, f64)> = xs.iter().filter_map(|xv| s.with(&[("x", *xv), ("t", *tv)])
            .and_then(|a| num_of(&a, "u")).map(|v| (*xv, v))).collect();
        if pts.len() >= 3 {
            series.push((format!("t = {}", num(*tv)), pts));
        }
    }
    if series.len() < 2 {
        return None;
    }
    let f = s.txt("f").unwrap_or_default();
    let (title, cap) = match which {
        "heat" => ("A heated rod cooling: temperature profiles".to_string(),
                   format!("Temperature profiles u(x, t) of the heat equation on 0 <= x <= {} with ends held at 0, initial \
                            profile f(x) = {f}, alpha = {}, {} series terms: the model's series at {} positions for each \
                            time shown. Marked: u({}, {}) = {}.", num(nv(s, "L")?), num(nv(s, "alpha")?),
                           nv(s, "N")?, xs.len(), num(x), num(t), num(u))),
        "wave" => ("A vibrating string, snapshot by snapshot".to_string(),
                   format!("Snapshots of a plucked string u(x, t), fixed ends, length L = {}, wave speed c = {}, initial \
                            shape f(x) = {f}, {} series terms: the model's series at {} positions for each time shown. \
                            Marked: u({}, {}) = {}.", num(nv(s, "L")?), num(nv(s, "c")?), nv(s, "N")?, xs.len(),
                           num(x), num(t), num(u))),
        _ => ("D'Alembert's solution: the wave at four times".to_string(),
              format!("D'Alembert's solution of the wave equation with wave speed c = {}, initial shape f(x) = {f} and \
                       initial velocity g(x) = {}: the model's solution at {} positions for each time shown, the two \
                       halves travelling at +-c. Marked: u({}, {}) = {}.", num(nv(s, "c")?), s.txt("g").unwrap_or_default(),
                      xs.len(), num(x), num(t), num(u))),
    };
    let _ = range_of(series.iter().flat_map(|x| x.1.iter().map(|p| p.1)))?;
    let body = k::lines_y(&series, "Position x", "u(x, t)", &format!("u = {}", num(u)), Some((x, u)),
                          &format!("x = {}, t = {}", num(x), num(t)), false, true);
    Some(s.chart("profiles", &title, &cap, &body))
}

pub fn laplace_field(s: &Solved) -> Option<Figure> {
    let (a, b, x, y, u) = (nv(s, "a")?, nv(s, "b")?, nv(s, "x")?, nv(s, "y")?, s.out("u")?);
    if !(a > 0.0 && b > 0.0) {
        return None;
    }
    let xs: Vec<f64> = (1..=13).map(|i| a * i as f64 / 14.0).collect();
    let ys: Vec<f64> = (1..=9).map(|j| b * j as f64 / 10.0).collect();
    let mut vals = vec![];
    let mut got = 0;
    for xv in &xs {
        let mut col = vec![];
        for yv in &ys {
            let v = s.with(&[("x", *xv), ("y", *yv)]).and_then(|an| num_of(&an, "u"));
            got += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if got < 10 {
        return None;
    }
    let f = s.txt("f").unwrap_or_default();
    let body = k::field(&xs, &ys, &vals, (x, y), "x", "y", &format!("u({}, {}) = {}", num(x), num(y), num(u)), (false, false));
    let cap = format!("Steady temperature u(x, y) in the rectangle 0 <= x <= {}, 0 <= y <= {} with the edge y = {} held at \
                       f = {f} and the others at 0 ({} series terms): the model's solution at {} interior points, shaded \
                       by value. Marked: u({}, {}) = {}.", num(a), num(b), num(b), nv(s, "N")?, got, num(x), num(y), num(u));
    Some(s.chart("field", "Steady temperature in the plate", &cap, &body))
}
