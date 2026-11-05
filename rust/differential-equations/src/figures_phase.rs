//! Phase lines, phase portraits, cobwebs and bifurcation diagrams: every trajectory point is a run of the model's
//! fourth-order system solver from the previous point, every map or rate value a run of its slope evaluator, every
//! equilibrium and verdict the model's own.

use super::{cx_text, fl, int, lit, nv, range_of, text_of, tx};
use ax_engine::figkit::{self as k, num, num_of, DPlot, Solved};
use ax_engine::figures::Figure;

/// A trajectory of x' = f, y' = g from `start`: each point one run of the model's RK4 system solver (`per` steps of
/// size h from the previous point), at most `npts` points; it stops where the model refuses or the state leaves `far`.
fn traj(s: &Solved, f: &str, g: &str, start: (f64, f64), h: f64, per: i64, npts: usize, far: f64) -> Vec<(f64, f64)> {
    let mut out = vec![start];
    let mut cur = start;
    for _ in 0..npts {
        let Some(a) = s.solve_other("runge_kutta_4_system", &[("f", tx(f)), ("g", tx(g)), ("t0", fl(0.0)), ("x0", fl(cur.0)),
                                                           ("y0", fl(cur.1)), ("h", fl(h)), ("n", int(per))]) else { break };
        let (Some(x), Some(y)) = (num_of(&a, "x_n"), num_of(&a, "y_n")) else { break };
        cur = (x, y);
        out.push(cur);
        if x.abs() > far || y.abs() > far {
            break;
        }
    }
    out
}

/// The slope the model evaluates for y' = f at (t, y).
fn slope(s: &Solved, f: &str, t: f64, y: f64) -> Option<f64> {
    let a = s.solve_other("slope_field", &[("f", tx(f)), ("t", fl(t)), ("y", fl(y))])?;
    num_of(&a, "slope")
}

/// An arrowhead along the axis y = 0 at x pointing right (dir > 0) or left.
fn axis_arrow(p: &mut DPlot, x: f64, w: f64, dir: f64) {
    p.arrow((x - dir * w, 0.0), (x + dir * w, 0.0), 0);
}

fn trajectories_plot(p: &mut DPlot, ts: &[Vec<(f64, f64)>]) {
    for (i, t) in ts.iter().enumerate() {
        let kk = [0, 2, 1][i % 3];
        p.curve(t, kk);
        if t.len() >= 4 {
            let m = t.len() / 3;
            p.arrow(t[m - 1], t[m], kk);
        }
        p.dot(t[0].0, t[0].1, 3, 3.0);
    }
}

// ------------------------------------------------------------------------------------------------ phase lines
pub fn phase_line(s: &Solved) -> Option<Figure> {
    let f = s.txt("f")?;
    let eq = s.out_list("equilibria")?;
    let st: Vec<String> = s.outj("stability")?.as_array()?.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect();
    if eq.is_empty() || eq.len() > 8 {
        return None;
    }
    let (lo, hi) = range_of(eq.iter().copied())?;
    let pad = ((hi - lo) * 0.4).max(1.0);
    let ys = k::span(lo - pad, hi + pad, 41, false);
    let pts: Vec<(f64, f64)> = ys.iter().filter_map(|y| slope(s, &f, 0.0, *y).map(|v| (*y, v))).collect();
    if pts.len() < 5 {
        return None;
    }
    let (vlo, vhi) = range_of(pts.iter().map(|p| p.1).chain([0.0]))?;
    let mut p = DPlot::new((ys[0], ys[40]), (vlo, vhi), "y", "f(y) = y'", &format!("y' = {f}"));
    p.hline(0.0);
    p.curve(&pts, 2);
    // arrows on the axis between equilibria, pointing where y moves (the sign of the model's f there)
    let mut cuts = vec![ys[0]];
    cuts.extend(eq.iter().copied());
    cuts.push(ys[40]);
    let w = (ys[40] - ys[0]) * 0.025;
    for c in cuts.windows(2) {
        let mid = (c[0] + c[1]) / 2.0;
        if let Some(v) = slope(s, &f, 0.0, mid) {
            if v != 0.0 {
                axis_arrow(&mut p, mid, w, v.signum());
            }
        }
    }
    for (i, e) in eq.iter().enumerate() {
        let lab = st.get(i).cloned().unwrap_or_default();
        p.dot(*e, 0.0, if lab.starts_with("stable") { 0 } else { 3 }, 6.0);
        let dy = (vhi - vlo) * if i % 2 == 0 { 0.09 } else { 0.17 };
        p.label(*e, -dy, "middle", "axp-bt", &format!("y = {} {}", num(*e), lab));
    }
    p.mark(eq[0], 0.0, "");
    let cap = format!("Phase line of y' = {f}: f(y) evaluated by the model at {} points (curve); arrows on the axis show \
                       where y moves between equilibria (y grows where f > 0). Marked: the model's equilibria {}.",
                      pts.len(), eq.iter().zip(st.iter()).map(|(e, t)| format!("y = {} ({t})", num(*e)))
                          .collect::<Vec<_>>().join(", "));
    Some(s.chart("phase-line", "Phase line: equilibria and their stability", &cap, &p.body()))
}

pub fn harvesting(s: &Solved) -> Option<Figure> {
    let (r, kk, h) = (nv(s, "r")?, nv(s, "K")?, nv(s, "H")?);
    let (yh, yl, hc) = (s.out("y_high")?, s.out("y_low")?, s.out("H_critical")?);
    let (sh, sl) = (s.out_text("stability_high").unwrap_or_default(), s.out_text("stability_low").unwrap_or_default());
    if !(kk > 0.0) {
        return None;
    }
    let ys = k::span(0.0, kk * 1.1, 33, false);
    let rate = |hh: f64| -> Vec<(f64, f64)> {
        let f = format!("{}*y*(1 - y/{}) - {}", lit(r), lit(kk), lit(hh));
        ys.iter().filter_map(|y| slope(s, &f, 0.0, *y).map(|v| (*y, v))).collect()
    };
    let main = rate(h);
    if main.len() < 5 {
        return None;
    }
    let none = rate(0.0);
    let crit = rate(hc);
    let (vlo, vhi) = range_of(main.iter().chain(none.iter()).chain(crit.iter()).map(|p| p.1).chain([0.0]))?;
    let mut p = DPlot::new((0.0, kk * 1.1), (vlo, vhi), "Population y", "Growth rate y'",
                          &format!("r = {}, K = {}, H = {}", num(r), num(kk), num(h)));
    p.hline(0.0);
    p.dashed(&none, 3);
    p.key("H = 0", 3);
    p.dashed(&crit, 2);
    p.key(&format!("H = {}", num(hc)), 2);
    p.curve(&main, 0);
    p.key(&format!("H = {}", num(h)), 0);
    let w = kk * 0.025;
    for (a, b) in [(0.0, yl), (yl, yh), (yh, kk * 1.1)] {
        let mid = (a + b) / 2.0;
        if let Some(v) = main.iter().min_by(|x, y| (x.0 - mid).abs().partial_cmp(&(y.0 - mid).abs()).unwrap()).map(|q| q.1) {
            if v != 0.0 {
                axis_arrow(&mut p, mid, w, v.signum());
            }
        }
    }
    p.dot(yl, 0.0, 3, 6.0);
    p.dot(yh, 0.0, 0, 6.0);
    let dy = (vhi - vlo) * 0.09;
    p.label(yl, -dy, "middle", "axp-bt", &format!("{} {sl}", num(yl)));
    p.label(yh, -dy, "middle", "axp-bt", &format!("{} {sh}", num(yh)));
    p.mark(yh, 0.0, "");
    let cap = format!("Logistic growth with constant harvesting, y' = r y (1 - y/K) - H, r = {}, K = {}: the growth rate \
                       evaluated by the model at {} populations for H = {} (solid, with arrows showing where y moves), no \
                       harvest and the model's critical harvest {} (dashed). Marked: the equilibria y = {} ({sl}) and \
                       y = {} ({sh}).", num(r), num(kk), main.len(), num(h), num(hc), num(yl), num(yh));
    Some(s.chart("phase-line", "Harvesting a logistic population: equilibria", &cap, &p.body()))
}

// ------------------------------------------------------------------------------------------------ logistic map
pub fn cobweb(s: &Solved, two: bool) -> Option<Figure> {
    let r = nv(s, "r")?;
    let map = format!("{}*y*(1 - y)", lit(r));
    let g = |x: f64| slope(s, &map, 0.0, x);
    let xs = k::span(0.0, 1.0, 41, false);
    let curve: Vec<(f64, f64)> = xs.iter().filter_map(|x| g(*x).map(|y| (*x, y))).collect();
    if curve.len() < 10 {
        return None;
    }
    let (x0, nit) = if two { (0.3, 24) } else { (0.08, 14) };
    let mut web = vec![(x0, 0.0)];
    let mut x = x0;
    for _ in 0..nit {
        let Some(y) = g(x) else { break };
        if !(-0.5..=1.5).contains(&y) {
            break;
        }
        web.push((x, y));
        web.push((y, y));
        x = y;
    }
    if web.len() < 4 {
        return None;
    }
    let top = curve.iter().map(|p| p.1).fold(1.0, f64::max);
    let mut p = DPlot::equal(&[0.0, 1.0], &[0.0, top], "x_k", "x_k+1", &format!("x -> {} x (1 - x)", num(r)));
    p.dashed(&[(0.0, 0.0), (1.0, 1.0)], 3);
    p.curve(&curve, 2);
    p.curve(&web, 0);
    let cap;
    if two {
        let (pp, qq, mu) = (s.out("p")?, s.out("q")?, s.out("multiplier")?);
        let stab = s.out_text("stability").unwrap_or_default();
        p.outline(&[(pp, pp), (pp, qq), (qq, qq), (qq, pp)], 1);
        p.dot(pp, pp, 1, 5.0);
        p.dot(qq, qq, 1, 5.0);
        p.mark(qq, qq, &format!("q = {}", num(qq)));
        p.label(pp, pp, "start", "axp-vt", &format!("  p = {}", num(pp)));
        cap = format!("Cobweb diagram of the logistic map at r = {}: map values from the model's slope evaluator ({} \
                       points and one value per iterate) and {} iterates from x0 = {} settling onto the square p -> q -> \
                       p. Marked: the model's 2-cycle p = {}, q = {} ({stab}, multiplier {}).", num(r), curve.len(),
                      web.len() / 2, x0, num(pp), num(qq), num(mu));
    } else {
        let fp = s.out_list("fixed_points")?;
        let mu = s.out_list("multipliers")?;
        let st: Vec<String> = s.outj("stability")?.as_array()?.iter().map(|v| v.as_str().unwrap_or("").to_string()).collect();
        for (i, v) in fp.iter().enumerate() {
            p.dot(*v, *v, if st.get(i).map(|t| t == "stable").unwrap_or(false) { 1 } else { 3 }, 5.5);
        }
        if let Some(last) = fp.last() {
            p.mark(*last, *last, &format!("{} {}", num(*last), st.last().cloned().unwrap_or_default()));
        }
        cap = format!("Cobweb diagram of the logistic map x -> r x (1 - x) at r = {}: the map's values from the model's \
                       slope evaluator ({} points and one value per iterate) and {} iterates from x0 = {}. Marked: the \
                       model's fixed points {}.", num(r), curve.len(), web.len() / 2, x0,
                      fp.iter().enumerate().map(|(i, v)| format!("{} ({}, multiplier {})", num(*v),
                          st.get(i).cloned().unwrap_or_default(), mu.get(i).map(|m| num(*m)).unwrap_or_default()))
                          .collect::<Vec<_>>().join(", "));
    }
    let title = if two { "Cobweb: iterates settling on a 2-cycle" } else { "Cobweb: iterates and fixed points" };
    Some(s.chart("cobweb", title, &cap, &p.body()))
}

pub fn bifurcation(s: &Solved) -> Option<Figure> {
    let r = nv(s, "r")?;
    let (pp, qq, mu) = (s.out("p")?, s.out("q")?, s.out("multiplier")?);
    let rhi = (r + 0.25).clamp(3.45, 4.0);
    let rs = k::span(0.05, rhi, 56, false);
    let mut fs = vec![];
    let mut fu = vec![];
    let mut cs = vec![];
    let mut cu = vec![];
    for rv in &rs {
        if let Some(a) = s.solve_other("logistic_map_fixed_points", &[("r", fl(*rv))]) {
            let st = a.values.iter().find(|x| x.0 == "stability").and_then(|x| x.1.as_array().cloned()).unwrap_or_default();
            if let Some(v) = k::list_of(&a, "fixed_points") {
                for (i, x) in v.iter().enumerate() {
                    let stable = st.get(i).and_then(|z| z.as_str()) == Some("stable");
                    if stable { fs.push((*rv, *x)) } else { fu.push((*rv, *x)) }
                }
            }
        }
        if *rv > 3.0 {
            if let Some(a) = s.with(&[("r", *rv)]) {
                let stable = text_of(&a, "stability").as_deref() == Some("stable");
                for sym in ["p", "q"] {
                    if let Some(v) = num_of(&a, sym) {
                        if stable { cs.push((*rv, v)) } else { cu.push((*rv, v)) }
                    }
                }
            }
        }
    }
    if fs.len() + fu.len() < 10 {
        return None;
    }
    let mut p = DPlot::new((0.0, rhi), (0.0, 1.0), "Parameter r", "x", "Fixed points and 2-cycles");
    for q in &fu {
        p.dot(q.0, q.1, 3, 2.2);
    }
    for q in &fs {
        p.dot(q.0, q.1, 0, 2.6);
    }
    for q in &cu {
        p.dot(q.0, q.1, 2, 2.2);
    }
    for q in &cs {
        p.dot(q.0, q.1, 1, 2.6);
    }
    p.key("stable fixed point", 0);
    p.key("unstable", 3);
    p.key("stable 2-cycle", 1);
    if !cu.is_empty() {
        p.key("unstable 2-cycle", 2);
    }
    p.vline(r);
    p.mark(r, qq, &format!("r = {}: {} <-> {}", num(r), num(pp), num(qq)));
    p.dot(r, pp, 1, 5.0);
    let cap = format!("Period doubling of the logistic map x -> r x (1 - x): its fixed points and 2-cycles for {} values \
                       of r from 0.05 to {}, with their stability, all from the model; the fixed point loses stability \
                       at r = 3 where the 2-cycle appears. Marked: r = {}, cycle {} <-> {} (multiplier {}).", rs.len(),
                      num(rhi), num(r), num(pp), num(qq), num(mu));
    Some(s.chart("bifurcation", "Bifurcation of the logistic map: fixed points and 2-cycles", &cap, &p.body()))
}

pub fn lorenz(s: &Solved) -> Option<Figure> {
    let (sigma, rho, beta) = (nv(s, "sigma")?, nv(s, "rho")?, nv(s, "beta")?);
    let (count, ost) = (s.out("count")?, s.out_text("origin_stability")?);
    let xc = s.out("x_c");
    let rhi = (rho * 1.4).max(40.0);
    let rs = k::span(0.05, rhi, 60, false);
    let mut os = vec![];
    let mut ou = vec![];
    let mut cp = vec![];
    for rv in &rs {
        let Some(a) = s.with(&[("rho", *rv)]) else { continue };
        let st = text_of(&a, "origin_stability").unwrap_or_default();
        if st.contains("asymptotically stable") || st == "stable" { os.push((*rv, 0.0)) } else { ou.push((*rv, 0.0)) }
        if let Some(x) = num_of(&a, "x_c") {
            cp.push((*rv, x));
        }
    }
    if os.len() + ou.len() < 10 {
        return None;
    }
    let rh = s.solve_other("lorenz_hopf_threshold", &[("sigma", fl(sigma)), ("beta", fl(beta))])
        .and_then(|a| num_of(&a, "rho_H"));
    let ym = cp.iter().map(|q| q.1.abs()).fold(1.0, f64::max);
    let mut p = DPlot::new((0.0, rhi), (-ym, ym), "Rayleigh parameter rho", "Equilibrium x",
                          &format!("sigma = {}, beta = {}", num(sigma), num(beta)));
    p.curve(&os, 0);
    p.dashed(&ou, 3);
    let neg: Vec<(f64, f64)> = cp.iter().map(|q| (q.0, -q.1)).collect();
    p.curve(&cp, 1);
    p.curve(&neg, 1);
    p.key("origin stable", 0);
    p.key("origin unstable", 3);
    p.key("C+ and C-", 1);
    if let Some(h) = rh {
        if h > 0.0 && h < rhi {
            p.vline(h);
            p.label(h, -ym * 0.8, "start", "axp-lt", &format!(" rho_H = {}", num(h)));
        }
    }
    match xc {
        Some(x) => p.mark(rho, x, &format!("rho = {}: x_c = {}", num(rho), num(x))),
        None => p.mark(rho, 0.0, &format!("rho = {}", num(rho))),
    }
    let cap = format!("Equilibria of the Lorenz system (sigma = {}, beta = {}) for {} values of rho from 0.05 to {}, from \
                       the model: the origin (solid where stable, dashed where unstable) and the pair C+ and C- at x = +-x_c \
                       that appears at rho = 1{}. Marked: the request rho = {}: {} equilibria, origin {ost}{}.", num(sigma),
                      num(beta), rs.len(), num(rhi),
                      rh.map(|h| format!("; the model's Hopf threshold rho_H = {} (dashed) is where C+ and C- lose stability", num(h))).unwrap_or_default(),
                      num(rho), num(count), xc.map(|x| format!(", x_c = {}", num(x))).unwrap_or_default());
    Some(s.chart("bifurcation", "Lorenz equilibria as rho grows: a pitchfork", &cap, &p.body()))
}

// ------------------------------------------------------------------------------------------------ phase portraits
fn portrait_fig(s: &Solved, ts: Vec<Vec<(f64, f64)>>, eqp: (f64, f64), eqlab: &str, xlab: &str, ylab: &str,
                header: &str, title: &str, cap: String) -> Option<Figure> {
    let ts: Vec<Vec<(f64, f64)>> = ts.into_iter().filter(|t| t.len() >= 3).collect();
    if ts.is_empty() {
        return None;
    }
    let xs: Vec<f64> = ts.iter().flat_map(|t| t.iter().map(|q| q.0)).chain([eqp.0]).collect();
    let ys: Vec<f64> = ts.iter().flat_map(|t| t.iter().map(|q| q.1)).chain([eqp.1]).collect();
    let (xl, xh) = range_of(xs.into_iter())?;
    let (yl, yh) = range_of(ys.into_iter())?;
    let mut p = DPlot::new((xl, xh), (yl, yh), xlab, ylab, header);
    trajectories_plot(&mut p, &ts);
    p.mark(eqp.0, eqp.1, eqlab);
    Some(s.chart("phase", title, &cap, &p.body()))
}

pub fn brusselator(s: &Solved) -> Option<Figure> {
    let (a, b) = (nv(s, "a")?, nv(s, "b")?);
    let (xe, ye, bc) = (s.out("x_eq")?, s.out("y_eq")?, s.out("b_critical")?);
    let ty = s.out_text("type")?;
    let f = format!("{} - ({} + 1)*x + x^2*y", lit(a), lit(b));
    let g = format!("{}*x - x^2*y", lit(b));
    let far = 50.0 * (xe.abs() + ye.abs() + 1.0);
    let starts = [(xe + 0.1 * xe.abs().max(0.1), ye), (xe * 0.25, ye * 0.25 + 0.1)];
    let ts: Vec<Vec<(f64, f64)>> = starts.iter().map(|st| traj(s, &f, &g, *st, 0.02, 8, 90, far)).collect();
    let cap = format!("The Brusselator x' = {f}, y' = {g} (a = {}, b = {}): two trajectories, each point one run of the \
                       model's fourth-order system solver (step 0.02, 8 steps per point), one starting next to the \
                       equilibrium and one further out. The model classifies the equilibrium ({}, {}) as {ty}; its Hopf \
                       value is b_c = {}.", num(a), num(b), num(xe), num(ye), num(bc));
    portrait_fig(s, ts, (xe, ye), &format!("({}, {}) {ty}", num(xe), num(ye)), "x", "y",
                 &format!("b = {} against b_c = {}", num(b), num(bc)), "Brusselator trajectories near its Hopf bifurcation",
                 cap)
}

pub fn competing(s: &Solved) -> Option<Figure> {
    let v: Vec<f64> = ["a1", "b11", "b12", "a2", "b21", "b22"].iter().map(|x| nv(s, x)).collect::<Option<Vec<_>>>()?;
    let (xe, ye) = (s.out("x_eq")?, s.out("y_eq")?);
    let ty = s.out_text("type")?;
    let f = format!("x*({} - {}*x - {}*y)", lit(v[0]), lit(v[1]), lit(v[2]));
    let g = format!("y*({} - {}*x - {}*y)", lit(v[3]), lit(v[4]), lit(v[5]));
    let (sx, sy) = ((xe.abs() * 1.6).max(0.5), (ye.abs() * 1.6).max(0.5));
    let starts = [(0.1 * sx, 0.1 * sy), (sx, 0.1 * sy), (0.1 * sx, sy), (sx, sy), (0.5 * sx, 0.04 * sy), (0.04 * sx, 0.5 * sy)];
    let far = 20.0 * (sx + sy);
    let ts: Vec<Vec<(f64, f64)>> = starts.iter().map(|st| traj(s, &f, &g, *st, 0.05, 3, 40, far)).collect();
    let cap = format!("Competing species x' = {f}, y' = {g}: trajectories from {} starts, each point one run of the model's \
                       fourth-order system solver (step 0.05, 3 steps per point). Marked: the model's coexistence \
                       equilibrium ({}, {}), {ty}.", starts.len(), num(xe), num(ye));
    portrait_fig(s, ts, (xe, ye), &format!("({}, {}) {ty}", num(xe), num(ye)), "Species x", "Species y",
                 "Competing species", "Competing species settling over time", cap)
}

pub fn predator_prey(s: &Solved) -> Option<Figure> {
    let (a, b, c, d) = (nv(s, "a")?, nv(s, "b")?, nv(s, "c")?, nv(s, "d")?);
    let (xe, ye, per) = (s.out("x_eq")?, s.out("y_eq")?, s.out("period")?);
    if !(per > 0.0) {
        return None;
    }
    let f = format!("{}*x - {}*x*y", lit(a), lit(b));
    let g = format!("-{}*y + {}*x*y", lit(c), lit(d));
    let h = per * 1.25 / 240.0;
    let far = 100.0 * (xe.abs() + ye.abs() + 1.0);
    let ts: Vec<Vec<(f64, f64)>> = [1.2, 1.5, 1.9, 2.5].iter()
        .map(|m| traj(s, &f, &g, (xe * m, ye), h, 4, 60, far)).collect();
    let cap = format!("Predator-prey cycles x' = {f}, y' = {g} from four starting states, each point one run of the model's \
                       fourth-order system solver (step {}, 4 steps per point). Marked: the coexistence equilibrium ({}, \
                       {}) the model returns, around which small cycles have period {}.", num(h), num(xe), num(ye), num(per));
    portrait_fig(s, ts, (xe, ye), &format!("({}, {})", num(xe), num(ye)), "Prey x", "Predators y",
                 &format!("period of small cycles {}", num(per)), "Predator-prey cycles around their equilibrium", cap)
}

pub fn nonlinear(s: &Solved) -> Option<Figure> {
    let (f, g, x0, y0) = (s.txt("f")?, s.txt("g")?, nv(s, "x0")?, nv(s, "y0")?);
    let ty = s.out_text("type")?;
    let (tr, det) = (s.out("trace")?, s.out("det")?);
    let r = 2.0f64.max(0.5 * x0.abs().max(y0.abs()));
    let mut starts = vec![];
    for i in 0..8 {
        let th = i as f64 * std::f64::consts::PI / 4.0 + 0.3;
        starts.push((x0 + 0.9 * r * th.cos(), y0 + 0.9 * r * th.sin()));
    }
    let ts: Vec<Vec<(f64, f64)>> = starts.iter().map(|st| traj(s, &f, &g, *st, 0.05, 4, 28, 3.0 * r + x0.abs() + y0.abs()))
        .collect();
    let cap = format!("Trajectories of x' = {f}, y' = {g} from {} starts round ({}, {}), each point one run of the model's \
                       fourth-order system solver (step 0.05, 4 steps per point). The model classifies the equilibrium \
                       ({}, {}) from its Jacobian (trace {}, determinant {}) as {ty}.", starts.len(), num(x0), num(y0),
                      num(x0), num(y0), num(tr), num(det));
    portrait_fig(s, ts, (x0, y0), &format!("({}, {}) {ty}", num(x0), num(y0)), "x", "y", &format!("{ty}"),
                 "Phase portrait round the equilibrium", cap)
}

pub fn rk4_orbit(s: &Solved) -> Option<Figure> {
    let (f, g, x0, y0, h, n) = (s.txt("f")?, s.txt("g")?, nv(s, "x0")?, nv(s, "y0")?, nv(s, "h")?, nv(s, "n")?);
    let (xn, yn, tn) = (s.out("x_n")?, s.out("y_n")?, s.out("t_n")?);
    if !(h != 0.0) {
        return None;
    }
    let far = 1e3 * (x0.abs() + y0.abs() + 1.0);
    let k1 = (n as usize).clamp(60, 120);
    let small = traj(s, &f, &g, (x0, y0), h, 1, k1, far);
    // the same span of time in steps five times larger
    let big = traj(s, &f, &g, (x0, y0), h * 5.0, 1, (k1 / 5).max(6), far);
    if small.len() < 3 {
        return None;
    }
    let pts: Vec<(f64, f64)> = small.iter().chain(big.iter()).copied().collect();
    let (xl, xh) = range_of(pts.iter().map(|q| q.0).chain([xn]))?;
    let (yl, yh) = range_of(pts.iter().map(|q| q.1).chain([yn]))?;
    let mut p = DPlot::equal(&[xl, xh], &[yl, yh], "x", "y", &format!("x' = {f}, y' = {g}"));
    if big.len() >= 3 {
        p.curve(&big, 2);
        for q in &big {
            p.dot(q.0, q.1, 2, 2.5);
        }
        p.key(&format!("h = {}", num(h * 5.0)), 2);
    }
    p.curve(&small, 0);
    p.key(&format!("h = {}", num(h)), 0);
    p.mark(xn, yn, &format!("t = {}: ({}, {})", num(tn), num(xn), num(yn)));
    let sl = *small.last()?;
    let cap = format!("The system x' = {f}, y' = {g} from ({}, {}) stepped by the model's fourth-order system solver, one \
                       run per step: with h = {} ({} steps, last state ({}, {})) and with h = {} ({} steps{}). Marked: the \
                       request's state at t = {}, ({}, {}).", num(x0), num(y0), num(h), small.len() - 1, num(sl.0),
                      num(sl.1), num(h * 5.0), big.len().saturating_sub(1),
                      big.last().map(|b| format!(", last state ({}, {})", num(b.0), num(b.1))).unwrap_or_default(),
                      num(tn), num(xn), num(yn));
    Some(s.chart("phase", "Fourth-order Runge-Kutta orbits: small and large steps", &cap, &p.body()))
}

pub fn linear_portrait(s: &Solved) -> Option<Figure> {
    let a = s.mat("A")?;
    if a.len() != 2 || a[0].len() != 2 {
        return None;
    }
    let ty = s.out_text("type")?;
    let st = s.out_text("stability").unwrap_or_default();
    let (l1, l2) = (super::cx_out(s.a, "lambda1")?, super::cx_out(s.a, "lambda2")?);
    let am = format!("[[{}, {}], [{}, {}]]", a[0][0], a[0][1], a[1][0], a[1][1]);
    // a growing solution leaves the window quickly: a shorter time span for a positive real part (the model's own
    // eigenvalues)
    let grow = l1.0.max(l2.0);
    let tmax = if grow > 0.0 { (2.0 / grow).min(4.0) } else { 4.0 };
    let ts_ = k::span(0.0, tmax, 18, false);
    let mut ts = vec![];
    for i in 0..8 {
        let th = i as f64 * std::f64::consts::PI / 4.0 + 0.2;
        let x0 = format!("{}, {}", th.cos(), th.sin());
        let mut path = vec![(th.cos(), th.sin())];
        for t in ts_.iter().skip(1) {
            let Some(an) = s.solve_other("linear_system_solution", &[("A", tx(&am)), ("x0", tx(&x0)), ("t", fl(*t))]) else { break };
            let Some(v) = k::list_of(&an, "x") else { break };
            if v.len() != 2 {
                break;
            }
            path.push((v[0], v[1]));
            if v[0].abs() > 50.0 || v[1].abs() > 50.0 {
                break;
            }
        }
        ts.push(path);
    }
    let mut p = DPlot::equal(&[-1.6, 1.6], &[-1.6, 1.6], "x1", "x2", &format!("{ty}: {st}"));
    p.circle_guide();
    trajectories_plot(&mut p, &ts);
    p.mark(0.0, 0.0, &ty);
    let cap = format!("Trajectories of x' = A x with A = {am}, from 8 starting points on the unit circle, each state x(t) for \
                       {} times from t = 0 to {} computed by the model's linear-system solver. The model classifies the \
                       origin as {ty} ({st}), eigenvalues {} and {}.", ts_.len(), num(tmax), cx_text(l1), cx_text(l2));
    Some(s.chart("phase", "Phase portrait of the linear system", &cap, &p.body()))
}

pub fn trace_det(s: &Solved) -> Option<Figure> {
    let (tr, det) = (s.out("trace")?, s.out("det")?);
    let ty = s.out_text("type")?;
    let m = (1.5 * tr.abs()).max(4.0);
    let d_hi = (1.5 * det.abs()).max(m * m / 4.0 * 1.3).max(4.0);
    let xs = k::span(-m, m, 15, false);
    let ys = k::span(-0.4 * d_hi, d_hi, 11, false);
    let mut cells = vec![];
    let mut names: Vec<String> = vec![];
    for (i, t) in xs.iter().enumerate() {
        for (j, d) in ys.iter().enumerate() {
            let am = format!("[[0, 1], [{}, {}]]", -d, t);
            let Some(a) = s.solve_other("phase_portrait_2x2", &[("A", tx(&am))]) else { continue };
            let Some(v) = text_of(&a, "type") else { continue };
            if !names.contains(&v) {
                names.push(v.clone());
            }
            cells.push((i, j, v));
        }
    }
    if cells.len() < 10 {
        return None;
    }
    let (body, h) = super::verdict_map(&xs, &ys, &cells, &names, (tr, det), "Trace", "Determinant",
                                       &format!("{ty}: trace {}, det {}", num(tr), num(det)));
    let counts: Vec<String> = names.iter().map(|n| format!("{n} {}", cells.iter().filter(|c| &c.2 == n).count())).collect();
    let cap = format!("Equilibrium types on the trace-determinant plane: each of the {} x {} cells is a 2 x 2 system with \
                       that trace and determinant ([[0, 1], [-det, trace]]) classified by the model ({} cells). Marked: \
                       the request's matrix, trace {}, determinant {}: {ty}.", xs.len(), ys.len(), counts.join(", "),
                      num(tr), num(det));
    Some(s.figure("stability", "Equilibrium types on the trace-determinant plane", &cap, &body, h))
}

pub fn slope_field(s: &Solved) -> Option<Figure> {
    let (f, t, y) = (s.txt("f")?, nv(s, "t")?, nv(s, "y")?);
    let sl = s.out("slope")?;
    let tw = 2.0f64.max(t.abs() * 0.5);
    let yw = 2.0f64.max(y.abs() * 0.5);
    let ts = k::span(t - tw, t + tw, 15, false);
    let ys = k::span(y - yw, y + yw, 9, false);
    let mut p = DPlot::new((t - tw, t + tw), (y - yw, y + yw), "t", "y", &format!("y' = {f}"));
    let mut n = 0;
    for tv in &ts {
        for yv in &ys {
            let Some(m) = slope(s, &f, *tv, *yv) else { continue };
            n += 1;
            let (a, b) = (p.px(*tv, *yv), p.px(tv + 1e-3 * tw, yv + m * 1e-3 * tw));
            let (dx, dy) = (b.0 - a.0, b.1 - a.1);
            let d = (dx * dx + dy * dy).sqrt();
            if !(d > 0.0) || !d.is_finite() {
                continue;
            }
            let (ux, uy) = (dx / d * 9.0, dy / d * 9.0);
            p.raw(&format!("<path class=\"axp-c4\" d=\"M{:.1},{:.1} L{:.1},{:.1}\"/>", a.0 - ux, a.1 - uy, a.0 + ux, a.1 + uy));
        }
    }
    if n < 10 {
        return None;
    }
    // solutions by the model's fourth-order solver: each point one run from the previous one
    let h = 2.0 * tw / 40.0;
    let mut sols = 0;
    for (i, y0) in [y - 0.6 * yw, y, y + 0.6 * yw].iter().enumerate() {
        let (start, pts) = if i == 1 { ((t, y), 20) } else { ((t - tw, *y0), 40) };
        let mut path = vec![start];
        let mut cur = start;
        for _ in 0..pts {
            let Some(a) = s.solve_other("runge_kutta_4", &[("f", tx(&f)), ("t0", fl(cur.0)), ("y0", fl(cur.1)), ("h", fl(h)),
                                                          ("n", int(1))]) else { break };
            let (Some(tn), Some(yn)) = (num_of(&a, "t_n"), num_of(&a, "y_n")) else { break };
            cur = (tn, yn);
            path.push(cur);
            if (yn - y).abs() > 4.0 * yw {
                break;
            }
        }
        if path.len() > 2 {
            sols += 1;
            p.curve(&path, [2, 0, 1][i]);
        }
    }
    p.mark(t, y, &format!("slope {} at ({}, {})", num(sl), num(t), num(y)));
    let cap = format!("Slope field of y' = {f}: the direction at {n} grid points, each slope from the model, with {sols} \
                       solutions stepped by its fourth-order solver (step {}, one run per step; the accent one through the \
                       request's point). Marked: the slope {} at (t, y) = ({}, {}).", num(h), num(sl), num(t), num(y));
    Some(s.chart("phase", "Slope field and solution curves", &cap, &p.body()))
}

trait Guide {
    fn circle_guide(&mut self);
}
impl Guide for DPlot {
    /// The unit circle (dashed), where the starting points lie.
    fn circle_guide(&mut self) {
        let c: Vec<(f64, f64)> = (0..=72).map(|i| {
            let th = i as f64 * std::f64::consts::PI / 36.0;
            (th.cos(), th.sin())
        }).collect();
        self.dashed(&c, 3);
    }
}
