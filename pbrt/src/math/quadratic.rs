use std::mem;
use prelude::*;

pub fn quadratic(a: Efloat, b: Efloat, c: Efloat) -> Option<(Efloat, Efloat)> {
    let discrim = {
        let a = a.raw() as f64;
        let b = b.raw() as f64;
        let c = c.raw() as f64;
        b * b - 4.0 * a * c
    };

    if discrim < 0.0 {
        None
    } else {
        let root_discrim = discrim.sqrt();
        let root_discrim = efloat(root_discrim as FloatPrim, MACHINE_EPSILON as FloatPrim * root_discrim as FloatPrim);
        let q = if b < 0.0 {
            (b - root_discrim) * efloat0(-0.5)
        } else {
            (b + root_discrim) * efloat0(-0.5)
        };
        let mut t0 = q / a;
        let mut t1 = c / q;

        if t0 > t1 {
            mem::swap(&mut t0, &mut t1);
        }

        Some((t0, t1))
    }
}
