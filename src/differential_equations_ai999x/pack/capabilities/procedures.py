"""Differential-equations procedures: deterministic algorithms with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

Scope: first- and second-order ODEs (closed-form solutions of the standard models), forced linear equations,
linear systems and phase portraits, Laplace transforms, stability of equilibria, numerical methods (one-step
methods, stability limits, finite differences), classic PDEs by separation of variables (heat, wave, Laplace) and
dynamical systems (logistic map, Lotka-Volterra, Lorenz, Brusselator, SIR).

Conventions (fixed, so run and reference agree exactly):
  * Every question is "<capability phrase> ... name = value, name = value". y(0) = .. is read as y0, y'(0) = .. as v0;
    "find y(2)" asks for the value at t = 2. A number or a name the capability does not take is refused.
  * Expressions given as text (a right-hand side f(t, y), an initial profile f(x), a transform F(s)) are read by the
    runtime's allowlisted expression parser (runtime/expr.py; nothing is evaluated as code). Write products with *
    or by juxtaposition (2t, 3y), powers with ^ or **; e is Euler's number.
  * Outputs built from exp/log/trig are rounded to 12 significant digits (identical answers on every platform);
    results below 1e-13 of the scale of their parts are snapped to 0 or refused when cancellation makes them unsound.
  * Roots and eigenvalues: real ascending, then complex by real part with the positive imaginary part first.
  * Numerical methods use t_k = t0 + k*h exactly as written (no accumulated time step).

Every procedure carries an INDEPENDENT reference used only by certification: a closed form is checked against
numerical integration of the differential equation (mpmath's Taylor-series ODE solver at 20+ digits, or scipy's
solve_ivp at tight tolerance), against scipy.linalg / scipy.signal / numpy, or against sympy's symbolic solvers;
and a deliberately wrong variant that the checks must reject.
"""
from __future__ import annotations

import cmath
import functools
import math
import re
from fractions import Fraction

import sympy as sp

try:                                                   # the installed package: the engine's own parser
    from .expr import ExpressionError as _ExprError, parse as _allow_parse
except ImportError:                                    # the factory's source tree / certification
    from ai999x_factory.runtime.expr import ExpressionError as _ExprError, parse as _allow_parse

# ------------------------------------------------------------------------------------------- numbers and rounding
TS, XS, YS, SS = sp.Symbol("t", real=True), sp.Symbol("x", real=True), sp.Symbol("y", real=True), sp.Symbol("s")
_SYM = {"t": TS, "x": XS, "y": YS, "s": SS}


def _r(v):
    """12 significant digits (portable output); complex parts separately."""
    if isinstance(v, complex):
        return complex(_r(v.real), _r(v.imag))
    v = float(v)
    if not math.isfinite(v):
        raise ValueError("the result is not finite (it overflows)")
    return 0.0 if v == 0 else float(f"{v:.12g}")


def _exp(x):
    try:
        return math.exp(x)
    except OverflowError:
        raise ValueError("the result overflows (exponent too large)") from None


def _guard(total, *parts):
    """Refuse a result that is lost to cancellation (its parts are huge compared with the sum)."""
    big = sum(abs(p) for p in parts)
    if big > 1e5 * max(1.0, abs(total)):
        raise ValueError("the result is lost to cancellation between large terms (ill-conditioned inputs)")
    if abs(total) < 1e-13 * big:
        return 0.0
    return total


def _fin(*vals):
    for v in vals:
        if not (isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)):
            raise ValueError("every input must be a finite number")


def _safe(fn):
    try:
        return fn()
    except Exception:
        return float("nan")


def _diff(a, b):
    """Relative difference of two outputs (0 when equal); structured outputs compared entry by entry."""
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)):
        return 0.0 if a == b else 1.0
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return 1.0
        return max((_diff(x, y) for x, y in zip(a, b)), default=0.0)
    if a is None or b is None:
        return 0.0 if a is b else 1.0
    try:
        d = abs(complex(a) - complex(b))
        if d != d:
            return 1.0
        return 0.0 if d <= 1e-15 else d / max(abs(complex(a)), abs(complex(b)))
    except (TypeError, ValueError):
        return 1.0


def _verify_by(fn):
    """A verifier: the outputs against an alternative plain computation (`fn`), relative difference."""
    def verify(inputs, outputs):
        try:
            want = fn(**inputs)
        except Exception:
            return 1.0
        return max((_diff(outputs.get(k), v) for k, v in want.items()), default=0.0)
    return verify


# ------------------------------------------------------------------------------------------- expressions as text
_FUNCS = ("sqrt", "sin", "cos", "tan", "exp", "log", "asin", "acos", "atan", "sinh", "cosh", "tanh")


def _prep(text: str, names) -> str:
    """Written mathematics -> the allowlisted syntax: ^ -> **, implicit products (2t, 3(y+1), ty, y(1-y)), e -> exp(1)."""
    s = text.strip().replace("^", "**").replace("−", "-").replace("·", "*").replace("×", "*").replace("π", "pi")
    s = s.replace("²", "**2").replace("³", "**3").replace("[", "(").replace("]", ")")
    s = re.sub(r"\bln\b", "log", s)
    s = re.sub(r"(\d)\s*(?![eE][-+]?\d)([A-Za-z(])", r"\1*\2", s)            # 2t, 3(y+1); not 1e-3

    def word(m):
        w, after = m.group(0), s[m.end():m.end() + 1]
        if w in _FUNCS:
            return w
        if w == "pi":
            return w
        if w == "e":
            return "exp(1)"
        if w in names:
            return w + ("*" if after == "(" else "")
        if all(c in names for c in w):                                         # ty -> t*y
            return "*".join(w) + ("*" if after == "(" else "")
        return w
    s = re.sub(r"(?<![\w.])[A-Za-z_]+", word, s)
    s = re.sub(r"\)\s*([A-Za-z0-9(])", r")*\1", s)                             # (t+1)(t-1), (t+1)y
    return s


def parse_text(text, names=("t", "y")) -> sp.Expr:
    """A text expression over the named variables, through the runtime's allowlisted parser; ValueError otherwise."""
    if not isinstance(text, str) or not text.strip():
        raise ValueError("the expression is missing")
    src = _prep(text, names)
    hyp = {}
    for h in ("sinh", "cosh", "tanh"):                 # not in the runtime's function list: rewrite through exp
        if h in src:
            hyp[h] = True
    src2 = src
    for h in hyp:
        src2 = re.sub(r"\b" + h + r"\(", "_" + h + "(", src2)
    syms = {n: _SYM[n] for n in names}
    try:
        if hyp:
            e = _parse_hyp(src2, syms)
        else:
            e = _allow_parse(src2, syms, symbolic_powers=True)
    except _ExprError as exc:
        raise ValueError(f"cannot read the expression {text!r}: {exc}") from None
    except (ZeroDivisionError, TypeError):
        raise ValueError(f"cannot read the expression {text!r}") from None
    if e.has(sp.zoo, sp.nan, sp.oo, -sp.oo):
        raise ValueError("the expression is undefined")
    return e


def _parse_hyp(src, syms):
    """sinh/cosh/tanh(u): parse the argument with the allowlisted parser and rebuild through exp (still allowlisted)."""
    out, i = "", 0
    pieces = {}
    while True:
        m = re.search(r"_(sinh|cosh|tanh)\(", src[i:])
        if not m:
            out += src[i:]
            break
        j0 = i + m.end() - 1
        depth = 0
        for j in range(j0, len(src)):
            depth += {"(": 1, ")": -1}.get(src[j], 0)
            if depth == 0:
                break
        if depth != 0:
            raise _ExprError("unbalanced parentheses")
        arg = _allow_parse(src[j0 + 1:j], syms, symbolic_powers=True)
        name = f"H{len(pieces)}"
        pieces[name] = {"sinh": sp.sinh, "cosh": sp.cosh, "tanh": sp.tanh}[m.group(1)](arg)
        out += src[i:i + m.start()] + name
        i = j + 1
    tmp = {**syms, **{k: sp.Symbol(k) for k in pieces}}
    e = _allow_parse(out, tmp, symbolic_powers=True)
    return e.subs({tmp[k]: v for k, v in pieces.items()})


_MF = {"sin": math.sin, "cos": math.cos, "tan": math.tan, "exp": math.exp, "log": math.log, "asin": math.asin,
       "acos": math.acos, "atan": math.atan, "sinh": math.sinh, "cosh": math.cosh, "tanh": math.tanh}


def compile_float(e, names):
    """A float callable f(*values) for a parsed expression, built node by node (no code is generated)."""
    idx = {_SYM[n]: i for i, n in enumerate(names)}

    def build(e):
        if e.is_Symbol:
            if e not in idx:
                raise ValueError(f"unexpected variable {e}")
            i = idx[e]
            return lambda v: v[i]
        if e.is_Rational:
            c = e.p / e.q
            return lambda v: c
        if e is sp.pi or e is sp.E or e.is_Float:
            c = float(e)
            return lambda v: c
        if e.is_Add:
            fs = [build(a) for a in e.args]
            return lambda v: math.fsum(f(v) for f in fs)
        if e.is_Mul:
            fs = [build(a) for a in e.args]

            def mul(v):
                out = 1.0
                for f in fs:
                    out *= f(v)
                return out
            return mul
        if e.is_Pow:
            b = build(e.base)
            if e.exp.is_Integer:
                k = int(e.exp)
                return lambda v: b(v) ** k
            if e.exp == sp.Rational(1, 2):
                return lambda v: math.sqrt(b(v))
            ex = build(e.exp)

            def pw(v):
                r = b(v) ** ex(v)
                if isinstance(r, complex):
                    raise ValueError("not a real number")
                return r
            return pw
        if e.func.__name__ in _MF and len(e.args) == 1:
            f, a = _MF[e.func.__name__], build(e.args[0])
            return lambda v: f(a(v))
        raise ValueError(f"cannot evaluate {e.func.__name__}")
    g = build(e)

    def call(*vals):
        try:
            r = g(vals)
        except (ZeroDivisionError, OverflowError, ValueError) as exc:
            raise ValueError(f"the expression is not defined there ({type(exc).__name__})") from None
        if not math.isfinite(r):
            raise ValueError("the expression is not finite there")
        return r
    return call


@functools.lru_cache(maxsize=512)
def _fn(text, names=("t", "y")):
    return compile_float(parse_text(text, names), names)


def ref_fn(text, names=("t", "y")):
    """Build-time only: sympy's own mpmath evaluator of the parsed expression (independent of compile_float)."""
    e = parse_text(text, names)
    return sp.lambdify([_SYM[n] for n in names], e, modules="mpmath")


# ------------------------------------------------------------------------------------------- question reading
_NUMRE = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?(?:\s*/\s*(?:\d+(?:\.\d*)?|\.\d+))?"


def _num(s):
    s = s.replace(" ", "")
    if "/" in s:
        a, b = s.split("/")
        if Fraction(b) == 0:
            raise ValueError("division by zero")
        return float(Fraction(a) / Fraction(b))
    return float(s)


def _norm(q: str) -> str:
    q = q.replace("−", "-").replace("’", "'").replace("′", "'").replace("″", "''")
    q = re.sub(r"\b([xy])'\(\s*0\s*\)\s*=", "v0 =", q)
    q = re.sub(r"\b([A-Za-z])\(\s*0\s*\)\s*=", r"\g<1>0 =", q)
    return q


_TRIG: dict = {}


def _both(a, b):
    return rf"(?:{a}).*(?:{b})|(?:{b}).*(?:{a})"


def _trig(key, priority, pats, unless=()):
    _TRIG[key] = (priority, [re.compile(p) for p in pats], [re.compile(u) for u in unless])


@functools.lru_cache(maxsize=1024)
def _owner(q: str):
    ql = _norm(q).lower()
    best, who = None, []
    for k, (pr, pats, unless) in _TRIG.items():
        if any(u.search(ql) for u in unless):
            continue
        hits = [m for m in (p.search(ql) for p in pats) if m]
        if not hits:
            continue
        score = (pr, max(len(m.group(0)) for m in hits))
        if best is None or score > best:
            best, who = score, [k]
        elif score == best:
            who.append(k)
    return who[0] if len(who) == 1 else None


_TARGET = r"\b(?:y|x|P|T|N|A|v|h|u|S|I|Q)\(\s*(" + _NUMRE + r")\s*\)(?!\s*=)"
_ASSIGN = r"(?<![\w'/])([A-Za-z][A-Za-z0-9_]*)\s*=\s*(" + _NUMRE + r")(?!\d)(?!\.\d)(?!\s*[*/^(])(?![A-Za-z])"
_STOP = r"(?=\s*(?:,|;|\bwith\b|\bfrom\b|\busing\b|\bwhere\b|\band\b|\bfor\b|\bat\b|\?|\.\s|\.$|$))"


def _rhs(lhs_alts):
    return r"(?:" + lhs_alts + r")\s*=\s*(.+?)" + _STOP


RHS_TY = _rhs(r"dy/dt|y'|f\(t,\s*y\)")


def parser(key, names, text=None, mats=(), target="t", post=None, pre=None):
    """Read '<phrase> ... name = value, ...' for capability `key`: None unless the phrase names this capability;
    any assignment to a name the capability does not take refuses the reading."""
    names = tuple(names)

    def parse(q):
        if _owner(q) != key:
            return None
        nq = _norm(q)
        out = {}
        if pre:
            got = pre(nq)
            if got is None:
                return None
            nq, out = got
        for nm, pat in (text or {}).items():
            m = re.search(pat, nq, re.IGNORECASE)
            if m:
                out[nm] = m.group(1).strip()
                nq = nq[:m.start()] + " ; " + nq[m.end():]
        for nm in mats:
            m = re.search(r"(?<![\w'])" + nm + r"\s*=\s*(\[\[.*?\]\]|\[[^\[\]]*\])", nq)
            if m:
                out[nm] = m.group(1)
                nq = nq[:m.start()] + " ; " + nq[m.end():]
        for name, val in re.findall(_ASSIGN, nq):
            if name not in names:
                return None
            try:
                out[name] = _num(val)
            except (ValueError, ZeroDivisionError):
                return None
        if "t0" in names and "t0" not in out and re.search(r"\b[xy]\(\s*0\s*\)\s*=", q):
            out["t0"] = 0.0                                    # y(0) = .. states the initial time
        if target and target in names and target not in out:
            m = re.search(_TARGET, nq)
            if m:
                out[target] = _num(m.group(1))
        if post:
            out = post(nq, out)
        return out
    return parse


PROCEDURES: dict = {}


def N(sym, meaning, **kw):
    return {"symbol": sym, "kind": "number", "meaning": meaning, "required": kw.pop("required", True), **kw}


def O(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


def add(key, *, name, topic, purpose, inputs, outputs, primary, conditions, method, latex, run, reference, wrong,
        rand, templates, examples, questions, trig, verify=None, text=None, mats=(), target="t", post=None,
        unless=(), pre=None):
    pr, pats = trig
    _trig(key, pr, pats, unless)
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": conditions, "method": method, "latex": latex, "run": run,
        "verify": verify or _verify_by(run), "reference": reference, "wrong": wrong, "random_inputs": rand,
        "parse": parser(key, [i["symbol"] for i in inputs], text, mats, target, post, pre),
        "templates": templates, "examples": [{"inputs": i, "outputs": o} for i, o in examples],
        "questions": questions, "keywords": [p for p in pats if re.fullmatch(r"[a-z' -]+", p)]}


# ------------------------------------------------------------------------------------------- reference helpers
def ivp_ref(F, t0, y0, t):
    """Build-time only: scipy's DOP853 (8th order) at rtol 1e-13 from t0 to t (a fast independent integration)."""
    from scipy.integrate import solve_ivp
    y_init = list(y0) if isinstance(y0, (list, tuple)) else [y0]
    sol = solve_ivp(lambda s, u: F(s, u), (t0, t), y_init, method="DOP853", rtol=1e-13, atol=1e-15)
    v = sol.y[:, -1]
    return [float(x) for x in v] if isinstance(y0, (list, tuple)) else float(v[0])


def _mpq(x):
    import mpmath
    f = Fraction(x)
    return mpmath.mpf(f.numerator) / f.denominator


def _ln2_rate(T):
    """Build-time only: the rate k with exp(-k T) = 1/2, found by mpmath's root finder (not the closed form)."""
    import mpmath
    with mpmath.workdps(25):
        lo, hi = mpmath.mpf(0), mpmath.mpf(5) / T             # exp(-k T) - 1/2 decreases in k: bisection
        for _ in range(100):
            mid = (lo + hi) / 2
            lo, hi = (mid, hi) if mpmath.exp(-mid * T) > mpmath.mpf(1) / 2 else (lo, mid)
        return float((lo + hi) / 2)


def ode_ref(F, t0, y0, t, dps=20):
    """Build-time only: mpmath's Taylor-series ODE solver (arbitrary precision) from t0 to t."""
    import mpmath
    with mpmath.workdps(dps):
        y_init = [mpmath.mpf(v) for v in y0] if isinstance(y0, (list, tuple)) else mpmath.mpf(y0)
        sol = mpmath.odefun(F, mpmath.mpf(t0), y_init)
        v = sol(mpmath.mpf(t))
        return [float(x) for x in v] if isinstance(v, (list, tuple)) else float(v)


def ode_root_ref(F, t0, y0, level, guess, dps=20):
    """Build-time only: the time at which the numerically integrated solution reaches `level` (mpmath findroot)."""
    import mpmath
    with mpmath.workdps(dps):
        sol = mpmath.odefun(F, mpmath.mpf(t0), mpmath.mpf(y0))
        g = mpmath.mpf(guess)
        f = lambda s: sol(s) - level
        lo, hi = g / 2, g * 2
        while f(lo) * f(hi) > 0 and hi < 1e6 * g:             # widen until the level is bracketed
            lo, hi = lo / 2, hi * 2
        flo = f(lo)
        for _ in range(90):                                   # bisection on the integrated solution
            mid = (lo + hi) / 2
            fm = f(mid)
            if (fm > 0) == (flo > 0):
                lo, flo = mid, fm
            else:
                hi = mid
        return float((lo + hi) / 2)


def _u(rng, a, b, nd=2):
    return round(rng.uniform(a, b), nd)


def _nz(rng, a, b, nd=2):
    while True:
        v = round(rng.uniform(a, b), nd)
        if v != 0:
            return v


# =================================================================================================================
# FIRST-ORDER MODELS
# =================================================================================================================
def exp_growth(y0, k, t):
    _fin(y0, k, t)
    return {"y": _r(y0 * _exp(k * t))}


add("exponential_growth_value", name="Exponential growth or decay: value at a time",
    topic="first-order ODEs / exponential models",
    purpose="Solve dy/dt = k*y with y(0) = y0 and give y(t).",
    inputs=[N("y0", "initial value y(0)"), N("k", "growth rate (negative for decay)"), N("t", "time")],
    outputs=[O("y", "value y(t)")], primary="y", conditions=["k > 0 growth, k < 0 decay"],
    method="separation of variables: y = y0*exp(k*t)", latex=r"y(t) = y_0 e^{k t}",
    run=exp_growth, reference=lambda y0, k, t: {"y": ode_ref(lambda s, y: k * y, 0, y0, t)},
    wrong=lambda y0, k, t: {"y": y0 * (1 + k) ** t},
    verify=_verify_by(lambda y0, k, t: {"y": _r(y0 * 2 ** (k * t / math.log(2)))}),
    rand=lambda rng: {"y0": _u(rng, 1, 1000), "k": _nz(rng, -0.5, 0.5, 3), "t": _u(rng, 0.5, 10)},
    templates=["Exponential growth with y0 = 100, k = 0.03, t = 10. Find y.",
               "Exponential decay: y(0) = 50, k = -0.2; find y(4)."],
    examples=[({"y0": 100, "k": 0.03, "t": 10}, {"y": 134.985880758}),
              ({"y0": 50, "k": -0.2, "t": 4}, {"y": 22.4664482058})],
    questions=[{"q": "Exponential growth with y0 = 100, k = 0.03, t = 10. Find y.", "outputs": {"y": 134.985880758}},
               {"q": "Exponential decay: y(0) = 50, k = -0.2; find y(4).", "outputs": {"y": 22.4664482058}},
               {"q": "Exponential growth with y0 = 100, k = 0.03.", "outcome": "NEEDS_INPUT"}],
    trig=(1, [r"exponential (growth|decay)", r"grows exponentially", r"decays exponentially"]))


def half_life_decay(N0, T, t):
    _fin(N0, T, t)
    if T <= 0:
        raise ValueError("the half-life T must be positive")
    return {"N": _r(N0 * 2.0 ** (-t / T)), "k": _r(math.log(2) / T)}


def _ref_half_life(N0, T, t):
    k = _ln2_rate(T)
    return {"N": ode_ref(lambda s, y: -k * y, 0, N0, t), "k": k}


add("half_life_decay", name="Radioactive decay from a half-life", topic="first-order ODEs / exponential models",
    purpose="Amount left after time t when dN/dt = -k*N and the half-life is T; also the decay constant k.",
    inputs=[N("N0", "initial amount"), N("T", "half-life (> 0)"), N("t", "elapsed time")],
    outputs=[O("N", "amount remaining N(t)"), O("k", "decay constant ln 2 / T")], primary="N",
    conditions=["T > 0"], method="N = N0 * 2^(-t/T), k = ln 2 / T", latex=r"N(t) = N_0\,2^{-t/T},\quad k = \frac{\ln 2}{T}",
    run=half_life_decay, reference=_ref_half_life,
    wrong=lambda N0, T, t: {"N": N0 * math.exp(-t / T), "k": 1 / T},
    verify=_verify_by(lambda N0, T, t: {"N": _r(N0 * math.exp(-math.log(2) * t / T)), "k": _r(math.log(2) / T)}),
    rand=lambda rng: {"N0": _u(rng, 1, 500), "T": _u(rng, 0.5, 30), "t": _u(rng, 0.5, 60)},
    templates=["A sample with half-life T = 5730 has N0 = 100; find N at t = 2000.",
               "Half-life decay: N0 = 80, T = 3, t = 9."],
    examples=[({"N0": 80, "T": 3, "t": 9}, {"N": 10.0}), ({"N0": 100, "T": 5730, "t": 2000}, {"N": 78.5106274623})],
    questions=[{"q": "Half-life decay: N0 = 80, T = 3, t = 9.", "outputs": {"N": 10.0}},
               {"q": "A sample with half-life T = 5730 has N0 = 100; find N at t = 2000.", "outputs": {"N": 78.5106274623}},
               {"q": "Half-life decay: N0 = 80, T = -3, t = 9.", "outcome": "INVALID_INPUT"}],
    trig=(2, [r"half-?\s?life"]))


def char_time(k):
    _fin(k)
    if k == 0:
        raise ValueError("k must not be 0 (no growth or decay)")
    return {"t_char": _r(math.log(2) / abs(k)), "kind": "doubling time" if k > 0 else "half-life"}


def _ref_char_time(k):
    return {"t_char": ode_root_ref(lambda s, y: k * y, 0, 1, 2.0 if k > 0 else 0.5, math.log(2) / abs(k))}


add("doubling_time", name="Doubling time or half-life from a rate constant", topic="first-order ODEs / exponential models",
    purpose="For dy/dt = k*y: the doubling time (k > 0) or half-life (k < 0).",
    inputs=[N("k", "rate constant", condition="k != 0")],
    outputs=[O("t_char", "doubling time or half-life ln 2 / |k|"), O("kind", "doubling time or half-life", "text")],
    primary="t_char", conditions=["k != 0"], method="solve e^{k t} = 2 (or 1/2): t = ln 2 / |k|",
    latex=r"t_{2} = \frac{\ln 2}{|k|}", run=char_time, reference=_ref_char_time,
    wrong=lambda k: {"t_char": 1 / abs(k), "kind": "?"},
    verify=_verify_by(lambda k: {"t_char": _r(math.log1p(1) / abs(k)), "kind": "doubling time" if k > 0 else "half-life"}),
    rand=lambda rng: {"k": _nz(rng, -1.5, 1.5, 3)},
    templates=["Find the doubling time for exponential growth with k = 0.07.", "What is the doubling time when k = 0.25?"],
    examples=[({"k": 0.07}, {"t_char": 9.90210257942}), ({"k": -0.1}, {"t_char": 6.93147180560, "kind": "half-life"})],
    questions=[{"q": "Find the doubling time for exponential growth with k = 0.07.", "outputs": {"t_char": 9.90210257942}},
               {"q": "What is the doubling time when k = 0?", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"doubling time", r"half-?\s?life\b.*\b(rate|decay) constant", r"(rate|decay) constant\b.*\bhalf-?\s?life"]))


def time_to_reach(y0, y1, k):
    _fin(y0, y1, k)
    if k == 0 or y0 == 0 or y1 / y0 <= 0:
        raise ValueError("needs k != 0 and y1, y0 of the same sign")
    t = math.log(y1 / y0) / k
    if t < 0:
        raise ValueError("the target is not reached for t >= 0 with this rate")
    return {"t": _r(t)}


def _ref_time_to_reach(y0, y1, k):
    if y1 == y0:
        return {"t": 0.0}
    return {"t": ode_root_ref(lambda s, y: k * y, 0, y0, y1, math.log(y1 / y0) / k)}


add("exponential_time_to_reach", name="Time for exponential growth or decay to reach a value",
    topic="first-order ODEs / exponential models",
    purpose="For dy/dt = k*y, y(0) = y0: the time at which y reaches y1.",
    inputs=[N("y0", "initial value"), N("y1", "target value"), N("k", "rate constant", condition="k != 0")],
    outputs=[O("t", "time to reach y1")], primary="t", conditions=["k != 0", "y1/y0 > 0", "the target lies ahead (t >= 0)"],
    method="t = ln(y1/y0)/k", latex=r"t = \frac{1}{k}\ln\frac{y_1}{y_0}", run=time_to_reach, reference=_ref_time_to_reach,
    wrong=lambda y0, y1, k: {"t": math.log10(y1 / y0) / k},
    verify=_verify_by(lambda y0, y1, k: {"t": _r((math.log(abs(y1)) - math.log(abs(y0))) / k)}),
    rand=lambda rng: (lambda k, y0: {"y0": y0, "k": k, "y1": round(y0 * (rng.uniform(1.2, 20) if k > 0 else rng.uniform(0.05, 0.9)), 2)})(
        _nz(rng, -0.8, 0.8, 3), _u(rng, 1, 500)),
    templates=["Exponential growth from y0 = 100 at k = 0.05: how long to reach y1 = 300?",
               "How long until exponential decay with y0 = 80, k = -0.3 reaches y1 = 10?"],
    examples=[({"y0": 100, "y1": 300, "k": 0.05}, {"t": 21.9722457734}), ({"y0": 80, "y1": 10, "k": -0.3}, {"t": 6.93147180560})],
    questions=[{"q": "Exponential growth from y0 = 100 at k = 0.05: how long to reach y1 = 300?", "outputs": {"t": 21.9722457734}},
               {"q": "How long until exponential decay with y0 = 80, k = -0.3 reaches y1 = 100?", "outcome": "INVALID_INPUT"}],
    trig=(3, [_both(r"exponential", r"\bhow long\b|\btime (to|until|needed|required)\b")]), target=None)


def two_points(t1, y1, t2, y2):
    _fin(t1, y1, t2, y2)
    if t1 == t2 or y1 == 0 or y2 / y1 <= 0:
        raise ValueError("needs two different times and two nonzero values of the same sign")
    k = math.log(y2 / y1) / (t2 - t1)
    return {"k": _r(k), "y0": _r(y1 * _exp(-k * t1))}


def _ref_two_points(t1, y1, t2, y2):
    # y'=k y scales time: the unit-rate solution u' = +-u reaches the ratio y2/y1 at tau, so |k| = tau/|t2 - t1|
    R = y2 / y1
    if R == 1:
        k = 0.0
    else:
        sgn = 1.0 if (R > 1) == (t2 > t1) else -1.0
        up = R if R > 1 else 1 / R
        tau = ode_root_ref(lambda s, y: y, 0, 1, up, math.log(up))
        k = sgn * tau / abs(t2 - t1)
    return {"k": k, "y0": y1 / ivp_ref(lambda s, u: [k * u[0]], 0, 1.0, t1) if t1 else y1}


add("exponential_from_two_points", name="Exponential model through two measurements",
    topic="first-order ODEs / exponential models",
    purpose="The rate k and initial value y0 of y = y0*e^{k t} through (t1, y1) and (t2, y2).",
    inputs=[N("t1", "first time"), N("y1", "value at t1"), N("t2", "second time"), N("y2", "value at t2")],
    outputs=[O("k", "rate constant"), O("y0", "value at t = 0")], primary="k",
    conditions=["t1 != t2", "y1, y2 nonzero, same sign"], method="k = ln(y2/y1)/(t2 - t1), y0 = y1 e^{-k t1}",
    latex=r"k = \frac{\ln(y_2/y_1)}{t_2 - t_1},\; y_0 = y_1 e^{-k t_1}", run=two_points, reference=_ref_two_points,
    wrong=lambda t1, y1, t2, y2: {"k": (y2 - y1) / (y1 * (t2 - t1)), "y0": y1},
    verify=_verify_by(lambda t1, y1, t2, y2: (lambda k: {"k": _r(k), "y0": _r(y2 * _exp(-k * t2))})(
        (math.log(abs(y2)) - math.log(abs(y1))) / (t2 - t1))),
    rand=lambda rng: (lambda t1: {"t1": t1, "y1": _u(rng, 5, 200), "t2": round(t1 + rng.uniform(1, 6), 2),
                                  "y2": _u(rng, 5, 400)})(_u(rng, 0, 4)),
    templates=["Exponential model through two measurements: y1 = 100 at t1 = 2 and y2 = 300 at t2 = 5.",
               "Fit exponential growth to two data points t1 = 0, y1 = 40, t2 = 3, y2 = 70."],
    examples=[({"t1": 2, "y1": 100, "t2": 5, "y2": 300}, {"k": 0.366204096223, "y0": 48.0749856769}),
              ({"t1": 0, "y1": 40, "t2": 3, "y2": 70}, {"y0": 40.0})],
    questions=[{"q": "Exponential model through two measurements: y1 = 100 at t1 = 2 and y2 = 300 at t2 = 5.",
                "outputs": {"k": 0.366204096223}},
               {"q": "Exponential model through two measurements: y1 = 100 at t1 = 2 and y2 = 300 at t2 = 2.",
                "outcome": "INVALID_INPUT"}],
    trig=(3, [_both(r"exponential", r"two (measurements|data points|points|observations)")]), target=None)


def decay_age(T, p):
    _fin(T, p)
    if T <= 0 or not 0 < p <= 1:
        raise ValueError("needs a positive half-life T and a remaining fraction p in (0, 1]")
    return {"t": _r(T * math.log(1 / p) / math.log(2))}


def _ref_age(T, p):
    if p == 1:
        return {"t": 0.0}
    k = _ln2_rate(T)                                           # time scaling: unit-rate decay reaches p at tau
    return {"t": ode_root_ref(lambda s, y: -y, 0, 1, p, math.log(1 / p)) / k}


add("radiometric_age", name="Age from the fraction remaining (radioactive dating)", topic="first-order ODEs / exponential models",
    purpose="The elapsed time when a fraction p of a radioactive substance with half-life T remains.",
    inputs=[N("T", "half-life (> 0)"), N("p", "fraction remaining, 0 < p <= 1")],
    outputs=[O("t", "age (elapsed time)")], primary="t", conditions=["T > 0", "0 < p <= 1"],
    method="t = T ln(1/p) / ln 2", latex=r"t = T\,\frac{\ln(1/p)}{\ln 2}", run=decay_age, reference=_ref_age,
    wrong=lambda T, p: {"t": T * (1 - p) * 2},
    verify=_verify_by(lambda T, p: {"t": _r(-T * math.log2(p))}),
    rand=lambda rng: {"T": _u(rng, 1, 100, 1), "p": _u(rng, 0.03, 0.97, 3)},
    templates=["Radiocarbon dating: half-life T = 5730 and fraction remaining p = 0.25. What is the age?",
               "How old is a sample with half-life T = 10 when p = 0.6 remains?"],
    examples=[({"T": 5730, "p": 0.25}, {"t": 11460.0}), ({"T": 10, "p": 0.6}, {"t": 7.36965594166})],
    questions=[{"q": "Radiocarbon dating: half-life T = 5730 and fraction remaining p = 0.25. What is the age?",
                "outputs": {"t": 11460.0}},
               {"q": "How old is a sample with half-life T = 10 when p = 1.5 remains?", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"\b(age|how old|dating)\b", r"half-?\s?life")]), target=None)


_COOL = r"newton'?s? (law of )?cooling|\bcooling law\b|newton cooling"


def cooling_T(T0, Ta, k, t):
    _fin(T0, Ta, k, t)
    if k <= 0:
        raise ValueError("the cooling constant k must be positive")
    return {"T": _r(Ta + (T0 - Ta) * _exp(-k * t))}


add("newton_cooling_temperature", name="Newton's law of cooling: temperature at a time", topic="first-order ODEs / applications",
    purpose="Solve dT/dt = -k (T - Ta), T(0) = T0, and give T(t).",
    inputs=[N("T0", "initial temperature"), N("Ta", "ambient temperature"), N("k", "cooling constant (> 0)"), N("t", "time")],
    outputs=[O("T", "temperature T(t)")], primary="T", conditions=["k > 0"],
    method="T = Ta + (T0 - Ta) e^{-k t}", latex=r"T(t) = T_a + (T_0 - T_a)e^{-k t}", run=cooling_T,
    reference=lambda T0, Ta, k, t: {"T": ode_ref(lambda s, y: -k * (y - Ta), 0, T0, t)},
    wrong=lambda T0, Ta, k, t: {"T": T0 * math.exp(-k * t)},
    rand=lambda rng: {"T0": _u(rng, 40, 200), "Ta": _u(rng, -10, 35), "k": _u(rng, 0.01, 0.8, 3), "t": _u(rng, 0.5, 20)},
    templates=["Newton's law of cooling: T0 = 90, Ta = 20, k = 0.1; find T(5).",
               "By Newton cooling with T(0) = 200, Ta = 25, k = 0.05, what is T at t = 10?"],
    examples=[({"T0": 90, "Ta": 20, "k": 0.1, "t": 5}, {"T": 62.4571461799})],
    questions=[{"q": "Newton's law of cooling: T0 = 90, Ta = 20, k = 0.1; find T(5).", "outputs": {"T": 62.4571461799}},
               {"q": "Newton's law of cooling: T0 = 90, Ta = 20, k = -0.1; find T(5).", "outcome": "INVALID_INPUT"}],
    trig=(2, [_COOL]))


def cooling_time(T0, Ta, k, T1):
    _fin(T0, Ta, k, T1)
    if k <= 0:
        raise ValueError("the cooling constant k must be positive")
    if T0 == Ta or not (min(T0, Ta) < T1 <= max(T0, Ta)) or (T1 - Ta) / (T0 - Ta) <= 0:
        raise ValueError("T1 must lie between the ambient temperature and T0")
    return {"t": _r(math.log((T0 - Ta) / (T1 - Ta)) / k)}


def _ref_cooling_time(T0, Ta, k, T1):
    if T1 == T0:
        return {"t": 0.0}
    return {"t": ode_root_ref(lambda s, y: -k * (y - Ta), 0, T0, T1, math.log((T0 - Ta) / (T1 - Ta)) / k)}


add("newton_cooling_time", name="Newton's law of cooling: time to reach a temperature", topic="first-order ODEs / applications",
    purpose="With dT/dt = -k (T - Ta), T(0) = T0: when does the temperature reach T1?",
    inputs=[N("T0", "initial temperature"), N("Ta", "ambient temperature"), N("k", "cooling constant (> 0)"),
            N("T1", "target temperature (between Ta and T0)")],
    outputs=[O("t", "time to reach T1")], primary="t", conditions=["k > 0", "T1 between Ta and T0"],
    method="t = ln((T0 - Ta)/(T1 - Ta))/k", latex=r"t = \frac{1}{k}\ln\frac{T_0 - T_a}{T_1 - T_a}",
    run=cooling_time, reference=_ref_cooling_time,
    wrong=lambda T0, Ta, k, T1: {"t": math.log(T0 / T1) / k},
    rand=lambda rng: (lambda T0, Ta: {"T0": T0, "Ta": Ta, "k": _u(rng, 0.02, 0.8, 3),
                                      "T1": round(Ta + (T0 - Ta) * rng.uniform(0.1, 0.9), 2)})(_u(rng, 60, 200), _u(rng, 5, 35)),
    templates=["Newton's law of cooling: how long until T1 = 40 if T0 = 90, Ta = 20, k = 0.1?",
               "Under Newton cooling with T0 = 200, Ta = 25, k = 0.05, when does it reach T1 = 100?"],
    examples=[({"T0": 90, "Ta": 20, "k": 0.1, "T1": 40}, {"t": 12.5276296850})],
    questions=[{"q": "Newton's law of cooling: how long until T1 = 40 if T0 = 90, Ta = 20, k = 0.1?", "outputs": {"t": 12.5276296850}},
               {"q": "Newton's law of cooling: how long until T1 = 10 if T0 = 90, Ta = 20, k = 0.1?", "outcome": "INVALID_INPUT"}],
    trig=(3, [_both(_COOL, r"\bhow long\b|\bwhen\b|\btime (to|until|needed|required)\b")]), target=None)


def cooling_k(T0, Ta, T1, t1):
    _fin(T0, Ta, T1, t1)
    if t1 <= 0 or T0 == Ta or not (min(T0, Ta) < T1 < max(T0, Ta)):
        raise ValueError("needs t1 > 0 and T1 strictly between Ta and T0")
    return {"k": _r(math.log((T0 - Ta) / (T1 - Ta)) / t1)}


def _ref_cooling_k(T0, Ta, T1, t1):
    # the unit-rate solution reaches T1 at tau; time scales as 1/k, so k = tau / t1
    tau = ode_root_ref(lambda s, y: -(y - Ta), 0, T0, T1, math.log((T0 - Ta) / (T1 - Ta)))
    return {"k": tau / t1}


add("newton_cooling_constant", name="Newton's law of cooling: cooling constant from a measurement",
    topic="first-order ODEs / applications",
    purpose="The cooling constant k when a body at T0 in surroundings Ta reads T1 after time t1.",
    inputs=[N("T0", "initial temperature"), N("Ta", "ambient temperature"), N("T1", "measured temperature"),
            N("t1", "time of the measurement (> 0)")],
    outputs=[O("k", "cooling constant")], primary="k", conditions=["t1 > 0", "T1 strictly between Ta and T0"],
    method="k = ln((T0 - Ta)/(T1 - Ta))/t1", latex=r"k = \frac{1}{t_1}\ln\frac{T_0 - T_a}{T_1 - T_a}",
    run=cooling_k, reference=_ref_cooling_k, wrong=lambda T0, Ta, T1, t1: {"k": (T0 - T1) / (t1 * (T0 - Ta))},
    rand=lambda rng: (lambda T0, Ta: {"T0": T0, "Ta": Ta, "t1": _u(rng, 1, 20),
                                      "T1": round(Ta + (T0 - Ta) * rng.uniform(0.1, 0.9), 2)})(_u(rng, 60, 200), _u(rng, 5, 35)),
    templates=["Newton's law of cooling: find the cooling constant if T0 = 90, Ta = 20 and T1 = 60 at t1 = 5."],
    examples=[({"T0": 90, "Ta": 20, "T1": 60, "t1": 5}, {"k": 0.111923157587})],
    questions=[{"q": "Newton's law of cooling: find the cooling constant if T0 = 90, Ta = 20 and T1 = 60 at t1 = 5.",
                "outputs": {"k": 0.111923157587}},
               {"q": "Newton's law of cooling: find the cooling constant if T0 = 90, Ta = 20 and T1 = 60.",
                "outcome": "NEEDS_INPUT"}],
    trig=(3, [_both(_COOL, r"cooling constant|\bfind k\b|value of k|rate constant|constant k")]), target=None)


_LOG = r"\blogistic\b"


def logistic(P0, r, K, t):
    _fin(P0, r, K, t)
    if P0 <= 0 or K <= 0 or r <= 0:
        raise ValueError("needs P0 > 0, K > 0 and r > 0")
    return {"P": _r(K * P0 / (P0 + (K - P0) * _exp(-r * t)))}


add("logistic_value", name="Logistic growth: population at a time", topic="first-order ODEs / population models",
    purpose="Solve dP/dt = r P (1 - P/K), P(0) = P0, and give P(t).",
    inputs=[N("P0", "initial population (> 0)"), N("r", "intrinsic growth rate (> 0)"), N("K", "carrying capacity (> 0)"),
            N("t", "time")],
    outputs=[O("P", "population P(t)")], primary="P", conditions=["P0 > 0", "r > 0", "K > 0"],
    method="P = K / (1 + (K/P0 - 1) e^{-r t})", latex=r"P(t) = \frac{K}{1 + \left(\frac{K}{P_0} - 1\right)e^{-r t}}",
    run=logistic, reference=lambda P0, r, K, t: {"P": ode_ref(lambda s, y: r * y * (1 - y / K), 0, P0, t)},
    wrong=lambda P0, r, K, t: {"P": K / (1 + (K / P0 - 1) * math.exp(-r * t / 2))},
    verify=_verify_by(lambda P0, r, K, t: {"P": _r(K / (1 + (K / P0 - 1) * _exp(-r * t)))}),
    rand=lambda rng: {"P0": _u(rng, 1, 400), "r": _u(rng, 0.05, 1.2, 3), "K": _u(rng, 100, 1000), "t": _u(rng, 0.5, 15)},
    templates=["Logistic growth with P0 = 10, r = 0.5, K = 1000; find P(8).", "A logistic population has P(0) = 50, r = 0.2, K = 500. Find P at t = 10."],
    examples=[({"P0": 10, "r": 0.5, "K": 1000, "t": 8}, {"P": 355.460987137})],
    questions=[{"q": "Logistic growth with P0 = 10, r = 0.5, K = 1000; find P(8).", "outputs": {"P": 355.460987137}},
               {"q": "Logistic growth with P0 = 10, r = 0.5, K = -1000; find P(8).", "outcome": "INVALID_INPUT"}],
    trig=(2, [_LOG]), unless=[r"logistic map"])


def logistic_inflection(P0, r, K):
    _fin(P0, r, K)
    if not (r > 0 and K > 0 and 0 < P0 < K / 2):
        raise ValueError("needs r > 0, K > 0 and 0 < P0 < K/2 (the inflection lies ahead)")
    return {"t_inflection": _r(math.log(K / P0 - 1) / r), "P_inflection": _r(K / 2), "max_rate": _r(r * K / 4)}


def _ref_logistic_inflection(P0, r, K):
    Pv = sp.Symbol("P")
    f = sp.Rational(repr(float(r))) * Pv * (1 - Pv / sp.Rational(repr(float(K))))
    Pstar = float(sp.solve(sp.diff(f, Pv), Pv)[0])
    t = ode_root_ref(lambda s, y: r * y * (1 - y / K), 0, P0, Pstar, math.log(K / P0 - 1) / r)
    return {"t_inflection": t, "P_inflection": Pstar, "max_rate": float(f.subs(Pv, sp.Rational(Pstar)))}


add("logistic_inflection", name="Logistic growth: inflection time and maximum growth rate",
    topic="first-order ODEs / population models",
    purpose="When a logistic population grows fastest: the inflection time, the population there (K/2) and the maximum rate rK/4.",
    inputs=[N("P0", "initial population (0 < P0 < K/2)"), N("r", "growth rate (> 0)"), N("K", "carrying capacity")],
    outputs=[O("t_inflection", "time of the inflection point"), O("P_inflection", "population at the inflection (K/2)"),
             O("max_rate", "largest growth rate dP/dt (rK/4)")], primary="t_inflection",
    conditions=["r > 0", "0 < P0 < K/2"], method="P'' = 0 at P = K/2; t* = ln(K/P0 - 1)/r",
    latex=r"t^* = \frac{1}{r}\ln\left(\frac{K}{P_0} - 1\right),\; \dot P_{\max} = \frac{rK}{4}",
    run=logistic_inflection, reference=_ref_logistic_inflection,
    wrong=lambda P0, r, K: {"t_inflection": math.log(K / P0) / r, "P_inflection": K / 2, "max_rate": r * K / 2},
    rand=lambda rng: (lambda K: {"K": K, "P0": round(K * rng.uniform(0.01, 0.4), 2), "r": _u(rng, 0.05, 1.2, 3)})(_u(rng, 100, 1000)),
    templates=["Logistic growth with P0 = 10, r = 0.5, K = 1000: when is the inflection point?",
               "For the logistic model P(0) = 20, r = 0.3, K = 600, find the maximum growth rate."],
    examples=[({"P0": 10, "r": 0.5, "K": 1000}, {"t_inflection": 9.19023970027, "max_rate": 125.0})],
    questions=[{"q": "Logistic growth with P0 = 10, r = 0.5, K = 1000: when is the inflection point?",
                "outputs": {"t_inflection": 9.19023970027, "P_inflection": 500.0}},
               {"q": "Logistic growth with P0 = 700, r = 0.5, K = 1000: when is the inflection point?", "outcome": "INVALID_INPUT"}],
    trig=(3, [_both(_LOG, r"inflection|fastest|maximum growth rate|max(imum)? rate")]), unless=[r"logistic map"], target=None)


def harvesting(r, K, H):
    _fin(r, K, H)
    if r <= 0 or K <= 0 or H < 0:
        raise ValueError("needs r > 0, K > 0 and H >= 0")
    Hc = r * K / 4
    if H > Hc:
        raise ValueError(f"no equilibrium: the harvest exceeds the maximum sustainable yield rK/4 = {Hc:g} (collapse)")
    if H == Hc:
        return {"y_high": _r(K / 2), "y_low": _r(K / 2), "H_critical": _r(Hc), "stability_high": "semi-stable",
                "stability_low": "semi-stable"}
    hi = K / 2 * (1 + math.sqrt(1 - 4 * H / (r * K)))
    lo = H * K / (r * hi)
    return {"y_high": _r(hi), "y_low": _r(lo), "H_critical": _r(Hc), "stability_high": "stable", "stability_low": "unstable"}


def _ref_harvest(r, K, H):
    import numpy as np
    roots = sorted(float(z.real) for z in np.roots([-r / K, r, -H]))
    return {"y_low": roots[0], "y_high": roots[1]}


add("logistic_harvesting", name="Logistic growth with constant harvesting: equilibria",
    topic="first-order ODEs / equilibria and stability",
    purpose="Equilibria of dy/dt = r y (1 - y/K) - H, their stability, and the critical (maximum sustainable) harvest.",
    inputs=[N("r", "growth rate (> 0)"), N("K", "carrying capacity (> 0)"), N("H", "harvest rate (>= 0)")],
    outputs=[O("y_high", "upper equilibrium (stable)"), O("y_low", "lower equilibrium (unstable threshold)"),
             O("H_critical", "maximum sustainable harvest rK/4"), O("stability_high", "stability", "text"),
             O("stability_low", "stability", "text")], primary="y_high",
    conditions=["r, K > 0", "0 <= H <= rK/4 (otherwise the population collapses)"],
    method="roots of r y (1 - y/K) = H; sign of f' at each", latex=r"y_\pm = \frac{K}{2}\left(1 \pm \sqrt{1 - \frac{4H}{rK}}\right)",
    run=harvesting, reference=_ref_harvest,
    wrong=lambda r, K, H: {"y_high": K / 2 * (1 + math.sqrt(max(0.0, 1 - H / (r * K)))), "y_low": K / 2 * (1 - math.sqrt(max(0.0, 1 - H / (r * K))))},
    rand=lambda rng: (lambda r, K: {"r": r, "K": K, "H": round(r * K / 4 * rng.uniform(0.05, 0.95), 3)})(_u(rng, 0.1, 2, 3), _u(rng, 100, 1000)),
    templates=["Logistic harvesting with r = 0.5, K = 1000, H = 100: find the equilibria."],
    examples=[({"r": 0.5, "K": 1000, "H": 100}, {"y_high": 723.606797750, "y_low": 276.393202250, "H_critical": 125.0})],
    questions=[{"q": "Logistic harvesting with r = 0.5, K = 1000, H = 100: find the equilibria.", "outputs": {"y_high": 723.606797750}},
               {"q": "Logistic harvesting with r = 0.5, K = 1000, H = 200: find the equilibria.", "outcome": "INVALID_INPUT"}],
    trig=(3, [_both(_LOG, r"harvest")]), unless=[r"logistic map"], target=None)


def lin1(a, b, y0, t):
    _fin(a, b, y0, t)
    if a == 0:
        raise ValueError("a must not be 0 (then y = y0 + b t directly)")
    ys = b / a
    tr = (y0 - ys) * _exp(-a * t)
    return {"y": _r(_guard(ys + tr, ys, tr)), "y_steady": _r(ys)}


add("linear_first_order", name="First-order linear ODE with constant coefficients (integrating factor)",
    topic="first-order ODEs / linear equations",
    purpose="Solve y' + a y = b with y(0) = y0 and give y(t) and the equilibrium b/a.",
    inputs=[N("a", "coefficient of y", condition="a != 0"), N("b", "constant forcing"), N("y0", "initial value"), N("t", "time")],
    outputs=[O("y", "value y(t)"), O("y_steady", "equilibrium b/a (the limit when a > 0)")], primary="y",
    conditions=["a != 0"], method="integrating factor e^{a t}: y = b/a + (y0 - b/a) e^{-a t}",
    latex=r"y(t) = \frac{b}{a} + \left(y_0 - \frac{b}{a}\right)e^{-a t}", run=lin1,
    reference=lambda a, b, y0, t: {"y": ode_ref(lambda s, y: b - a * y, 0, y0, t), "y_steady": b / a},
    wrong=lambda a, b, y0, t: {"y": b / a + (y0 - b / a) * math.exp(a * t), "y_steady": b / a},
    rand=lambda rng: {"a": _nz(rng, -0.6, 2), "b": _u(rng, -10, 10), "y0": _u(rng, -10, 10), "t": _u(rng, 0.2, 6)},
    templates=["Solve y' + a y = b with a = 2, b = 6, y(0) = 1; find y(1).",
               "Use an integrating factor for a = 0.5, b = 3, y0 = 0, t = 4."],
    examples=[({"a": 2, "b": 6, "y0": 1, "t": 1}, {"y": 2.72932943353, "y_steady": 3.0})],
    questions=[{"q": "Solve y' + a y = b with a = 2, b = 6, y(0) = 1; find y(1).", "outputs": {"y": 2.72932943353}},
               {"q": "Solve y' + a y = b with a = 0, b = 6, y(0) = 1; find y(1).", "outcome": "INVALID_INPUT"}],
    trig=(2, [r"y'\s*\+\s*a\s*\*?\s*y\s*=\s*b(?!\s*\*?\s*e)", r"integrating factor"]))


def lin1_exp(a, b, c, y0, t):
    _fin(a, b, c, y0, t)
    if a + c == 0:
        y = (y0 + b * t) * _exp(-a * t)
        return {"y": _r(y), "resonant": True}
    A = b / (a + c)
    p, h = A * _exp(c * t), (y0 - A) * _exp(-a * t)
    return {"y": _r(_guard(p + h, p, h)), "resonant": False}


add("linear_first_order_exponential_forcing", name="First-order linear ODE with exponential forcing",
    topic="first-order ODEs / linear equations",
    purpose="Solve y' + a y = b e^{c t}, y(0) = y0, and give y(t) (including the resonant case c = -a).",
    inputs=[N("a", "coefficient of y"), N("b", "forcing amplitude"), N("c", "forcing exponent"), N("y0", "initial value"), N("t", "time")],
    outputs=[O("y", "value y(t)"), O("resonant", "c = -a (secular term t e^{-a t})", "boolean")], primary="y",
    conditions=["any real a, b, c"], method="integrating factor: y = b/(a+c) e^{ct} + (y0 - b/(a+c)) e^{-at}; c = -a: (y0 + b t) e^{-at}",
    latex=r"y = \frac{b}{a+c}e^{ct} + \left(y_0 - \frac{b}{a+c}\right)e^{-at}", run=lin1_exp,
    reference=lambda a, b, c, y0, t: {"y": ode_ref(lambda s, y: b * __import__("mpmath").exp(c * s) - a * y, 0, y0, t)},
    wrong=lambda a, b, c, y0, t: {"y": (b / (c - a) if c != a else b) * math.exp(c * t) + (y0 - (b / (c - a) if c != a else b)) * math.exp(-a * t)},
    rand=lambda rng: (lambda a: {"a": a, "b": _u(rng, -5, 5), "c": (-a if rng.random() < 0.15 else _u(rng, -1.5, 1.5)),
                                 "y0": _u(rng, -5, 5), "t": _u(rng, 0.2, 4)})(_u(rng, -1, 2)),
    templates=["Solve y' + a y = b e^(c t) with a = 1, b = 2, c = 0.5, y(0) = 0; find y(2)."],
    examples=[({"a": 1, "b": 2, "c": 0.5, "y0": 0, "t": 2}, {"y": 3.44392872696}),
              ({"a": 1, "b": 2, "c": -1, "y0": 1, "t": 2}, {"y": 0.676676416183, "resonant": True})],
    questions=[{"q": "Solve y' + a y = b e^(c t) with a = 1, b = 2, c = 0.5, y(0) = 0; find y(2).", "outputs": {"y": 3.44392872696}},
               {"q": "Solve y' + a y = b e^(c t) with a = 1, b = 2, y(0) = 0; find y(2).", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"y'\s*\+\s*a\s*\*?\s*y\s*=\s*b\s*\*?\s*e\^"]))


def mixing(V, r, c_in, A0, t):
    _fin(V, r, c_in, A0, t)
    if V <= 0 or r <= 0 or c_in < 0 or A0 < 0:
        raise ValueError("needs V > 0, r > 0, c_in >= 0, A0 >= 0")
    Ainf = c_in * V
    return {"A": _r(Ainf + (A0 - Ainf) * _exp(-r * t / V)), "A_limit": _r(Ainf),
            "concentration": _r((Ainf + (A0 - Ainf) * _exp(-r * t / V)) / V)}


add("mixing_tank", name="Mixing tank (brine): amount of salt at a time", topic="first-order ODEs / applications",
    purpose="A well-stirred tank of volume V with inflow = outflow rate r at concentration c_in: amount A(t) of solute.",
    inputs=[N("V", "volume (constant, > 0)"), N("r", "flow rate in = out (> 0)"), N("c_in", "inflow concentration (>= 0)"),
            N("A0", "initial amount of solute"), N("t", "time")],
    outputs=[O("A", "amount A(t)"), O("A_limit", "long-run amount c_in V"), O("concentration", "concentration A(t)/V")],
    primary="A", conditions=["V > 0", "r > 0"], method="A' = r c_in - (r/V) A: A = c_in V + (A0 - c_in V) e^{-r t/V}",
    latex=r"A(t) = c_{in}V + (A_0 - c_{in}V)e^{-rt/V}", run=mixing,
    reference=lambda V, r, c_in, A0, t: {"A": ode_ref(lambda s, y: r * c_in - r * y / V, 0, A0, t)},
    wrong=lambda V, r, c_in, A0, t: {"A": c_in * V + (A0 - c_in * V) * math.exp(-V * t / r)},
    rand=lambda rng: {"V": _u(rng, 10, 1000), "r": _u(rng, 1, 30), "c_in": _u(rng, 0, 2), "A0": _u(rng, 0, 100), "t": _u(rng, 1, 60)},
    templates=["A brine mixing tank has V = 100, r = 5, c_in = 0.2, A0 = 0; find A(10).",
               "Salt in a mixing tank: V = 500, r = 10, c_in = 0, A0 = 50, t = 30."],
    examples=[({"V": 100, "r": 5, "c_in": 0.2, "A0": 0, "t": 10}, {"A": 7.86938680575, "A_limit": 20.0})],
    questions=[{"q": "A brine mixing tank has V = 100, r = 5, c_in = 0.2, A0 = 0; find A(10).", "outputs": {"A": 7.86938680575}},
               {"q": "A brine mixing tank has V = 0, r = 5, c_in = 0.2, A0 = 0; find A(10).", "outcome": "INVALID_INPUT"}],
    trig=(2, [r"\b(mixing tank|brine|salt)\b", r"tank\b.*\b(concentration|mixture)"]))


def separable_power(a, n, y0, t):
    _fin(a, n, y0, t)
    if n < 0 or t < 0:
        raise ValueError("needs n >= 0 and t >= 0")
    return {"y": _r(y0 * _exp(a * t ** (n + 1) / (n + 1)))}


add("separable_power", name="Separable equation y' = a t^n y", topic="first-order ODEs / separable equations",
    purpose="Solve the separable equation y' = a t^n y with y(0) = y0 and give y(t).",
    inputs=[N("a", "coefficient"), N("n", "power of t (>= 0)"), N("y0", "initial value"), N("t", "time (>= 0)")],
    outputs=[O("y", "value y(t)")], primary="y", conditions=["n >= 0", "t >= 0"],
    method="separate: dy/y = a t^n dt, ln(y/y0) = a t^{n+1}/(n+1)", latex=r"y(t) = y_0\exp\left(\frac{a\,t^{n+1}}{n+1}\right)",
    run=separable_power, reference=lambda a, n, y0, t: {"y": ode_ref(lambda s, y: a * s ** n * y, 0, y0, t)},
    wrong=lambda a, n, y0, t: {"y": y0 * math.exp(a * t ** (n + 1))},
    rand=lambda rng: {"a": _nz(rng, -1, 1), "n": rng.choice([0, 1, 2, 3, 0.5]), "y0": _u(rng, -5, 5), "t": _u(rng, 0.2, 2)},
    templates=["Solve the separable equation y' = a t^n y with a = 2, n = 1, y(0) = 3; find y(1)."],
    examples=[({"a": 2, "n": 1, "y0": 3, "t": 1}, {"y": 8.15484548538})],
    questions=[{"q": "Solve the separable equation y' = a t^n y with a = 2, n = 1, y(0) = 3; find y(1).", "outputs": {"y": 8.15484548538}},
               {"q": "Solve the separable equation y' = a t^n y with a = 2, n = -1, y(0) = 3; find y(1).", "outcome": "INVALID_INPUT"}],
    trig=(2, [r"y'\s*=\s*a\s*\*?\s*t\^n\s*\*?\s*y", r"\bseparable\b"]))


def riccati_sq(k, y0, t):
    _fin(k, y0, t)
    if t < 0:
        raise ValueError("needs t >= 0")
    d = 1 - k * y0 * t
    if d <= 0:
        raise ValueError(f"the solution blows up at t = {1 / (k * y0):g}, before the requested time")
    out = {"y": _r(y0 / d)}
    if k * y0 > 0:
        out["blowup_time"] = _r(1 / (k * y0))
    return out


add("quadratic_ode_blowup", name="Equation y' = k y^2: solution and blow-up time", topic="first-order ODEs / nonlinear equations",
    purpose="Solve y' = k y^2, y(0) = y0; give y(t) and the finite blow-up time 1/(k y0) when k y0 > 0.",
    inputs=[N("k", "coefficient"), N("y0", "initial value"), N("t", "time (>= 0)")],
    outputs=[O("y", "value y(t)"), O("blowup_time", "time at which y becomes infinite (k y0 > 0)")], primary="y",
    conditions=["t before the blow-up time"], method="separate: -1/y = k t - 1/y0", latex=r"y(t) = \frac{y_0}{1 - k y_0 t}",
    run=riccati_sq, reference=lambda k, y0, t: {"y": ode_ref(lambda s, y: k * y * y, 0, y0, t)},
    wrong=lambda k, y0, t: {"y": y0 / (1 + k * y0 * t)},
    rand=lambda rng: (lambda k, y0: {"k": k, "y0": y0, "t": round(rng.uniform(0.05, 0.8) / abs(k * y0), 3) if k * y0 > 0 else _u(rng, 0.1, 3)})(
        _nz(rng, -1, 1), _nz(rng, -3, 3)),
    templates=["Solve y' = k y^2 with k = 1, y(0) = 0.5; find y(1).", "When does y' = k y^2 blow up for k = 2, y0 = 1, t = 0.25?"],
    examples=[({"k": 1, "y0": 0.5, "t": 1}, {"y": 1.0, "blowup_time": 2.0})],
    questions=[{"q": "Solve y' = k y^2 with k = 1, y(0) = 0.5; find y(1).", "outputs": {"y": 1.0, "blowup_time": 2.0}},
               {"q": "Solve y' = k y^2 with k = 1, y(0) = 0.5; find y(3).", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"y'\s*=\s*k\s*\*?\s*y\^2", r"blow[- ]?s? ?up"]))


def bernoulli(p, q, n, y0, t):
    _fin(p, q, n, y0, t)
    if p == 0 or n == 1 or y0 <= 0 or t < 0:
        raise ValueError("needs p != 0, n != 1, y0 > 0 and t >= 0")
    m = 1 - n
    vs = q / p
    v = vs + (y0 ** m - vs) * _exp(-m * p * t)
    v0 = y0 ** m
    if v <= 0 or (vs <= 0 and v <= 1e-12 * v0):
        raise ValueError("the solution reaches zero or becomes unbounded before t")
    return {"y": _r(v ** (1 / m))}


add("bernoulli_equation", name="Bernoulli equation y' + p y = q y^n", topic="first-order ODEs / nonlinear equations",
    purpose="Solve the Bernoulli equation with constant p, q by the substitution v = y^{1-n}; give y(t).",
    inputs=[N("p", "coefficient of y", condition="p != 0"), N("q", "coefficient of y^n"), N("n", "exponent (n != 1)"),
            N("y0", "initial value (> 0)"), N("t", "time (>= 0)")],
    outputs=[O("y", "value y(t)")], primary="y", conditions=["p != 0", "n != 1", "y0 > 0", "solution stays positive"],
    method="v = y^{1-n}: v' + (1-n) p v = (1-n) q, linear", latex=r"y^{1-n} = \frac{q}{p} + \left(y_0^{1-n} - \frac{q}{p}\right)e^{-(1-n)pt}",
    run=bernoulli, reference=lambda p, q, n, y0, t: {"y": ode_ref(lambda s, y: q * y ** n - p * y, 0, y0, t)},
    wrong=lambda p, q, n, y0, t: {"y": max(1e-300, q / p + (y0 ** (1 - n) - q / p) * math.exp(-p * t)) ** (1 / (1 - n))},
    rand=lambda rng: {"p": _nz(rng, 0.2, 2), "q": _u(rng, 0.1, 2), "n": rng.choice([2, 3, 0.5, -1, 1.5]), "y0": _u(rng, 0.2, 2), "t": _u(rng, 0.1, 2)},
    templates=["Solve the Bernoulli equation y' + p y = q y^n with p = 1, q = 1, n = 2, y(0) = 0.5; find y(1)."],
    examples=[({"p": 1, "q": 1, "n": 2, "y0": 0.5, "t": 1}, {"y": 0.26894142137})],
    questions=[{"q": "Solve the Bernoulli equation y' + p y = q y^n with p = 1, q = 1, n = 2, y(0) = 0.5; find y(1).",
                "outputs": {"y": 0.26894142137}},
               {"q": "Solve the Bernoulli equation y' + p y = q y^n with p = 1, q = 1, n = 1, y(0) = 0.5; find y(1).",
                "outcome": "INVALID_INPUT"}],
    trig=(3, [r"bernoulli"]))


def drag_linear(m, c, g, v0, t):
    _fin(m, c, g, v0, t)
    if m <= 0 or c <= 0:
        raise ValueError("needs m > 0 and c > 0")
    vt = m * g / c
    return {"v": _r(vt + (v0 - vt) * _exp(-c * t / m)), "v_terminal": _r(vt)}


add("linear_drag_velocity", name="Falling body with linear drag: velocity", topic="first-order ODEs / applications",
    purpose="Solve m v' = m g - c v with v(0) = v0: velocity at time t and terminal velocity m g / c.",
    inputs=[N("m", "mass (> 0)"), N("c", "linear drag coefficient (> 0)"), N("g", "gravitational acceleration"),
            N("v0", "initial velocity (downward positive)"), N("t", "time")],
    outputs=[O("v", "velocity v(t)"), O("v_terminal", "terminal velocity m g / c")], primary="v", conditions=["m > 0", "c > 0"],
    method="linear first-order ODE: v = mg/c + (v0 - mg/c) e^{-ct/m}", latex=r"v(t) = \frac{mg}{c} + \left(v_0 - \frac{mg}{c}\right)e^{-ct/m}",
    run=drag_linear, reference=lambda m, c, g, v0, t: {"v": ode_ref(lambda s, v: g - c * v / m, 0, v0, t), "v_terminal": m * g / c},
    wrong=lambda m, c, g, v0, t: {"v": m * g / c * (1 - math.exp(-c * t)) + v0, "v_terminal": g / c},
    rand=lambda rng: {"m": _u(rng, 0.5, 100), "c": _u(rng, 0.1, 20), "g": 9.81, "v0": _u(rng, 0, 20), "t": _u(rng, 0.2, 20)},
    templates=["A falling body with linear drag: m = 2, c = 0.5, g = 9.81, v0 = 0; find v(3)."],
    examples=[({"m": 2, "c": 0.5, "g": 9.81, "v0": 0, "t": 3}, {"v": 20.7043364704, "v_terminal": 39.24})],
    questions=[{"q": "A falling body with linear drag: m = 2, c = 0.5, g = 9.81, v0 = 0; find v(3).", "outputs": {"v": 20.7043364704}},
               {"q": "A falling body with linear drag: m = 2, c = 0.5, v0 = 0; find v(3).", "outcome": "NEEDS_INPUT"}],
    trig=(2, [r"linear (air )?(drag|resistance)", r"drag proportional to (the )?(velocity|speed)\b(?! squared)"]))


def drag_quadratic(m, c, g, v0, t):
    _fin(m, c, g, v0, t)
    if m <= 0 or c <= 0 or g <= 0 or v0 < 0 or t < 0:
        raise ValueError("needs m, c, g > 0, v0 >= 0 and t >= 0")
    vt = math.sqrt(m * g / c)
    s = g * t / vt
    if v0 == vt:
        v = vt
    elif v0 < vt:
        v = vt * math.tanh(s + math.atanh(v0 / vt))
    else:
        v = vt / math.tanh(s + math.atanh(vt / v0))
    return {"v": _r(v), "v_terminal": _r(vt)}


add("quadratic_drag_velocity", name="Falling body with quadratic drag: velocity", topic="first-order ODEs / applications",
    purpose="Solve m v' = m g - c v^2 with v(0) = v0 >= 0: velocity at time t and terminal velocity sqrt(m g / c).",
    inputs=[N("m", "mass (> 0)"), N("c", "quadratic drag coefficient (> 0)"), N("g", "gravitational acceleration (> 0)"),
            N("v0", "initial downward velocity (>= 0)"), N("t", "time (>= 0)")],
    outputs=[O("v", "velocity v(t)"), O("v_terminal", "terminal velocity")], primary="v", conditions=["m, c, g > 0", "v0 >= 0"],
    method="separable: v = v_t tanh(g t / v_t + artanh(v0/v_t)) (coth form when v0 > v_t)",
    latex=r"v(t) = v_t \tanh\left(\frac{g t}{v_t} + \operatorname{artanh}\frac{v_0}{v_t}\right),\; v_t = \sqrt{mg/c}",
    run=drag_quadratic, reference=lambda m, c, g, v0, t: {"v": ode_ref(lambda s, v: g - c * v * v / m, 0, v0, t)},
    wrong=lambda m, c, g, v0, t: {"v": math.sqrt(m * g / c) * math.tanh(g * t / math.sqrt(m * g / c) / 2 + math.atanh(min(v0 / math.sqrt(m * g / c), 0.999)))},
    rand=lambda rng: (lambda m, c: {"m": m, "c": c, "g": 9.81, "t": _u(rng, 0.1, 8),
                                    "v0": round(math.sqrt(m * 9.81 / c) * rng.choice([0, rng.uniform(0, 0.9), rng.uniform(1.1, 2)]), 2)})(
        _u(rng, 1, 100), _u(rng, 0.05, 2)),
    templates=["A falling body with quadratic drag: m = 80, c = 0.25, g = 9.81, v0 = 0; find v(5)."],
    examples=[({"m": 80, "c": 0.25, "g": 9.81, "v0": 0, "t": 5}, {"v": 39.4514324566, "v_terminal": 56.0285641437})],
    questions=[{"q": "A falling body with quadratic drag: m = 80, c = 0.25, g = 9.81, v0 = 0; find v(5).", "outputs": {"v": 39.4514324566}},
               {"q": "A falling body with quadratic drag: m = 80, c = 0, g = 9.81, v0 = 0; find v(5).", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"quadratic (air )?(drag|resistance)", r"drag proportional to the square of the (velocity|speed)"]))


def torricelli(h0, k, t):
    _fin(h0, k, t)
    if h0 <= 0 or k <= 0 or t < 0:
        raise ValueError("needs h0 > 0, k > 0 and t >= 0")
    T = 2 * math.sqrt(h0) / k
    h = (math.sqrt(h0) - k * t / 2) ** 2 if t < T else 0.0
    return {"h": _r(h), "t_empty": _r(T)}


def _ref_torricelli(h0, k, t):
    import mpmath
    with mpmath.workdps(25):
        T = mpmath.quad(lambda h: 1 / (k * mpmath.sqrt(h)), [0, h0])
    out = {"t_empty": float(T)}
    if t < 0.95 * float(T):
        out["h"] = ode_ref(lambda s, h: -k * mpmath.sqrt(h), 0, h0, t)
    return out


add("torricelli_draining", name="Draining tank (Torricelli's law): height and emptying time", topic="first-order ODEs / applications",
    purpose="Solve dh/dt = -k sqrt(h), h(0) = h0: water height at time t and the time to empty.",
    inputs=[N("h0", "initial height (> 0)"), N("k", "drain constant (> 0)"), N("t", "time (>= 0)")],
    outputs=[O("h", "height h(t) (0 once empty)"), O("t_empty", "time to empty 2 sqrt(h0)/k")], primary="t_empty",
    conditions=["h0 > 0", "k > 0"], method="separable: sqrt(h) = sqrt(h0) - k t / 2", latex=r"h(t) = \left(\sqrt{h_0} - \tfrac{k t}{2}\right)^2,\; t_{empty} = \frac{2\sqrt{h_0}}{k}",
    run=torricelli, reference=_ref_torricelli, wrong=lambda h0, k, t: {"h": (math.sqrt(h0) - k * t) ** 2, "t_empty": math.sqrt(h0) / k},
    rand=lambda rng: (lambda h0, k: {"h0": h0, "k": k, "t": round(2 * math.sqrt(h0) / k * rng.uniform(0.05, 0.9), 3)})(_u(rng, 0.5, 10), _u(rng, 0.05, 1, 3)),
    templates=["A draining tank obeys Torricelli's law with h0 = 4, k = 0.2; find h(5) and the emptying time."],
    examples=[({"h0": 4, "k": 0.2, "t": 5}, {"h": 2.25, "t_empty": 20.0})],
    questions=[{"q": "A draining tank obeys Torricelli's law with h0 = 4, k = 0.2; find h(5) and the emptying time.",
                "outputs": {"h": 2.25, "t_empty": 20.0}},
               {"q": "A draining tank obeys Torricelli's law with h0 = -4, k = 0.2; find h(5).", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"torricelli", r"drain(ing|s)?\b.*\btank|tank\b.*\bdrain"]))


def gompertz(y0, r, K, t):
    _fin(y0, r, K, t)
    if y0 <= 0 or K <= 0:
        raise ValueError("needs y0 > 0 and K > 0")
    return {"y": _r(K * _exp(math.log(y0 / K) * _exp(-r * t)))}


add("gompertz_growth", name="Gompertz growth: value at a time", topic="first-order ODEs / population models",
    purpose="Solve dy/dt = r y ln(K/y), y(0) = y0 (tumour and population growth) and give y(t).",
    inputs=[N("y0", "initial value (> 0)"), N("r", "growth rate"), N("K", "carrying capacity (> 0)"), N("t", "time")],
    outputs=[O("y", "value y(t)")], primary="y", conditions=["y0 > 0", "K > 0"],
    method="u = ln(y/K) satisfies u' = -r u", latex=r"y(t) = K\exp\left(\ln\frac{y_0}{K}\,e^{-rt}\right)", run=gompertz,
    reference=lambda y0, r, K, t: {"y": ivp_ref(lambda s, u: [r * u[0] * math.log(K / u[0])], 0, y0, t)},
    wrong=lambda y0, r, K, t: {"y": K * math.exp(math.log(y0 / K) * math.exp(r * t))},
    rand=lambda rng: {"y0": _u(rng, 1, 300), "r": _u(rng, 0.05, 1, 3), "K": _u(rng, 100, 1000), "t": _u(rng, 0.5, 10)},
    templates=["Gompertz growth with y0 = 10, r = 0.3, K = 1000; find y(5)."],
    examples=[({"y0": 10, "r": 0.3, "K": 1000, "t": 5}, {"y": 357.881854966})],
    questions=[{"q": "Gompertz growth with y0 = 10, r = 0.3, K = 1000; find y(5).", "outputs": {"y": 357.881854966}},
               {"q": "Gompertz growth with y0 = -10, r = 0.3, K = 1000; find y(5).", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"gompertz"]))


def slope_field(f, t, y):
    _fin(t, y)
    m = _fn(f)(t, y)
    return {"slope": _r(m), "angle_deg": _r(math.degrees(math.atan(m)))}


def _ref_slope(f, t, y):
    import mpmath
    with mpmath.workdps(30):
        m = ref_fn(f)(mpmath.mpf(t), mpmath.mpf(y))
        return {"slope": float(m), "angle_deg": float(mpmath.degrees(mpmath.atan(m)))}


_RAND_F = ["t + y", "y - t^2 + 1", "-2*y + t", "t*y", "sin(t) - y", "y*(1 - y)", "cos(t)*y", "t^2 - y", "exp(-t) - y",
           "y/(1 + t)", "-y + 2*t + 1", "0.5*y", "1 - t*y", "t - y^2"]

add("slope_field", name="Slope (direction) field value", topic="first-order ODEs / qualitative methods",
    purpose="The slope dy/dt = f(t, y) of the direction field at a point, and its angle.",
    inputs=[{"symbol": "f", "type": "text", "kind": "number", "meaning": "right-hand side f(t, y)", "required": True, "max_length": 200},
            N("t", "t-coordinate"), N("y", "y-coordinate")],
    outputs=[O("slope", "slope f(t, y)"), O("angle_deg", "angle of the field line, degrees")], primary="slope",
    conditions=["f defined at the point"], method="evaluate f; angle = atan(slope)", latex=r"m = f(t, y),\; \theta = \arctan m",
    run=slope_field, reference=_ref_slope, wrong=lambda f, t, y: {"slope": _safe(lambda: _fn(f)(y, t))},
    rand=lambda rng: {"f": rng.choice(_RAND_F), "t": _u(rng, -2, 2), "y": _u(rng, -2, 2)},
    templates=["Slope field of dy/dt = t + y at t = 1, y = 2.", "What is the direction field slope of y' = t*y - 1 at t = 2, y = 0.5?"],
    examples=[({"f": "t + y", "t": 1, "y": 2}, {"slope": 3.0, "angle_deg": 71.5650511771})],
    questions=[{"q": "Slope field of dy/dt = t + y at t = 1, y = 2.", "outputs": {"slope": 3.0}},
               {"q": "Slope field of dy/dt = log(y) at t = 1, y = -2.", "outcome": "INVALID_INPUT"}],
    trig=(2, [r"slope field|direction field"]), text={"f": RHS_TY}, target=None)


# =================================================================================================================
# NUMERICAL METHODS
# =================================================================================================================
def _one_step(method, f, t0, y0, h, n):
    _fin(t0, y0, h)
    if h <= 0:
        raise ValueError("the step h must be positive")
    if not 1 <= n <= 10000:
        raise ValueError("the number of steps n must be between 1 and 10000")
    y = y0
    for k in range(n):
        t = t0 + k * h
        if method == "euler":
            y = y + h * f(t, y)
        elif method == "heun":
            k1 = f(t, y)
            k2 = f(t0 + (k + 1) * h, y + h * k1)
            y = y + h * (k1 + k2) / 2
        elif method == "midpoint":
            k1 = f(t, y)
            y = y + h * f(t + h / 2, y + h * k1 / 2)
        elif method == "rk4":
            k1 = f(t, y)
            k2 = f(t + h / 2, y + h * k1 / 2)
            k3 = f(t + h / 2, y + h * k2 / 2)
            k4 = f(t0 + (k + 1) * h, y + h * k3)
            y = y + h * (k1 + 2 * k2 + 2 * k3 + k4) / 6
        if not math.isfinite(y) or abs(y) > 1e100:
            raise ValueError("the numerical solution overflows")
    return {"y_n": _r(y), "t_n": _r(t0 + n * h)}


_TABLEAU = {"euler": ([], ["1"], ["0"]), "heun": ([["1"]], ["1/2", "1/2"], ["0", "1"]),
            "midpoint": ([["1/2"]], ["0", "1"], ["0", "1/2"]),
            "rk4": ([["1/2"], ["0", "1/2"], ["0", "0", "1"]], ["1/6", "1/3", "1/3", "1/6"], ["0", "1/2", "1/2", "1"])}


def ref_rk(method, f, t0, y0, h, n, names=("t", "y")):
    """Build-time only: a generic explicit Runge-Kutta stepper from the Butcher tableau, mpmath arithmetic, sympy's evaluator."""
    import mpmath
    A, b, c = _TABLEAU[method]
    with mpmath.workdps(30):
        F = ref_fn(f, names) if isinstance(f, str) else f
        A = [[_mpq(x) for x in row] for row in A]
        b = [_mpq(x) for x in b]
        c = [_mpq(x) for x in c]
        H, y = mpmath.mpf(h), mpmath.mpf(y0)
        for k in range(int(n)):
            t = mpmath.mpf(t0) + k * H
            ks = []
            for i in range(len(b)):
                yi = y + H * sum((A[i - 1][j] * ks[j] for j in range(i)), mpmath.mpf(0)) if i else y
                ks.append(F(t + c[i] * H, yi))
            y = y + H * sum(bi * ki for bi, ki in zip(b, ks))
        return {"y_n": float(y), "t_n": float(mpmath.mpf(t0) + n * H)}


def _int_in(sym, meaning, lo=1, hi=10000):
    return {"symbol": sym, "type": "integer", "kind": "number", "meaning": meaning, "required": True, "min": lo, "max": hi}


_TXT_F = {"symbol": "f", "type": "text", "kind": "number", "meaning": "right-hand side f(t, y) of y' = f(t, y)", "required": True,
          "max_length": 200}
_METHOD_IN = [_TXT_F, N("t0", "initial time"), N("y0", "initial value y(t0)"), N("h", "step size (> 0)"),
              _int_in("n", "number of steps")]
_METHOD_OUT = [O("y_n", "approximation of y(t0 + n h)"), O("t_n", "final time t0 + n h")]


def _rand_method(rng):
    return {"f": rng.choice(_RAND_F), "t0": _u(rng, 0, 1), "y0": _u(rng, -1.5, 1.5), "h": _u(rng, 0.02, 0.2, 3),
            "n": rng.randint(1, 20)}


def _method(key, method, name, level_phrase, latex, method_text, wrong, trig, unless=(), ex=None, q2=None):
    run = (lambda f, t0, y0, h, n: _one_step(method, _fn(f), t0, y0, h, int(n)))
    (inp, out), = [ex]
    add(key, name=name, topic="numerical methods / one-step methods",
        purpose=f"Approximate the solution of y' = f(t, y), y(t0) = y0, after n steps of size h with {level_phrase}.",
        inputs=_METHOD_IN, outputs=_METHOD_OUT, primary="y_n", conditions=["h > 0", "1 <= n <= 10000", "f defined along the steps"],
        method=method_text, latex=latex, run=run,
        reference=lambda f, t0, y0, h, n: ref_rk(method, f, t0, y0, h, n), wrong=wrong, rand=_rand_method,
        templates=[q2[0]], examples=[(inp, out)],
        questions=[{"q": q2[0], "outputs": q2[1]},
                   {"q": q2[0].split(",")[0] + ", y(0) = 1, h = 0.1.", "outcome": "NEEDS_INPUT"}],
        trig=trig, unless=unless, text={"f": RHS_TY}, target=None)


def _wrong_euler(f, t0, y0, h, n):
    g, y = _fn(f), y0
    for k in range(int(n)):
        y = y + h * g(t0 + (k + 1) * h, y)                     # slope taken at the END of the step
    return {"y_n": y}


def _wrong_heun(f, t0, y0, h, n):
    g, y = _fn(f), y0
    for k in range(int(n)):
        t = t0 + k * h
        y = y + h * (g(t, y) + g(t + h, y)) / 2                # corrector without the predictor
    return {"y_n": y}


def _wrong_midpoint(f, t0, y0, h, n):
    g, y = _fn(f), y0
    for k in range(int(n)):
        t = t0 + k * h
        y = y + h * g(t + h / 2, y)                            # midpoint time, no half step in y
    return {"y_n": y}


def _wrong_rk4(f, t0, y0, h, n):
    g, y = _fn(f), y0
    for k in range(int(n)):
        t = t0 + k * h
        k1 = g(t, y)
        k2 = g(t + h / 2, y + h * k1 / 2)
        k3 = g(t + h / 2, y + h * k1 / 2)                      # k3 built from k1 instead of k2
        k4 = g(t + h, y + h * k3)
        y = y + h * (k1 + 2 * k2 + 2 * k3 + k4) / 6
    return {"y_n": y}


_method("euler_method", "euler", "Euler's method", "Euler's method", r"y_{k+1} = y_k + h f(t_k, y_k)",
        "explicit Euler: y_{k+1} = y_k + h f(t_k, y_k)", _wrong_euler,
        (2, [r"euler'?s? method", r"\beuler method"]), unless=[r"improved|modified|backward|implicit|cauchy|heun"],
        ex=({"f": "t + y", "t0": 0, "y0": 1, "h": 0.1, "n": 2}, {"y_n": 1.22, "t_n": 0.2}),
        q2=("Use Euler's method for dy/dt = t + y, y(0) = 1, h = 0.1, n = 2.", {"y_n": 1.22}))
_method("improved_euler_method", "heun", "Improved Euler (Heun) method", "the improved Euler (Heun) method",
        r"y_{k+1} = y_k + \tfrac{h}{2}\left[f(t_k, y_k) + f(t_{k+1}, y_k + h f(t_k, y_k))\right]",
        "Heun predictor-corrector: average the slopes at both ends", _wrong_heun,
        (3, [r"improved euler|\bheun|modified euler"]),
        ex=({"f": "t + y", "t0": 0, "y0": 1, "h": 0.1, "n": 2}, {"y_n": 1.242050, "t_n": 0.2}),
        q2=("Use the improved Euler method for dy/dt = t + y, y(0) = 1, h = 0.1, n = 2.", {"y_n": 1.242050}))
_method("midpoint_method", "midpoint", "Explicit midpoint method (RK2)", "the explicit midpoint method",
        r"y_{k+1} = y_k + h f\left(t_k + \tfrac{h}{2}, y_k + \tfrac{h}{2} f(t_k, y_k)\right)",
        "second-order Runge-Kutta, slope at the midpoint", _wrong_midpoint,
        (3, [r"midpoint method"]),
        ex=({"f": "t + y", "t0": 0, "y0": 1, "h": 0.1, "n": 2}, {"y_n": 1.242050, "t_n": 0.2}),
        q2=("Apply the midpoint method to dy/dt = t + y, y(0) = 1, h = 0.1, n = 2.", {"y_n": 1.242050}))
_method("runge_kutta_4", "rk4", "Classical Runge-Kutta method (RK4)", "the classical fourth-order Runge-Kutta method",
        r"y_{k+1} = y_k + \tfrac{h}{6}(k_1 + 2k_2 + 2k_3 + k_4)", "classical RK4 with four slope evaluations per step",
        _wrong_rk4, (3, [r"runge[- ]kutta|\brk4\b"]), unless=[r"\bsystem\b|x'\s*=|dx/dt"],
        ex=({"f": "t + y", "t0": 0, "y0": 1, "h": 0.1, "n": 2}, {"y_n": 1.2428051417, "t_n": 0.2}),
        q2=("Use RK4 for dy/dt = t + y, y(0) = 1, h = 0.1, n = 2.", {"y_n": 1.2428051417}))


def backward_euler(f, t0, y0, h, n):
    _fin(t0, y0, h)
    n = int(n)
    if h <= 0 or not 1 <= n <= 10000:
        raise ValueError("needs h > 0 and 1 <= n <= 10000")
    e = parse_text(f)
    g, gy = compile_float(e, ("t", "y")), compile_float(sp.diff(e, YS), ("t", "y"))
    y = y0
    for k in range(n):
        t1 = t0 + (k + 1) * h
        z = y + h * g(t0 + k * h, y)                           # predictor
        for _ in range(100):
            F, J = z - y - h * g(t1, z), 1 - h * gy(t1, z)
            if J == 0:
                raise ValueError("Newton's method met a zero derivative")
            dz = F / J
            z -= dz
            if abs(dz) <= max(1e-15, 4e-16 * abs(z)):
                break
        else:
            raise ValueError("Newton's method did not converge for the implicit step")
        if h * abs(gy(t1, z)) >= 1:
            raise ValueError("the step is too large for a unique implicit solution (needs h*|df/dy| < 1)")
        y = z
        if not math.isfinite(y) or abs(y) > 1e100:
            raise ValueError("the numerical solution overflows")
    return {"y_n": _r(y), "t_n": _r(t0 + n * h)}


def _ref_backward_euler(f, t0, y0, h, n):
    from scipy.optimize import brentq
    F = ref_fn(f)
    y = y0
    for k in range(int(n)):
        t1 = t0 + (k + 1) * h
        g = lambda z: float(z - y - h * F(t1, z))
        lo, hi = y - 1.0, y + 1.0
        while g(lo) * g(hi) > 0:
            lo, hi = lo - (hi - lo), hi + (hi - lo)
        y = brentq(g, lo, hi, xtol=1e-15, rtol=8.9e-16, maxiter=500)
    return {"y_n": y}


add("backward_euler_method", name="Backward (implicit) Euler method", topic="numerical methods / one-step methods",
    purpose="Approximate y' = f(t, y), y(t0) = y0 after n implicit Euler steps: y_{k+1} = y_k + h f(t_{k+1}, y_{k+1}).",
    inputs=_METHOD_IN, outputs=_METHOD_OUT, primary="y_n", conditions=["h > 0", "h*|df/dy| < 1 at each step (unique step)"],
    method="each step solved by Newton's method from the explicit Euler predictor", latex=r"y_{k+1} = y_k + h f(t_{k+1}, y_{k+1})",
    run=backward_euler, reference=_ref_backward_euler,
    wrong=lambda f, t0, y0, h, n: _one_step("euler", _fn(f), t0, y0, h, int(n)),
    rand=lambda rng: {"f": rng.choice(["-2*y + t", "t - y", "-y + sin(t)", "-3*y", "1 - y", "cos(t) - 2*y", "-y^3", "t - y^2"]),
                      "t0": _u(rng, 0, 1), "y0": _u(rng, 0.1, 1.5), "h": _u(rng, 0.02, 0.15, 3), "n": rng.randint(1, 15)},
    templates=["Use the backward Euler method for dy/dt = -2*y + t, y(0) = 1, h = 0.1, n = 2."],
    examples=[({"f": "-2*y + t", "t0": 0, "y0": 1, "h": 0.1, "n": 2}, {"y_n": 0.718055555556})],
    questions=[{"q": "Use the backward Euler method for dy/dt = -2*y + t, y(0) = 1, h = 0.1, n = 2.", "outputs": {"y_n": 0.718055555556}},
               {"q": "Use the backward Euler method for dy/dt = -2*y + t, y(0) = 1, h = 0.1.", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"backward euler|implicit euler"]), text={"f": RHS_TY}, target=None)


def euler_error(k, y0, h, n):
    _fin(k, y0, h)
    n = int(n)
    if h <= 0 or not 1 <= n <= 10000:
        raise ValueError("needs h > 0 and 1 <= n <= 10000")
    ye, yx = y0 * (1 + h * k) ** n, y0 * _exp(k * n * h)
    return {"y_euler": _r(ye), "y_exact": _r(yx), "error": _r(_guard(yx - ye, yx, ye)),
            "relative_error": _r(_guard(yx - ye, yx, ye) / yx) if yx != 0 else 0.0}


def _ref_euler_error(k, y0, h, n):
    num = ref_rk("euler", lambda t, y: k * y, 0, y0, h, n)["y_n"]
    ex = ode_ref(lambda t, y: k * y, 0, y0, n * h)
    return {"y_euler": num, "y_exact": ex}


add("euler_error_exponential", name="Euler's method error for exponential growth", topic="numerical methods / error analysis",
    purpose="For y' = k y, y(0) = y0: the Euler approximation after n steps of size h, the exact value and the global error.",
    inputs=[N("k", "rate constant"), N("y0", "initial value"), N("h", "step size (> 0)"), _int_in("n", "number of steps")],
    outputs=[O("y_euler", "Euler value y0 (1 + h k)^n"), O("y_exact", "exact value y0 e^{k n h}"),
             O("error", "global error exact - Euler"), O("relative_error", "error / exact")], primary="y_euler",
    conditions=["h > 0"], method="Euler on a linear equation multiplies by (1 + hk) per step", latex=r"y_n = y_0(1 + hk)^n,\; y(t_n) = y_0 e^{k t_n}",
    run=euler_error, reference=_ref_euler_error,
    wrong=lambda k, y0, h, n: {"y_euler": y0 * (1 + h * k) ** (int(n) + 1), "y_exact": y0 * math.exp(k * int(n) * h)},
    rand=lambda rng: {"k": _nz(rng, -2, 2), "y0": _u(rng, 0.5, 5), "h": _u(rng, 0.01, 0.2, 3), "n": rng.randint(1, 40)},
    templates=["Find the global error of Euler's method for y' = k y with k = 1, y0 = 1, h = 0.1, n = 10."],
    examples=[({"k": 1, "y0": 1, "h": 0.1, "n": 10}, {"y_euler": 2.5937424601, "y_exact": 2.71828182846})],
    questions=[{"q": "Find the global error of Euler's method for y' = k y with k = 1, y0 = 1, h = 0.1, n = 10.",
                "outputs": {"y_euler": 2.5937424601}},
               {"q": "Find the global error of Euler's method for y' = k y with k = 1, y0 = 1, h = -0.1, n = 10.",
                "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"euler'?s? method|\beuler method", r"\berror\b|\bexact\b")]), unless=[r"cauchy"], target=None)


_RHS_X = _rhs(r"dx/dt|x'")
_RHS_Y = _rhs(r"dy/dt|y'")


def rk4_system(f, g, t0, x0, y0, h, n):
    _fin(t0, x0, y0, h)
    n = int(n)
    if h <= 0 or not 1 <= n <= 10000:
        raise ValueError("needs h > 0 and 1 <= n <= 10000")
    F, G = _fn(f, ("t", "x", "y")), _fn(g, ("t", "x", "y"))
    x, y = x0, y0
    for k in range(n):
        t = t0 + k * h
        a1, b1 = F(t, x, y), G(t, x, y)
        a2, b2 = F(t + h / 2, x + h * a1 / 2, y + h * b1 / 2), G(t + h / 2, x + h * a1 / 2, y + h * b1 / 2)
        a3, b3 = F(t + h / 2, x + h * a2 / 2, y + h * b2 / 2), G(t + h / 2, x + h * a2 / 2, y + h * b2 / 2)
        t1 = t0 + (k + 1) * h
        a4, b4 = F(t1, x + h * a3, y + h * b3), G(t1, x + h * a3, y + h * b3)
        x, y = x + h * (a1 + 2 * a2 + 2 * a3 + a4) / 6, y + h * (b1 + 2 * b2 + 2 * b3 + b4) / 6
        if not (math.isfinite(x) and math.isfinite(y)) or max(abs(x), abs(y)) > 1e100:
            raise ValueError("the numerical solution overflows")
    return {"x_n": _r(x), "y_n": _r(y), "t_n": _r(t0 + n * h)}


def _ref_rk4_system(f, g, t0, x0, y0, h, n):
    import mpmath
    A, b, c = _TABLEAU["rk4"]
    with mpmath.workdps(30):
        F, G = ref_fn(f, ("t", "x", "y")), ref_fn(g, ("t", "x", "y"))
        A = [[_mpq(v) for v in row] for row in A]
        b = [_mpq(v) for v in b]
        c = [_mpq(v) for v in c]
        H, u = mpmath.mpf(h), [mpmath.mpf(x0), mpmath.mpf(y0)]
        for k in range(int(n)):
            t = mpmath.mpf(t0) + k * H
            ks = []
            for i in range(4):
                ui = [u[d] + H * sum((A[i - 1][j] * ks[j][d] for j in range(i)), mpmath.mpf(0)) if i else u[d] for d in range(2)]
                ks.append([F(t + c[i] * H, *ui), G(t + c[i] * H, *ui)])
            u = [u[d] + H * sum(b[i] * ks[i][d] for i in range(4)) for d in range(2)]
        return {"x_n": float(u[0]), "y_n": float(u[1])}


def _wrong_rk4_system(f, g, t0, x0, y0, h, n):
    F, G = _fn(f, ("t", "x", "y")), _fn(g, ("t", "x", "y"))
    x, y = x0, y0
    for k in range(int(n)):                                    # each equation stepped with the OLD value of the other
        t = t0 + k * h
        x, y = x + h * F(t, x, y), y + h * G(t, x, y)
    return {"x_n": x, "y_n": y}


add("runge_kutta_4_system", name="RK4 for a system of two first-order equations", topic="numerical methods / systems",
    purpose="Approximate x' = f(t, x, y), y' = g(t, x, y) with x(t0) = x0, y(t0) = y0 after n RK4 steps of size h.",
    inputs=[{"symbol": "f", "type": "text", "kind": "number", "meaning": "x' = f(t, x, y)", "required": True, "max_length": 200},
            {"symbol": "g", "type": "text", "kind": "number", "meaning": "y' = g(t, x, y)", "required": True, "max_length": 200},
            N("t0", "initial time"), N("x0", "initial x"), N("y0", "initial y"), N("h", "step size (> 0)"),
            _int_in("n", "number of steps")],
    outputs=[O("x_n", "approximation of x(t0 + n h)"), O("y_n", "approximation of y(t0 + n h)"), O("t_n", "final time")],
    primary="x_n", conditions=["h > 0"], method="classical RK4 applied to the vector (x, y)",
    latex=r"\mathbf{u}_{k+1} = \mathbf{u}_k + \tfrac{h}{6}(\mathbf{k}_1 + 2\mathbf{k}_2 + 2\mathbf{k}_3 + \mathbf{k}_4)",
    run=rk4_system, reference=_ref_rk4_system, wrong=_wrong_rk4_system,
    rand=lambda rng: dict(zip(("f", "g"), rng.choice([("y", "-x"), ("x - x*y", "x*y - y"), ("-x + y", "-x - y"),
                                                         ("y", "-x - 0.5*y"), ("x*(1 - y)", "y*(x - 1)"), ("y + t", "-x")])),
                          t0=0.0, x0=_u(rng, 0.2, 2), y0=_u(rng, 0.2, 2), h=_u(rng, 0.02, 0.2, 3), n=rng.randint(1, 20)),
    templates=["Use RK4 on the system dx/dt = y, dy/dt = -x with x(0) = 1, y(0) = 0, h = 0.1, n = 10."],
    examples=[({"f": "y", "g": "-x", "t0": 0, "x0": 1, "y0": 0, "h": 0.1, "n": 10}, {"x_n": 0.540302967117, "y_n": -0.8414704778})],
    questions=[{"q": "Use RK4 on the system dx/dt = y, dy/dt = -x with x(0) = 1, y(0) = 0, t0 = 0, h = 0.1, n = 10.",
                "outputs": {"x_n": 0.540302967117}},
               {"q": "Use RK4 on the system dx/dt = y, dy/dt = -x with x(0) = 1, y(0) = 0, t0 = 0, h = 0.1.",
                "outcome": "NEEDS_INPUT"}],
    trig=(4, [_both(r"runge[- ]kutta|\brk4\b", r"\bsystem\b|x'\s*=|dx/dt")]), text={"f": _RHS_X, "g": _RHS_Y}, target=None)

# =================================================================================================================
# SECOND-ORDER LINEAR EQUATIONS
# =================================================================================================================
def _CS(d2, t):
    """C(t) = cosh(dt) / cos / 1 and S(t) = sinh(dt)/d / sin(wt)/w / t for d^2 = d2 (series near 0: no cancellation)."""
    z = d2 * t * t
    if abs(z) < 1:
        C, S, tc, ts, k = 1.0, t, 1.0, t, 1
        while True:
            tc *= z / ((2 * k - 1) * (2 * k))
            ts *= z / ((2 * k) * (2 * k + 1))
            C, S, k = C + tc, S + ts, k + 1
            if abs(tc) < 1e-18 and abs(ts) < 1e-18 * max(1.0, abs(t)):
                return C, S
    try:
        if d2 > 0:
            d = math.sqrt(d2)
            return math.cosh(d * t), math.sinh(d * t) / d
    except OverflowError:
        raise ValueError("the solution overflows") from None
    w = math.sqrt(-d2)
    return math.cos(w * t), math.sin(w * t) / w


def hom2(a, b, c, y0, v0, t):
    """y(t) of a y'' + b y' + c y = 0, y(0) = y0, y'(0) = v0 (one formula for every root case)."""
    m = -b / (2 * a)
    d2 = m * m - c / a
    E = _exp(m * t)
    C, S = _CS(d2, t)
    p1, p2 = y0 * C, (v0 - m * y0) * S
    return E * _guard(p1 + p2, p1, p2)


def _roots2(a, b, c):
    """Roots of a r^2 + b r + c (canonical order) and the exact discriminant sign."""
    A, B, Cc = Fraction(a), Fraction(b), Fraction(c)
    D = B * B - 4 * A * Cc
    if D > 0:
        sq = math.sqrt(D)
        q = -(b + math.copysign(sq, b)) / 2
        r = sorted([q / a, c / q])
        return r[0], r[1], D
    if D == 0:
        r = float(-B / (2 * A))
        return r, r, D
    re_, im = float(-B / (2 * A)) + 0.0, math.sqrt(-D) / (2 * abs(a))
    return complex(re_, im), complex(re_, -im), D


def _damping(a, b, c, D):
    if a < 0:
        a, b, c = -a, -b, -c
    if not (c > 0 and b >= 0):
        return "not a damped oscillator (needs a, c of one sign and b of that sign or 0)"
    if b == 0:
        return "undamped"
    return "overdamped" if D > 0 else ("critically damped" if D == 0 else "underdamped")


def char_roots(a, b, c):
    _fin(a, b, c)
    if a == 0:
        raise ValueError("a must not be 0 (the equation would be first order)")
    r1, r2, D = _roots2(a, b, c)
    if D > 0:
        form, kind = "y = C1 e^(r1 t) + C2 e^(r2 t)", "two distinct real roots"
    elif D == 0:
        form, kind = "y = (C1 + C2 t) e^(r t)", "one repeated real root"
    else:
        form, kind = "y = e^(alpha t) (C1 cos(beta t) + C2 sin(beta t)), alpha = Re r1, beta = Im r1", "complex conjugate roots"
    return {"r1": _r(r1), "r2": _r(r2), "discriminant": _r(float(D)), "roots_type": kind,
            "damping": _damping(a, b, c, D), "solution_form": form}


def _canon(rs):
    real = sorted(float(z.real) for z in rs if abs(complex(z).imag) == 0)
    cplx = sorted((complex(z) for z in rs if abs(complex(z).imag) > 0), key=lambda z: (z.real, -z.imag))
    return real + cplx


def _ref_char_roots(a, b, c):
    s = sp.Symbol("r")
    A, B, Cc = (sp.Rational(Fraction(v).numerator, Fraction(v).denominator) for v in (a, b, c))
    rs = []
    for z, mlt in sp.roots(sp.Poly(A * s**2 + B * s + Cc, s)).items():
        rs += [complex(sp.N(z, 30))] * mlt
    r = _canon([complex(z.real, z.imag) if abs(z.imag) > 1e-25 else complex(z.real, 0) for z in rs])
    return {"r1": r[0], "r2": r[1], "discriminant": float(B**2 - 4 * A * Cc)}


_LHS2 = r"(?:[-+]?\s*(?:" + _NUMRE + r")?\s*\*?\s*y(?:''|')?\s*)+"


def _eq2_pre(rhs_kind):
    """Read 'a y'' + b y' + c y = <rhs>' with numeric coefficients (or leave the named form); blank the equation."""
    def pre(nq):
        out = {}
        for m in re.finditer(r"(" + _LHS2 + r")=\s*([^,;?]+?)" + _STOP, nq):
            lhs, rhs = m.group(1), m.group(2).strip()
            if "y''" not in lhs:
                continue
            co = {"y''": 0.0, "y'": 0.0, "y": 0.0}
            pos = 0
            for t in re.finditer(r"([-+]?)\s*(" + _NUMRE + r")?\s*\*?\s*(y''|y'|y)", lhs):
                if lhs[pos:t.start()].strip():
                    return None
                v = _num(t.group(2)) if t.group(2) else 1.0
                co[t.group(3)] += -v if t.group(1) == "-" else v
                pos = t.end()
            if lhs[pos:].strip():
                return None
            out.update({"a": co["y''"], "b": co["y'"], "c": co["y"]})
            got = rhs_kind(rhs)
            if got is None:
                return None
            out.update(got)
            nq = nq[:m.start()] + " ; " + nq[m.end():]
            return nq, out
        # the named form "a y'' + b y' + c y = ...": blank the equation so its letters are not read as values
        return _blank_eq(nq), out
    return pre


def _blank_eq(nq):
    """Blank the written equation (named form 'a y'' + b y' + c y = ...') so its letters are not read as values."""
    return re.sub(r"[^,;:]*y''[^=]*=\s*.*?(?=\s*(?:,|;|\bwith\b|\bwhere\b|\bfor\b|\bat\b|$))", " ; ", nq, count=1)


def _rhs_zero(rhs):
    return {} if re.fullmatch(r"0(?:\.0*)?", rhs.strip()) else None


def _verify_char(i, o):
    """Each root substituted into a r^2 + b r + c, and the pair against Vieta (sum -b/a, product c/a)."""
    a, b, c = i["a"], i["b"], i["c"]
    try:
        worst = 0.0
        for r in (complex(o["r1"]), complex(o["r2"])):
            terms = [a * r * r, b * r, c]
            worst = max(worst, abs(sum(terms)) / (sum(abs(x) for x in terms) or 1.0))
        r1, r2 = complex(o["r1"]), complex(o["r2"])
        worst = max(worst, _diff(r1 + r2, -b / a) * (1 if abs(b / a) > 1e-12 else 0), _diff(r1 * r2, c / a) * (1 if c else 0))
        return max(worst, _diff(o.get("damping"), char_roots(**i)["damping"]))
    except Exception:
        return 1.0


def _rhs_exp(rhs):
    m = re.fullmatch(r"(" + _NUMRE + r"|[-+])?\s*\*?\s*(?:e\s*\^\s*\(?|exp\()\s*(" + _NUMRE + r"|[-+])?\s*\*?\s*t\s*\)?", rhs.replace(" ", ""))
    if not m:
        return {} if re.fullmatch(r"[A-Za-z]\w*\s*\*?\s*e\^\(?[a-z]\s*\*?\s*t\)?", rhs) else None
    f = lambda g: 1.0 if g in (None, "", "+") else (-1.0 if g == "-" else _num(g))
    return {"F": f(m.group(1)), "s": f(m.group(2))}


def _rhs_cos(rhs):
    m = re.fullmatch(r"(" + _NUMRE + r")?\*?cos\((" + _NUMRE + r")?\*?t\)", rhs.replace(" ", ""))
    if not m:
        return {} if re.fullmatch(r"[A-Za-z]\w*\s*\*?\s*cos\(\s*[a-z]\w*\s*\*?\s*t\s*\)", rhs) else None
    return {"F0": _num(m.group(1)) if m.group(1) else 1.0, "w": _num(m.group(2)) if m.group(2) else 1.0}


def _rhs_poly(rhs):
    try:
        p = sp.Poly(parse_text(rhs, ("t",)), TS)
    except (ValueError, sp.PolynomialError, sp.GeneratorsNeeded):
        return {} if re.fullmatch(r"[a-z]\w*(\(t\))?", rhs) else None
    co = [float(v) for v in reversed(p.all_coeffs())]
    if len(co) > 5:
        return None
    return {"p": ", ".join(repr(v) for v in co)}


def ivp2(a, b, c, y0, v0, t):
    _fin(a, b, c, y0, v0, t)
    if a == 0:
        raise ValueError("a must not be 0")
    r1, r2, D = _roots2(a, b, c)
    return {"y": _r(hom2(a, b, c, y0, v0, t)), "damping": _damping(a, b, c, D)}


def _ref_ivp2(a, b, c, y0, v0, t):
    y = ode_ref(lambda s, u: [u[1], -(b * u[1] + c * u[0]) / a], 0, [y0, v0], t)
    return {"y": y[0]}


def _wrong_ivp2(a, b, c, y0, v0, t):
    return {"y": hom2(a, -b, c, y0, v0, t)}                    # sign of the damping term flipped


_IVP2_IN = [N("a", "coefficient of y''", condition="a != 0"), N("b", "coefficient of y'"), N("c", "coefficient of y"),
            N("y0", "initial value y(0)"), N("v0", "initial slope y'(0)"), N("t", "time")]


def _rand2(rng, b_lo=-1.0):
    a = rng.choice([1, 1, 2, 0.5])
    b = rng.choice([0, round(rng.uniform(b_lo, 4), 1), rng.randint(0, 6)])
    c = rng.choice([round(rng.uniform(-2, 10), 1), rng.randint(1, 9)])
    return {"a": a, "b": b, "c": c, "y0": _u(rng, -3, 3), "v0": _u(rng, -3, 3), "t": _u(rng, 0.2, 4)}


add("characteristic_roots", name="Second-order constant-coefficient ODE: characteristic roots", topic="second-order ODEs / homogeneous",
    purpose="Roots of the characteristic equation of a y'' + b y' + c y = 0, the form of the general solution and the damping case.",
    inputs=[N("a", "coefficient of y''", condition="a != 0"), N("b", "coefficient of y'"), N("c", "coefficient of y")],
    outputs=[O("r1", "first root (real ascending; complex: positive imaginary part first)"), O("r2", "second root"),
             O("discriminant", "b^2 - 4ac"), O("roots_type", "distinct real / repeated / complex", "text"),
             O("damping", "overdamped / critically damped / underdamped / undamped", "text"),
             O("solution_form", "form of the general solution", "text")],
    primary="r1", conditions=["a != 0"], method="a r^2 + b r + c = 0 (stable quadratic formula); case from the exact sign of b^2 - 4ac",
    latex=r"a r^2 + b r + c = 0,\quad r = \frac{-b \pm \sqrt{b^2 - 4ac}}{2a}", run=char_roots, reference=_ref_char_roots,
    wrong=lambda a, b, c: (lambda o: {"r1": -o["r2"] if isinstance(o["r2"], float) else -o["r2"].conjugate(),
                                       "r2": -o["r1"] if isinstance(o["r1"], float) else -o["r1"].conjugate()})(char_roots(a, b, c)),
    verify=lambda i, o: _verify_char(i, o),
    rand=lambda rng: {"a": rng.choice([1, 2, -1, 0.5]), "b": rng.choice([rng.randint(-6, 6), _nz(rng, -5, 5, 1)]),
                      "c": rng.choice([rng.randint(-6, 9), _u(rng, -5, 9, 1)])},
    templates=["Find the characteristic roots of y'' + 3y' + 2y = 0.",
               "Characteristic equation of a y'' + b y' + c y = 0 with a = 1, b = 2, c = 5."],
    examples=[({"a": 1, "b": 3, "c": 2}, {"r1": -2.0, "r2": -1.0, "damping": "overdamped"}),
              ({"a": 1, "b": 2, "c": 5}, {"r1": complex(-1, 2), "r2": complex(-1, -2), "damping": "underdamped"}),
              ({"a": 1, "b": 4, "c": 4}, {"r1": -2.0, "r2": -2.0, "damping": "critically damped"})],
    questions=[{"q": "Find the characteristic roots of y'' + 3y' + 2y = 0.", "outputs": {"r1": -2.0, "r2": -1.0}},
               {"q": "Characteristic equation of a y'' + b y' + c y = 0 with a = 1, b = 2, c = 5.", "outputs": {"r1": complex(-1, 2)}},
               {"q": "Find the characteristic roots of y'' + 9y = 0.", "outputs": {"r1": complex(0, 3), "damping": "undamped"}},
               {"q": "Characteristic equation of a y'' + b y' + c y = 0 with a = 0, b = 2, c = 5.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"characteristic (equation|roots?|polynomial)"]), pre=_eq2_pre(_rhs_zero), target=None)

add("second_order_homogeneous_ivp", name="Second-order homogeneous linear ODE: initial value problem", topic="second-order ODEs / homogeneous",
    purpose="Solve a y'' + b y' + c y = 0 with y(0) = y0, y'(0) = v0 and give y(t) (overdamped, critical or underdamped).",
    inputs=_IVP2_IN, outputs=[O("y", "value y(t)"), O("damping", "damping case", "text")], primary="y", conditions=["a != 0"],
    method="y = e^{mt}[y0 C(t) + (v0 - m y0) S(t)], m = -b/(2a), C, S = cosh/sinh, 1/t or cos/sin by the sign of the discriminant",
    latex=r"y(t) = e^{mt}\left[y_0\cosh(\delta t) + (v_0 - m y_0)\frac{\sinh(\delta t)}{\delta}\right],\; m = -\frac{b}{2a},\; \delta^2 = m^2 - \frac{c}{a}",
    run=ivp2, reference=_ref_ivp2, wrong=_wrong_ivp2, rand=_rand2,
    templates=["Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).",
               "Second-order homogeneous equation a y'' + b y' + c y = 0 with a = 1, b = 2, c = 5, y0 = 1, v0 = 0, t = 2."],
    examples=[({"a": 1, "b": 3, "c": 2, "y0": 1, "v0": 0, "t": 1}, {"y": 0.600423599106}),
              ({"a": 1, "b": 0, "c": 4, "y0": 1, "v0": 0, "t": 1}, {"y": -0.416146836547})],
    questions=[{"q": "Solve y'' + 3y' + 2y = 0 with y(0) = 1, y'(0) = 0; find y(1).", "outputs": {"y": 0.600423599106}},
               {"q": "Solve y'' + 4y = 0 with y(0) = 1, y'(0) = 0; find y(1).", "outputs": {"y": -0.416146836547}},
               {"q": "Solve y'' + 3y' + 2y = 0 with y(0) = 1; find y(1).", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"y''[^=]*=\s*0\b(?!\.\d)", r"second[- ]order homogeneous"]), unless=[r"characteristic|cauchy|euler"], pre=_eq2_pre(_rhs_zero))


def _forced(a, b, c, y0, v0, t, yp_t, yp0, ypd0):
    h = hom2(a, b, c, y0 - yp0, v0 - ypd0, t)
    return _guard(yp_t + h, yp_t, h)


def uc_exp(a, b, c, F, s, y0, v0, t):
    _fin(a, b, c, F, s, y0, v0, t)
    if a == 0:
        raise ValueError("a must not be 0")
    A_, B_, C_, S_ = (Fraction(v) for v in (a, b, c, s))
    P, dP = A_ * S_ * S_ + B_ * S_ + C_, 2 * A_ * S_ + B_
    if P != 0:
        mlt, A = 0, F / float(P)
    elif dP != 0:
        mlt, A = 1, F / float(dP)
    else:
        mlt, A = 2, F / (2 * a)
    e = _exp(s * t)
    yp_t = A * t ** mlt * e
    yp0, ypd0 = (A, A * s) if mlt == 0 else ((0.0, A) if mlt == 1 else (0.0, 0.0))
    return {"y": _r(_forced(a, b, c, y0, v0, t, yp_t, yp0, ypd0)), "A": _r(A), "resonance_multiplicity": mlt}


def _ref_uc_exp(a, b, c, F, s, y0, v0, t):
    import mpmath
    y = ode_ref(lambda x, u: [u[1], (F * mpmath.exp(s * x) - b * u[1] - c * u[0]) / a], 0, [y0, v0], t)
    return {"y": y[0]}


_FORCE_IN = [N("a", "coefficient of y''", condition="a != 0"), N("b", "coefficient of y'"), N("c", "coefficient of y")]

add("undetermined_coefficients_exponential", name="Undetermined coefficients: exponential forcing", topic="second-order ODEs / forced equations",
    purpose="Solve a y'' + b y' + c y = F e^{s t}, y(0) = y0, y'(0) = v0: the particular coefficient (with resonance) and y(t).",
    inputs=_FORCE_IN + [N("F", "forcing amplitude"), N("s", "forcing exponent"), N("y0", "initial value"), N("v0", "initial slope"), N("t", "time")],
    outputs=[O("y", "value y(t)"), O("A", "coefficient of the particular solution A t^m e^{st}"),
             O("resonance_multiplicity", "m: 0, or 1/2 when s is a simple/double characteristic root")], primary="y",
    conditions=["a != 0"], method="y_p = A t^m e^{st}, A = F/P(s), F/P'(s) or F/(2a); plus the homogeneous solution fitted to the initial values",
    latex=r"y_p = \frac{F}{a s^2 + b s + c}\,e^{st}", run=uc_exp, reference=_ref_uc_exp,
    wrong=lambda a, b, c, F, s, y0, v0, t: {"y": uc_exp(a, b, c, F, s, y0, v0, t)["y"] - hom2(a, b, c, -uc_exp(a, b, c, F, s, 0, 0, 0)["A"] * (uc_exp(a, b, c, F, s, 0, 0, 0)["resonance_multiplicity"] == 0), -uc_exp(a, b, c, F, s, 0, 0, 0)["A"] * (s if uc_exp(a, b, c, F, s, 0, 0, 0)["resonance_multiplicity"] == 0 else (1 if uc_exp(a, b, c, F, s, 0, 0, 0)["resonance_multiplicity"] == 1 else 0)), t)},
    rand=lambda rng: (lambda d: dict(d, F=_nz(rng, -5, 5), s=rng.choice([_u(rng, -1.5, 1.0), -1.0, -2.0])))(_rand2(rng)),
    templates=["Solve y'' + 3y' + 2y = 4e^(3t) with y(0) = 0, y'(0) = 0; find y(1).",
               "Undetermined coefficients for a y'' + b y' + c y = F e^(s t) with a = 1, b = 3, c = 2, F = 1, s = -1, y0 = 0, v0 = 0, t = 2."],
    examples=[({"a": 1, "b": 3, "c": 2, "F": 4, "s": 3, "y0": 0, "v0": 0, "t": 1}, {"y": 3.75749617006, "A": 0.2, "resonance_multiplicity": 0}),
              ({"a": 1, "b": 3, "c": 2, "F": 1, "s": -1, "y0": 0, "v0": 0, "t": 2}, {"y": 0.153650922125, "A": 1.0, "resonance_multiplicity": 1})],
    questions=[{"q": "Solve y'' + 3y' + 2y = 4e^(3t) with y(0) = 0, y'(0) = 0; find y(1).", "outputs": {"A": 0.2}},
               {"q": "Solve y'' + 3y' + 2y = 4e^(3t) with y(0) = 0; find y(1).", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"y''[^=]*=[^,;]*(e\s*\^|exp\()"]), pre=_eq2_pre(_rhs_exp))


def uc_poly(a, b, c, p, y0, v0, t):
    _fin(a, b, c, y0, v0, t)
    if a == 0:
        raise ValueError("a must not be 0")
    if c == 0:
        raise ValueError("c must not be 0 (integrate once: the equation is first order in y')")
    if not 1 <= len(p) <= 5:
        raise ValueError("the forcing polynomial has degree at most 4 (1 to 5 coefficients p0, p1, ...)")
    m = len(p) - 1
    q = [0.0] * (m + 3)
    for k in range(m, -1, -1):
        q[k] = (p[k] - b * (k + 1) * q[k + 1] - a * (k + 2) * (k + 1) * q[k + 2]) / c
    q = q[:m + 1]
    yp_t = sum(qk * t ** k for k, qk in enumerate(q))
    return {"y": _r(_forced(a, b, c, y0, v0, t, yp_t, q[0], q[1] if m >= 1 else 0.0)),
            "particular": [_r(v) for v in q]}


def _ref_uc_poly(a, b, c, p, y0, v0, t):
    pol = lambda x: sum(pk * x ** k for k, pk in enumerate(p))
    y = ode_ref(lambda x, u: [u[1], (pol(x) - b * u[1] - c * u[0]) / a], 0, [y0, v0], t)
    return {"y": y[0]}


add("undetermined_coefficients_polynomial", name="Undetermined coefficients: polynomial forcing", topic="second-order ODEs / forced equations",
    purpose="Solve a y'' + b y' + c y = p0 + p1 t + ... (degree <= 4), y(0) = y0, y'(0) = v0: particular polynomial and y(t).",
    inputs=_FORCE_IN[:2] + [N("c", "coefficient of y", condition="c != 0"),
                           {"symbol": "p", "type": "vector", "kind": "number", "meaning": "forcing coefficients p0, p1, ... (ascending powers of t)",
                            "required": True, "min_length": 1, "max_length": 5},
                           N("y0", "initial value"), N("v0", "initial slope"), N("t", "time")],
    outputs=[O("y", "value y(t)"), O("particular", "coefficients q0, q1, ... of the particular polynomial", "vector")], primary="y",
    conditions=["a != 0", "c != 0", "degree <= 4"], method="match powers of t from the top: c q_k + b (k+1) q_{k+1} + a (k+2)(k+1) q_{k+2} = p_k",
    latex=r"y_p = \sum_k q_k t^k,\quad c\,q_k + b(k+1)q_{k+1} + a(k+2)(k+1)q_{k+2} = p_k", run=uc_poly, reference=_ref_uc_poly,
    wrong=lambda a, b, c, p, y0, v0, t: {"y": sum(pk * t ** k for k, pk in enumerate(p)) / c + hom2(a, b, c, y0, v0, t)},
    rand=lambda rng: dict(_rand2(rng), c=rng.choice([rng.randint(1, 9), _nz(rng, -3, 9, 1)]),
                          p=[_u(rng, -4, 4) for _ in range(rng.randint(1, 4))]),
    templates=["Solve y'' + 3y' + 2y = 4t^2 + 1 with y(0) = 0, y'(0) = 0; find y(1).",
               "Undetermined coefficients with polynomial forcing: a = 1, b = 0, c = 4, p = [0, 8], y0 = 1, v0 = 0, t = 0.5."],
    examples=[({"a": 1, "b": 0, "c": 4, "p": "0, 8", "y0": 1, "v0": 0, "t": 0.5}, {"y": 0.698831321060, "particular": [0.0, 2.0]})],
    questions=[{"q": "Undetermined coefficients with polynomial forcing: a = 1, b = 0, c = 4, p = [0, 8], y0 = 1, v0 = 0, t = 0.5.",
                "outputs": {"particular": [0.0, 2.0]}},
               {"q": "Solve y'' + 3y' = 4t^2 + 1 with y(0) = 0, y'(0) = 0; find y(1).", "outcome": "INVALID_INPUT"}],
    trig=(2, [r"y''[^=]*=(?![^,;]*(e\s*\^|exp|cos|sin))[^,;]*(?<![a-z])t(?![a-z])", r"polynomial forcing"]), pre=_eq2_pre(_rhs_poly), mats=("p",))


def _cos_particular(a, b, c, F0, w):
    A_, B_, C_, W_ = (Fraction(v) for v in (a, b, c, w))
    if B_ == 0 and abs(C_ - A_ * W_ * W_) <= Fraction(1, 10 ** 13) * (abs(C_) + abs(A_ * W_ * W_)):
        return None                       # (within 1e-13 of) resonance: the secular solution, continuous in the detuning
    Z = F0 / complex(c - a * w * w, b * w)
    return Z


def forced_cos(a, b, c, F0, w, y0, v0, t):
    _fin(a, b, c, F0, w, y0, v0, t)
    if a == 0 or w <= 0:
        raise ValueError("needs a != 0 and w > 0")
    Z = _cos_particular(a, b, c, F0, w)
    if Z is None:
        K = F0 / (2 * a * w)
        y = _forced(a, b, c, y0, v0, t, K * t * math.sin(w * t), 0.0, 0.0)
        return {"y": _r(y), "resonant": True}
    yp = Z.real * math.cos(w * t) - Z.imag * math.sin(w * t)
    return {"y": _r(_forced(a, b, c, y0, v0, t, yp, Z.real, -Z.imag * w)), "resonant": False}


def _ref_forced_cos(a, b, c, F0, w, y0, v0, t):
    import mpmath
    y = ode_ref(lambda x, u: [u[1], (F0 * mpmath.cos(w * x) - b * u[1] - c * u[0]) / a], 0, [y0, v0], t)
    return {"y": y[0]}


add("forced_oscillator_ivp", name="Sinusoidally forced linear oscillator: initial value problem", topic="second-order ODEs / forced equations",
    purpose="Solve a y'' + b y' + c y = F0 cos(w t), y(0) = y0, y'(0) = v0 (including pure resonance) and give y(t).",
    inputs=_FORCE_IN + [N("F0", "forcing amplitude"), N("w", "forcing angular frequency (> 0)"), N("y0", "initial value"),
                        N("v0", "initial slope"), N("t", "time")],
    outputs=[O("y", "value y(t)"), O("resonant", "undamped forcing at the natural frequency", "boolean")], primary="y",
    conditions=["a != 0", "w > 0"], method="complex amplitude Z = F0/(c - a w^2 + i b w); resonance: y_p = F0/(2aw) t sin(wt); plus homogeneous part",
    latex=r"y_p = \operatorname{Re}\frac{F_0 e^{i\omega t}}{c - a\omega^2 + i b\omega}", run=forced_cos, reference=_ref_forced_cos,
    wrong=lambda a, b, c, F0, w, y0, v0, t: {"y": forced_cos(a, b, c, F0, w, y0, v0, t)["y"] + hom2(a, b, c, y0, v0, t) - hom2(a, b, c, y0 - (_cos_particular(a, b, c, F0, w) or 0j).real, v0 + (_cos_particular(a, b, c, F0, w) or 0j).imag * w, t)},
    rand=lambda rng: (lambda d: dict(d, F0=_nz(rng, -5, 5), w=(math.sqrt(d["c"] / d["a"]) if (d["b"] == 0 and d["c"] > 0 and rng.random() < 0.2) else _u(rng, 0.3, 4))))(_rand2(rng)),
    templates=["Solve y'' + 2y' + 5y = 3cos(2t) with y(0) = 0, y'(0) = 0; find y(1).",
               "A forced oscillator a y'' + b y' + c y = F0 cos(w t) with a = 1, b = 0, c = 4, F0 = 1, w = 2, y0 = 0, v0 = 0, t = 3."],
    examples=[({"a": 1, "b": 0, "c": 4, "F0": 1, "w": 2, "y0": 0, "v0": 0, "t": 3}, {"y": -0.209561623649, "resonant": True})],
    questions=[{"q": "A forced oscillator a y'' + b y' + c y = F0 cos(w t) with a = 1, b = 0, c = 4, F0 = 1, w = 2, y0 = 0, v0 = 0, t = 3.",
                "outputs": {"resonant": True}},
               {"q": "Solve y'' + 2y' + 5y = 3cos(2t) with y(0) = 0; find y(1).", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"y''[^=]*=[^,;]*cos\(", r"forced oscillator"]), unless=[r"steady[- ]state|amplitude"], pre=_eq2_pre(_rhs_cos))


def steady_state(a, b, c, F0, w):
    _fin(a, b, c, F0, w)
    if a <= 0 or b <= 0 or c <= 0 or w <= 0:
        raise ValueError("needs a, b, c > 0 (damped oscillator) and w > 0")
    den = math.hypot(c - a * w * w, b * w)
    return {"amplitude": _r(abs(F0) / den), "phase_lag": _r(math.atan2(b * w, c - a * w * w)), "gain": _r(1 / den)}


def _ref_steady(a, b, c, F0, w):
    import numpy as np
    from scipy.signal import freqs
    _, H = freqs([1.0], [a, b, c], worN=[w])
    return {"amplitude": abs(F0) * abs(H[0]), "phase_lag": float(-np.angle(H[0])), "gain": abs(H[0])}


add("steady_state_amplitude", name="Forced damped oscillator: steady-state amplitude and phase", topic="second-order ODEs / forced equations",
    purpose="Steady-state response of a y'' + b y' + c y = F0 cos(w t): amplitude, phase lag and gain.",
    inputs=[N("a", "mass-like coefficient (> 0)"), N("b", "damping (> 0)"), N("c", "stiffness (> 0)"), N("F0", "forcing amplitude"),
            N("w", "forcing angular frequency (> 0)")],
    outputs=[O("amplitude", "steady-state amplitude"), O("phase_lag", "phase lag in radians (0 to pi)"), O("gain", "amplitude per unit force")],
    primary="amplitude", conditions=["a, b, c > 0", "w > 0"], method="|F0| / sqrt((c - a w^2)^2 + (b w)^2), lag atan2(b w, c - a w^2)",
    latex=r"A = \frac{F_0}{\sqrt{(c - a\omega^2)^2 + (b\omega)^2}},\; \tan\delta = \frac{b\omega}{c - a\omega^2}",
    run=steady_state, reference=_ref_steady,
    wrong=lambda a, b, c, F0, w: {"amplitude": abs(F0) / math.sqrt((c - a * w * w) ** 2 + b * w), "phase_lag": math.atan(b * w / (c - a * w * w)) if c != a * w * w else 0.0},
    rand=lambda rng: {"a": _u(rng, 0.5, 3), "b": _u(rng, 0.1, 3), "c": _u(rng, 1, 20), "F0": _nz(rng, -5, 5), "w": _u(rng, 0.2, 5)},
    templates=["Find the steady-state amplitude of y'' + b y' + c y = F0 cos(w t) with a = 1, b = 0.5, c = 4, F0 = 2, w = 1.5."],
    examples=[({"a": 1, "b": 0.5, "c": 4, "F0": 2, "w": 1.5}, {"amplitude": 1.05045146288})],
    questions=[{"q": "Find the steady-state amplitude of y'' + b y' + c y = F0 cos(w t) with a = 1, b = 0.5, c = 4, F0 = 2, w = 1.5.",
                "outputs": {"amplitude": 1.05045146288}},
               {"q": "Find the steady-state amplitude with a = 1, b = 0, c = 4, F0 = 2, w = 1.5.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"steady[- ]state (amplitude|response|solution)"]), pre=lambda nq: (_blank_eq(nq), {}),
    target=None)


def practical_resonance(m, c, k, F0):
    _fin(m, c, k, F0)
    if m <= 0 or c <= 0 or k <= 0:
        raise ValueError("needs m, c, k > 0")
    w2 = k / m - c * c / (2 * m * m)
    if w2 <= 0:
        return {"omega_r": 0.0, "amplitude_max": _r(abs(F0) / k), "has_peak": False}
    return {"omega_r": _r(math.sqrt(w2)), "amplitude_max": _r(abs(F0) / (c * math.sqrt(k / m - c * c / (4 * m * m)))), "has_peak": True}


def _ref_practical(m, c, k, F0):
    w = sp.Symbol("w", positive=True)
    M, Cc, K = (sp.Rational(repr(float(v))) for v in (m, c, k))
    den2 = (K - M * w**2) ** 2 + (Cc * w) ** 2
    crit = [z for z in sp.solve(sp.diff(den2, w), w) if z.is_real and z > 0]
    if not crit:
        return {"omega_r": 0.0, "amplitude_max": abs(F0) / k}
    wr = crit[0]
    return {"omega_r": float(wr), "amplitude_max": float(abs(sp.Rational(repr(float(F0)))) / sp.sqrt(den2.subs(w, wr)))}


add("practical_resonance", name="Damped oscillator: resonance frequency and peak amplitude", topic="second-order ODEs / resonance",
    purpose="For m y'' + c y' + k y = F0 cos(w t): the forcing frequency of maximum steady-state amplitude and that amplitude.",
    inputs=[N("m", "mass (> 0)"), N("c", "damping coefficient (> 0)"), N("k", "spring constant (> 0)"), N("F0", "forcing amplitude")],
    outputs=[O("omega_r", "resonance (peak) frequency, 0 when there is no peak"), O("amplitude_max", "peak amplitude"),
             O("has_peak", "c^2 < 2 m k", "boolean")], primary="amplitude_max", conditions=["m, c, k > 0"],
    method="minimise (k - m w^2)^2 + (c w)^2 over w: w_r^2 = k/m - c^2/(2m^2)",
    latex=r"\omega_r = \sqrt{\frac{k}{m} - \frac{c^2}{2m^2}},\quad A_{\max} = \frac{F_0}{c\sqrt{k/m - c^2/(4m^2)}}",
    run=practical_resonance, reference=_ref_practical,
    wrong=lambda m, c, k, F0: {"omega_r": math.sqrt(k / m), "amplitude_max": abs(F0) / (c * math.sqrt(k / m))},
    rand=lambda rng: {"m": _u(rng, 0.5, 5), "c": _u(rng, 0.05, 2), "k": _u(rng, 1, 30), "F0": _nz(rng, -10, 10)},
    templates=["Find the practical resonance frequency for m = 1, c = 0.4, k = 4, F0 = 2."],
    examples=[({"m": 1, "c": 0.4, "k": 4, "F0": 2}, {"omega_r": 1.97989898732, "amplitude_max": 2.51259453814})],
    questions=[{"q": "Find the practical resonance frequency for m = 1, c = 0.4, k = 4, F0 = 2.", "outputs": {"omega_r": 1.97989898732}},
               {"q": "Find the practical resonance frequency for m = 1, c = 0.4, k = 4.", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"practical resonance|resonance (frequency|peak)|peak amplitude"]), target=None)


def pure_resonance(m, k, F0, t):
    _fin(m, k, F0, t)
    if m <= 0 or k <= 0:
        raise ValueError("needs m > 0 and k > 0")
    w0 = math.sqrt(k / m)
    return {"y": _r(F0 / (2 * m * w0) * t * math.sin(w0 * t)), "envelope": _r(abs(F0) * t / (2 * m * w0)), "omega0": _r(w0)}


add("undamped_resonance", name="Undamped resonance: growing response", topic="second-order ODEs / resonance",
    purpose="m y'' + k y = F0 cos(w0 t) forced exactly at the natural frequency, starting from rest: y(t) and its linearly growing envelope.",
    inputs=[N("m", "mass (> 0)"), N("k", "spring constant (> 0)"), N("F0", "forcing amplitude"), N("t", "time")],
    outputs=[O("y", "displacement y(t)"), O("envelope", "amplitude envelope F0 t / (2 m w0)"), O("omega0", "natural frequency")],
    primary="y", conditions=["m, k > 0", "y(0) = y'(0) = 0"], method="particular solution t sin(w0 t)/(2 m w0) (secular term)",
    latex=r"y(t) = \frac{F_0}{2m\omega_0}\,t\sin(\omega_0 t),\quad \omega_0 = \sqrt{k/m}", run=pure_resonance,
    reference=lambda m, k, F0, t: {"y": ivp_ref(lambda x, u: [u[1], (F0 * math.cos(math.sqrt(k / m) * x) - k * u[0]) / m], 0, [0.0, 0.0], t)[0]},
    wrong=lambda m, k, F0, t: {"y": F0 / (m * math.sqrt(k / m)) * t * math.sin(math.sqrt(k / m) * t)},
    rand=lambda rng: {"m": _u(rng, 0.5, 5), "k": _u(rng, 1, 20), "F0": _nz(rng, -5, 5), "t": _u(rng, 0.3, 8)},
    templates=["Undamped resonance with m = 1, k = 4, F0 = 2: find y(3)."],
    examples=[({"m": 1, "k": 4, "F0": 2, "t": 3}, {"y": -0.419123247298, "envelope": 1.5})],
    questions=[{"q": "Undamped resonance with m = 1, k = 4, F0 = 2: find y(3).", "outputs": {"envelope": 1.5}},
               {"q": "Undamped resonance with m = 1, k = -4, F0 = 2: find y(3).", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"(undamped|pure) resonance"]))


def impulse2(a, b, c, t):
    _fin(a, b, c, t)
    if a == 0 or t < 0:
        raise ValueError("needs a != 0 and t >= 0")
    return {"h": _r(hom2(a, b, c, 0.0, 1.0 / a, t))}


def _ref_impulse(a, b, c, t):
    import numpy as np
    from scipy.linalg import expm
    A = np.array([[0.0, 1.0], [-c / a, -b / a]])
    return {"h": float((expm(A * t) @ np.array([0.0, 1.0 / a]))[0])}


add("ode_impulse_response", name="Impulse response of a second-order ODE", topic="Laplace transforms / impulse and step",
    purpose="The response h(t) of a y'' + b y' + c y = delta(t) from rest (the inverse Laplace transform of 1/(a s^2 + b s + c)).",
    inputs=[N("a", "coefficient of y''", condition="a != 0"), N("b", "coefficient of y'"), N("c", "coefficient of y"), N("t", "time (>= 0)")],
    outputs=[O("h", "impulse response h(t)")], primary="h", conditions=["a != 0", "t >= 0"],
    method="the impulse gives y(0+) = 0, y'(0+) = 1/a; then the homogeneous solution", latex=r"h(t) = \mathcal{L}^{-1}\left\{\frac{1}{as^2 + bs + c}\right\}",
    run=impulse2, reference=_ref_impulse, wrong=lambda a, b, c, t: {"h": hom2(a, b, c, 0.0, 1.0, t) if a != 1 else hom2(a, b, c, 1.0, 0.0, t)},
    rand=lambda rng: (lambda d: {"a": d["a"], "b": d["b"], "c": d["c"], "t": d["t"]})(_rand2(rng)),
    templates=["Find the impulse response of y'' + 2y' + 5y = delta(t) at t = 1."],
    examples=[({"a": 1, "b": 2, "c": 5, "t": 1}, {"h": 0.16725591462})],
    questions=[{"q": "Find the impulse response of y'' + 2y' + 5y = delta(t) at t = 1.", "outputs": {"h": 0.16725591462}},
               {"q": "Find the impulse response of y'' + 2y' + 5y = delta(t).", "outcome": "NEEDS_INPUT"}],
    trig=(4, [r"impulse response|dirac|delta\(t\)"]),
    pre=lambda nq: (lambda m: (nq, {}) if not m else _eq2_pre(lambda r: {} if re.fullmatch(r"(delta|δ)\(t\)", r) else None)(nq))(re.search(r"y''", nq)))


def step2(a, b, c, t):
    _fin(a, b, c, t)
    if a == 0 or c == 0 or t < 0:
        raise ValueError("needs a != 0, c != 0 and t >= 0")
    ys = 1.0 / c
    h = hom2(a, b, c, -ys, 0.0, t)
    out = {"y": _r(_guard(ys + h, ys, h))}
    if b / a > 0 and c / a > 0:
        out["final_value"] = _r(ys)
    return out


add("ode_step_response", name="Unit-step response of a second-order ODE", topic="Laplace transforms / impulse and step",
    purpose="The response of a y'' + b y' + c y = u(t) (unit step) from rest, and its final value 1/c when stable.",
    inputs=[N("a", "coefficient of y''", condition="a != 0"), N("b", "coefficient of y'"), N("c", "coefficient of y", condition="c != 0"),
            N("t", "time (>= 0)")],
    outputs=[O("y", "step response y(t)"), O("final_value", "limit 1/c (stable systems)")], primary="y", conditions=["a, c != 0", "t >= 0"],
    method="Y(s) = 1/(s(as^2 + bs + c)): constant 1/c plus the homogeneous solution with y(0) = -1/c, y'(0) = 0",
    latex=r"y(t) = \mathcal{L}^{-1}\left\{\frac{1}{s(as^2 + bs + c)}\right\}", run=step2,
    reference=lambda a, b, c, t: {"y": ode_ref(lambda x, u: [u[1], (1 - b * u[1] - c * u[0]) / a], 0, [0, 0], t)[0]},
    wrong=lambda a, b, c, t: {"y": 1 / c + hom2(a, b, c, -1 / c, -1 / c, t)},
    rand=lambda rng: (lambda d: {"a": d["a"], "b": d["b"], "c": rng.choice([rng.randint(1, 9), _nz(rng, -2, 9, 1)]), "t": d["t"]})(_rand2(rng)),
    templates=["Find the unit step response of y'' + 2y' + 5y = u(t) at t = 1."],
    examples=[({"a": 1, "b": 2, "c": 5, "t": 1}, {"y": 0.197167190211, "final_value": 0.2})],
    questions=[{"q": "Find the unit step response of y'' + 2y' + 5y = u(t) at t = 1.", "outputs": {"final_value": 0.2}},
               {"q": "Find the unit step response of y'' + 2y' = u(t) at t = 1.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"(unit[- ])?step response|heaviside step input"]),
    pre=lambda nq: (lambda m: (nq, {}) if not m else _eq2_pre(lambda r: {} if re.fullmatch(r"u\(t\)|h\(t\)|1", r) else None)(nq))(re.search(r"y''", nq)))


def oscillator_params(m, c, k):
    _fin(m, c, k)
    if m <= 0 or c < 0 or k <= 0:
        raise ValueError("needs m > 0, c >= 0, k > 0")
    w0 = math.sqrt(k / m)
    zeta = c / (2 * math.sqrt(m * k))
    D = Fraction(c) ** 2 - 4 * Fraction(m) * Fraction(k)
    out = {"omega0": _r(w0), "zeta": _r(zeta), "damping": _damping(m, c, k, D)}
    if D < 0:
        out["omega_d"] = _r(math.sqrt(4 * m * k - c * c) / (2 * m))
        out["period_d"] = _r(2 * math.pi / (math.sqrt(4 * m * k - c * c) / (2 * m)))
    return out


def _ref_osc(m, c, k):
    import numpy as np
    from scipy.linalg import eigvals
    lam = eigvals(np.array([[0.0, 1.0], [-k / m, -c / m]]))
    w0 = math.sqrt(abs(lam[0] * lam[1]))
    out = {"omega0": w0, "zeta": float(-(lam[0] + lam[1]).real / (2 * w0))}
    if abs(lam[0].imag) > 1e-9:
        out["omega_d"] = abs(lam[0].imag)
    return out


add("damped_oscillator_parameters", name="Mass-spring-damper: natural frequency, damping ratio, damped frequency",
    topic="second-order ODEs / oscillations",
    purpose="For m x'' + c x' + k x = 0: natural frequency, damping ratio, damping case and (underdamped) damped frequency and period.",
    inputs=[N("m", "mass (> 0)"), N("c", "damping coefficient (>= 0)"), N("k", "spring constant (> 0)")],
    outputs=[O("omega0", "natural frequency sqrt(k/m)"), O("zeta", "damping ratio c/(2 sqrt(mk))"), O("damping", "damping case", "text"),
             O("omega_d", "damped frequency (underdamped)"), O("period_d", "damped period (underdamped)")], primary="zeta",
    conditions=["m > 0", "c >= 0", "k > 0"], method="w0 = sqrt(k/m), zeta = c/(2 sqrt(mk)), w_d = w0 sqrt(1 - zeta^2)",
    latex=r"\omega_0 = \sqrt{k/m},\; \zeta = \frac{c}{2\sqrt{mk}},\; \omega_d = \omega_0\sqrt{1 - \zeta^2}", run=oscillator_params,
    reference=_ref_osc, wrong=lambda m, c, k: {"omega0": math.sqrt(k / m), "zeta": c / math.sqrt(m * k)},
    rand=lambda rng: {"m": _u(rng, 0.2, 10), "c": rng.choice([_u(rng, 0.05, 3), _u(rng, 3, 20)]), "k": _u(rng, 0.5, 50)},
    templates=["Find the damping ratio of a mass-spring-damper with m = 2, c = 1, k = 8."],
    examples=[({"m": 2, "c": 1, "k": 8}, {"omega0": 2.0, "zeta": 0.125, "damping": "underdamped"})],
    questions=[{"q": "Find the damping ratio of a mass-spring-damper with m = 2, c = 1, k = 8.", "outputs": {"zeta": 0.125}},
               {"q": "Find the damping ratio of a mass-spring-damper with m = 2, c = 1.", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"damping ratio|natural frequency|damped frequency|mass[- ]spring[- ]damper"]), target=None)


def shm(m, k, x0, v0, t):
    _fin(m, k, x0, v0, t)
    if m <= 0 or k <= 0:
        raise ValueError("needs m > 0 and k > 0")
    w = math.sqrt(k / m)
    return {"x": _r(x0 * math.cos(w * t) + v0 / w * math.sin(w * t)), "amplitude": _r(math.hypot(x0, v0 / w)),
            "omega": _r(w), "period": _r(2 * math.pi / w)}


def _ref_shm(m, k, x0, v0, t):
    import numpy as np
    from scipy.linalg import eigvals
    lam = eigvals(np.array([[0.0, 1.0], [-k / m, 0.0]]))
    w = abs(lam[0].imag)
    E = 0.5 * m * v0 * v0 + 0.5 * k * x0 * x0                  # energy: amplitude where all of it is potential
    return {"x": ode_ref(lambda s, u: [u[1], -k * u[0] / m], 0, [x0, v0], t)[0], "period": 2 * math.pi / w,
            "amplitude": math.sqrt(2 * E / k)}


add("simple_harmonic_motion", name="Simple harmonic motion: position, amplitude, period", topic="second-order ODEs / oscillations",
    purpose="Solve m x'' + k x = 0 with x(0) = x0, x'(0) = v0: position at t, amplitude, angular frequency and period.",
    inputs=[N("m", "mass (> 0)"), N("k", "spring constant (> 0)"), N("x0", "initial position"), N("v0", "initial velocity"), N("t", "time")],
    outputs=[O("x", "position x(t)"), O("amplitude", "amplitude"), O("omega", "angular frequency"), O("period", "period")],
    primary="x", conditions=["m, k > 0"], method="x = x0 cos(wt) + (v0/w) sin(wt), w = sqrt(k/m)",
    latex=r"x(t) = x_0\cos\omega t + \frac{v_0}{\omega}\sin\omega t,\; A = \sqrt{x_0^2 + (v_0/\omega)^2}", run=shm, reference=_ref_shm,
    wrong=lambda m, k, x0, v0, t: {"x": x0 * math.cos(math.sqrt(k / m) * t) + v0 * math.sin(math.sqrt(k / m) * t),
                                   "amplitude": math.hypot(x0, v0), "period": 2 * math.pi * math.sqrt(k / m)},
    rand=lambda rng: {"m": _u(rng, 0.2, 10), "k": _u(rng, 0.5, 50), "x0": _u(rng, -2, 2), "v0": _u(rng, -3, 3), "t": _u(rng, 0.1, 6)},
    templates=["Simple harmonic motion with m = 0.5, k = 8, x(0) = 0.1, x'(0) = 0; find x(1)."],
    examples=[({"m": 0.5, "k": 8, "x0": 0.1, "v0": 0, "t": 1}, {"omega": 4.0, "amplitude": 0.1})],
    questions=[{"q": "Simple harmonic motion with m = 0.5, k = 8, x(0) = 0.1, x'(0) = 0; find x(1).", "outputs": {"omega": 4.0}},
               {"q": "Simple harmonic motion with m = 0.5, k = 8, x(0) = 0.1; find x(1).", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"simple harmonic"]))


def cauchy_euler(a, b, c, y1, v1, t):
    _fin(a, b, c, y1, v1, t)
    if a == 0 or t <= 0:
        raise ValueError("needs a != 0 and t > 0")
    r1, r2, D = _roots2(a, b - a, c)
    return {"y": _r(hom2(a, b - a, c, y1, v1, math.log(t))), "m1": _r(r1), "m2": _r(r2)}


def _cauchy_pre(nq):
    nq = re.sub(r"\by\(\s*1\s*\)\s*=", "y1 =", nq)
    nq = re.sub(r"\by'\(\s*1\s*\)\s*=", "v1 =", nq)
    out = {}
    m = re.search(r"(" + _NUMRE + r")?\s*\*?\s*t\^2\s*\*?\s*y''\s*([-+])\s*(" + _NUMRE + r")?\s*\*?\s*t\s*\*?\s*y'\s*([-+])\s*(" + _NUMRE + r")?\s*\*?\s*y\s*=\s*0", nq)
    if m:
        out["a"] = _num(m.group(1)) if m.group(1) else 1.0
        out["b"] = (_num(m.group(3)) if m.group(3) else 1.0) * (-1 if m.group(2) == "-" else 1)
        out["c"] = (_num(m.group(5)) if m.group(5) else 1.0) * (-1 if m.group(4) == "-" else 1)
        nq = nq[:m.start()] + " ; " + nq[m.end():]
    else:
        nq = _blank_eq(nq)
    return nq, out


add("cauchy_euler_ivp", name="Cauchy-Euler (equidimensional) equation: initial value problem", topic="second-order ODEs / variable coefficients",
    purpose="Solve a t^2 y'' + b t y' + c y = 0 (t > 0) with y(1) = y1, y'(1) = v1 and give y(t) and the indicial roots.",
    inputs=[N("a", "coefficient of t^2 y''", condition="a != 0"), N("b", "coefficient of t y'"), N("c", "coefficient of y"),
            N("y1", "value y(1)"), N("v1", "slope y'(1)"), N("t", "time (> 0)")],
    outputs=[O("y", "value y(t)"), O("m1", "first indicial root"), O("m2", "second indicial root")], primary="y",
    conditions=["a != 0", "t > 0"], method="t = e^x turns it into a Y'' + (b - a) Y' + c Y = 0; indicial a m(m-1) + b m + c = 0",
    latex=r"a\,m(m-1) + b\,m + c = 0,\quad y = t^{m}", run=cauchy_euler,
    reference=lambda a, b, c, y1, v1, t: {"y": ode_ref(lambda x, u: [u[1], -(b * x * u[1] + c * u[0]) / (a * x * x)], 1, [y1, v1], t)[0]},
    wrong=lambda a, b, c, y1, v1, t: {"y": hom2(a, b, c, y1, v1, math.log(t))},
    rand=lambda rng: {"a": rng.choice([1, 2, 1]), "b": _u(rng, -3, 5, 1), "c": _u(rng, -3, 6, 1), "y1": _u(rng, -2, 2), "v1": _u(rng, -2, 2),
                      "t": _u(rng, 1.1, 4)},
    templates=["Solve the Cauchy-Euler equation t^2 y'' + 2t y' - 6y = 0 with y(1) = 1, y'(1) = 0; find y(2)."],
    examples=[({"a": 1, "b": 2, "c": -6, "y1": 1, "v1": 0, "t": 2}, {"y": 2.45, "m1": -3.0, "m2": 2.0})],
    questions=[{"q": "Solve the Cauchy-Euler equation t^2 y'' + 2t y' - 6y = 0 with y(1) = 1, y'(1) = 0; find y(2).",
                "outputs": {"y": 2.45, "m1": -3.0}},
               {"q": "Solve the Cauchy-Euler equation t^2 y'' + 2t y' - 6y = 0 with y(1) = 1, y'(1) = 0; find y(-2).", "outcome": "INVALID_INPUT"}],
    trig=(5, [r"cauchy[- ]euler|euler[- ]cauchy|equidimensional"]), pre=_cauchy_pre)


def bvp2(p, q, r, xa, xb, ya, yb, x):
    _fin(p, q, r, xa, xb, ya, yb, x)
    if xb <= xa:
        raise ValueError("needs xa < xb")
    if q != 0:
        yp, ypd = (lambda s: r / q), (lambda s: 0.0)
    elif p != 0:
        yp, ypd = (lambda s: r * s / p), (lambda s: r / p)
    else:
        yp, ypd = (lambda s: r * s * s / 2), (lambda s: r * s)
    L = xb - xa
    u1, u2 = hom2(1, p, q, 1.0, 0.0, L), hom2(1, p, q, 0.0, 1.0, L)
    if abs(u2) <= 1e-12 * (abs(u1) + L):
        raise ValueError("the boundary value problem has no unique solution (an eigenvalue case)")
    alpha = ya - yp(xa)
    beta = (yb - yp(xb) - alpha * u1) / u2
    h = hom2(1, p, q, alpha, beta, x - xa) if x >= xa else None
    if h is None:
        raise ValueError("x must lie in [xa, xb]")
    if x > xb:
        raise ValueError("x must lie in [xa, xb]")
    return {"y": _r(_guard(yp(x) + h, yp(x), h)), "slope_a": _r(beta + ypd(xa))}


def _ref_bvp(p, q, r, xa, xb, ya, yb, x):
    # shooting on the numerically integrated equation; the problem is linear, so two shots fix the slope exactly
    F = lambda s, u: [u[1], r - p * u[1] - q * u[0]]
    s0, s1 = 0.0, 1.0
    y0b = ode_ref(F, xa, [ya, s0], xb)[0]
    y1b = ode_ref(F, xa, [ya, s1], xb)[0]
    s = s0 + (yb - y0b) * (s1 - s0) / (y1b - y0b)
    return {"y": ode_ref(F, xa, [ya, s], x)[0] if x > xa else ya, "slope_a": s}


add("boundary_value_problem", name="Linear two-point boundary value problem (constant coefficients)", topic="second-order ODEs / boundary value problems",
    purpose="Solve y'' + p y' + q y = r on [xa, xb] with y(xa) = ya, y(xb) = yb; give y(x) and the slope at xa.",
    inputs=[N("p", "coefficient of y'"), N("q", "coefficient of y"), N("r", "constant right-hand side"), N("xa", "left end"),
            N("xb", "right end (> xa)"), N("ya", "value at xa"), N("yb", "value at xb"), N("x", "point in [xa, xb]")],
    outputs=[O("y", "value y(x)"), O("slope_a", "slope y'(xa)")], primary="y",
    conditions=["xa < xb", "unique solution (not an eigenvalue case)"], method="particular + homogeneous basis; boundary conditions fix the two constants",
    latex=r"y'' + p y' + q y = r,\; y(x_a) = y_a,\; y(x_b) = y_b", run=bvp2, reference=_ref_bvp,
    wrong=lambda p, q, r, xa, xb, ya, yb, x: {"y": ya + (yb - ya) * (x - xa) / (xb - xa)},
    rand=lambda rng: (lambda xa: {"p": _u(rng, -2, 2), "q": rng.choice([_u(rng, -3, 3), 0.0]), "r": _u(rng, -3, 3), "xa": xa,
                                  "xb": round(xa + rng.uniform(0.5, 2), 2), "ya": _u(rng, -2, 2), "yb": _u(rng, -2, 2),
                                  "x": round(xa + rng.uniform(0.1, 0.45), 3)})(_u(rng, -1, 1)),
    templates=["Solve the boundary value problem y'' + p y' + q y = r with p = 0, q = -1, r = 0, xa = 0, xb = 1, ya = 0, yb = 1, x = 0.5."],
    examples=[({"p": 0, "q": -1, "r": 0, "xa": 0, "xb": 1, "ya": 0, "yb": 1, "x": 0.5}, {"y": 0.443409441985})],
    questions=[{"q": "Solve the boundary value problem y'' + p y' + q y = r with p = 0, q = -1, r = 0, xa = 0, xb = 1, ya = 0, yb = 1, x = 0.5.",
                "outputs": {"y": 0.443409441985}},
               {"q": "Solve the boundary value problem y'' + p y' + q y = r with p = 0, q = 9.869604401089358, r = 0, xa = 0, xb = 1, ya = 0, yb = 1, x = 0.5.",
                "outcome": "INVALID_INPUT"}],
    trig=(4, [r"boundary[- ]value problem"]), pre=lambda nq: (_blank_eq(nq), {}), target=None)

# =================================================================================================================
# LAPLACE TRANSFORMS
# =================================================================================================================
def _expo_terms(text, exact=False):
    """f(t) as a sum of c t^n e^{z t} (z complex): sums and products of polynomials, exponentials, sin, cos, sinh, cosh."""
    e = parse_text(text, ("t",))
    e = sp.expand(e.rewrite(sp.exp))
    terms = []
    for term in sp.Add.make_args(e):
        c, n, z = sp.Integer(1), 0, sp.Integer(0)
        for f in sp.Mul.make_args(term):
            if not f.has(TS):
                c *= f
            elif f == TS:
                n += 1
            elif f.is_Pow and f.base == TS and f.exp.is_Integer and f.exp > 0:
                n += int(f.exp)
            elif f.func == sp.exp:
                arg = sp.expand(f.args[0])
                k = arg.coeff(TS, 1)
                rest = sp.expand(arg - k * TS)
                if k.has(TS) or rest.has(TS):
                    raise ValueError("supported: sums of c t^n e^{a t} sin/cos(b t) (an exponent not linear in t)")
                z += k
                c *= sp.exp(rest)
            else:
                raise ValueError("supported: sums of c t^n e^{a t} sin/cos/sinh/cosh(b t)")
        if n > 12:
            raise ValueError("powers of t up to 12 are supported")
        terms.append((c, n, z) if exact else (complex(sp.N(c, 30)), n, complex(sp.N(z, 30))))
    if not terms or all(abs(complex(sp.N(c))) == 0 for c, _, _ in terms):
        raise ValueError("the function is identically zero")
    return terms


def _laplace_sum(terms, s, power_shift=1):
    vals = [c * math.factorial(n) / (s - z) ** (n + power_shift) for c, n, z in terms]
    tot = sum(vals)
    big = sum(abs(v) for v in vals)
    if abs(tot.imag) > 1e-9 * big:
        raise ValueError("the function is not real")
    return _guard(tot.real, *[v.real for v in vals] or [0.0])


def laplace_value(f, s):
    _fin(s)
    terms = _expo_terms(f)
    sigma = max(z.real for _, _, z in terms)
    if s <= sigma:
        raise ValueError(f"the transform exists only for s > {sigma:g}")
    return {"F": _r(_laplace_sum(terms, s)), "sigma": _r(sigma)}


def _ref_laplace(f, s, shift=0.0):
    import mpmath
    with mpmath.workdps(30):
        g = ref_fn(f, ("t",))
        pts = [mpmath.mpf(shift)] + [mpmath.mpf(shift) + 2 ** k for k in range(0, 9)] + [mpmath.inf]
        v = mpmath.quad(lambda t: mpmath.exp(-s * t) * g(t - shift), pts)
        return {"F": float(mpmath.re(v))}


def _rand_expo(rng):
    a, b, n = _u(rng, -1.5, 1.5, 1), _u(rng, 0.5, 4, 1), rng.randint(0, 3)
    return rng.choice([f"{_nz(rng, -3, 3, 1)}*exp({a}*t)", f"t^{n}", f"sin({b}*t)", f"cos({b}*t)", f"exp({a}*t)*sin({b}*t)",
                       f"exp({a}*t)*cos({b}*t)", f"t*exp({a}*t)", f"cosh({b}*t)", f"sinh({b}*t)", f"2 + t^2", f"t*sin({b}*t)",
                       f"3*exp({a}*t) - t"])


def _rand_laplace(rng):
    f = _rand_expo(rng)
    sigma = max(z.real for _, _, z in _expo_terms(f))
    return {"f": f, "s": round(sigma + rng.uniform(0.5, 4), 2)}


_TXT_T = {"symbol": "f", "type": "text", "kind": "number", "meaning": "function f(t)", "required": True, "max_length": 200}
_RHS_LT = r"(?:laplace transform of|transform of|of)\s+(?:f\(t\)\s*=\s*)?(.+?)" + _STOP

add("laplace_transform", name="Laplace transform of a function at a point", topic="Laplace transforms / transforms",
    purpose="F(s) = integral from 0 to infinity of e^{-st} f(t) dt for sums of t^n e^{at} sin/cos(bt) (also sinh, cosh), and the abscissa of convergence.",
    inputs=[_TXT_T, N("s", "transform variable (s > abscissa)")],
    outputs=[O("F", "transform value F(s)"), O("sigma", "abscissa of convergence (F exists for s > sigma)")], primary="F",
    conditions=["f a sum of t^n e^{at} sin/cos(bt) terms", "s > sigma"],
    method="table: L{t^n e^{zt}} = n!/(s - z)^{n+1}, with sin and cos written through complex exponentials",
    latex=r"\mathcal{L}\{t^n e^{zt}\}(s) = \frac{n!}{(s - z)^{n+1}}", run=laplace_value, reference=_ref_laplace,
    wrong=lambda f, s: {"F": _laplace_sum(_expo_terms(f), s, power_shift=0)}, rand=_rand_laplace,
    templates=["Find the Laplace transform of t^2 e^(3t) at s = 5.", "Laplace transform of sin(2t) at s = 1."],
    examples=[({"f": "t^2*exp(3*t)", "s": 5}, {"F": 0.25, "sigma": 3.0}), ({"f": "sin(2t)", "s": 1}, {"F": 0.4})],
    questions=[{"q": "Find the Laplace transform of t^2 e^(3t) at s = 5.", "outputs": {"F": 0.25}},
               {"q": "Laplace transform of sin(2t) at s = 1.", "outputs": {"F": 0.4}},
               {"q": "Laplace transform of exp(3t) at s = 2.", "outcome": "INVALID_INPUT"},
               {"q": "Laplace transform of log(t) at s = 2.", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"laplace transform"]), unless=[r"inverse|shift|u\(t\s*-|heaviside"], text={"f": _RHS_LT}, target=None)


def laplace_shifted(f, c, s):
    _fin(c, s)
    if c < 0:
        raise ValueError("the delay c must be >= 0")
    terms = _expo_terms(f)
    sigma = max(z.real for _, _, z in terms)
    if s <= sigma:
        raise ValueError(f"the transform exists only for s > {sigma:g}")
    G = _laplace_sum(terms, s)
    return {"F": _r(math.exp(-c * s) * G), "G": _r(G)}


add("laplace_time_shift", name="Laplace transform of a delayed function u(t - c) f(t - c)", topic="Laplace transforms / shifting theorems",
    purpose="Second shifting theorem: the transform of u(t - c) f(t - c) is e^{-cs} F(s).",
    inputs=[_TXT_T, N("c", "delay (>= 0)"), N("s", "transform variable")],
    outputs=[O("F", "transform of the delayed function"), O("G", "transform of f itself")], primary="F",
    conditions=["c >= 0", "s > abscissa of f"], method="e^{-cs} times the table transform of f",
    latex=r"\mathcal{L}\{u(t - c) f(t - c)\} = e^{-cs}F(s)", run=laplace_shifted,
    reference=lambda f, c, s: _ref_laplace(f, s, shift=c),
    wrong=lambda f, c, s: {"F": math.exp(c * s) * laplace_value(f, s)["F"]},
    rand=lambda rng: dict(_rand_laplace(rng), c=_u(rng, 0, 2)),
    templates=["Find the Laplace transform of the delayed function u(t - c) f(t - c) with f(t) = t, c = 2, s = 1."],
    examples=[({"f": "t", "c": 2, "s": 1}, {"F": 0.135335283237, "G": 1.0})],
    questions=[{"q": "Find the Laplace transform of the delayed function u(t - c) f(t - c) with f(t) = t, c = 2, s = 1.",
                "outputs": {"F": 0.135335283237}},
               {"q": "Find the Laplace transform of the delayed function u(t - c) f(t - c) with f(t) = t, c = -2, s = 1.",
                "outcome": "INVALID_INPUT"}],
    trig=(4, [r"delayed function|u\(t\s*-\s*c\)|second shifting|time[- ]shift"]),
    text={"f": r"f\(t\)\s*=\s*(.+?)" + _STOP}, target=None)


def _ratfun(text):
    e = parse_text(text, ("s",))
    num, den = sp.fraction(sp.cancel(sp.together(e)))
    try:
        P, Q = sp.Poly(num, SS), sp.Poly(den, SS)
    except sp.PolynomialError:
        raise ValueError("F(s) must be a rational function of s") from None
    if not all(c.is_rational for c in P.all_coeffs() + Q.all_coeffs()):
        raise ValueError("F(s) needs rational coefficients")
    if P.is_zero:
        raise ValueError("F(s) is zero")
    if P.degree() >= Q.degree():
        raise ValueError("F(s) must be strictly proper (degree of numerator < degree of denominator)")
    if Q.degree() > 10:
        raise ValueError("denominators up to degree 10 are supported")
    return P, Q


def _poles(Q):
    """[(pole, multiplicity)] at 40 digits; multiplicities exact from the square-free factorisation."""
    import mpmath
    out = []
    for q, mlt in Q.sqf_list()[1]:
        co = [mpmath.mpf(c.p) / c.q for c in q.all_coeffs()]
        rs = [-co[1] / co[0]] if q.degree() == 1 else mpmath.polyroots(co, maxsteps=500, extraprec=300)
        out += [(mpmath.mpc(r), mlt) for r in rs]
    return out


def _taylor(co, p, m):
    """Coefficients of poly(p + u) for u^0 .. u^{m-1} (coefficients highest first), by repeated synthetic division."""
    co, res = list(co), []
    for _ in range(m):
        q, r = [], 0
        for c in co:
            r = r * p + c
            q.append(r)
        res.append(q.pop())
        co = q
        if not co:
            break
    return res + [0] * (m - len(res))


def inverse_laplace(F, t):
    import mpmath
    _fin(t)
    if t < 0:
        raise ValueError("t must be >= 0")
    P, Q = _ratfun(F)
    with mpmath.workdps(40):
        Pc = [mpmath.mpf(c.p) / c.q for c in P.all_coeffs()]
        Qc = [mpmath.mpf(c.p) / c.q for c in Q.all_coeffs()]
        T = mpmath.mpf(t)
        total, parts = mpmath.mpc(0), []
        for p, m in _poles(Q):
            R = list(Qc)
            for _ in range(m):                                  # deflate (s - p)^m
                q, r = [], 0
                for c in R:
                    r = r * p + c
                    q.append(r)
                q.pop()
                R = q
            n_t, r_t = _taylor(Pc, p, m), _taylor(R, p, m)
            g = []
            for k in range(m):
                g.append((n_t[k] - sum(r_t[j] * g[k - j] for j in range(1, k + 1))) / r_t[0])
            term = mpmath.exp(p * T) * sum(g[k] * T ** (m - 1 - k) / mpmath.factorial(m - 1 - k) for k in range(m))
            parts.append(term)
            total += term
        big = sum(abs(x) for x in parts)
        if abs(total.imag) > mpmath.mpf(10) ** -25 * max(big, 1):
            raise ValueError("internal: the inverse is not real")
        val = float(total.real)
        if big > 1e20 * max(abs(val), 1e-300):
            raise ValueError("the result is lost to cancellation")
    poles = _canon_c([complex(p) for p, _ in _poles(Q)])
    return {"f": _r(val), "poles": [_r(z) for z in poles]}


def _canon_c(zs):
    zs = [complex(z.real, 0.0) if abs(z.imag) <= 1e-25 * max(1.0, abs(z)) else z for z in zs]
    real = sorted(z.real for z in zs if z.imag == 0)
    cplx = sorted((z for z in zs if z.imag != 0), key=lambda z: (round(z.real, 9), -z.imag))
    return real + cplx


def _state_space_impulse(num, den, t):
    """Build-time only: inverse Laplace transform of num/den (strictly proper) as C e^{At} B (controllable form, scipy expm)."""
    import numpy as np
    from scipy.linalg import expm
    den = np.array(den, dtype=complex)
    num = np.array(num, dtype=complex) / den[0]
    den = den / den[0]
    n = len(den) - 1
    A = np.zeros((n, n), dtype=complex)
    A[:-1, 1:] = np.eye(n - 1)
    A[-1, :] = -den[1:][::-1]
    Cc = np.zeros(n, dtype=complex)
    Cc[:len(num)] = num[::-1]
    B = np.zeros(n, dtype=complex)
    B[-1] = 1
    return complex(Cc @ expm(A * t) @ B)


def _ref_inverse(F, t):
    e = parse_text(F, ("s",))
    num, den = sp.fraction(sp.cancel(sp.together(e)))
    P, Q = sp.Poly(num, SS), sp.Poly(den, SS)
    return {"f": _state_space_impulse([complex(c) for c in P.all_coeffs()], [complex(c) for c in Q.all_coeffs()], t).real}


def _rand_rational(rng, distinct=False):
    while True:
        k = rng.randint(1, 2 if distinct else 3)
        roots = []
        for _ in range(k):
            if rng.random() < 0.35:
                a, b = rng.randint(-3, 1), rng.randint(1, 3)
                roots.append(f"((s - ({a}))^2 + {b * b})")
            else:
                roots.append(f"(s - ({rng.randint(-4, 2)}))")
        if not distinct and rng.random() < 0.3:
            roots.append(roots[0])
        den = "*".join(roots)
        numer = rng.choice(["1", f"s + {rng.randint(-3, 3)}", f"{rng.randint(1, 4)}*s - 1", "2"])
        text = f"({numer})/({den})"
        try:
            P, Q = _ratfun(text)
            if distinct and any(m > 1 for _, m in _poles(Q)):
                continue
            return text
        except ValueError:
            continue


add("inverse_laplace_transform", name="Inverse Laplace transform of a rational function", topic="Laplace transforms / inverse transforms",
    purpose="f(t) for a strictly proper rational F(s) (any poles, repeated or complex), by residues / partial fractions.",
    inputs=[{"symbol": "F", "type": "text", "kind": "number", "meaning": "F(s), a rational function of s", "required": True, "max_length": 200},
            N("t", "time (>= 0)")],
    outputs=[O("f", "value f(t)"), O("poles", "poles of F(s) (canonical order)", "vector")], primary="f",
    conditions=["F strictly proper with rational coefficients", "denominator degree <= 10"],
    method="f(t) = sum over poles p (order m) of the residue of F(s) e^{st}: e^{pt} sum_k g_k t^{m-1-k}/(m-1-k)!",
    latex=r"f(t) = \sum_{p}\operatorname{Res}_{s=p}\,F(s)e^{st}", run=inverse_laplace, reference=_ref_inverse,
    wrong=lambda F, t: {"f": inverse_laplace(F, t)["f"] * math.exp(-t) if t else 1e3},
    rand=lambda rng: {"F": _rand_rational(rng), "t": _u(rng, 0.1, 3)},
    templates=["Find the inverse Laplace transform of (s + 3)/(s^2 + 3s + 2) at t = 1.",
               "Inverse Laplace transform of 1/(s^2 + 4) at t = 2."],
    examples=[({"F": "(s + 3)/(s^2 + 3s + 2)", "t": 1}, {"f": 0.600423599106}), ({"F": "1/(s^2 + 4)", "t": 2}, {"f": -0.378401247653})],
    questions=[{"q": "Find the inverse Laplace transform of (s + 3)/(s^2 + 3s + 2) at t = 1.", "outputs": {"f": 0.600423599106}},
               {"q": "Inverse Laplace transform of 1/(s^2 + 4) at t = 2.", "outputs": {"f": -0.378401247653}},
               {"q": "Inverse Laplace transform of 1/(s + 1)^2 at t = 1.", "outputs": {"f": 0.367879441171}},
               {"q": "Inverse Laplace transform of s^2/(s + 1) at t = 1.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"inverse laplace"]), text={"F": r"inverse laplace transform of\s+(?:f\(s\)\s*=\s*)?(.+?)" + _STOP}, target=None)


def partial_fractions(F):
    P, Q = _ratfun(F)
    ps = _poles(Q)
    if any(m > 1 for _, m in ps):
        raise ValueError("repeated poles: this capability lists simple poles only (use the inverse transform)")
    dQ = Q.diff(SS)
    Pc, dQc = [complex(c) for c in P.all_coeffs()], [complex(c) for c in dQ.all_coeffs()]
    ev = lambda co, z: sum(c * z ** (len(co) - 1 - i) for i, c in enumerate(co))
    pairs = []
    for p, _ in ps:
        z = complex(p)
        if abs(z.imag) <= 1e-25 * max(1.0, abs(z)):
            z = complex(z.real, 0.0)
        r = ev(Pc, z) / ev(dQc, z)
        pairs.append((z, r))
    order = _canon_c([z for z, _ in pairs])
    pz = {}
    for z, r in pairs:
        pz[z] = r
    out_p, out_r = [], []
    for z in order:
        zz = z if isinstance(z, complex) else complex(z, 0.0)
        r = pz[zz]
        out_p.append(_r(z if isinstance(z, complex) else float(z)))
        out_r.append(_r(complex(r.real, 0.0).real) if zz.imag == 0 else _r(r))
    return {"poles": out_p, "residues": out_r, "count": len(out_p)}


def _ref_partial(F):
    """Build-time only: sympy's exact roots of the denominator and sympy's residue (Laurent series) at each."""
    e = sp.cancel(sp.together(parse_text(F, ("s",))))
    num, den = sp.fraction(e)
    pr = []
    for z in sp.roots(sp.Poly(den, SS)):
        pr.append((complex(sp.N(z, 30)), complex(sp.N(sp.residue(e, SS, z), 30))))
    pairs = sorted(pr, key=lambda z: (round(z[0].real, 9), 0 if abs(z[0].imag) < 1e-9 else 1, -z[0].imag))
    poles = [float(z.real) if abs(z.imag) < 1e-9 else complex(z) for z, _ in pairs]
    res = [float(w.real) if abs(z.imag) < 1e-9 else complex(w) for z, w in pairs]
    real_first = sorted(range(len(poles)), key=lambda i: (0 if isinstance(poles[i], float) else 1,
                                                          poles[i].real if isinstance(poles[i], complex) else poles[i],
                                                          -poles[i].imag if isinstance(poles[i], complex) else 0))
    return {"poles": [poles[i] for i in real_first], "residues": [res[i] for i in real_first]}


add("partial_fraction_residues", name="Partial fractions of a rational function (simple poles)", topic="Laplace transforms / partial fractions",
    purpose="F(s) = sum r_i/(s - p_i): the poles and residues of a strictly proper rational function with simple poles.",
    inputs=[{"symbol": "F", "type": "text", "kind": "number", "meaning": "F(s), a rational function of s", "required": True, "max_length": 200}],
    outputs=[O("poles", "poles p_i (real ascending, then complex)", "vector"), O("residues", "residues r_i in the same order", "vector"),
             O("count", "number of poles")], primary="count",
    conditions=["strictly proper", "simple (distinct) poles"], method="r_i = P(p_i)/Q'(p_i) (cover-up rule)",
    latex=r"F(s) = \sum_i \frac{r_i}{s - p_i},\quad r_i = \frac{P(p_i)}{Q'(p_i)}", run=partial_fractions, reference=_ref_partial,
    wrong=lambda F: (lambda o: dict(o, residues=[-x for x in o["residues"]]))(partial_fractions(F)),
    verify=lambda i, o: _verify_partial(i, o),
    rand=lambda rng: {"F": _rand_rational(rng, distinct=True)},
    templates=["Find the partial fractions of (s + 3)/(s^2 + 3s + 2)."],
    examples=[({"F": "(s + 3)/(s^2 + 3s + 2)"}, {"poles": [-2.0, -1.0], "residues": [-1.0, 2.0]})],
    questions=[{"q": "Find the partial fractions of (s + 3)/(s^2 + 3s + 2).", "outputs": {"poles": [-2.0, -1.0], "residues": [-1.0, 2.0]}},
               {"q": "Find the partial fractions of 1/(s + 1)^2.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"partial fractions?"]), text={"F": r"partial fractions? (?:decomposition )?of\s+(.+?)" + _STOP}, target=None)


def _verify_partial(i, o):
    """Rebuild F at three test points from the poles and residues and compare with F itself."""
    try:
        P, Q = _ratfun(i["F"])
        Pc, Qc = [complex(c) for c in P.all_coeffs()], [complex(c) for c in Q.all_coeffs()]
        ev = lambda co, z: sum(c * z ** (len(co) - 1 - k) for k, c in enumerate(co))
        worst = 0.0
        for z in (complex(0.37, 1.91), complex(-2.3, 0.7), complex(5.1, -0.4)):
            want = ev(Pc, z) / ev(Qc, z)
            got = sum(complex(r) / (z - complex(p)) for p, r in zip(o["poles"], o["residues"]))
            worst = max(worst, abs(got - want) / max(abs(want), 1e-300))
        return max(worst if worst > 5e-11 else 0.0, _diff(o.get("count"), len(o["poles"])), _diff(len(o["poles"]), Q.degree()))
    except Exception:
        return 1.0


def convolution(f, g, t):
    import mpmath
    _fin(t)
    if t < 0:
        raise ValueError("t must be >= 0")
    if t == 0:
        return {"value": 0.0}
    F, G = _fn(f, ("t",)), _fn(g, ("t",))
    with mpmath.workdps(20):
        v, err = mpmath.quad(lambda u: F(float(u)) * G(t - float(u)), [0, t / 2, t], error=True)
    if err > 1e-13 * max(1.0, abs(v)):
        raise ValueError("the convolution integral did not converge")
    return {"value": _r(float(v))}


def _ref_conv(f, g, t):
    Fs = sum(c * sp.factorial(n) / (SS - z) ** (n + 1) for c, n, z in _expo_terms(f, exact=True))
    Gs = sum(c * sp.factorial(n) / (SS - z) ** (n + 1) for c, n, z in _expo_terms(g, exact=True))
    num, den = sp.fraction(sp.cancel(sp.together(Fs * Gs)))
    P, Q = sp.Poly(sp.expand(num), SS), sp.Poly(sp.expand(den), SS)
    return {"value": _state_space_impulse([complex(sp.N(c, 30)) for c in P.all_coeffs()],
                                          [complex(sp.N(c, 30)) for c in Q.all_coeffs()], t).real}


add("convolution_integral", name="Convolution integral (f * g)(t)", topic="Laplace transforms / convolution",
    purpose="(f * g)(t) = integral from 0 to t of f(u) g(t - u) du, whose transform is F(s) G(s).",
    inputs=[_TXT_T, {"symbol": "g", "type": "text", "kind": "number", "meaning": "function g(t)", "required": True, "max_length": 200},
            N("t", "time (>= 0)")],
    outputs=[O("value", "(f * g)(t)")], primary="value", conditions=["f, g integrable on [0, t]"],
    method="adaptive tanh-sinh quadrature of the convolution integral at 20 digits", latex=r"(f*g)(t) = \int_0^t f(u)\,g(t - u)\,du",
    run=convolution, reference=_ref_conv, wrong=lambda f, g, t: {"value": t * _fn(f, ("t",))(t / 2) * _fn(g, ("t",))(t / 2) * 1.5},
    rand=lambda rng: {"f": _rand_expo(rng), "g": _rand_expo(rng), "t": _u(rng, 0.2, 3)},
    templates=["Find the convolution of f(t) = exp(-t) and g(t) = sin(t) at t = 2."],
    examples=[({"f": "exp(-t)", "g": "sin(t)", "t": 2}, {"value": 0.730389773305}), ({"f": "1", "g": "t", "t": 2}, {"value": 2.0})],
    questions=[{"q": "Find the convolution of f(t) = exp(-t) and g(t) = sin(t) at t = 2.", "outputs": {"value": 0.730389773305}},
               {"q": "Find the convolution of f(t) = exp(-t) and g(t) = sin(t).", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"convolution"]), text={"f": r"f\(t\)\s*=\s*(.+?)" + _STOP, "g": r"g\(t\)\s*=\s*(.+?)" + _STOP}, target=None)


# =================================================================================================================
# LINEAR AND NONLINEAR SYSTEMS, STABILITY
# =================================================================================================================
def _classify(tr, det, scalar=False, zero=False):
    """Phase portrait from exact trace and determinant."""
    disc = tr * tr - 4 * det
    if det < 0:
        return "saddle", "unstable"
    if det == 0:
        if tr < 0:
            return "degenerate (line of equilibria)", "stable (not asymptotically)"
        if tr > 0:
            return "degenerate (line of equilibria)", "unstable"
        return ("degenerate (every point an equilibrium)", "stable (not asymptotically)") if zero else ("degenerate (shear)", "unstable")
    if tr == 0:
        return "center", "stable (not asymptotically)"
    side, stab = ("stable", "asymptotically stable") if tr < 0 else ("unstable", "unstable")
    if disc > 0:
        return f"{side} node", stab
    if disc < 0:
        return f"{side} spiral", stab
    return (f"{side} star (proper node)" if scalar else f"{side} degenerate node"), stab


def phase_portrait(A):
    if len(A) != 2 or len(A[0]) != 2:
        raise ValueError("A must be a 2x2 matrix")
    a, b, c, d = (Fraction(v) for v in (A[0][0], A[0][1], A[1][0], A[1][1]))
    tr, det = a + d, a * d - b * c
    kind, stab = _classify(tr, det, scalar=(b == 0 and c == 0 and a == d), zero=(a == b == c == d == 0))
    r1, r2, _ = _roots2(1.0, -float(tr), float(det))
    return {"type": kind, "stability": stab, "lambda1": _r(r1), "lambda2": _r(r2), "trace": _r(float(tr)), "det": _r(float(det))}


def _ref_phase(A):
    M = sp.Matrix([[sp.Rational(Fraction(v).numerator, Fraction(v).denominator) for v in row] for row in A])
    ev = M.eigenvects()
    lams = []
    for lam, mult, vecs in ev:
        lams += [(lam, len(vecs))] * mult
    l1, l2 = lams[0][0], lams[1][0]
    re1, re2 = sp.re(l1), sp.re(l2)
    if sp.im(l1) != 0:
        kind = "center" if re1 == 0 else ("stable spiral" if re1 < 0 else "unstable spiral")
        stab = "stable (not asymptotically)" if re1 == 0 else ("asymptotically stable" if re1 < 0 else "unstable")
    elif l1 * l2 < 0:
        kind, stab = "saddle", "unstable"
    elif l1 == 0 or l2 == 0:
        other = l2 if l1 == 0 else l1
        if other < 0:
            kind, stab = "degenerate (line of equilibria)", "stable (not asymptotically)"
        elif other > 0:
            kind, stab = "degenerate (line of equilibria)", "unstable"
        elif M.is_zero_matrix:
            kind, stab = "degenerate (every point an equilibrium)", "stable (not asymptotically)"
        else:
            kind, stab = "degenerate (shear)", "unstable"
    else:
        side, stab = ("stable", "asymptotically stable") if l1 < 0 else ("unstable", "unstable")
        if l1 != l2:
            kind = f"{side} node"
        else:
            kind = f"{side} star (proper node)" if lams[0][1] == 2 else f"{side} degenerate node"
    zs = _canon([complex(sp.N(l1, 30)), complex(sp.N(l2, 30))])
    return {"type": kind, "stability": stab, "lambda1": zs[0], "lambda2": zs[1]}


_MAT2 = {"symbol": "A", "type": "matrix", "kind": "number", "meaning": "the 2x2 coefficient matrix of x' = A x", "required": True}

add("phase_portrait_2x2", name="Linear 2x2 system: equilibrium type and stability (trace-determinant plane)",
    topic="linear systems / phase portraits",
    purpose="Classify the origin of x' = A x (node, saddle, spiral, center, star, degenerate) and its stability from the trace and determinant.",
    inputs=[_MAT2], outputs=[O("type", "phase portrait type", "text"), O("stability", "stability", "text"), O("lambda1", "first eigenvalue"),
                             O("lambda2", "second eigenvalue"), O("trace", "trace"), O("det", "determinant")],
    primary="trace", conditions=["A real 2x2"], method="exact trace T, determinant D and T^2 - 4D: the trace-determinant plane",
    latex=r"\lambda^2 - T\lambda + D = 0,\; T = \operatorname{tr}A,\; D = \det A", run=phase_portrait, reference=_ref_phase,
    wrong=lambda A: (lambda o: dict(o, stability={"asymptotically stable": "unstable", "unstable": "asymptotically stable"}.get(o["stability"], "unstable"),
                                    type=o["type"].replace("unstable", "X").replace("stable", "unstable").replace("X", "stable")))(phase_portrait(A)),
    rand=lambda rng: {"A": [[rng.randint(-4, 4), rng.randint(-4, 4)], [rng.randint(-4, 4), rng.randint(-4, 4)]]},
    templates=["Classify the equilibrium of the linear system x' = A x with A = [[0, 1], [-2, -3]].",
               "Phase portrait of x' = A x for A = [[1, 2], [3, 0]]."],
    examples=[({"A": "[[0, 1], [-2, -3]]"}, {"type": "stable node", "lambda1": -2.0, "lambda2": -1.0}),
              ({"A": "[[0, 1], [-1, 0]]"}, {"type": "center"}), ({"A": "[[1, 2], [3, 0]]"}, {"type": "saddle"}),
              ({"A": "[[-1, 0], [0, -1]]"}, {"type": "stable star (proper node)"})],
    questions=[{"q": "Classify the equilibrium of the linear system x' = A x with A = [[0, 1], [-2, -3]].", "outputs": {"type": "stable node"}},
               {"q": "Phase portrait of x' = A x for A = [[1, -5], [1, 1]].", "outputs": {"type": "unstable spiral"}},
               {"q": "Phase portrait of x' = A x for A = [[1, 2, 3], [3, 0]].", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"phase portrait", _both(r"classify", r"x'\s*=\s*a\s*\*?\s*x|(?<!non)linear system")]), mats=("A",), target=None)


def linear_system_solution(A, x0, t):
    import mpmath
    _fin(t)
    n = len(A)
    if n < 1 or n > 6 or any(len(r) != n for r in A):
        raise ValueError("A must be square, of size 1 to 6")
    if len(x0) != n:
        raise ValueError("x0 needs one entry per row of A")
    with mpmath.workdps(30):
        E = mpmath.expm(mpmath.matrix([[mpmath.mpf(v) for v in r] for r in A]) * mpmath.mpf(t))
        x = [sum(E[i, j] * mpmath.mpf(x0[j]) for j in range(n)) for i in range(n)]
        big = max(sum(abs(E[i, j] * x0[j]) for j in range(n)) for i in range(n))
    xs = [float(v) for v in x]
    for v in xs:
        if abs(v) < 1e-6 * float(big):
            raise ValueError("a component is lost to cancellation (it is near zero)")
    return {"x": [_r(v) for v in xs], "x1": _r(xs[0])}


def _ref_linsys(A, x0, t):
    import numpy as np
    from scipy.linalg import expm
    x = expm(np.array(A, dtype=float) * t) @ np.array(x0, dtype=float)
    return {"x": [float(v) for v in x], "x1": float(x[0])}


add("linear_system_solution", name="Linear system x' = A x: solution at a time", topic="linear systems / solutions",
    purpose="Solve x' = A x, x(0) = x0 (n up to 6) and give x(t) = e^{At} x0.",
    inputs=[{"symbol": "A", "type": "matrix", "kind": "number", "meaning": "n x n coefficient matrix", "required": True},
            {"symbol": "x0", "type": "vector", "kind": "number", "meaning": "initial vector x(0)", "required": True, "min_length": 1, "max_length": 6},
            N("t", "time")],
    outputs=[O("x", "state x(t)", "vector"), O("x1", "first component of x(t)")], primary="x1", conditions=["A square, n <= 6"],
    method="matrix exponential at 30 digits (scaling and squaring of the Taylor series)", latex=r"\mathbf{x}(t) = e^{At}\mathbf{x}_0",
    run=linear_system_solution, reference=_ref_linsys,
    wrong=lambda A, x0, t: _ref_linsys_wrong(A, x0, t),
    rand=lambda rng: (lambda n: {"A": [[rng.randint(-3, 3) for _ in range(n)] for _ in range(n)], "x0": [rng.randint(-3, 3) or 1 for _ in range(n)],
                                 "t": _u(rng, 0.1, 1.5)})(rng.randint(2, 4)),
    templates=["Solve the linear system x' = A x with A = [[0, 1], [-2, -3]], x0 = [1, 0] at t = 1."],
    examples=[({"A": "[[0, 1], [-2, -3]]", "x0": "1, 0", "t": 1}, {"x1": 0.600423599106})],
    questions=[{"q": "Solve the linear system x' = A x with A = [[0, 1], [-2, -3]], x0 = [1, 0] at t = 1.", "outputs": {"x1": 0.600423599106}},
               {"q": "Solve the linear system x' = A x with A = [[0, 1], [-2, -3]], x0 = [1, 0, 2] at t = 1.", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"solve|solution|state", r"x'\s*=\s*a\s*\*?\s*x"), r"(?<!non)linear system.*\bx0\s*="]), mats=("A", "x0"))


def _ref_linsys_wrong(A, x0, t):
    import mpmath
    n = len(A)
    with mpmath.workdps(20):
        E = mpmath.expm(mpmath.matrix([[mpmath.mpf(A[j][i]) for j in range(n)] for i in range(n)]) * t)   # transposed A
        x = [float(sum(E[i, j] * x0[j] for j in range(n))) for i in range(n)]
    return {"x": x, "x1": x[0]}


def _charpoly(A):
    """Characteristic polynomial coefficients (highest first) by Faddeev-LeVerrier, exact rationals."""
    n = len(A)
    M = [[Fraction(v) for v in r] for r in A]
    I = [[Fraction(int(i == j)) for j in range(n)] for i in range(n)]
    c = [Fraction(1)]
    Mk = [[Fraction(0)] * n for _ in range(n)]
    for k in range(1, n + 1):
        Mk = [[sum(M[i][l] * Mk[l][j] for l in range(n)) + c[-1] * I[i][j] for j in range(n)] for i in range(n)]
        AM = [[sum(M[i][l] * Mk[l][j] for l in range(n)) for j in range(n)] for i in range(n)]
        c.append(-sum(AM[i][i] for i in range(n)) / k)
    return c


def _routh(co):
    """Routh array first column and the number of sign changes (= roots in the right half plane); regular cases only."""
    co = [Fraction(v) for v in co]
    while co and co[0] == 0:
        co.pop(0)
    if len(co) < 2:
        raise ValueError("the polynomial needs degree >= 1")
    if co[0] < 0:
        co = [-v for v in co]
    r0, r1 = co[0::2], co[1::2]
    rows = [r0, r1 + [Fraction(0)] * (len(r0) - len(r1))]
    n = len(co) - 1
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        if b[0] == 0:
            raise ValueError("the Routh array has a zero in its first column (a root on or symmetric about the imaginary axis): "
                             "this version decides regular cases only")
        new = [(b[0] * a[j + 1] - a[0] * b[j + 1]) / b[0] for j in range(len(a) - 1)] + [Fraction(0)]
        rows.append(new)
    first = [r[0] for r in rows[:n + 1]]
    if any(v == 0 for v in first):
        raise ValueError("the Routh array has a zero in its first column: this version decides regular cases only")
    changes = sum(1 for x, y in zip(first, first[1:]) if (x > 0) != (y > 0))
    return first, changes


def routh_hurwitz(coeffs):
    if not 2 <= len(coeffs) <= 9:
        raise ValueError("give 2 to 9 coefficients (degree 1 to 8), highest power first")
    if coeffs[0] == 0:
        raise ValueError("the leading coefficient must not be 0")
    first, ch = _routh(coeffs)
    return {"stable": ch == 0, "rhp_roots": ch, "first_column": [_r(float(v)) for v in first]}


def _ref_routh(coeffs):
    import numpy as np
    r = np.roots(coeffs)
    k = int(sum(1 for z in r if z.real > 0))
    return {"stable": k == 0, "rhp_roots": k}


add("routh_hurwitz", name="Routh-Hurwitz stability test of a characteristic polynomial", topic="linear systems / stability",
    purpose="How many roots of a polynomial lie in the right half plane (Routh array), hence whether the system is asymptotically stable.",
    inputs=[{"symbol": "coeffs", "type": "vector", "kind": "number", "meaning": "coefficients, highest power first", "required": True,
             "min_length": 2, "max_length": 9}],
    outputs=[O("stable", "all roots in the open left half plane", "boolean"), O("rhp_roots", "number of right-half-plane roots"),
             O("first_column", "first column of the Routh array", "vector")], primary="rhp_roots",
    conditions=["regular Routh array (no zero in the first column)"], method="Routh array; sign changes in the first column",
    latex=r"\#\{\operatorname{Re} s > 0\} = \text{sign changes in the first column}", run=routh_hurwitz, reference=_ref_routh,
    wrong=lambda coeffs: {"stable": all(c > 0 for c in coeffs), "rhp_roots": 0 if all(c > 0 for c in coeffs) else 1},
    rand=lambda rng: {"coeffs": [1] + [rng.randint(-3, 9) for _ in range(rng.randint(2, 5))]},
    templates=["Apply the Routh-Hurwitz test to coeffs = [1, 6, 11, 6]."],
    examples=[({"coeffs": "1, 6, 11, 6"}, {"stable": True, "rhp_roots": 0}), ({"coeffs": "1, 1, 2, 8"}, {"stable": False, "rhp_roots": 2})],
    questions=[{"q": "Apply the Routh-Hurwitz test to coeffs = [1, 6, 11, 6].", "outputs": {"stable": True}},
               {"q": "Apply the Routh-Hurwitz test to coeffs = [1, 1, 2, 8].", "outputs": {"rhp_roots": 2}},
               {"q": "Apply the Routh-Hurwitz test to coeffs = [1, 0, 4].", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"routh|hurwitz"]), mats=("coeffs",), target=None)


def linear_stability(A):
    n = len(A)
    if n < 1 or n > 8 or any(len(r) != n for r in A):
        raise ValueError("A must be square, of size 1 to 8")
    c = _charpoly(A)
    if n == 1:
        lam = -c[1]
        if lam == 0:
            raise ValueError("an eigenvalue lies on the imaginary axis: linear stability is not decided by eigenvalue signs")
        k = int(lam > 0)
        return {"stable": k == 0, "unstable_eigenvalues": k, "char_poly": [_r(float(v)) for v in c]}
    first, k = _routh(c)
    return {"stable": k == 0, "unstable_eigenvalues": k, "char_poly": [_r(float(v)) for v in c]}


def _ref_linstab(A):
    import numpy as np
    ev = np.linalg.eigvals(np.array(A, dtype=float))
    k = int(sum(1 for z in ev if z.real > 0))
    return {"stable": k == 0, "unstable_eigenvalues": k}


add("linear_system_stability", name="Stability of x' = A x (n x n) from the characteristic polynomial", topic="linear systems / stability",
    purpose="Whether the origin of x' = A x is asymptotically stable and how many eigenvalues have positive real part, without computing eigenvalues.",
    inputs=[{"symbol": "A", "type": "matrix", "kind": "number", "meaning": "n x n coefficient matrix (n <= 8)", "required": True}],
    outputs=[O("stable", "asymptotically stable (all eigenvalues Re < 0)", "boolean"),
             O("unstable_eigenvalues", "number of eigenvalues with Re > 0"), O("char_poly", "characteristic polynomial coefficients", "vector")],
    primary="unstable_eigenvalues", conditions=["regular Routh array (no eigenvalue on the imaginary axis)"],
    method="characteristic polynomial by Faddeev-LeVerrier (exact), then the Routh array", latex=r"\det(\lambda I - A) = \lambda^n + c_1\lambda^{n-1} + \dots + c_n",
    run=linear_stability, reference=_ref_linstab,
    wrong=lambda A: {"stable": sum(A[i][i] for i in range(len(A))) < 0, "unstable_eigenvalues": int(sum(A[i][i] for i in range(len(A))) >= 0)},
    rand=lambda rng: (lambda n: {"A": [[rng.randint(-4, 3) for _ in range(n)] for _ in range(n)]})(rng.randint(2, 4)),
    templates=["Is the linear system x' = A x stable for A = [[-1, 2, 0], [0, -3, 1], [1, 0, -2]]?"],
    examples=[({"A": "[[-1, 2, 0], [0, -3, 1], [1, 0, -2]]"}, {"stable": True, "unstable_eigenvalues": 0})],
    questions=[{"q": "Is the linear system x' = A x stable for A = [[-1, 2, 0], [0, -3, 1], [1, 0, -2]]?", "outputs": {"stable": True}},
               {"q": "Is the linear system x' = A x stable for A = [[0, 1], [-1, 0]]?", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"\bstable\b|\bstability\b", r"x'\s*=\s*a\s*\*?\s*x|(?<!non)linear system")]), mats=("A",), target=None)


def equilibria_1d(f):
    e = parse_text(f, ("y",))
    try:
        P = sp.Poly(sp.expand(e), YS)
    except sp.PolynomialError:
        raise ValueError("f(y) must be a polynomial in y for this capability") from None
    if not all(c.is_rational for c in P.all_coeffs()):
        raise ValueError("the polynomial needs rational coefficients")
    if P.degree() < 1 or P.degree() > 8:
        raise ValueError("polynomials of degree 1 to 8 are supported")
    roots = sorted(set(sp.real_roots(P)), key=lambda r: float(sp.N(r, 30)))
    vals = [float(sp.N(r, 30)) for r in roots]
    tests = [sp.Rational(math.floor(vals[0]) - 1)] if vals else []
    for i in range(len(vals) - 1):
        m = sp.Rational(repr((vals[i] + vals[i + 1]) / 2))
        if not (roots[i] < m < roots[i + 1]):
            raise ValueError("two equilibria are too close to separate")
        tests.append(m)
    if vals:
        tests.append(sp.Rational(math.ceil(vals[-1]) + 1))
    sign = [int(sp.sign(P.eval(x))) for x in tests]
    stab = []
    for i in range(len(vals)):
        left, right = sign[i], sign[i + 1]
        stab.append("stable" if left > 0 > right else ("unstable" if left < 0 < right else "semi-stable"))
    return {"equilibria": [_r(v) for v in vals], "stability": stab, "count": len(vals)}


def _ref_eq1d(f):
    import mpmath
    e = parse_text(f, ("y",))
    co = [sp.Rational(c) for c in sp.Poly(sp.expand(e), YS).all_coeffs()]
    with mpmath.workdps(80):
        cm = [mpmath.mpf(c.p) / c.q for c in co]
        rs = mpmath.polyroots(cm, maxsteps=800, extraprec=600)
        real = sorted(float(r.real) if isinstance(r, mpmath.mpc) else float(r) for r in rs
                      if not isinstance(r, mpmath.mpc) or abs(r.imag) < mpmath.mpf(10) ** -25)
        uniq = []
        for v in real:
            if not uniq or abs(v - uniq[-1]) > 1e-12 * max(1, abs(v)):
                uniq.append(v)
        pts = ([uniq[0] - 1] + [(a + b) / 2 for a, b in zip(uniq, uniq[1:])] + [uniq[-1] + 1]) if uniq else []
        sg = [mpmath.sign(mpmath.polyval(cm, mpmath.mpf(x))) for x in pts]
    stab = ["stable" if sg[i] > 0 > sg[i + 1] else ("unstable" if sg[i] < 0 < sg[i + 1] else "semi-stable") for i in range(len(uniq))]
    return {"equilibria": uniq, "stability": stab, "count": len(uniq)}


add("autonomous_equilibria", name="Equilibria of y' = f(y) and their stability (phase line)", topic="first-order ODEs / equilibria and stability",
    purpose="All equilibrium solutions of an autonomous equation y' = f(y) with polynomial f and their stability from the sign of f.",
    inputs=[{"symbol": "f", "type": "text", "kind": "number", "meaning": "right-hand side f(y), a polynomial", "required": True, "max_length": 200}],
    outputs=[O("equilibria", "equilibrium values (ascending)", "vector"), O("stability", "stable / unstable / semi-stable for each", "vector"),
             O("count", "number of equilibria")], primary="count", conditions=["f a polynomial in y with rational coefficients, degree <= 8"],
    method="exact real roots of f; the sign of f between consecutive roots (phase line)", latex=r"f(y^*) = 0;\; f > 0 \text{ left},\, f < 0 \text{ right} \Rightarrow \text{stable}",
    run=equilibria_1d, reference=_ref_eq1d,
    wrong=lambda f: (lambda o: dict(o, stability=[{"stable": "unstable", "unstable": "stable"}.get(s, s) for s in o["stability"]]))(equilibria_1d(f)),
    rand=lambda rng: {"f": rng.choice([f"y*({rng.randint(1, 5)} - y)", f"y^2 - {rng.randint(1, 9)}", f"(y - 1)^2*(y + {rng.randint(1, 4)})",
                                       f"y^3 - {rng.randint(1, 6)}*y", f"-y*(y - 1)*(y - {rng.randint(2, 5)})", f"{rng.randint(1, 3)}*y - y^3",
                                       f"y^2*(y - {rng.randint(1, 3)})", f"0.5*y*(1 - y/{rng.randint(2, 9)})"])},
    templates=["Find the equilibria of dy/dt = y*(1 - y) and their stability."],
    examples=[({"f": "y*(1 - y)"}, {"equilibria": [0.0, 1.0], "stability": ["unstable", "stable"]}),
              ({"f": "y^2*(y - 2)"}, {"equilibria": [0.0, 2.0], "stability": ["semi-stable", "unstable"]})],
    questions=[{"q": "Find the equilibria of dy/dt = y*(1 - y) and their stability.", "outputs": {"equilibria": [0.0, 1.0], "stability": ["unstable", "stable"]}},
               {"q": "Find the equilibria of dy/dt = sin(y) and their stability.", "outcome": "INVALID_INPUT"}],
    trig=(3, [_both(r"equilibri(um|a)|critical points?|phase line", r"dy/dt|y'")]), unless=[r"system|x'|dx/dt|logistic"],
    text={"f": _rhs(r"dy/dt|y'")}, target=None)


def lotka_volterra(a, b, c, d):
    _fin(a, b, c, d)
    if min(a, b, c, d) <= 0:
        raise ValueError("all four parameters must be positive")
    w = math.sqrt(a * c)
    return {"x_eq": _r(c / d), "y_eq": _r(a / b), "omega": _r(w), "period": _r(2 * math.pi / w)}


def _ref_lv(a, b, c, d):
    import numpy as np
    x, y = sp.symbols("x y")
    A, B, Cc, D = (sp.Rational(repr(float(v))) for v in (a, b, c, d))
    f, g = A * x - B * x * y, -Cc * y + D * x * y
    sol = [s for s in sp.solve([f, g], [x, y], dict=True) if s[x] != 0]
    s0 = sol[0]
    J = sp.Matrix([f, g]).jacobian([x, y]).subs(s0)
    ev = np.linalg.eigvals(np.array(J.evalf(30).tolist(), dtype=float))
    w = abs(ev[0].imag)
    return {"x_eq": float(s0[x]), "y_eq": float(s0[y]), "omega": w, "period": 2 * math.pi / w}


add("lotka_volterra_equilibrium", name="Lotka-Volterra predator-prey: coexistence equilibrium and oscillation period",
    topic="dynamical systems / population models",
    purpose="For x' = a x - b x y (prey), y' = -c y + d x y (predator): the coexistence equilibrium and the period of small oscillations about it.",
    inputs=[N("a", "prey growth rate (> 0)"), N("b", "predation rate (> 0)"), N("c", "predator death rate (> 0)"), N("d", "predator efficiency (> 0)")],
    outputs=[O("x_eq", "prey at equilibrium c/d"), O("y_eq", "predators at equilibrium a/b"), O("omega", "small-oscillation angular frequency sqrt(ac)"),
             O("period", "small-oscillation period 2 pi / sqrt(ac)")], primary="period", conditions=["a, b, c, d > 0"],
    method="equilibrium of the nullclines; the Jacobian there has eigenvalues +- i sqrt(ac)",
    latex=r"(x^*, y^*) = \left(\frac{c}{d}, \frac{a}{b}\right),\; T = \frac{2\pi}{\sqrt{ac}}", run=lotka_volterra, reference=_ref_lv,
    wrong=lambda a, b, c, d: {"x_eq": a / b, "y_eq": c / d, "omega": math.sqrt(a * c), "period": 2 * math.pi / (a * c)},
    rand=lambda rng: {"a": _u(rng, 0.2, 3), "b": _u(rng, 0.01, 1, 3), "c": _u(rng, 0.2, 3), "d": _u(rng, 0.01, 1, 3)},
    templates=["Lotka-Volterra predator-prey model with a = 1, b = 0.1, c = 1.5, d = 0.075: find the equilibrium and period."],
    examples=[({"a": 1, "b": 0.1, "c": 1.5, "d": 0.075}, {"x_eq": 20.0, "y_eq": 10.0, "period": 5.13019932064})],
    questions=[{"q": "Lotka-Volterra predator-prey model with a = 1, b = 0.1, c = 1.5, d = 0.075: find the equilibrium and period.",
                "outputs": {"x_eq": 20.0, "y_eq": 10.0}},
               {"q": "Lotka-Volterra predator-prey model with a = 1, b = 0.1, c = 1.5: find the equilibrium.", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"lotka|predator[- ]prey"]), target=None)


def competing_species(a1, b11, b12, a2, b21, b22):
    _fin(a1, b11, b12, a2, b21, b22)
    A1, B11, B12, A2, B21, B22 = (Fraction(v) for v in (a1, b11, b12, a2, b21, b22))
    det = B11 * B22 - B12 * B21
    if det == 0:
        raise ValueError("the nullclines are parallel: no isolated coexistence equilibrium")
    x = (A1 * B22 - B12 * A2) / det
    y = (B11 * A2 - B21 * A1) / det
    if x <= 0 or y <= 0:
        raise ValueError("there is no coexistence equilibrium with both populations positive")
    J = [[-B11 * x, -B12 * x], [-B21 * y, -B22 * y]]
    kind, stab = _classify(J[0][0] + J[1][1], J[0][0] * J[1][1] - J[0][1] * J[1][0])
    return {"x_eq": _r(float(x)), "y_eq": _r(float(y)), "type": kind, "stability": stab}


def _ref_comp(a1, b11, b12, a2, b21, b22):
    import numpy as np
    x, y = np.linalg.solve(np.array([[b11, b12], [b21, b22]], dtype=float), np.array([a1, a2], dtype=float))
    ev = np.linalg.eigvals(np.array([[-b11 * x, -b12 * x], [-b21 * y, -b22 * y]]))
    if abs(ev[0].imag) > 1e-12:
        kind = "stable spiral" if ev[0].real < 0 else "unstable spiral"
    elif ev[0].real * ev[1].real < 0:
        kind = "saddle"
    else:
        kind = "stable node" if ev[0].real < 0 else "unstable node"
    return {"x_eq": float(x), "y_eq": float(y), "type": kind}


add("competing_species_equilibrium", name="Competing species: coexistence equilibrium and its stability", topic="dynamical systems / population models",
    purpose="For x' = x(a1 - b11 x - b12 y), y' = y(a2 - b21 x - b22 y): the coexistence equilibrium and its type (stable coexistence or saddle).",
    inputs=[N("a1", "growth rate of x"), N("b11", "self-limitation of x"), N("b12", "effect of y on x"), N("a2", "growth rate of y"),
            N("b21", "effect of x on y"), N("b22", "self-limitation of y")],
    outputs=[O("x_eq", "x at coexistence"), O("y_eq", "y at coexistence"), O("type", "equilibrium type", "text"), O("stability", "stability", "text")],
    primary="x_eq", conditions=["both coexistence values positive", "nullclines not parallel"],
    method="solve the linear nullcline equations exactly; classify the Jacobian there by trace and determinant",
    latex=r"J = \begin{pmatrix} -b_{11}x^* & -b_{12}x^* \\ -b_{21}y^* & -b_{22}y^* \end{pmatrix}", run=competing_species, reference=_ref_comp,
    wrong=lambda a1, b11, b12, a2, b21, b22: {"x_eq": a1 / b11, "y_eq": a2 / b22, "type": "stable node"},
    rand=lambda rng: {"a1": _u(rng, 0.5, 3), "b11": _u(rng, 0.2, 2), "b12": _u(rng, 0.1, 2), "a2": _u(rng, 0.5, 3), "b21": _u(rng, 0.1, 2),
                      "b22": _u(rng, 0.2, 2)},
    templates=["Competing species with a1 = 3, b11 = 1, b12 = 1, a2 = 2, b21 = 0.5, b22 = 1: find the coexistence equilibrium."],
    examples=[({"a1": 3, "b11": 1, "b12": 1, "a2": 2, "b21": 0.5, "b22": 1}, {"x_eq": 2.0, "y_eq": 1.0, "type": "stable node"})],
    questions=[{"q": "Competing species with a1 = 3, b11 = 1, b12 = 1, a2 = 2, b21 = 0.5, b22 = 1: find the coexistence equilibrium.",
                "outputs": {"x_eq": 2.0, "y_eq": 1.0, "type": "stable node"}},
               {"q": "Competing species with a1 = 1, b11 = 1, b12 = 2, a2 = 3, b21 = 1, b22 = 1: find the coexistence equilibrium.",
                "outcome": "INVALID_INPUT"}],
    trig=(3, [r"compet(ing|ition) species|species compet"]), target=None)


def jacobian_classify(f, g, x0, y0):
    _fin(x0, y0)
    F, G = parse_text(f, ("x", "y")), parse_text(g, ("x", "y"))
    pt = {XS: sp.Rational(repr(float(x0))), YS: sp.Rational(repr(float(y0)))}
    for h in (F, G):
        v = sp.N(h.subs(pt), 40)
        if abs(complex(v)) > 1e-12:
            raise ValueError("the point is not an equilibrium (f and g do not both vanish there)")
    J = [[sp.diff(h, v).subs(pt) for v in (XS, YS)] for h in (F, G)]
    tr, det = sp.nsimplify(sp.N(J[0][0] + J[1][1], 50), rational=True, tolerance=1e-40), sp.nsimplify(
        sp.N(J[0][0] * J[1][1] - J[0][1] * J[1][0], 50), rational=True, tolerance=1e-40)
    trF, detF = Fraction(int(tr.p), int(tr.q)), Fraction(int(det.p), int(det.q))
    Jf = [[float(sp.N(v, 30)) for v in row] for row in J]
    kind, stab = _classify(trF, detF, scalar=(Jf[0][1] == 0 and Jf[1][0] == 0 and Jf[0][0] == Jf[1][1]),
                           zero=all(v == 0 for row in Jf for v in row))
    if kind == "center" or kind.startswith("degenerate"):
        stab = "linearization inconclusive"
    return {"type": kind, "stability": stab, "jacobian": [[_r(v) for v in row] for row in Jf], "trace": _r(float(trF)), "det": _r(float(detF))}


def _ref_jac(f, g, x0, y0):
    import mpmath
    with mpmath.workdps(30):
        F, G = ref_fn(f, ("x", "y")), ref_fn(g, ("x", "y"))
        J = [[mpmath.diff(h, (x0, y0), (1, 0)), mpmath.diff(h, (x0, y0), (0, 1))] for h in (F, G)]
        tr, det = J[0][0] + J[1][1], J[0][0] * J[1][1] - J[0][1] * J[1][0]
        z = lambda v: 0 if abs(v) < mpmath.mpf(10) ** -20 else (1 if v > 0 else -1)
        disc = tr * tr - 4 * det
        if z(det) < 0:
            kind = "saddle"
        elif z(tr) == 0 and z(det) > 0:
            kind = "center"
        elif z(det) > 0:
            side = "stable" if z(tr) < 0 else "unstable"
            kind = f"{side} node" if z(disc) > 0 else (f"{side} spiral" if z(disc) < 0 else None)
        else:
            kind = None
        out = {"jacobian": [[float(v) for v in row] for row in J]}
        if kind:
            out["type"] = kind
        return out


_SYS2 = [("y", "-sin(x) - 0.5*y", 0, 0), ("x - x*y", "x*y - y", 1, 1), ("x*(3 - x - 2*y)", "y*(2 - x - y)", 1, 1),
         ("y", "-x + (1 - x^2)*y", 0, 0), ("x^2 - y", "x - y", 1, 1), ("y", "x - x^3", 1, 0), ("y", "x - x^3", 0, 0),
         ("-x + x*y", "-2*y + x^2", 0, 0), ("x + y^2", "-y + x", 0, 0), ("y - x", "x^2 - y", 1, 1), ("2*x - x*y", "x*y - 3*y", 3, 2)]

add("nonlinear_equilibrium_type", name="Nonlinear 2-D system: type of an equilibrium by linearization", topic="dynamical systems / linearization",
    purpose="For x' = f(x, y), y' = g(x, y) and an equilibrium (x0, y0): the Jacobian there, the equilibrium type and stability (Hartman-Grobman).",
    inputs=[{"symbol": "f", "type": "text", "kind": "number", "meaning": "x' = f(x, y)", "required": True, "max_length": 200},
            {"symbol": "g", "type": "text", "kind": "number", "meaning": "y' = g(x, y)", "required": True, "max_length": 200},
            N("x0", "equilibrium x"), N("y0", "equilibrium y")],
    outputs=[O("type", "type of the linearization", "text"), O("stability", "stability (inconclusive for a center or a degenerate case)", "text"),
             O("jacobian", "Jacobian at the equilibrium", "matrix"), O("trace", "trace of the Jacobian"), O("det", "determinant of the Jacobian")],
    primary="trace", conditions=["(x0, y0) is an equilibrium"], method="exact symbolic Jacobian at the point; trace-determinant classification",
    latex=r"J = \begin{pmatrix} f_x & f_y \\ g_x & g_y \end{pmatrix}_{(x_0, y_0)}", run=jacobian_classify, reference=_ref_jac,
    wrong=lambda f, g, x0, y0: (lambda o: dict(o, jacobian=[[o["jacobian"][0][0], o["jacobian"][1][0]], [o["jacobian"][0][1] + 1, o["jacobian"][1][1]]],
                                               type="saddle" if o["type"] != "saddle" else "stable node"))(jacobian_classify(f, g, x0, y0)),
    rand=lambda rng: dict(zip(("f", "g", "x0", "y0"), rng.choice(_SYS2))),
    templates=["Classify the equilibrium of the nonlinear system dx/dt = y, dy/dt = -sin(x) - 0.5*y at x0 = 0, y0 = 0."],
    examples=[({"f": "y", "g": "-sin(x) - 0.5*y", "x0": 0, "y0": 0}, {"type": "stable spiral", "stability": "asymptotically stable"}),
              ({"f": "x - x*y", "g": "x*y - y", "x0": 1, "y0": 1}, {"type": "center", "stability": "linearization inconclusive"})],
    questions=[{"q": "Classify the equilibrium of the nonlinear system dx/dt = y, dy/dt = -sin(x) - 0.5*y at x0 = 0, y0 = 0.",
                "outputs": {"type": "stable spiral"}},
               {"q": "Classify the equilibrium of the nonlinear system dx/dt = y, dy/dt = -sin(x) - 0.5*y at x0 = 1, y0 = 0.",
                "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"nonlinear system|linearization|jacobian", r"equilibri|classify|critical point")]),
    text={"f": _RHS_X, "g": _RHS_Y}, target=None)


# =================================================================================================================
# NUMERICAL STABILITY AND FINITE DIFFERENCES
# =================================================================================================================
_STAB_POLY = {"euler": [1, 1], "heun": [1, 1, 0.5], "midpoint": [1, 1, 0.5], "rk4": [1, 1, 0.5, 1 / 6, 1 / 24]}


def _R(method, z):
    if method in _STAB_POLY:
        return sum(c * z ** k for k, c in enumerate(_STAB_POLY[method]))
    if method == "backward euler":
        if z == 1:
            raise ValueError("h*lambda = 1 makes the implicit step singular")
        return 1 / (1 - z)
    if z == 2:
        raise ValueError("h*lambda = 2 makes the trapezoidal step singular")
    return (1 + z / 2) / (1 - z / 2)


def amplification(method, lam, h):
    _fin(lam, h)
    if h <= 0:
        raise ValueError("the step h must be positive")
    z = lam * h
    R = _R(method, z)
    return {"R": _r(R), "abs_R": _r(abs(R)), "stable": abs(R) <= 1, "z": _r(z)}


def _ref_amp(method, lam, h):
    from scipy.optimize import brentq
    if method in _TABLEAU:
        y1 = ref_rk(method, lambda t, y: lam * y, 0, 1, h, 1)["y_n"]
    elif method == "backward euler":
        y1 = brentq(lambda y: y - 1 - h * lam * y, -1e6, 1e6, xtol=1e-15, rtol=8.9e-16)
    else:
        y1 = brentq(lambda y: y - 1 - h / 2 * (lam + lam * y), -1e6, 1e6, xtol=1e-15, rtol=8.9e-16)
    return {"R": y1, "stable": abs(y1) <= 1}


_METHODS6 = ["euler", "heun", "midpoint", "rk4", "backward euler", "trapezoidal"]


def _rand_amp(rng):
    while True:
        m, lam, h = rng.choice(_METHODS6), _nz(rng, -40, 5, 1), _u(rng, 0.01, 0.6, 3)
        try:
            if abs(abs(_R(m, lam * h)) - 1) > 1e-6:
                return {"method": m, "lam": lam, "h": h}
        except ValueError:
            pass


add("amplification_factor", name="Stability function (amplification factor) of a one-step method", topic="numerical methods / stability",
    purpose="For y' = lambda y: the factor R(h lambda) a method multiplies by per step, its size, and whether the step is stable (|R| <= 1).",
    inputs=[{"symbol": "method", "type": "choice", "kind": "number", "choices": _METHODS6, "meaning": "the one-step method", "required": True},
            N("lam", "lambda (real)"), N("h", "step size (> 0)")],
    outputs=[O("R", "amplification factor R(h lambda)"), O("abs_R", "|R|"), O("stable", "|R| <= 1", "boolean"), O("z", "h lambda")],
    primary="R", conditions=["h > 0", "real lambda"], method="R(z): 1 + z (Euler), 1 + z + z^2/2 (Heun, midpoint), Taylor to z^4/24 (RK4), 1/(1 - z), (1 + z/2)/(1 - z/2)",
    latex=r"y_{k+1} = R(h\lambda)\,y_k", run=amplification, reference=_ref_amp,
    wrong=lambda method, lam, h: {"R": 1 - lam * h if method == "euler" else _R("euler", lam * h), "stable": abs(1 - lam * h) <= 1},
    rand=_rand_amp,
    templates=["Find the amplification factor of the rk4 method for lam = -10, h = 0.2.",
               "Amplification factor of the backward euler method with lam = -50, h = 0.1."],
    examples=[({"method": "rk4", "lam": -10, "h": 0.2}, {"R": 0.333333333333, "stable": True}),
              ({"method": "euler", "lam": -10, "h": 0.25}, {"R": -1.5, "stable": False})],
    questions=[{"q": "Find the amplification factor of the rk4 method for lam = -10, h = 0.2.", "outputs": {"R": 0.333333333333}},
               {"q": "Amplification factor of the backward euler method with lam = -50, h = 0.1.", "outputs": {"R": 1 / 6, "stable": True}},
               {"q": "Amplification factor of the euler method with lam = -10.", "outcome": "NEEDS_INPUT"}],
    trig=(5, [r"amplification factor|stability function"]),
    post=lambda nq, out: (lambda m: dict(out, method=m.group(1)) if m else out)(
        re.search(r"\b(backward euler|trapezoidal|euler|heun|midpoint|rk4)\b", nq.lower())), target=None)


def step_limit(method, lam):
    _fin(lam)
    if lam >= 0:
        raise ValueError("needs lambda < 0 (a decaying mode)")
    co = _STAB_POLY[method]
    g = lambda x: abs(sum(c * (-x) ** k for k, c in enumerate(co))) - 1
    lo, hi = 1.0, 4.0                                          # |R(-x)| crosses 1 once in (1, 4) for these methods
    for _ in range(200):
        mid = (lo + hi) / 2
        if g(mid) <= 0:
            lo = mid
        else:
            hi = mid
        if hi - lo <= 4e-16 * hi:
            break
    return {"h_max": _r(lo / abs(lam)), "z_boundary": _r(-lo)}


def _ref_step_limit(method, lam):
    from scipy.optimize import brentq
    x = brentq(lambda x: abs(ref_rk(method, lambda t, y: -x * y, 0, 1, 1, 1)["y_n"]) - 1, 1.5, 3.5, xtol=1e-15, rtol=8.9e-16)
    return {"h_max": x / abs(lam), "z_boundary": -x}


add("explicit_step_limit", name="Largest stable step of an explicit method for y' = lambda y", topic="numerical methods / stability",
    purpose="The absolute-stability step limit h_max of Euler, Heun, midpoint or RK4 on a decaying mode y' = lambda y (lambda < 0).",
    inputs=[{"symbol": "method", "type": "choice", "kind": "number", "choices": ["euler", "heun", "midpoint", "rk4"], "meaning": "explicit method",
             "required": True}, N("lam", "lambda (< 0)")],
    outputs=[O("h_max", "largest stable step"), O("z_boundary", "real stability boundary h lambda")], primary="h_max", conditions=["lambda < 0"],
    method="bisection for |R(-x)| = 1 on the stability polynomial", latex=r"|R(h\lambda)| \le 1:\; h \le \frac{x^*}{|\lambda|}\;(x^* = 2 \text{ Euler}, 2.785 \text{ RK4})",
    run=step_limit, reference=_ref_step_limit, wrong=lambda method, lam: {"h_max": 1 / abs(lam)},
    rand=lambda rng: {"method": rng.choice(["euler", "heun", "midpoint", "rk4"]), "lam": _u(rng, -100, -0.1, 2)},
    templates=["Largest stable step of the rk4 method for lam = -20."],
    examples=[({"method": "euler", "lam": -20}, {"h_max": 0.1}), ({"method": "rk4", "lam": -1}, {"h_max": 2.78529356340})],
    questions=[{"q": "Largest stable step of the rk4 method for lam = -20.", "outputs": {"h_max": 0.139264678170}},
               {"q": "Largest stable step of the euler method for lam = 3.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"(largest|maximum) stable step|step(size)? limit"]),
    post=lambda nq, out: (lambda m: dict(out, method=m.group(1)) if m else out)(re.search(r"\b(euler|heun|midpoint|rk4)\b", nq.lower())),
    target=None)


def _vn_max(G):
    """Build-time only: max |G(theta)| over a fine theta grid (von Neumann analysis)."""
    import numpy as np
    th = np.linspace(0, 2 * np.pi, 2001)
    return float(np.max(np.abs(G(th))))


def cfl(c, dt, dx):
    _fin(c, dt, dx)
    if c == 0 or dt <= 0 or dx <= 0:
        raise ValueError("needs c != 0, dt > 0 and dx > 0")
    C = abs(c) * dt / dx
    return {"courant": _r(C), "stable": C <= 1, "dt_max": _r(dx / abs(c))}


def _ref_cfl(c, dt, dx):
    import numpy as np
    G = lambda C: (lambda th: 1 - C * (1 - np.exp(-1j * th)))       # first-order upwind for u_t + c u_x = 0
    ok = lambda d: _vn_max(G(abs(c) * d / dx)) <= 1 + 1e-12
    lo, hi = 0.0, 10 * dx / abs(c)
    for _ in range(80):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if ok(mid) else (lo, mid)
    return {"stable": ok(dt), "dt_max": lo}


def _rand_cfl(rng):
    while True:
        c, dx, dt = _nz(rng, -5, 5), _u(rng, 0.01, 0.5, 3), _u(rng, 0.001, 0.3, 4)
        if abs(abs(c) * dt / dx - 1) > 1e-3:
            return {"c": c, "dt": dt, "dx": dx}


add("cfl_condition", name="CFL condition for the advection equation (upwind scheme)", topic="numerical methods / PDE stability",
    purpose="Courant number C = |c| dt/dx of the explicit upwind scheme for u_t + c u_x = 0, its stability (C <= 1) and the largest stable dt.",
    inputs=[N("c", "advection speed", condition="c != 0"), N("dt", "time step (> 0)"), N("dx", "grid spacing (> 0)")],
    outputs=[O("courant", "Courant number |c| dt / dx"), O("stable", "C <= 1", "boolean"), O("dt_max", "largest stable time step dx / |c|")],
    primary="courant", conditions=["c != 0", "dt, dx > 0"], method="Courant-Friedrichs-Lewy condition (von Neumann: |1 - C(1 - e^{-i theta})| <= 1)",
    latex=r"C = \frac{|c|\,\Delta t}{\Delta x} \le 1", run=cfl, reference=_ref_cfl,
    wrong=lambda c, dt, dx: {"courant": abs(c) * dt / dx ** 2, "stable": abs(c) * dt / dx ** 2 <= 1, "dt_max": dx * dx / abs(c)},
    rand=_rand_cfl,
    templates=["Check the CFL condition for c = 2, dt = 0.01, dx = 0.05."],
    examples=[({"c": 2, "dt": 0.01, "dx": 0.05}, {"courant": 0.4, "stable": True, "dt_max": 0.025})],
    questions=[{"q": "Check the CFL condition for c = 2, dt = 0.01, dx = 0.05.", "outputs": {"courant": 0.4, "stable": True}},
               {"q": "Check the CFL condition for c = 2, dt = 0.01.", "outcome": "NEEDS_INPUT"}],
    trig=(3, [r"\bcfl\b|courant"]), target=None)


def heat_stability(alpha, dt, dx):
    _fin(alpha, dt, dx)
    if alpha <= 0 or dt <= 0 or dx <= 0:
        raise ValueError("needs alpha, dt, dx > 0")
    r = alpha * dt / dx ** 2
    return {"r": _r(r), "stable": r <= 0.5, "dt_max": _r(dx * dx / (2 * alpha))}


def _ref_heat_stab(alpha, dt, dx):
    import numpy as np
    G = lambda r: (lambda th: 1 - 4 * r * np.sin(th / 2) ** 2)
    ok = lambda d: _vn_max(G(alpha * d / dx ** 2)) <= 1 + 1e-12
    lo, hi = 0.0, 10 * dx * dx / alpha
    for _ in range(80):
        mid = (lo + hi) / 2
        lo, hi = (mid, hi) if ok(mid) else (lo, mid)
    return {"stable": ok(dt), "dt_max": lo}


def _rand_heat(rng):
    while True:
        a, dx, dt = _u(rng, 0.01, 2, 3), _u(rng, 0.01, 0.5, 3), _u(rng, 0.0001, 0.05, 5)
        if abs(a * dt / dx ** 2 - 0.5) > 1e-3:
            return {"alpha": a, "dt": dt, "dx": dx}


add("explicit_heat_stability", name="Stability of the explicit (FTCS) scheme for the heat equation", topic="numerical methods / PDE stability",
    purpose="The mesh ratio r = alpha dt / dx^2 of the explicit scheme for u_t = alpha u_xx, its stability (r <= 1/2) and the largest stable dt.",
    inputs=[N("alpha", "diffusivity (> 0)"), N("dt", "time step (> 0)"), N("dx", "grid spacing (> 0)")],
    outputs=[O("r", "mesh ratio alpha dt / dx^2"), O("stable", "r <= 1/2", "boolean"), O("dt_max", "largest stable time step dx^2/(2 alpha)")],
    primary="r", conditions=["alpha, dt, dx > 0"], method="von Neumann: G = 1 - 4 r sin^2(theta/2), |G| <= 1 iff r <= 1/2",
    latex=r"r = \frac{\alpha\,\Delta t}{\Delta x^2} \le \frac{1}{2}", run=heat_stability, reference=_ref_heat_stab,
    wrong=lambda alpha, dt, dx: {"r": alpha * dt / dx, "stable": alpha * dt / dx <= 0.5, "dt_max": dx / (2 * alpha)},
    rand=_rand_heat,
    templates=["Is the explicit FTCS scheme for the heat equation stable with alpha = 1, dt = 0.001, dx = 0.05?"],
    examples=[({"alpha": 1, "dt": 0.001, "dx": 0.05}, {"r": 0.4, "stable": True, "dt_max": 0.00125})],
    questions=[{"q": "Is the explicit FTCS scheme for the heat equation stable with alpha = 1, dt = 0.001, dx = 0.05?", "outputs": {"stable": True}},
               {"q": "Is the explicit FTCS scheme for the heat equation stable with alpha = 1, dt = 0.002, dx = 0.05?", "outputs": {"stable": False}}],
    trig=(4, [_both(r"explicit|ftcs", r"heat|diffusion") + r"(?=.*\bstab)", r"(?=.*\bstab).*(" + _both(r"explicit|ftcs", r"heat|diffusion") + ")"]),
    target=None)


def ftcs(u, r, n):
    n = int(n)
    if len(u) < 3:
        raise ValueError("give at least 3 grid values")
    if r <= 0 or not 1 <= n <= 1000:
        raise ValueError("needs r > 0 and 1 <= n <= 1000")
    v = list(map(float, u))
    for _ in range(n):
        v = [v[0]] + [v[i] + r * (v[i + 1] - 2 * v[i] + v[i - 1]) for i in range(1, len(v) - 1)] + [v[-1]]
        if max(abs(x) for x in v) > 1e100:
            raise ValueError("the scheme blew up (unstable for r > 1/2)")
    return {"u": [_r(x) for x in v], "stable_ratio": r <= 0.5}


def _ref_ftcs(u, r, n):
    import numpy as np
    m = len(u)
    M = np.eye(m)
    for i in range(1, m - 1):
        M[i, i - 1], M[i, i], M[i, i + 1] = r, 1 - 2 * r, r
    return {"u": [float(x) for x in np.linalg.matrix_power(M, int(n)) @ np.array(u, dtype=float)]}


add("heat_ftcs_steps", name="Explicit finite-difference (FTCS) steps for the heat equation", topic="numerical methods / finite differences",
    purpose="Advance grid values of u_t = alpha u_xx by n explicit steps u_i <- u_i + r (u_{i+1} - 2 u_i + u_{i-1}), ends held fixed.",
    inputs=[{"symbol": "u", "type": "vector", "kind": "number", "meaning": "grid values (first and last are boundary values)", "required": True,
             "min_length": 3, "max_length": 200}, N("r", "mesh ratio alpha dt / dx^2 (> 0)"), _int_in("n", "number of time steps", 1, 1000)],
    outputs=[O("u", "grid values after n steps", "vector"), O("stable_ratio", "r <= 1/2", "boolean")], primary="u",
    conditions=["Dirichlet ends (held fixed)", "r > 0"], method="forward time, centred space", latex=r"u_i^{k+1} = u_i^k + r\left(u_{i+1}^k - 2u_i^k + u_{i-1}^k\right)",
    run=ftcs, reference=_ref_ftcs,
    wrong=lambda u, r, n: {"u": (lambda v: [v[0]] + [v[i] + r * (v[i + 1] + v[i - 1]) for i in range(1, len(v) - 1)] + [v[-1]])(list(u))},
    rand=lambda rng: {"u": [0.0] + [_u(rng, 0, 100) for _ in range(rng.randint(2, 8))] + [0.0], "r": _u(rng, 0.05, 0.5, 3), "n": rng.randint(1, 30)},
    templates=["Apply the explicit FTCS heat step to u = [0, 50, 100, 50, 0] with r = 0.25, n = 1."],
    examples=[({"u": "0, 50, 100, 50, 0", "r": 0.25, "n": 1}, {"u": [0.0, 50.0, 75.0, 50.0, 0.0]})],
    questions=[{"q": "Apply the explicit FTCS heat step to u = [0, 50, 100, 50, 0] with r = 0.25, n = 1.", "outputs": {"u": [0.0, 50.0, 75.0, 50.0, 0.0]}},
               {"q": "Apply the explicit FTCS heat step to u = [0, 50] with r = 0.25, n = 1.", "outcome": "INVALID_INPUT"}],
    trig=(5, [_both(r"ftcs|explicit", r"\bstep") + r"(?=.*\bu\s*=)"]), mats=("u",), target=None)


def crank_nicolson(u, r, n):
    n = int(n)
    if len(u) < 3:
        raise ValueError("give at least 3 grid values")
    if r <= 0 or not 1 <= n <= 1000:
        raise ValueError("needs r > 0 and 1 <= n <= 1000")
    v = list(map(float, u))
    m = len(v) - 2
    for _ in range(n):
        d = [r / 2 * v[i - 1] + (1 - r) * v[i] + r / 2 * v[i + 1] for i in range(1, m + 1)]
        d[0] += r / 2 * v[0]
        d[-1] += r / 2 * v[-1]
        a, b, c = -r / 2, 1 + r, -r / 2                      # Thomas algorithm for the tridiagonal system
        cp, dp = [0.0] * m, [0.0] * m
        cp[0], dp[0] = c / b, d[0] / b
        for i in range(1, m):
            den = b - a * cp[i - 1]
            cp[i], dp[i] = c / den, (d[i] - a * dp[i - 1]) / den
        x = [0.0] * m
        x[-1] = dp[-1]
        for i in range(m - 2, -1, -1):
            x[i] = dp[i] - cp[i] * x[i + 1]
        v = [v[0]] + x + [v[-1]]
    return {"u": [_r(x) for x in v]}


def _ref_cn(u, r, n):
    import numpy as np
    from scipy.linalg import solve_banded
    v = np.array(u, dtype=float)
    m = len(v) - 2
    ab = np.zeros((3, m))
    ab[0, 1:], ab[1, :], ab[2, :-1] = -r / 2, 1 + r, -r / 2
    for _ in range(int(n)):
        rhs = r / 2 * v[:-2] + (1 - r) * v[1:-1] + r / 2 * v[2:]
        rhs[0] += r / 2 * v[0]
        rhs[-1] += r / 2 * v[-1]
        v = np.concatenate([[v[0]], solve_banded((1, 1), ab, rhs), [v[-1]]])
    return {"u": [float(x) for x in v]}


add("crank_nicolson_steps", name="Crank-Nicolson steps for the heat equation", topic="numerical methods / finite differences",
    purpose="Advance grid values of u_t = alpha u_xx by n Crank-Nicolson steps (unconditionally stable), ends held fixed.",
    inputs=[{"symbol": "u", "type": "vector", "kind": "number", "meaning": "grid values (first and last are boundary values)", "required": True,
             "min_length": 3, "max_length": 200}, N("r", "mesh ratio alpha dt / dx^2 (> 0)"), _int_in("n", "number of time steps", 1, 1000)],
    outputs=[O("u", "grid values after n steps", "vector")], primary="u", conditions=["Dirichlet ends", "r > 0"],
    method="average of explicit and implicit steps; each step a tridiagonal solve (Thomas algorithm)",
    latex=r"-\tfrac{r}{2}u_{i-1}^{k+1} + (1 + r)u_i^{k+1} - \tfrac{r}{2}u_{i+1}^{k+1} = \tfrac{r}{2}u_{i-1}^k + (1 - r)u_i^k + \tfrac{r}{2}u_{i+1}^k",
    run=crank_nicolson, reference=_ref_cn, wrong=lambda u, r, n: ftcs(u, r, n),
    rand=lambda rng: {"u": [_u(rng, 0, 20)] + [_u(rng, 0, 100) for _ in range(rng.randint(2, 8))] + [_u(rng, 0, 20)], "r": _u(rng, 0.1, 3, 2),
                      "n": rng.randint(1, 20)},
    templates=["Apply Crank-Nicolson to u = [0, 50, 100, 50, 0] with r = 1, n = 1."],
    examples=[({"u": "0, 50, 100, 50, 0", "r": 1, "n": 1}, {"u": [0.0, 35.7142857143, 42.8571428571, 35.7142857143, 0.0]})],
    questions=[{"q": "Apply Crank-Nicolson to u = [0, 50, 100, 50, 0] with r = 1, n = 1.",
                "outputs": {"u": [0.0, 35.7142857143, 42.8571428571, 35.7142857143, 0.0]}},
               {"q": "Apply Crank-Nicolson to u = [0, 50, 100, 50, 0] with r = 1.", "outcome": "NEEDS_INPUT"}],
    trig=(5, [r"crank[- ]nicolson"]), mats=("u",), target=None)

# =================================================================================================================
# CLASSIC PDEs BY SEPARATION OF VARIABLES
# =================================================================================================================
def _poly_x(text, max_deg=6):
    """A polynomial f(x) with rational coefficients: ascending coefficient list (floats)."""
    e = parse_text(text, ("x",))
    try:
        P = sp.Poly(sp.expand(e), XS)
    except sp.PolynomialError:
        raise ValueError("this capability needs f(x) to be a polynomial in x") from None
    if not all(c.is_rational for c in P.all_coeffs()):
        raise ValueError("the polynomial needs rational coefficients")
    if P.degree() > max_deg:
        raise ValueError(f"polynomials up to degree {max_deg} are supported")
    return [float(c) for c in reversed(P.all_coeffs())]


def _sine_coeff(co, L, n):
    """b_n = (2/L) * integral_0^L f(x) sin(n pi x / L) dx for a polynomial f (exact integration by parts)."""
    w = n * math.pi / L
    sgn = -1.0 if n % 2 else 1.0                            # cos(n pi)
    S, C = [(1 - sgn) / w], [0.0]
    for k in range(1, len(co)):
        S.append(-L ** k * sgn / w + k / w * C[k - 1])
        C.append(-k / w * S[k - 1])
    terms = [a * s for a, s in zip(co, S)]
    return 2 / L * _guard(sum(terms), *terms) if any(terms) else 0.0


def _ref_sine_coeff(f, L, n):
    from scipy.integrate import quad
    co = _poly_x(f)
    v, _ = quad(lambda x: sum(c * x ** k for k, c in enumerate(co)), 0, L, weight="sin", wvar=n * math.pi / L,
                epsabs=1e-15, epsrel=1e-14, limit=500)
    return 2 / L * v


_FX = {"symbol": "f", "type": "text", "kind": "number", "meaning": "initial profile f(x), a polynomial", "required": True, "max_length": 200}


def sine_coefficient(f, L, n):
    _fin(L)
    n = int(n)
    if L <= 0 or n < 1:
        raise ValueError("needs L > 0 and n >= 1")
    return {"b_n": _r(_sine_coeff(_poly_x(f), L, n))}


_RHS_FX = r"(?:f\(x\)|u\(x,\s*0\))\s*=\s*(.+?)" + _STOP

add("fourier_sine_coefficient", name="Fourier sine series coefficient (half-range expansion)", topic="classic PDEs / Fourier series",
    purpose="The coefficient b_n of the sine series of f(x) on [0, L], used for the heat and wave equations with fixed ends.",
    inputs=[_FX, N("L", "length of the interval (> 0)"), _int_in("n", "index n >= 1", 1, 10000)],
    outputs=[O("b_n", "sine coefficient b_n")], primary="b_n", conditions=["f a polynomial", "L > 0"],
    method="exact integration by parts of x^k sin(n pi x / L)", latex=r"b_n = \frac{2}{L}\int_0^L f(x)\sin\frac{n\pi x}{L}\,dx",
    run=sine_coefficient, reference=lambda f, L, n: {"b_n": _ref_sine_coeff(f, L, n)},
    wrong=lambda f, L, n: {"b_n": _sine_coeff(_poly_x(f), L, n) / 2},
    rand=lambda rng: {"f": rng.choice(["x*(1 - x)", "1", "x", "x^2", f"x*({rng.randint(2, 5)} - x)", "x^3 - x", "3 - x", "x^2*(2 - x)"]),
                      "L": float(rng.choice([1, 2, 3, math.pi, 0.5, 4])), "n": rng.randint(1, 15)},
    templates=["Find the Fourier sine coefficient of f(x) = x*(1 - x) with L = 1, n = 1."],
    examples=[({"f": "x*(1 - x)", "L": 1, "n": 1}, {"b_n": 0.258012275465}), ({"f": "1", "L": 1, "n": 2}, {"b_n": 0.0})],
    questions=[{"q": "Find the Fourier sine coefficient of f(x) = x*(1 - x) with L = 1, n = 1.", "outputs": {"b_n": 0.258012275465}},
               {"q": "Find the Fourier sine coefficient of f(x) = sin(x) with L = 1, n = 1.", "outcome": "INVALID_INPUT"}],
    trig=(3, [r"fourier sine (series )?coefficient|half[- ]range"]), text={"f": _RHS_FX}, target=None)


def heat_series(f, L, alpha, N, x, t):
    _fin(L, alpha, x, t)
    N = int(N)
    if L <= 0 or alpha <= 0 or t < 0 or not 0 <= x <= L or not 1 <= N <= 500:
        raise ValueError("needs L > 0, alpha > 0, t >= 0, 0 <= x <= L and 1 <= N <= 500")
    co = _poly_x(f)
    terms = [_sine_coeff(co, L, n) * math.exp(-alpha * (n * math.pi / L) ** 2 * t) * math.sin(n * math.pi * x / L) for n in range(1, N + 1)]
    return {"u": _r(_guard(math.fsum(terms), *terms)), "b_1": _r(_sine_coeff(co, L, 1))}


def _ref_heat_series(f, L, alpha, N, x, t):
    import numpy as np
    n = np.arange(1, int(N) + 1)
    b = np.array([_ref_sine_coeff(f, L, k) for k in n])
    return {"u": float(np.sum(b * np.exp(-alpha * (n * np.pi / L) ** 2 * t) * np.sin(n * np.pi * x / L)))}


def _rand_pde(rng):
    L = float(rng.choice([1, 2, 3, 0.5]))
    return {"f": rng.choice(["x*(L - x)".replace("L", repr(L)), "1", f"{rng.randint(10, 100)}", "x", f"x^2*({L} - x)"]), "L": L,
            "N": rng.randint(1, 60), "x": round(L * rng.uniform(0.05, 0.95), 3)}


add("heat_equation_series", name="Heat equation on a rod with ends at zero: Fourier series value", topic="classic PDEs / heat equation",
    purpose="u(x, t) for u_t = alpha u_xx, u(0, t) = u(L, t) = 0, u(x, 0) = f(x) (a polynomial), summed over the first N series terms.",
    inputs=[_FX, N("L", "rod length (> 0)"), N("alpha", "diffusivity (> 0)"), _int_in("N", "number of series terms (1 to 500)", 1, 500),
            N("x", "position (0 <= x <= L)"), N("t", "time (>= 0)")],
    outputs=[O("u", "temperature u(x, t) from N terms"), O("b_1", "first sine coefficient")], primary="u",
    conditions=["f a polynomial", "Dirichlet ends at 0", "series truncated after N terms (stated)"],
    method="separation of variables: u = sum b_n e^{-alpha (n pi/L)^2 t} sin(n pi x/L)",
    latex=r"u(x, t) = \sum_{n=1}^{N} b_n e^{-\alpha (n\pi/L)^2 t}\sin\frac{n\pi x}{L}", run=heat_series, reference=_ref_heat_series,
    wrong=lambda f, L, alpha, N, x, t: {"u": sum(_sine_coeff(_poly_x(f), L, n) * math.exp(-alpha * (n * math.pi / L) * t) * math.sin(n * math.pi * x / L)
                                             for n in range(1, int(N) + 1))},
    rand=lambda rng: dict(_rand_pde(rng), alpha=_u(rng, 0.01, 1, 3), t=_u(rng, 0.005, 0.5, 3)),
    templates=["Heat equation series with f(x) = x*(1 - x), L = 1, alpha = 1, N = 10, x = 0.5, t = 0.1."],
    examples=[({"f": "x*(1 - x)", "L": 1, "alpha": 1, "N": 10, "x": 0.5, "t": 0.1}, {"u": 0.0961618714343, "b_1": 0.258012275465})],
    questions=[{"q": "Heat equation series with f(x) = x*(1 - x), L = 1, alpha = 1, N = 10, x = 0.5, t = 0.1.", "outputs": {"b_1": 0.258012275465}},
               {"q": "Heat equation series with f(x) = x*(1 - x), L = 1, alpha = 1, N = 10, x = 2, t = 0.1.", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"heat equation|heat conduction|diffusion equation", r"series|fourier|separation")]), unless=[r"stab|ftcs|explicit|crank"],
    text={"f": _RHS_FX}, target=None)


def wave_series(f, L, c, N, x, t, g="0"):
    _fin(L, c, x, t)
    N = int(N)
    if L <= 0 or c <= 0 or not 0 <= x <= L or not 1 <= N <= 500:
        raise ValueError("needs L > 0, c > 0, 0 <= x <= L and 1 <= N <= 500")
    cf, cg = _poly_x(f), _poly_x(g)
    terms = []
    for n in range(1, N + 1):
        w = n * math.pi / L
        terms.append((_sine_coeff(cf, L, n) * math.cos(w * c * t) + _sine_coeff(cg, L, n) / (w * c) * math.sin(w * c * t)) * math.sin(w * x))
    return {"u": _r(_guard(math.fsum(terms), *terms))}


def _ref_wave_series(f, L, c, N, x, t, g="0"):
    import numpy as np
    n = np.arange(1, int(N) + 1)
    w = n * np.pi / L
    b = np.array([_ref_sine_coeff(f, L, k) for k in n])
    B = np.array([_ref_sine_coeff(g, L, k) for k in n])
    return {"u": float(np.sum((b * np.cos(w * c * t) + B / (w * c) * np.sin(w * c * t)) * np.sin(w * x)))}


add("wave_equation_string_series", name="Vibrating string with fixed ends: Fourier series value", topic="classic PDEs / wave equation",
    purpose="u(x, t) for u_tt = c^2 u_xx, u(0, t) = u(L, t) = 0, u(x, 0) = f(x), u_t(x, 0) = g(x) (polynomials), from N series terms.",
    inputs=[_FX, N("L", "string length (> 0)"), N("c", "wave speed (> 0)"), _int_in("N", "number of series terms", 1, 500),
            N("x", "position (0 <= x <= L)"), N("t", "time"),
            {"symbol": "g", "type": "text", "kind": "number", "meaning": "initial velocity g(x), a polynomial (default 0)", "required": False,
             "max_length": 200}],
    outputs=[O("u", "displacement u(x, t) from N terms")], primary="u", conditions=["polynomial f, g", "fixed ends", "N terms (stated)"],
    method="u = sum [b_n cos(n pi c t/L) + B_n L/(n pi c) sin(n pi c t/L)] sin(n pi x/L)",
    latex=r"u = \sum_{n=1}^{N}\left[b_n\cos\frac{n\pi c t}{L} + \frac{B_n L}{n\pi c}\sin\frac{n\pi c t}{L}\right]\sin\frac{n\pi x}{L}",
    run=wave_series, reference=_ref_wave_series,
    wrong=lambda f, L, c, N, x, t, g="0": {"u": sum(_sine_coeff(_poly_x(f), L, n) * math.cos(n * math.pi * t / L) * math.sin(n * math.pi * x / L)
                                                 for n in range(1, int(N) + 1)) + 0.01},
    rand=lambda rng: dict(_rand_pde(rng), c=_u(rng, 0.5, 3), t=_u(rng, 0.05, 2), g=rng.choice(["0", "1", "x"])),
    templates=["Vibrating string with f(x) = x*(1 - x), L = 1, c = 2, N = 20, x = 0.25, t = 0.3."],
    examples=[({"f": "x*(1 - x)", "L": 1, "c": 2, "N": 20, "x": 0.25, "t": 0.3}, {"u": -0.0499928996262})],
    questions=[{"q": "Vibrating string with f(x) = x*(1 - x), L = 1, c = 2, N = 20, x = 0.25, t = 0.3.", "outputs": {"u": -0.0499928996262}},
               {"q": "Vibrating string with f(x) = x*(1 - x), L = 1, c = 2, N = 20, x = 0.25.", "outcome": "NEEDS_INPUT"}],
    trig=(4, [r"vibrating string|string with fixed ends|plucked string", _both(r"wave equation", r"series|fourier|fixed ends")]),
    text={"f": _RHS_FX, "g": r"(?:g\(x\)|u_t\(x,\s*0\))\s*=\s*(.+?)" + _STOP}, target=None)


def dalembert(f, g, c, x, t):
    import mpmath
    _fin(c, x, t)
    if c <= 0:
        raise ValueError("the wave speed c must be positive")
    F, G = _fn(f, ("x",)), _fn(g, ("x",))
    a, b = x - c * t, x + c * t
    if a == b:
        integ, err = 0.0, 0.0
    else:
        with mpmath.workdps(20):
            v, err = mpmath.quad(lambda s: G(float(s)), [a, (a + b) / 2, b], error=True)
        integ = float(v)
        if err > 1e-13 * max(1.0, abs(integ)):
            raise ValueError("the velocity integral did not converge")
    p1, p2 = F(a) / 2, F(b) / 2
    p3 = integ / (2 * c)
    return {"u": _r(_guard(p1 + p2 + p3, p1, p2, p3))}


def _ref_dalembert(f, g, c, x, t):
    """Build-time only: the Taylor series of u in t at t = 0, from u_tt = c^2 u_xx (Cauchy-Kovalevskaya), 40 digits."""
    import mpmath
    ef, eg = parse_text(f, ("x",)), parse_text(g, ("x",))
    with mpmath.workdps(40):
        tot, fac, k = mpmath.mpf(0), mpmath.mpf(1), 0
        X = sp.Rational(repr(float(x)))
        df, dg = ef, eg
        T, C = mpmath.mpf(t), mpmath.mpf(c)
        small = 0
        for m in range(0, 120):
            tf = mpmath.mpf(sp.N(df.subs(XS, X), 45)) * C ** (2 * m) * T ** (2 * m) / mpmath.factorial(2 * m)
            tg = mpmath.mpf(sp.N(dg.subs(XS, X), 45)) * C ** (2 * m) * T ** (2 * m + 1) / mpmath.factorial(2 * m + 1)
            tot += tf + tg
            small = small + 1 if abs(tf) + abs(tg) < mpmath.mpf(10) ** -30 * max(1, abs(tot)) else 0
            if small >= 3:
                break
            df, dg = sp.diff(df, XS, 2), sp.diff(dg, XS, 2)
        return {"u": float(tot)}


_DAL_F = ["x^2", "sin(x)", "cos(2*x)", "x^3 - x", "exp(-x)", "1 + x", "sin(3*x)", "x^4"]
_DAL_G = ["0", "1", "x", "cos(x)", "sin(2*x)", "x^2", "exp(x)"]

add("dalembert_solution", name="Wave equation on the whole line: d'Alembert's formula", topic="classic PDEs / wave equation",
    purpose="u(x, t) for u_tt = c^2 u_xx on the real line with u(x, 0) = f(x), u_t(x, 0) = g(x).",
    inputs=[{"symbol": "f", "type": "text", "kind": "number", "meaning": "initial displacement f(x)", "required": True, "max_length": 200},
            {"symbol": "g", "type": "text", "kind": "number", "meaning": "initial velocity g(x)", "required": True, "max_length": 200},
            N("c", "wave speed (> 0)"), N("x", "position"), N("t", "time")],
    outputs=[O("u", "displacement u(x, t)")], primary="u", conditions=["c > 0", "g integrable"],
    method="u = [f(x - ct) + f(x + ct)]/2 + (1/2c) * integral of g over [x - ct, x + ct] (tanh-sinh quadrature)",
    latex=r"u(x,t) = \frac{f(x - ct) + f(x + ct)}{2} + \frac{1}{2c}\int_{x - ct}^{x + ct} g(s)\,ds", run=dalembert, reference=_ref_dalembert,
    wrong=lambda f, g, c, x, t: {"u": _fn(f, ("x",))(x - c * t) + _fn(f, ("x",))(x + c * t)},
    rand=lambda rng: {"f": rng.choice(_DAL_F), "g": rng.choice(_DAL_G), "c": _u(rng, 0.3, 2), "x": _u(rng, -2, 2), "t": _u(rng, 0.05, 1.5)},
    templates=["Use d'Alembert's formula with f(x) = sin(x), g(x) = 0, c = 2, x = 1, t = 0.5."],
    examples=[({"f": "sin(x)", "g": "0", "c": 2, "x": 1, "t": 0.5}, {"u": 0.454648713413}),
              ({"f": "0", "g": "1", "c": 2, "x": 1, "t": 0.5}, {"u": 0.5})],
    questions=[{"q": "Use d'Alembert's formula with f(x) = sin(x), g(x) = 0, c = 2, x = 1, t = 0.5.", "outputs": {"u": 0.454648713413}},
               {"q": "Use d'Alembert's formula with f(x) = sin(x), g(x) = 0, c = -2, x = 1, t = 0.5.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"d'?\s?alembert", r"infinite string"]), text={"f": r"(?:f\(x\)|u\(x,\s*0\))\s*=\s*(.+?)" + _STOP,
                                                          "g": r"(?:g\(x\)|u_t\(x,\s*0\))\s*=\s*(.+?)" + _STOP}, target=None)


def laplace_rect(f, a, b, N, x, y):
    _fin(a, b, x, y)
    N = int(N)
    if a <= 0 or b <= 0 or not 0 <= x <= a or not 0 <= y <= b or not 1 <= N <= 500:
        raise ValueError("needs a, b > 0, (x, y) in the rectangle and 1 <= N <= 500")
    co = _poly_x(f)
    terms = []
    for n in range(1, N + 1):
        k = n * math.pi / a
        if k * b > 20:
            ratio = math.exp(k * (y - b)) * (1 - math.exp(-2 * k * y)) / (1 - math.exp(-2 * k * b))
        else:
            ratio = math.sinh(k * y) / math.sinh(k * b)
        terms.append(_sine_coeff(co, a, n) * ratio * math.sin(k * x))
    return {"u": _r(_guard(math.fsum(terms), *terms) if any(terms) else 0.0)}


def _ref_laplace_rect(f, a, b, N, x, y):
    import mpmath
    with mpmath.workdps(30):
        tot = mpmath.mpf(0)
        for n in range(1, int(N) + 1):
            k = n * mpmath.pi / a
            tot += _ref_sine_coeff(f, a, n) * mpmath.sinh(k * y) / mpmath.sinh(k * b) * mpmath.sin(k * x)
        return {"u": float(tot)}


add("laplace_equation_rectangle", name="Laplace's equation on a rectangle: series value", topic="classic PDEs / Laplace equation",
    purpose="Steady state u_xx + u_yy = 0 on [0, a] x [0, b] with u = f(x) on the top edge y = b and 0 on the other three edges, from N terms.",
    inputs=[{"symbol": "f", "type": "text", "kind": "number", "meaning": "top-edge values f(x), a polynomial", "required": True, "max_length": 200},
            N("a", "width (> 0)"), N("b", "height (> 0)"), _int_in("N", "number of series terms", 1, 500), N("x", "x in [0, a]"), N("y", "y in [0, b]")],
    outputs=[O("u", "u(x, y) from N terms")], primary="u", conditions=["polynomial f", "zero on three edges", "N terms (stated)"],
    method="u = sum b_n sinh(n pi y/a)/sinh(n pi b/a) sin(n pi x/a)",
    latex=r"u(x,y) = \sum_{n=1}^{N} b_n\frac{\sinh(n\pi y/a)}{\sinh(n\pi b/a)}\sin\frac{n\pi x}{a}", run=laplace_rect, reference=_ref_laplace_rect,
    wrong=lambda f, a, b, N, x, y: {"u": sum(_sine_coeff(_poly_x(f), a, n) * (y / b) * math.sin(n * math.pi * x / a) for n in range(1, int(N) + 1))},
    rand=lambda rng: (lambda a, b: {"f": rng.choice(["1", f"{rng.randint(10, 100)}", "x", f"x*({a} - x)"]), "a": a, "b": b,
                                    "N": rng.randint(1, 60), "x": round(a * rng.uniform(0.05, 0.95), 3), "y": round(b * rng.uniform(0.05, 0.95), 3)})(
        float(rng.choice([1, 2, 0.5, 3])), float(rng.choice([1, 2, 0.5, 3]))),
    templates=["Solve Laplace's equation on a rectangle with top edge f(x) = 100, a = 1, b = 1, N = 50, x = 0.5, y = 0.5."],
    examples=[({"f": "100", "a": 1, "b": 1, "N": 50, "x": 0.5, "y": 0.5}, {"u": 25.0})],
    questions=[{"q": "Solve Laplace's equation on a rectangle with top edge f(x) = 100, a = 1, b = 1, N = 50, x = 0.5, y = 0.5.", "outputs": {"u": 25.0}},
               {"q": "Solve Laplace's equation on a rectangle with top edge f(x) = 100, a = 1, b = 1, N = 50, x = 0.5, y = 2.", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"laplace'?s? equation", r"rectangle|square|plate")]), text={"f": r"f\(x\)\s*=\s*(.+?)" + _STOP}, target=None)


_BC = ["dirichlet-dirichlet", "dirichlet-neumann", "neumann-dirichlet", "neumann-neumann"]


def sturm_liouville(bc, L, n):
    _fin(L)
    n = int(n)
    if L <= 0 or n < 1:
        raise ValueError("needs L > 0 and n >= 1")
    k = (n - 0.5) * math.pi / L if bc in ("dirichlet-neumann", "neumann-dirichlet") else n * math.pi / L
    fn = {"dirichlet-dirichlet": "sin", "dirichlet-neumann": "sin", "neumann-dirichlet": "cos", "neumann-neumann": "cos"}[bc]
    return {"lambda_n": _r(k * k), "sqrt_lambda": _r(k), "eigenfunction": f"{fn}({_r(k)!r} x)"}


def _shoot(lam, L, start, end):
    import mpmath
    y0 = [0, 1] if start == "dirichlet" else [1, 0]
    sol = mpmath.odefun(lambda x, u: [u[1], -lam * u[0]], 0, [mpmath.mpf(v) for v in y0])
    v = sol(L)
    return v[0] if end == "dirichlet" else v[1]


def _ref_sl(bc, L, n):
    from scipy.optimize import brentq
    start, end = bc.split("-")
    y0 = [0.0, 1.0] if start == "dirichlet" else [1.0, 0.0]
    shoot = lambda l: ivp_ref(lambda x, u: [u[1], -l * u[0]], 0, y0, L)[0 if end == "dirichlet" else 1]
    g = sturm_liouville(bc, L, n)["lambda_n"]
    return {"lambda_n": brentq(shoot, g * 0.85, g * 1.15, xtol=1e-14, rtol=8.9e-16)}


add("sturm_liouville_eigenvalues", name="Eigenvalues of y'' + lambda y = 0 with Dirichlet/Neumann ends", topic="classic PDEs / eigenvalue problems",
    purpose="The n-th eigenvalue and eigenfunction of y'' + lambda y = 0 on [0, L] with Dirichlet (y = 0) or Neumann (y' = 0) conditions at each end.",
    inputs=[{"symbol": "bc", "type": "choice", "kind": "number", "choices": _BC, "meaning": "boundary conditions at x = 0 and x = L", "required": True},
            N("L", "interval length (> 0)"), _int_in("n", "index n >= 1 (Neumann-Neumann also has lambda_0 = 0)", 1, 1000)],
    outputs=[O("lambda_n", "n-th eigenvalue"), O("sqrt_lambda", "sqrt(lambda_n)"), O("eigenfunction", "eigenfunction", "text")],
    primary="lambda_n", conditions=["L > 0", "n >= 1"], method="sin/cos solutions; the boundary conditions quantise sqrt(lambda)",
    latex=r"\lambda_n = \left(\frac{n\pi}{L}\right)^2 \text{ (DD, NN)},\quad \left(\frac{(n - \frac12)\pi}{L}\right)^2 \text{ (mixed)}",
    run=sturm_liouville, reference=_ref_sl,
    wrong=lambda bc, L, n: {"lambda_n": (int(n) * math.pi / L) ** 2 if bc in ("dirichlet-neumann", "neumann-dirichlet") else (int(n) * math.pi / L)},
    rand=lambda rng: {"bc": rng.choice(_BC), "L": _u(rng, 0.5, 4), "n": rng.randint(1, 5)},
    templates=["Find the eigenvalue of y'' + lambda y = 0 with dirichlet-dirichlet conditions, L = 1, n = 2."],
    examples=[({"bc": "dirichlet-dirichlet", "L": 1, "n": 2}, {"lambda_n": 39.4784176044}),
              ({"bc": "dirichlet-neumann", "L": 1, "n": 1}, {"lambda_n": 2.46740110027})],
    questions=[{"q": "Find the eigenvalue of y'' + lambda y = 0 with dirichlet-dirichlet conditions, L = 1, n = 2.", "outputs": {"lambda_n": 39.4784176044}},
               {"q": "Find the eigenvalue of y'' + lambda y = 0 with dirichlet-dirichlet conditions, L = -1, n = 2.", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"eigenvalues?", r"y''\s*\+\s*(lambda|λ)\s*\*?\s*y|sturm")]), unless=[r"robin"],
    pre=lambda nq: (re.sub(r"y''\s*\+\s*(lambda|λ)\s*\*?\s*y\s*=\s*0", " ; ", nq), {}),
    post=lambda nq, out: (lambda m: dict(out, bc=m.group(1)) if m else out)(re.search(r"\b((?:dirichlet|neumann)-(?:dirichlet|neumann))\b", nq.lower())),
    target=None)


def robin_eigen(L, h, n):
    _fin(L, h)
    n = int(n)
    if L <= 0 or h <= 0 or n < 1:
        raise ValueError("needs L > 0, h > 0 and n >= 1")
    F = lambda mu: mu * math.cos(mu * L) + h * math.sin(mu * L)
    lo, hi = (n - 0.5) * math.pi / L, n * math.pi / L
    flo = F(lo)
    for _ in range(200):
        mid = (lo + hi) / 2
        if (F(mid) > 0) == (flo > 0):
            lo, flo = mid, F(mid)
        else:
            hi = mid
        if hi - lo <= 4e-16 * hi:
            break
    mu = (lo + hi) / 2
    return {"lambda_n": _r(mu * mu), "sqrt_lambda": _r(mu)}


def _ref_robin(L, h, n):
    from scipy.optimize import brentq
    G = lambda lam: (lambda v: v[1] + h * v[0])(ivp_ref(lambda x, u: [u[1], -lam * u[0]], 0, [0.0, 1.0], L))
    a, b = ((n - 0.5) * math.pi / L) ** 2, (n * math.pi / L) ** 2
    return {"lambda_n": brentq(G, a * (1 + 1e-9), b * (1 - 1e-9), xtol=1e-14, rtol=8.9e-16)}


add("robin_eigenvalues", name="Eigenvalues with a Robin (convective) end condition", topic="classic PDEs / eigenvalue problems",
    purpose="The n-th eigenvalue of y'' + lambda y = 0, y(0) = 0, y'(L) + h y(L) = 0 (heat loss at one end): root of tan(mu L) = -mu/h.",
    inputs=[N("L", "interval length (> 0)"), N("h", "Robin coefficient (> 0)"), _int_in("n", "index n >= 1", 1, 1000)],
    outputs=[O("lambda_n", "n-th eigenvalue mu_n^2"), O("sqrt_lambda", "mu_n")], primary="lambda_n", conditions=["L > 0", "h > 0"],
    method="bisection for mu cos(mu L) + h sin(mu L) = 0 in ((n - 1/2) pi/L, n pi/L)", latex=r"\tan(\mu L) = -\frac{\mu}{h},\quad \lambda = \mu^2",
    run=robin_eigen, reference=_ref_robin, wrong=lambda L, h, n: {"lambda_n": ((int(n) - 0.5) * math.pi / L) ** 2},
    rand=lambda rng: {"L": _u(rng, 0.5, 3), "h": _u(rng, 0.1, 10), "n": rng.randint(1, 4)},
    templates=["Find the Robin eigenvalue with L = 1, h = 1, n = 1."],
    examples=[({"L": 1, "h": 1, "n": 1}, {"lambda_n": 4.11585836569, "sqrt_lambda": 2.02875783811})],
    questions=[{"q": "Find the Robin eigenvalue with L = 1, h = 1, n = 1.", "outputs": {"sqrt_lambda": 2.02875783811}},
               {"q": "Find the Robin eigenvalue with L = 1, h = -1, n = 1.", "outcome": "INVALID_INPUT"}],
    trig=(5, [r"robin"]), target=None)


# =================================================================================================================
# DYNAMICAL SYSTEMS
# =================================================================================================================
def _stab_word(m):
    return "stable" if abs(m) < 1 else ("unstable" if abs(m) > 1 else "neutral")


def lm_fixed(r):
    _fin(r)
    if not 0 < r <= 4:
        raise ValueError("the logistic map x -> r x (1 - x) needs 0 < r <= 4")
    pts, mult = [0.0], [r]
    if r > 1:
        pts.append(1 - 1 / r)
        mult.append(2 - r)
    return {"fixed_points": [_r(p) for p in pts], "multipliers": [_r(m) for m in mult], "stability": [_stab_word(m) for m in mult],
            "count": len(pts)}


def _ref_lm_fixed(r):
    import numpy as np
    roots = sorted(float(z.real) for z in np.roots([-r, r - 1, 0]) if abs(z.imag) < 1e-12 and -1e-12 <= z.real <= 1)
    uniq = []
    for v in roots:
        if not uniq or abs(v - uniq[-1]) > 1e-12:
            uniq.append(max(v, 0.0) if abs(v) < 1e-15 else v)
    d = np.polyder(np.poly1d([-r, r, 0]))
    return {"fixed_points": uniq, "multipliers": [float(d(v)) for v in uniq], "count": len(uniq)}


add("logistic_map_fixed_points", name="Logistic map: fixed points and their stability", topic="dynamical systems / logistic map",
    purpose="Fixed points of x -> r x (1 - x) in [0, 1], their multipliers f'(x*) and stability.",
    inputs=[N("r", "parameter (0 < r <= 4)")],
    outputs=[O("fixed_points", "fixed points in [0, 1]", "vector"), O("multipliers", "f'(x*) at each", "vector"),
             O("stability", "stable / unstable / neutral", "vector"), O("count", "number of fixed points")], primary="count",
    conditions=["0 < r <= 4"], method="x* = 0 and 1 - 1/r; multipliers r and 2 - r", latex=r"x^* = 1 - \frac{1}{r},\quad f'(x^*) = 2 - r",
    run=lm_fixed, reference=_ref_lm_fixed,
    wrong=lambda r: {"fixed_points": [0.0] + ([1 / r] if r > 1 else []), "multipliers": [r] + ([r - 2] if r > 1 else [])},
    rand=lambda rng: {"r": _u(rng, 0.1, 4)},
    templates=["Find the fixed points of the logistic map with r = 2.5."],
    examples=[({"r": 2.5}, {"fixed_points": [0.0, 0.6], "stability": ["unstable", "stable"]}), ({"r": 3.2}, {"fixed_points": [0.0, 0.6875], "stability": ["unstable", "unstable"]})],
    questions=[{"q": "Find the fixed points of the logistic map with r = 2.5.", "outputs": {"fixed_points": [0.0, 0.6]}},
               {"q": "Find the fixed points of the logistic map with r = 5.", "outcome": "INVALID_INPUT"}],
    trig=(4, [_both(r"logistic map", r"fixed points?")]), target=None)


def lm_cycle(r):
    _fin(r)
    if not 3 < r <= 4:
        raise ValueError("a 2-cycle exists for 3 < r <= 4")
    s = math.sqrt((r - 3) * (r + 1))
    p, q = (r + 1 - s) / (2 * r), (r + 1 + s) / (2 * r)
    m = 4 + 2 * r - r * r
    return {"p": _r(p), "q": _r(q), "multiplier": _r(m), "stability": _stab_word(m)}


def _ref_lm_cycle(r):
    import numpy as np
    f = np.poly1d([-r, r, 0])
    ff = np.polyval(np.poly1d([-r, r, 0]), f) - np.poly1d([1, 0])
    rs = sorted(float(z.real) for z in ff.roots if abs(z.imag) < 1e-9)
    fixed = [0.0, 1 - 1 / r]
    cyc = sorted(v for v in rs if min(abs(v - x) for x in fixed) > 1e-7)
    d = np.polyder(f)
    return {"p": cyc[0], "q": cyc[1], "multiplier": float(d(cyc[0]) * d(cyc[1]))}


add("logistic_map_two_cycle", name="Logistic map: period-2 cycle and its stability", topic="dynamical systems / logistic map",
    purpose="The 2-cycle {p, q} of x -> r x (1 - x) (3 < r <= 4), its multiplier f'(p) f'(q) and stability (stable for 3 < r < 1 + sqrt 6).",
    inputs=[N("r", "parameter (3 < r <= 4)")],
    outputs=[O("p", "lower cycle point"), O("q", "upper cycle point"), O("multiplier", "f'(p) f'(q) = 4 + 2r - r^2"), O("stability", "stability", "text")],
    primary="p", conditions=["3 < r <= 4"], method="roots of f(f(x)) = x other than the fixed points",
    latex=r"p, q = \frac{r + 1 \mp \sqrt{(r - 3)(r + 1)}}{2r},\quad f'(p)f'(q) = 4 + 2r - r^2", run=lm_cycle, reference=_ref_lm_cycle,
    wrong=lambda r: {"p": (r + 1 - math.sqrt((r - 3) * (r + 1))) / r / 2.2, "q": (r + 1 + math.sqrt((r - 3) * (r + 1))) / (2 * r), "multiplier": r * r - 2 * r - 4},
    rand=lambda rng: {"r": _u(rng, 3.02, 4, 3)},
    templates=["Find the period-2 cycle of the logistic map with r = 3.2."],
    examples=[({"r": 3.2}, {"p": 0.513044509089, "q": 0.799455490911, "multiplier": 0.16, "stability": "stable"})],
    questions=[{"q": "Find the period-2 cycle of the logistic map with r = 3.2.", "outputs": {"p": 0.513044509089}},
               {"q": "Find the period-2 cycle of the logistic map with r = 2.5.", "outcome": "INVALID_INPUT"}],
    trig=(5, [_both(r"logistic map", r"2-cycle|period[- ](2|two)|two[- ]cycle")]), target=None)


def lm_lyapunov(r, x0, n):
    _fin(r, x0)
    n = int(n)
    if not 0 < r <= 4 or not 0 < x0 < 1:
        raise ValueError("needs 0 < r <= 4 and 0 < x0 < 1")
    if n < 2 or n > 1000000 or n % 2:
        raise ValueError("n must be an even number of iterations between 2 and 1000000 (whole cycles are averaged)")
    x, hist = x0, []
    for k in range(1000000):
        x = r * x * (1 - x)
        hist.append(x)
        if len(hist) > 2 and abs(hist[-1] - hist[-3]) < 1e-14:
            break
    else:
        raise ValueError("the orbit does not settle on a fixed point or a 2-cycle (chaotic or slowly converging regime): "
                         "a finite-time exponent there depends on rounding, so it is not reported")
    tot = 0.0
    for _ in range(n):
        d = abs(r * (1 - 2 * x))
        if d == 0:
            raise ValueError("the orbit is superstable (f'(x) = 0): the exponent is minus infinity")
        tot += math.log(d)
        x = r * x * (1 - x)
    attractor = "fixed point" if abs(r * x * (1 - x) - x) < 1e-12 else "2-cycle"
    if attractor == "fixed point" and abs(x) < 1e-12:
        attractor = "fixed point 0"
    return {"lyapunov": _r(tot / n), "attractor": attractor}


def _ref_lyap(r, x0, n):
    import numpy as np
    if r < 1:
        return {"lyapunov": math.log(r)}
    if r < 3:
        x = (np.roots([-r, r - 1, 0]))
        xs = max(float(z.real) for z in x)
        return {"lyapunov": math.log(abs(float(np.polyder(np.poly1d([-r, r, 0]))(xs))))}
    c = _ref_lm_cycle(r)
    return {"lyapunov": 0.5 * math.log(abs(c["multiplier"]))}


def _rand_lyap(rng):
    while True:
        r = round(rng.uniform(0.2, 3.44), 3)
        if min(abs(r - 1), abs(r - 3), abs(r - 2)) > 0.05:
            return {"r": r, "x0": _u(rng, 0.05, 0.95, 3), "n": 2 * rng.randint(50, 2000)}


add("logistic_map_lyapunov", name="Lyapunov exponent of the logistic map (periodic regime)", topic="dynamical systems / logistic map",
    purpose="The Lyapunov exponent of x -> r x (1 - x): the average of ln|f'(x_k)| over n iterations after the orbit settles (fixed point or 2-cycle).",
    inputs=[N("r", "parameter (0 < r <= 4)"), N("x0", "initial value in (0, 1)"), _int_in("n", "iterations averaged (even)", 2, 1000000)],
    outputs=[O("lyapunov", "Lyapunov exponent (negative: stable periodic orbit)"), O("attractor", "attractor reached", "text")], primary="lyapunov",
    conditions=["the orbit settles on a fixed point or 2-cycle (|x_{k+2} - x_k| < 1e-14)", "n even"],
    method="iterate until settled, then average ln|r (1 - 2 x_k)| over n iterations", latex=r"\lambda = \frac{1}{n}\sum_{k=1}^{n}\ln\left|r(1 - 2x_k)\right|",
    run=lm_lyapunov, reference=_ref_lyap, wrong=lambda r, x0, n: {"lyapunov": lm_lyapunov(r, x0, n)["lyapunov"] / math.log(10)},
    rand=_rand_lyap,
    templates=["Find the Lyapunov exponent of the logistic map with r = 2.5, x0 = 0.3, n = 1000."],
    examples=[({"r": 2.5, "x0": 0.3, "n": 1000}, {"lyapunov": -0.69314718056, "attractor": "fixed point"})],
    questions=[{"q": "Find the Lyapunov exponent of the logistic map with r = 2.5, x0 = 0.3, n = 1000.", "outputs": {"lyapunov": -0.69314718056}},
               {"q": "Find the Lyapunov exponent of the logistic map with r = 3.9, x0 = 0.3, n = 1000.", "outcome": "INVALID_INPUT"}],
    trig=(5, [_both(r"logistic map", r"lyapunov")]), target=None)


def lorenz_eq(sigma, rho, beta):
    _fin(sigma, rho, beta)
    if sigma <= 0 or beta <= 0 or rho <= 0:
        raise ValueError("needs sigma, rho, beta > 0")
    out = {"origin_stability": "asymptotically stable" if rho < 1 else ("unstable" if rho > 1 else "marginal (rho = 1)")}
    if rho > 1:
        out.update({"count": 3, "x_c": _r(math.sqrt(beta * (rho - 1))), "z_c": _r(rho - 1)})
    else:
        out["count"] = 1
    return out


def _ref_lorenz_eq(sigma, rho, beta):
    import numpy as np
    x, y, z = sp.symbols("x y z", real=True)
    S, R, B = (sp.Rational(repr(float(v))) for v in (sigma, rho, beta))
    sols = sp.solve([S * (y - x), x * (R - z) - y, x * y - B * z], [x, y, z], dict=True)
    real = [s for s in sols if all(v.is_real for v in s.values())]
    ev = np.linalg.eigvals(np.array([[-sigma, sigma, 0], [rho, -1, 0], [0, 0, -beta]], dtype=float))
    out = {"count": len(real), "origin_stability": "asymptotically stable" if max(ev.real) < 0 else "unstable"}
    pos = [s for s in real if s[x] > 0]
    if pos:
        out["x_c"], out["z_c"] = float(pos[0][x]), float(pos[0][z])
    return out


add("lorenz_equilibria", name="Lorenz system: equilibria and stability of the origin", topic="dynamical systems / Lorenz system",
    purpose="Equilibria of x' = sigma(y - x), y' = x(rho - z) - y, z' = xy - beta z: the origin and C+- = (+-sqrt(beta(rho-1)), same, rho - 1).",
    inputs=[N("sigma", "Prandtl number (> 0)"), N("rho", "Rayleigh parameter (> 0)"), N("beta", "geometric factor (> 0)")],
    outputs=[O("count", "number of equilibria"), O("x_c", "x (and y) of C+"), O("z_c", "z of C+- (rho - 1)"),
             O("origin_stability", "stability of the origin", "text")], primary="count", conditions=["sigma, rho, beta > 0"],
    method="nullclines; the origin's Jacobian has eigenvalue signs set by rho - 1", latex=r"C_\pm = \left(\pm\sqrt{\beta(\rho - 1)}, \pm\sqrt{\beta(\rho - 1)}, \rho - 1\right)",
    run=lorenz_eq, reference=_ref_lorenz_eq,
    wrong=lambda sigma, rho, beta: {"count": 3, "x_c": math.sqrt(abs(rho - 1)), "z_c": rho, "origin_stability": "unstable"},
    rand=lambda rng: {"sigma": _u(rng, 1, 15), "rho": rng.choice([_u(rng, 0.1, 0.95), _u(rng, 1.1, 40)]), "beta": _u(rng, 0.5, 4)},
    templates=["Find the equilibria of the Lorenz system with sigma = 10, rho = 28, beta = 8/3."],
    examples=[({"sigma": 10, "rho": 28, "beta": 8 / 3}, {"count": 3, "x_c": 8.48528137424, "z_c": 27.0})],
    questions=[{"q": "Find the equilibria of the Lorenz system with sigma = 10, rho = 28, beta = 8/3.", "outputs": {"x_c": 8.48528137424, "z_c": 27.0}},
               {"q": "Find the equilibria of the Lorenz system with sigma = 10, rho = 28.", "outcome": "NEEDS_INPUT"}],
    trig=(4, [r"lorenz"]), unless=[r"hopf|critical|onset"], target=None)


def lorenz_hopf(sigma, beta):
    _fin(sigma, beta)
    if sigma <= 0 or beta <= 0 or sigma <= beta + 1:
        raise ValueError("needs sigma > beta + 1 (otherwise C+- never lose stability through a Hopf bifurcation)")
    return {"rho_H": _r(sigma * (sigma + beta + 3) / (sigma - beta - 1))}


def _ref_lorenz_hopf(sigma, beta):
    import numpy as np
    from scipy.optimize import brentq

    def growth(rho):
        xc = math.sqrt(beta * (rho - 1))
        J = np.array([[-sigma, sigma, 0], [1, -1, -xc], [xc, xc, -beta]], dtype=float)
        return max(np.linalg.eigvals(J).real)
    guess = sigma * (sigma + beta + 3) / (sigma - beta - 1)
    return {"rho_H": brentq(growth, (1 + guess) / 2, guess * 2, xtol=1e-14, rtol=8.9e-16)}


add("lorenz_hopf_threshold", name="Lorenz system: Hopf threshold rho_H (loss of stability of C+-)", topic="dynamical systems / Lorenz system",
    purpose="The Rayleigh parameter at which the equilibria C+- of the Lorenz system lose stability (subcritical Hopf; chaos follows).",
    inputs=[N("sigma", "Prandtl number (> beta + 1)"), N("beta", "geometric factor (> 0)")],
    outputs=[O("rho_H", "Hopf threshold")], primary="rho_H", conditions=["sigma > beta + 1"],
    method="Routh-Hurwitz on the characteristic cubic at C+-", latex=r"\rho_H = \frac{\sigma(\sigma + \beta + 3)}{\sigma - \beta - 1}",
    run=lorenz_hopf, reference=_ref_lorenz_hopf, wrong=lambda sigma, beta: {"rho_H": sigma * (sigma + beta + 3) / (sigma - beta + 1)},
    rand=lambda rng: (lambda b: {"beta": b, "sigma": round(b + 1 + rng.uniform(0.5, 15), 2)})(_u(rng, 0.5, 4)),
    templates=["Find the Hopf threshold of the Lorenz system with sigma = 10, beta = 8/3."],
    examples=[({"sigma": 10, "beta": 8 / 3}, {"rho_H": 24.7368421053})],
    questions=[{"q": "Find the Hopf threshold of the Lorenz system with sigma = 10, beta = 8/3.", "outputs": {"rho_H": 24.7368421053}},
               {"q": "Find the Hopf threshold of the Lorenz system with sigma = 2, beta = 8/3.", "outcome": "INVALID_INPUT"}],
    trig=(5, [_both(r"lorenz", r"hopf|critical|onset")]), target=None)


def brusselator(a, b):
    _fin(a, b)
    if a <= 0 or b <= 0:
        raise ValueError("needs a > 0 and b > 0")
    A, B = Fraction(a), Fraction(b)
    kind, stab = _classify(B - 1 - A * A, A * A)
    return {"x_eq": _r(a), "y_eq": _r(b / a), "b_critical": _r(1 + a * a), "type": kind, "stability": stab}


def _ref_bruss(a, b):
    import numpy as np
    from scipy.optimize import brentq
    x, y = sp.symbols("x y")
    A, B = sp.Rational(repr(float(a))), sp.Rational(repr(float(b)))
    s = sp.solve([A - (B + 1) * x + x**2 * y, B * x - x**2 * y], [x, y], dict=True)[0]
    J = lambda bb: np.array([[bb - 1, a * a], [-bb, -a * a]], dtype=float)
    bc = brentq(lambda bb: max(np.linalg.eigvals(J(bb)).real), 1e-9, 10 + 4 * a * a, xtol=1e-14, rtol=8.9e-16)
    ev = np.linalg.eigvals(J(b))
    side = "stable" if ev[0].real < 0 else "unstable"
    kind = f"{side} spiral" if abs(ev[0].imag) > 1e-12 else f"{side} node"
    return {"x_eq": float(s[x]), "y_eq": float(s[y]), "b_critical": bc, "type": kind}


add("brusselator_hopf", name="Brusselator: equilibrium, its type, and the Hopf value b = 1 + a^2", topic="dynamical systems / bifurcations",
    purpose="For x' = a - (b + 1) x + x^2 y, y' = b x - x^2 y: the equilibrium (a, b/a), its type and the critical b where a limit cycle is born.",
    inputs=[N("a", "parameter a (> 0)"), N("b", "parameter b (> 0)")],
    outputs=[O("x_eq", "x at equilibrium"), O("y_eq", "y at equilibrium"), O("b_critical", "Hopf bifurcation value 1 + a^2"),
             O("type", "equilibrium type", "text"), O("stability", "stability", "text")], primary="b_critical", conditions=["a, b > 0"],
    method="Jacobian [[b - 1, a^2], [-b, -a^2]]: trace b - 1 - a^2 changes sign at the Hopf point", latex=r"b_c = 1 + a^2",
    run=brusselator, reference=_ref_bruss, wrong=lambda a, b: {"x_eq": a, "y_eq": a / b, "b_critical": 1 + a, "type": "stable node"},
    rand=lambda rng: (lambda a: {"a": a, "b": round((1 + a * a) * rng.choice([rng.uniform(0.2, 0.9), rng.uniform(1.1, 2.5)]), 3)})(_u(rng, 0.3, 3)),
    templates=["Analyse the Brusselator with a = 1, b = 3."],
    examples=[({"a": 1, "b": 3}, {"x_eq": 1.0, "y_eq": 3.0, "b_critical": 2.0, "type": "unstable spiral"})],
    questions=[{"q": "Analyse the Brusselator with a = 1, b = 3.", "outputs": {"b_critical": 2.0, "type": "unstable spiral"}},
               {"q": "Analyse the Brusselator with a = 1, b = -3.", "outcome": "INVALID_INPUT"}],
    trig=(4, [r"brusselator"]), target=None)


def sir(beta, gamma, S0, I0):
    _fin(beta, gamma, S0, I0)
    if beta <= 0 or gamma <= 0 or not (0 < S0 and 0 < I0 and S0 + I0 <= 1):
        raise ValueError("needs beta, gamma > 0 and fractions S0, I0 > 0 with S0 + I0 <= 1")
    R0 = beta / gamma
    if R0 * S0 > 1:
        Imax = S0 + I0 - (1 + math.log(R0 * S0)) / R0
    else:
        Imax = I0
    h = lambda S: math.log(S / S0) - R0 * (S - S0 - I0)
    lo, hi = 0.0, 1 / R0
    for _ in range(300):                                       # h increases on (0, 1/R0): one root
        mid = (lo + hi) / 2
        if mid == 0 or h(mid) < 0:
            lo = mid
        else:
            hi = mid
        if hi - lo <= 4e-16 * hi:
            break
    Sinf = (lo + hi) / 2
    return {"R0": _r(R0), "I_max": _r(Imax), "S_inf": _r(Sinf), "attack_rate": _r(S0 - Sinf)}


def _ref_sir(beta, gamma, S0, I0):
    from scipy.integrate import solve_ivp
    f = lambda t, u: [-beta * u[0] * u[1], beta * u[0] * u[1] - gamma * u[1]]
    peak = lambda t, u: beta * u[0] - gamma
    peak.direction = -1
    done = lambda t, u: u[1] - 1e-13
    done.terminal, done.direction = True, -1
    sol = solve_ivp(f, (0, 1e5), [S0, I0], method="DOP853", rtol=1e-13, atol=1e-16, events=[peak, done])
    Imax = float(sol.y_events[0][0][1]) if len(sol.t_events[0]) else I0
    S = float(sol.y[0][-1])
    return {"I_max": Imax, "S_inf": S, "R0": beta / gamma}


add("sir_epidemic", name="SIR epidemic: R0, peak infections and final size", topic="dynamical systems / epidemic models",
    purpose="For S' = -beta S I, I' = beta S I - gamma I (fractions of the population): R0, the peak infected fraction and the final susceptible fraction.",
    inputs=[N("beta", "transmission rate (> 0)"), N("gamma", "recovery rate (> 0)"), N("S0", "initial susceptible fraction"), N("I0", "initial infected fraction")],
    outputs=[O("R0", "basic reproduction number beta/gamma"), O("I_max", "peak infected fraction"), O("S_inf", "final susceptible fraction"),
             O("attack_rate", "fraction infected during the epidemic S0 - S_inf")], primary="I_max", conditions=["S0, I0 > 0", "S0 + I0 <= 1"],
    method="first integrals: I + S - ln(S)/R0 is conserved; peak at S = 1/R0; final size from ln(S/S0) = R0 (S - S0 - I0)",
    latex=r"I_{\max} = S_0 + I_0 - \frac{1 + \ln(R_0 S_0)}{R_0},\quad \ln\frac{S_\infty}{S_0} = R_0(S_\infty - S_0 - I_0)",
    run=sir, reference=_ref_sir, wrong=lambda beta, gamma, S0, I0: {"I_max": 1 - 1 / (beta / gamma), "S_inf": gamma / beta, "R0": beta / gamma},
    rand=lambda rng: (lambda g, I0: {"gamma": g, "beta": round(g * rng.uniform(1.6, 5), 3), "I0": I0, "S0": round(1 - I0, 6)})(_u(rng, 0.05, 0.5, 3),
                                                                                                                         _u(rng, 0.0005, 0.01, 4)),
    templates=["SIR epidemic model with beta = 0.3, gamma = 0.1, S0 = 0.99, I0 = 0.01: find the peak and final size."],
    examples=[({"beta": 0.3, "gamma": 0.1, "S0": 0.99, "I0": 0.01}, {"R0": 3.0, "I_max": 0.303812682395})],
    questions=[{"q": "SIR epidemic model with beta = 0.3, gamma = 0.1, S0 = 0.99, I0 = 0.01: find the peak and final size.",
                "outputs": {"R0": 3.0, "I_max": 0.303812682395}},
               {"q": "SIR epidemic model with beta = 0.3, gamma = 0.1, S0 = 0.99: find the peak.", "outcome": "NEEDS_INPUT"}],
    trig=(4, [r"\bsir\b"]), target=None)
