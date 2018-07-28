use num;
use cg::prelude::*;
use cg::Quaternion;
use prelude::*;
use super::{ Decomposed, DerivativeTerm };

#[derive(Copy, Clone, Debug)]
pub(crate) struct TermsOfMotion {
    pub c1: [DerivativeTerm; 3],
    pub c2: [DerivativeTerm; 3],
    pub c3: [DerivativeTerm; 3],
    pub c4: [DerivativeTerm; 3],
    pub c5: [DerivativeTerm; 3],
}

impl TermsOfMotion {
    pub(crate) fn interval_find_zeros(&self, num: usize, p: Point3f, theta: Float, interval: Interval, depth: usize) -> (usize, [Float; 4]) {
        let c1 = self.c1[num].eval(p);
        let c2 = self.c2[num].eval(p);
        let c3 = self.c3[num].eval(p);
        let c4 = self.c4[num].eval(p);
        let c5 = self.c5[num].eval(p);

        let mut num = 0;
        let mut zeros = [float(0.0); 4];

        fn interval_calc(c1: Float, c2: Float, c3: Float, c4: Float, c5: Float, theta: Float, interval: Interval, depth: usize, num: &mut usize, zeros: &mut [Float; 4]) {
            let int = |i| Interval::point(i);

            let range = int(c1) +
                (int(c2) + int(c3) * interval) *
                (int(float(2.0) * theta) * interval).cos() +
                (int(c4) + int(c5) * interval) *
                (int(float(2.0) * theta) * interval).sin();

            if range.low > 0.0 || range.high < 0.0 || range.low == range.high {
                return;
            }

            if depth > 0 {
                let mid = (interval.low + interval.high) * float(0.5);
                interval_calc(c1, c2, c3, c4, c5, theta, Interval::new(interval.low, mid), depth - 1, num, zeros);
                interval_calc(c1, c2, c3, c4, c5, theta, Interval::new(mid, interval.high), depth - 1, num, zeros);
            } else {
                let mut t_newton = (interval.low + interval.high) * float(0.5);
                for i in 0..4 {
                    let f_newton = c1 +
                        (c2 + c3 + t_newton) * (float(2.0) * theta * t_newton).cos() +
                        (c4 + c5 + t_newton) * (float(2.0) * theta * t_newton).sin();
                    let f_prime_newton =
                        (c3 + float(2.0) * (c4 + c5 * f_newton) * theta) *
                            (float(2.0) * f_newton * theta).cos() +
                        (c5 - float(2.0) * (c2 + c3 * f_newton) * theta) *
                            (float(2.0) * f_newton * theta).sin();
                    if f_newton == 0.0 || f_prime_newton == 0.0 {
                        break;
                    }
                    t_newton = t_newton - f_newton / f_prime_newton;
                }
                zeros[*num] = t_newton;
                *num += 1;
            }
        };

        interval_calc(c1, c2, c3, c4, c5, theta, interval, depth, &mut num, &mut zeros);

        (num, zeros)
    }

    pub(crate) fn new(start: Decomposed, end: Decomposed) -> Self {
        let r0 = start.rotate;
        let r1 = end.rotate;

        let s0 = start.scale;
        let s1 = end.scale;

        let t0 = start.translate;
        let t1 = end.translate;

        let cos_theta = r0.dot(r1);
        let theta = num::clamp(cos_theta, float(-1.0), float(1.0)).acos();
        let qperp = if r0 == r1 {
            Quaternion::from_sv(float(1.0), Vector3f::zero())
        } else {
            (r1 - r0 * cos_theta).normalize()
        };

        let t0x = t0.x;
        let t0y = t0.y;
        let t0z = t0.z;
        let t1x = t1.x;
        let t1y = t1.y;
        let t1z = t1.z;
        let q1x = r0.v.x;
        let q1y = r0.v.y;
        let q1z = r0.v.z;
        let q1w = r0.s;
        let qperpx = qperp.v.x;
        let qperpy = qperp.v.y;
        let qperpz = qperp.v.z;
        let qperpw = qperp.s;
        let s000 = s0[0][0];
        let s001 = s0[0][1];
        let s002 = s0[0][2];
        let s010 = s0[1][0];
        let s011 = s0[1][1];
        let s012 = s0[1][2];
        let s020 = s0[2][0];
        let s021 = s0[2][1];
        let s022 = s0[2][2];
        let s100 = s1[0][0];
        let s101 = s1[0][1];
        let s102 = s1[0][2];
        let s110 = s1[1][0];
        let s111 = s1[1][1];
        let s112 = s1[1][2];
        let s120 = s1[2][0];
        let s121 = s1[2][1];
        let s122 = s1[2][2];

        let c1_0 = DerivativeTerm {
            kc: -t0x + t1x,
            kx: (-float(1.0) + q1y * q1y + q1z * q1z + qperpy * qperpy + qperpz * qperpz) * s000
                + q1w * q1z * s010 - qperpx * qperpy * s010
                + qperpw * qperpz * s010 - q1w * q1y * s020
                - qperpw * qperpy * s020 - qperpx * qperpz * s020 + s100
                - q1y * q1y * s100 - q1z * q1z * s100
                - qperpy * qperpy * s100 - qperpz * qperpz * s100
                - q1w * q1z * s110 + qperpx * qperpy * s110
                - qperpw * qperpz * s110 + q1w * q1y * s120
                + qperpw * qperpy * s120 + qperpx * qperpz * s120
                + q1x * (-(q1y * s010) - q1z * s020 + q1y * s110 + q1z * s120),
            ky: (-float(1.0) + q1y * q1y + q1z * q1z + qperpy * qperpy + qperpz * qperpz) * s001
                + q1w * q1z * s011 - qperpx * qperpy * s011
                + qperpw * qperpz * s011 - q1w * q1y * s021
                - qperpw * qperpy * s021 - qperpx * qperpz * s021 + s101
                - q1y * q1y * s101 - q1z * q1z * s101
                - qperpy * qperpy * s101 - qperpz * qperpz * s101
                - q1w * q1z * s111 + qperpx * qperpy * s111
                - qperpw * qperpz * s111 + q1w * q1y * s121
                + qperpw * qperpy * s121 + qperpx * qperpz * s121
                + q1x * (-(q1y * s011) - q1z * s021 + q1y * s111 + q1z * s121),
            kz: (-float(1.0) + q1y * q1y + q1z * q1z + qperpy * qperpy + qperpz * qperpz) * s002
                + q1w * q1z * s012 - qperpx * qperpy * s012
                + qperpw * qperpz * s012 - q1w * q1y * s022
                - qperpw * qperpy * s022 - qperpx * qperpz * s022 + s102
                - q1y * q1y * s102 - q1z * q1z * s102
                - qperpy * qperpy * s102 - qperpz * qperpz * s102
                - q1w * q1z * s112 + qperpx * qperpy * s112
                - qperpw * qperpz * s112 + q1w * q1y * s122
                + qperpw * qperpy * s122 + qperpx * qperpz * s122
                + q1x * (-(q1y * s012) - q1z * s022 + q1y * s112 + q1z * s122),
        };
        let c2_0 = DerivativeTerm {
            kc: float(0.0),
            kx: -(qperpy * qperpy * s000) - qperpz * qperpz * s000 + qperpx * qperpy * s010
                - qperpw * qperpz * s010 + qperpw * qperpy * s020
                + qperpx * qperpz * s020 + q1y * q1y * (s000 - s100)
                + q1z * q1z * (s000 - s100) + qperpy * qperpy * s100
                + qperpz * qperpz * s100 - qperpx * qperpy * s110
                + qperpw * qperpz * s110 - qperpw * qperpy * s120
                - qperpx * qperpz * s120 + float(2.0) * q1x * qperpy * s010 * theta
                - float(2.0) * q1w * qperpz * s010 * theta
                + float(2.0) * q1w * qperpy * s020 * theta
                + float(2.0) * q1x * qperpz * s020 * theta
                + q1y * (q1x * (-s010 + s110) + q1w * (-s020 + s120)
                    + float(2.0) * (-float(2.0) * qperpy * s000 + qperpx * s010 + qperpw * s020) * theta)
                + q1z * (q1w * (s010 - s110) + q1x * (-s020 + s120)
                    - float(2.0) * (float(2.0) * qperpz * s000 + qperpw * s010 - qperpx * s020) * theta),
            ky: -(qperpy * qperpy * s001) - qperpz * qperpz * s001 + qperpx * qperpy * s011
                - qperpw * qperpz * s011 + qperpw * qperpy * s021
                + qperpx * qperpz * s021 + q1y * q1y * (s001 - s101)
                + q1z * q1z * (s001 - s101) + qperpy * qperpy * s101
                + qperpz * qperpz * s101 - qperpx * qperpy * s111
                + qperpw * qperpz * s111 - qperpw * qperpy * s121
                - qperpx * qperpz * s121 + float(2.0) * q1x * qperpy * s011 * theta
                - float(2.0) * q1w * qperpz * s011 * theta
                + float(2.0) * q1w * qperpy * s021 * theta
                + float(2.0) * q1x * qperpz * s021 * theta
                + q1y * (q1x * (-s011 + s111) + q1w * (-s021 + s121)
                    + float(2.0) * (-float(2.0) * qperpy * s001 + qperpx * s011 + qperpw * s021) * theta)
                + q1z * (q1w * (s011 - s111) + q1x * (-s021 + s121)
                    - float(2.0) * (float(2.0) * qperpz * s001 + qperpw * s011 - qperpx * s021) * theta),
            kz: -(qperpy * qperpy * s002) - qperpz * qperpz * s002 + qperpx * qperpy * s012
                - qperpw * qperpz * s012 + qperpw * qperpy * s022
                + qperpx * qperpz * s022 + q1y * q1y * (s002 - s102)
                + q1z * q1z * (s002 - s102) + qperpy * qperpy * s102
                + qperpz * qperpz * s102 - qperpx * qperpy * s112
                + qperpw * qperpz * s112 - qperpw * qperpy * s122
                - qperpx * qperpz * s122 + float(2.0) * q1x * qperpy * s012 * theta
                - float(2.0) * q1w * qperpz * s012 * theta
                + float(2.0) * q1w * qperpy * s022 * theta
                + float(2.0) * q1x * qperpz * s022 * theta
                + q1y * (q1x * (-s012 + s112) + q1w * (-s022 + s122)
                    + float(2.0) * (-float(2.0) * qperpy * s002 + qperpx * s012 + qperpw * s022) * theta)
                + q1z * (q1w * (s012 - s112) + q1x * (-s022 + s122)
                    - float(2.0) * (float(2.0) * qperpz * s002 + qperpw * s012 - qperpx * s022) * theta),
        };
        let c3_0 = DerivativeTerm {
            kc: float(0.0),
            kx: -float(2.0) * (q1x * qperpy * s010 - q1w * qperpz * s010 + q1w * qperpy * s020
                + q1x * qperpz * s020 - q1x * qperpy * s110
                + q1w * qperpz * s110 - q1w * qperpy * s120
                - q1x * qperpz * s120
                + q1y * (-float(2.0) * qperpy * s000 + qperpx * s010 + qperpw * s020
                    + float(2.0) * qperpy * s100 - qperpx * s110
                    - qperpw * s120)
                + q1z * (-float(2.0) * qperpz * s000 - qperpw * s010 + qperpx * s020
                    + float(2.0) * qperpz * s100 + qperpw * s110
                    - qperpx * s120)) * theta,
            ky: -float(2.0) * (q1x * qperpy * s011 - q1w * qperpz * s011 + q1w * qperpy * s021
                + q1x * qperpz * s021 - q1x * qperpy * s111
                + q1w * qperpz * s111 - q1w * qperpy * s121
                - q1x * qperpz * s121
                + q1y * (-float(2.0) * qperpy * s001 + qperpx * s011 + qperpw * s021
                    + float(2.0) * qperpy * s101 - qperpx * s111
                    - qperpw * s121)
                + q1z * (-float(2.0) * qperpz * s001 - qperpw * s011 + qperpx * s021
                    + float(2.0) * qperpz * s101 + qperpw * s111
                    - qperpx * s121)) * theta,
            kz: -float(2.0) * (q1x * qperpy * s012 - q1w * qperpz * s012 + q1w * qperpy * s022
                + q1x * qperpz * s022 - q1x * qperpy * s112
                + q1w * qperpz * s112 - q1w * qperpy * s122
                - q1x * qperpz * s122
                + q1y * (-float(2.0) * qperpy * s002 + qperpx * s012 + qperpw * s022
                    + float(2.0) * qperpy * s102 - qperpx * s112
                    - qperpw * s122)
                + q1z * (-float(2.0) * qperpz * s002 - qperpw * s012 + qperpx * s022
                    + float(2.0) * qperpz * s102 + qperpw * s112
                    - qperpx * s122)) * theta,
        };
        let c4_0 = DerivativeTerm {
            kc: float(0.0),
            kx: -(q1x * qperpy * s010) + q1w * qperpz * s010 - q1w * qperpy * s020
                - q1x * qperpz * s020 + q1x * qperpy * s110
                - q1w * qperpz * s110 + q1w * qperpy * s120
                + q1x * qperpz * s120 + float(2.0) * q1y * q1y * s000 * theta
                + float(2.0) * q1z * q1z * s000 * theta
                - float(2.0) * qperpy * qperpy * s000 * theta
                - float(2.0) * qperpz * qperpz * s000 * theta
                + float(2.0) * qperpx * qperpy * s010 * theta
                - float(2.0) * qperpw * qperpz * s010 * theta
                + float(2.0) * qperpw * qperpy * s020 * theta
                + float(2.0) * qperpx * qperpz * s020 * theta
                + q1y * (-(qperpx * s010) - qperpw * s020 + float(2.0) * qperpy * (s000 - s100)
                    + qperpx * s110 + qperpw * s120
                    - float(2.0) * q1x * s010 * theta
                    - float(2.0) * q1w * s020 * theta)
                + q1z * (float(2.0) * qperpz * s000 + qperpw * s010 - qperpx * s020
                    - float(2.0) * qperpz * s100 - qperpw * s110
                    + qperpx * s120 + float(2.0) * q1w * s010 * theta
                    - float(2.0) * q1x * s020 * theta),
            ky: -(q1x * qperpy * s011) + q1w * qperpz * s011 - q1w * qperpy * s021
                - q1x * qperpz * s021 + q1x * qperpy * s111
                - q1w * qperpz * s111 + q1w * qperpy * s121
                + q1x * qperpz * s121 + float(2.0) * q1y * q1y * s001 * theta
                + float(2.0) * q1z * q1z * s001 * theta
                - float(2.0) * qperpy * qperpy * s001 * theta
                - float(2.0) * qperpz * qperpz * s001 * theta
                + float(2.0) * qperpx * qperpy * s011 * theta
                - float(2.0) * qperpw * qperpz * s011 * theta
                + float(2.0) * qperpw * qperpy * s021 * theta
                + float(2.0) * qperpx * qperpz * s021 * theta
                + q1y * (-(qperpx * s011) - qperpw * s021 + float(2.0) * qperpy * (s001 - s101)
                    + qperpx * s111 + qperpw * s121
                    - float(2.0) * q1x * s011 * theta
                    - float(2.0) * q1w * s021 * theta)
                + q1z * (float(2.0) * qperpz * s001 + qperpw * s011 - qperpx * s021
                    - float(2.0) * qperpz * s101 - qperpw * s111
                    + qperpx * s121 + float(2.0) * q1w * s011 * theta
                    - float(2.0) * q1x * s021 * theta),
            kz: -(q1x * qperpy * s012) + q1w * qperpz * s012 - q1w * qperpy * s022
                - q1x * qperpz * s022 + q1x * qperpy * s112
                - q1w * qperpz * s112 + q1w * qperpy * s122
                + q1x * qperpz * s122 + float(2.0) * q1y * q1y * s002 * theta
                + float(2.0) * q1z * q1z * s002 * theta
                - float(2.0) * qperpy * qperpy * s002 * theta
                - float(2.0) * qperpz * qperpz * s002 * theta
                + float(2.0) * qperpx * qperpy * s012 * theta
                - float(2.0) * qperpw * qperpz * s012 * theta
                + float(2.0) * qperpw * qperpy * s022 * theta
                + float(2.0) * qperpx * qperpz * s022 * theta
                + q1y * (-(qperpx * s012) - qperpw * s022 + float(2.0) * qperpy * (s002 - s102)
                    + qperpx * s112 + qperpw * s122
                    - float(2.0) * q1x * s012 * theta
                    - float(2.0) * q1w * s022 * theta)
                + q1z * (float(2.0) * qperpz * s002 + qperpw * s012 - qperpx * s022
                    - float(2.0) * qperpz * s102 - qperpw * s112
                    + qperpx * s122 + float(2.0) * q1w * s012 * theta
                    - float(2.0) * q1x * s022 * theta),
        };
        let c5_0 = DerivativeTerm {
            kc: float(0.0),
            kx: float(2.0) * (qperpy * qperpy * s000 + qperpz * qperpz * s000 - qperpx * qperpy * s010
                + qperpw * qperpz * s010 - qperpw * qperpy * s020
                - qperpx * qperpz * s020 - qperpy * qperpy * s100
                - qperpz * qperpz * s100 + q1y * q1y * (-s000 + s100)
                + q1z * q1z * (-s000 + s100) + qperpx * qperpy * s110
                - qperpw * qperpz * s110
                + q1y * (q1x * (s010 - s110) + q1w * (s020 - s120))
                + qperpw * qperpy * s120 + qperpx * qperpz * s120
                + q1z * (-(q1w * s010) + q1x * s020 + q1w * s110 - q1x * s120))
                * theta,
            ky: float(2.0) * (qperpy * qperpy * s001 + qperpz * qperpz * s001 - qperpx * qperpy * s011
                + qperpw * qperpz * s011 - qperpw * qperpy * s021
                - qperpx * qperpz * s021 - qperpy * qperpy * s101
                - qperpz * qperpz * s101 + q1y * q1y * (-s001 + s101)
                + q1z * q1z * (-s001 + s101) + qperpx * qperpy * s111
                - qperpw * qperpz * s111
                + q1y * (q1x * (s011 - s111) + q1w * (s021 - s121))
                + qperpw * qperpy * s121 + qperpx * qperpz * s121
                + q1z * (-(q1w * s011) + q1x * s021 + q1w * s111 - q1x * s121))
                * theta,
            kz: float(2.0) * (qperpy * qperpy * s002 + qperpz * qperpz * s002 - qperpx * qperpy * s012
                + qperpw * qperpz * s012 - qperpw * qperpy * s022
                - qperpx * qperpz * s022 - qperpy * qperpy * s102
                - qperpz * qperpz * s102 + q1y * q1y * (-s002 + s102)
                + q1z * q1z * (-s002 + s102) + qperpx * qperpy * s112
                - qperpw * qperpz * s112
                + q1y * (q1x * (s012 - s112) + q1w * (s022 - s122))
                + qperpw * qperpy * s122 + qperpx * qperpz * s122
                + q1z * (-(q1w * s012) + q1x * s022 + q1w * s112 - q1x * s122))
                * theta,
        };
        let c1_1 = DerivativeTerm {
            kc: -t0y + t1y,
            kx: -(qperpx * qperpy * s000) - qperpw * qperpz * s000 - s010 + q1z * q1z * s010
                + qperpx * qperpx * s010 + qperpz * qperpz * s010
                - q1y * q1z * s020 + qperpw * qperpx * s020
                - qperpy * qperpz * s020 + qperpx * qperpy * s100
                + qperpw * qperpz * s100 + q1w * q1z * (-s000 + s100)
                + q1x * q1x * (s010 - s110) + s110 - q1z * q1z * s110
                - qperpx * qperpx * s110 - qperpz * qperpz * s110
                + q1x * (q1y * (-s000 + s100) + q1w * (s020 - s120))
                + q1y * q1z * s120 - qperpw * qperpx * s120
                + qperpy * qperpz * s120,
            ky: -(qperpx * qperpy * s001) - qperpw * qperpz * s001 - s011 + q1z * q1z * s011
                + qperpx * qperpx * s011 + qperpz * qperpz * s011
                - q1y * q1z * s021 + qperpw * qperpx * s021
                - qperpy * qperpz * s021 + qperpx * qperpy * s101
                + qperpw * qperpz * s101 + q1w * q1z * (-s001 + s101)
                + q1x * q1x * (s011 - s111) + s111 - q1z * q1z * s111
                - qperpx * qperpx * s111 - qperpz * qperpz * s111
                + q1x * (q1y * (-s001 + s101) + q1w * (s021 - s121))
                + q1y * q1z * s121 - qperpw * qperpx * s121
                + qperpy * qperpz * s121,
            kz: -(qperpx * qperpy * s002) - qperpw * qperpz * s002 - s012 + q1z * q1z * s012
                + qperpx * qperpx * s012 + qperpz * qperpz * s012
                - q1y * q1z * s022 + qperpw * qperpx * s022
                - qperpy * qperpz * s022 + qperpx * qperpy * s102
                + qperpw * qperpz * s102 + q1w * q1z * (-s002 + s102)
                + q1x * q1x * (s012 - s112) + s112 - q1z * q1z * s112
                - qperpx * qperpx * s112 - qperpz * qperpz * s112
                + q1x * (q1y * (-s002 + s102) + q1w * (s022 - s122))
                + q1y * q1z * s122 - qperpw * qperpx * s122
                + qperpy * qperpz * s122,
        };
        let c2_1 = DerivativeTerm {
            kc: float(0.0),
            kx: qperpx * qperpy * s000 + qperpw * qperpz * s000 + q1z * q1z * s010
                - qperpx * qperpx * s010 - qperpz * qperpz * s010
                - q1y * q1z * s020 - qperpw * qperpx * s020
                + qperpy * qperpz * s020 - qperpx * qperpy * s100
                - qperpw * qperpz * s100 + q1x * q1x * (s010 - s110)
                - q1z * q1z * s110 + qperpx * qperpx * s110
                + qperpz * qperpz * s110 + q1y * q1z * s120
                + qperpw * qperpx * s120 - qperpy * qperpz * s120
                + float(2.0) * q1z * qperpw * s000 * theta
                + float(2.0) * q1y * qperpx * s000 * theta
                - float(4.0) * q1z * qperpz * s010 * theta
                + float(2.0) * q1z * qperpy * s020 * theta
                + float(2.0) * q1y * qperpz * s020 * theta
                + q1x * (q1w * s020 + q1y * (-s000 + s100) - q1w * s120
                    + float(2.0) * qperpy * s000 * theta
                    - float(4.0) * qperpx * s010 * theta
                    - float(2.0) * qperpw * s020 * theta)
                + q1w * (-(q1z * s000) + q1z * s100 + float(2.0) * qperpz * s000 * theta
                    - float(2.0) * qperpx * s020 * theta),
            ky: qperpx * qperpy * s001 + qperpw * qperpz * s001 + q1z * q1z * s011
                - qperpx * qperpx * s011 - qperpz * qperpz * s011
                - q1y * q1z * s021 - qperpw * qperpx * s021
                + qperpy * qperpz * s021 - qperpx * qperpy * s101
                - qperpw * qperpz * s101 + q1x * q1x * (s011 - s111)
                - q1z * q1z * s111 + qperpx * qperpx * s111
                + qperpz * qperpz * s111 + q1y * q1z * s121
                + qperpw * qperpx * s121 - qperpy * qperpz * s121
                + float(2.0) * q1z * qperpw * s001 * theta
                + float(2.0) * q1y * qperpx * s001 * theta
                - float(4.0) * q1z * qperpz * s011 * theta
                + float(2.0) * q1z * qperpy * s021 * theta
                + float(2.0) * q1y * qperpz * s021 * theta
                + q1x * (q1w * s021 + q1y * (-s001 + s101) - q1w * s121
                    + float(2.0) * qperpy * s001 * theta
                    - float(4.0) * qperpx * s011 * theta
                    - float(2.0) * qperpw * s021 * theta)
                + q1w * (-(q1z * s001) + q1z * s101 + float(2.0) * qperpz * s001 * theta
                    - float(2.0) * qperpx * s021 * theta),
            kz: qperpx * qperpy * s002 + qperpw * qperpz * s002 + q1z * q1z * s012
                - qperpx * qperpx * s012 - qperpz * qperpz * s012
                - q1y * q1z * s022 - qperpw * qperpx * s022
                + qperpy * qperpz * s022 - qperpx * qperpy * s102
                - qperpw * qperpz * s102 + q1x * q1x * (s012 - s112)
                - q1z * q1z * s112 + qperpx * qperpx * s112
                + qperpz * qperpz * s112 + q1y * q1z * s122
                + qperpw * qperpx * s122 - qperpy * qperpz * s122
                + float(2.0) * q1z * qperpw * s002 * theta
                + float(2.0) * q1y * qperpx * s002 * theta
                - float(4.0) * q1z * qperpz * s012 * theta
                + float(2.0) * q1z * qperpy * s022 * theta
                + float(2.0) * q1y * qperpz * s022 * theta
                + q1x * (q1w * s022 + q1y * (-s002 + s102) - q1w * s122
                    + float(2.0) * qperpy * s002 * theta
                    - float(4.0) * qperpx * s012 * theta
                    - float(2.0) * qperpw * s022 * theta)
                + q1w * (-(q1z * s002) + q1z * s102 + float(2.0) * qperpz * s002 * theta
                    - float(2.0) * qperpx * s022 * theta),
        };
        let c3_1 = DerivativeTerm {
            kc: float(0.0),
            kx: float(2.0) * (-(q1x * qperpy * s000) - q1w * qperpz * s000 + float(2.0) * q1x * qperpx * s010
                + q1x * qperpw * s020 + q1w * qperpx * s020
                + q1x * qperpy * s100 + q1w * qperpz * s100
                - float(2.0) * q1x * qperpx * s110 - q1x * qperpw * s120
                - q1w * qperpx * s120
                + q1z * (float(2.0) * qperpz * s010 - qperpy * s020 + qperpw * (-s000 + s100)
                    - float(2.0) * qperpz * s110 + qperpy * s120)
                + q1y * (-(qperpx * s000) - qperpz * s020 + qperpx * s100 + qperpz * s120))
                * theta,
            ky: float(2.0) * (-(q1x * qperpy * s001) - q1w * qperpz * s001 + float(2.0) * q1x * qperpx * s011
                + q1x * qperpw * s021 + q1w * qperpx * s021
                + q1x * qperpy * s101 + q1w * qperpz * s101
                - float(2.0) * q1x * qperpx * s111 - q1x * qperpw * s121
                - q1w * qperpx * s121
                + q1z * (float(2.0) * qperpz * s011 - qperpy * s021 + qperpw * (-s001 + s101)
                    - float(2.0) * qperpz * s111 + qperpy * s121)
                + q1y * (-(qperpx * s001) - qperpz * s021 + qperpx * s101 + qperpz * s121))
                * theta,
            kz: float(2.0) * (-(q1x * qperpy * s002) - q1w * qperpz * s002 + float(2.0) * q1x * qperpx * s012
                + q1x * qperpw * s022 + q1w * qperpx * s022
                + q1x * qperpy * s102 + q1w * qperpz * s102
                - float(2.0) * q1x * qperpx * s112 - q1x * qperpw * s122
                - q1w * qperpx * s122
                + q1z * (float(2.0) * qperpz * s012 - qperpy * s022 + qperpw * (-s002 + s102)
                    - float(2.0) * qperpz * s112 + qperpy * s122)
                + q1y * (-(qperpx * s002) - qperpz * s022 + qperpx * s102 + qperpz * s122))
                * theta,
        };
        let c4_1 = DerivativeTerm {
            kc: float(0.0),
            kx: -(q1x * qperpy * s000) - q1w * qperpz * s000 + float(2.0) * q1x * qperpx * s010
                + q1x * qperpw * s020 + q1w * qperpx * s020
                + q1x * qperpy * s100 + q1w * qperpz * s100
                - float(2.0) * q1x * qperpx * s110 - q1x * qperpw * s120
                - q1w * qperpx * s120 + float(2.0) * qperpx * qperpy * s000 * theta
                + float(2.0) * qperpw * qperpz * s000 * theta
                + float(2.0) * q1x * q1x * s010 * theta
                + float(2.0) * q1z * q1z * s010 * theta
                - float(2.0) * qperpx * qperpx * s010 * theta
                - float(2.0) * qperpz * qperpz * s010 * theta
                + float(2.0) * q1w * q1x * s020 * theta
                - float(2.0) * qperpw * qperpx * s020 * theta
                + float(2.0) * qperpy * qperpz * s020 * theta
                + q1y * (-(qperpx * s000) - qperpz * s020 + qperpx * s100 + qperpz * s120
                    - float(2.0) * q1x * s000 * theta)
                + q1z * (float(2.0) * qperpz * s010 - qperpy * s020 + qperpw * (-s000 + s100)
                    - float(2.0) * qperpz * s110 + qperpy * s120
                    - float(2.0) * q1w * s000 * theta
                    - float(2.0) * q1y * s020 * theta),
            ky: -(q1x * qperpy * s001) - q1w * qperpz * s001 + float(2.0) * q1x * qperpx * s011
                + q1x * qperpw * s021 + q1w * qperpx * s021
                + q1x * qperpy * s101 + q1w * qperpz * s101
                - float(2.0) * q1x * qperpx * s111 - q1x * qperpw * s121
                - q1w * qperpx * s121 + float(2.0) * qperpx * qperpy * s001 * theta
                + float(2.0) * qperpw * qperpz * s001 * theta
                + float(2.0) * q1x * q1x * s011 * theta
                + float(2.0) * q1z * q1z * s011 * theta
                - float(2.0) * qperpx * qperpx * s011 * theta
                - float(2.0) * qperpz * qperpz * s011 * theta
                + float(2.0) * q1w * q1x * s021 * theta
                - float(2.0) * qperpw * qperpx * s021 * theta
                + float(2.0) * qperpy * qperpz * s021 * theta
                + q1y * (-(qperpx * s001) - qperpz * s021 + qperpx * s101 + qperpz * s121
                    - float(2.0) * q1x * s001 * theta)
                + q1z * (float(2.0) * qperpz * s011 - qperpy * s021 + qperpw * (-s001 + s101)
                    - float(2.0) * qperpz * s111 + qperpy * s121
                    - float(2.0) * q1w * s001 * theta
                    - float(2.0) * q1y * s021 * theta),
            kz: -(q1x * qperpy * s002) - q1w * qperpz * s002 + float(2.0) * q1x * qperpx * s012
                + q1x * qperpw * s022 + q1w * qperpx * s022
                + q1x * qperpy * s102 + q1w * qperpz * s102
                - float(2.0) * q1x * qperpx * s112 - q1x * qperpw * s122
                - q1w * qperpx * s122 + float(2.0) * qperpx * qperpy * s002 * theta
                + float(2.0) * qperpw * qperpz * s002 * theta
                + float(2.0) * q1x * q1x * s012 * theta
                + float(2.0) * q1z * q1z * s012 * theta
                - float(2.0) * qperpx * qperpx * s012 * theta
                - float(2.0) * qperpz * qperpz * s012 * theta
                + float(2.0) * q1w * q1x * s022 * theta
                - float(2.0) * qperpw * qperpx * s022 * theta
                + float(2.0) * qperpy * qperpz * s022 * theta
                + q1y * (-(qperpx * s002) - qperpz * s022 + qperpx * s102 + qperpz * s122
                    - float(2.0) * q1x * s002 * theta)
                + q1z * (float(2.0) * qperpz * s012 - qperpy * s022 + qperpw * (-s002 + s102)
                    - float(2.0) * qperpz * s112 + qperpy * s122
                    - float(2.0) * q1w * s002 * theta
                    - float(2.0) * q1y * s022 * theta),
        };
        let c5_1 = DerivativeTerm {
            kc: float(0.0),
            kx: -float(2.0) * (qperpx * qperpy * s000 + qperpw * qperpz * s000 + q1z * q1z * s010
                - qperpx * qperpx * s010 - qperpz * qperpz * s010
                - q1y * q1z * s020 - qperpw * qperpx * s020
                + qperpy * qperpz * s020 - qperpx * qperpy * s100
                - qperpw * qperpz * s100
                + q1w * q1z * (-s000 + s100)
                + q1x * q1x * (s010 - s110) - q1z * q1z * s110
                + qperpx * qperpx * s110 + qperpz * qperpz * s110
                + q1x * (q1y * (-s000 + s100) + q1w * (s020 - s120))
                + q1y * q1z * s120 + qperpw * qperpx * s120
                - qperpy * qperpz * s120) * theta,
            ky: -float(2.0) * (qperpx * qperpy * s001 + qperpw * qperpz * s001 + q1z * q1z * s011
                - qperpx * qperpx * s011 - qperpz * qperpz * s011
                - q1y * q1z * s021 - qperpw * qperpx * s021
                + qperpy * qperpz * s021 - qperpx * qperpy * s101
                - qperpw * qperpz * s101
                + q1w * q1z * (-s001 + s101)
                + q1x * q1x * (s011 - s111) - q1z * q1z * s111
                + qperpx * qperpx * s111 + qperpz * qperpz * s111
                + q1x * (q1y * (-s001 + s101) + q1w * (s021 - s121))
                + q1y * q1z * s121 + qperpw * qperpx * s121
                - qperpy * qperpz * s121) * theta,
            kz: -float(2.0) * (qperpx * qperpy * s002 + qperpw * qperpz * s002 + q1z * q1z * s012
                - qperpx * qperpx * s012 - qperpz * qperpz * s012
                - q1y * q1z * s022 - qperpw * qperpx * s022
                + qperpy * qperpz * s022 - qperpx * qperpy * s102
                - qperpw * qperpz * s102
                + q1w * q1z * (-s002 + s102)
                + q1x * q1x * (s012 - s112) - q1z * q1z * s112
                + qperpx * qperpx * s112 + qperpz * qperpz * s112
                + q1x * (q1y * (-s002 + s102) + q1w * (s022 - s122))
                + q1y * q1z * s122 + qperpw * qperpx * s122
                - qperpy * qperpz * s122) * theta,
        };
        let c1_2 = DerivativeTerm {
            kc: -t0z + t1z,
            kx: (qperpw * qperpy * s000 - qperpx * qperpz * s000 - q1y * q1z * s010
                - qperpw * qperpx * s010 - qperpy * qperpz * s010 - s020
                + q1y * q1y * s020 + qperpx * qperpx * s020
                + qperpy * qperpy * s020 - qperpw * qperpy * s100
                + qperpx * qperpz * s100 + q1x * q1z * (-s000 + s100)
                + q1y * q1z * s110 + qperpw * qperpx * s110
                + qperpy * qperpz * s110
                + q1w * (q1y * (s000 - s100) + q1x * (-s010 + s110))
                + q1x * q1x * (s020 - s120) + s120 - q1y * q1y * s120
                - qperpx * qperpx * s120 - qperpy * qperpy * s120),
            ky: (qperpw * qperpy * s001 - qperpx * qperpz * s001 - q1y * q1z * s011
                - qperpw * qperpx * s011 - qperpy * qperpz * s011 - s021
                + q1y * q1y * s021 + qperpx * qperpx * s021
                + qperpy * qperpy * s021 - qperpw * qperpy * s101
                + qperpx * qperpz * s101 + q1x * q1z * (-s001 + s101)
                + q1y * q1z * s111 + qperpw * qperpx * s111
                + qperpy * qperpz * s111
                + q1w * (q1y * (s001 - s101) + q1x * (-s011 + s111))
                + q1x * q1x * (s021 - s121) + s121 - q1y * q1y * s121
                - qperpx * qperpx * s121 - qperpy * qperpy * s121),
            kz: (qperpw * qperpy * s002 - qperpx * qperpz * s002 - q1y * q1z * s012
                - qperpw * qperpx * s012 - qperpy * qperpz * s012 - s022
                + q1y * q1y * s022 + qperpx * qperpx * s022
                + qperpy * qperpy * s022 - qperpw * qperpy * s102
                + qperpx * qperpz * s102 + q1x * q1z * (-s002 + s102)
                + q1y * q1z * s112 + qperpw * qperpx * s112
                + qperpy * qperpz * s112
                + q1w * (q1y * (s002 - s102) + q1x * (-s012 + s112))
                + q1x * q1x * (s022 - s122) + s122 - q1y * q1y * s122
                - qperpx * qperpx * s122 - qperpy * qperpy * s122),
        };
        let c2_2 = DerivativeTerm {
            kc: float(0.0),
            kx: (q1w * q1y * s000 - q1x * q1z * s000 - qperpw * qperpy * s000
                + qperpx * qperpz * s000 - q1w * q1x * s010
                - q1y * q1z * s010 + qperpw * qperpx * s010
                + qperpy * qperpz * s010 + q1x * q1x * s020
                + q1y * q1y * s020 - qperpx * qperpx * s020
                - qperpy * qperpy * s020 - q1w * q1y * s100
                + q1x * q1z * s100 + qperpw * qperpy * s100
                - qperpx * qperpz * s100 + q1w * q1x * s110
                + q1y * q1z * s110 - qperpw * qperpx * s110
                - qperpy * qperpz * s110 - q1x * q1x * s120
                - q1y * q1y * s120 + qperpx * qperpx * s120
                + qperpy * qperpy * s120
                - float(2.0) * q1y * qperpw * s000 * theta
                + float(2.0) * q1z * qperpx * s000 * theta
                - float(2.0) * q1w * qperpy * s000 * theta
                + float(2.0) * q1x * qperpz * s000 * theta
                + float(2.0) * q1x * qperpw * s010 * theta
                + float(2.0) * q1w * qperpx * s010 * theta
                + float(2.0) * q1z * qperpy * s010 * theta
                + float(2.0) * q1y * qperpz * s010 * theta
                - float(4.0) * q1x * qperpx * s020 * theta
                - float(4.0) * q1y * qperpy * s020 * theta),
            ky: (q1w * q1y * s001 - q1x * q1z * s001 - qperpw * qperpy * s001
                + qperpx * qperpz * s001 - q1w * q1x * s011
                - q1y * q1z * s011 + qperpw * qperpx * s011
                + qperpy * qperpz * s011 + q1x * q1x * s021
                + q1y * q1y * s021 - qperpx * qperpx * s021
                - qperpy * qperpy * s021 - q1w * q1y * s101
                + q1x * q1z * s101 + qperpw * qperpy * s101
                - qperpx * qperpz * s101 + q1w * q1x * s111
                + q1y * q1z * s111 - qperpw * qperpx * s111
                - qperpy * qperpz * s111 - q1x * q1x * s121
                - q1y * q1y * s121 + qperpx * qperpx * s121
                + qperpy * qperpy * s121
                - float(2.0) * q1y * qperpw * s001 * theta
                + float(2.0) * q1z * qperpx * s001 * theta
                - float(2.0) * q1w * qperpy * s001 * theta
                + float(2.0) * q1x * qperpz * s001 * theta
                + float(2.0) * q1x * qperpw * s011 * theta
                + float(2.0) * q1w * qperpx * s011 * theta
                + float(2.0) * q1z * qperpy * s011 * theta
                + float(2.0) * q1y * qperpz * s011 * theta
                - float(4.0) * q1x * qperpx * s021 * theta
                - float(4.0) * q1y * qperpy * s021 * theta),
            kz: (q1w * q1y * s002 - q1x * q1z * s002 - qperpw * qperpy * s002
                + qperpx * qperpz * s002 - q1w * q1x * s012
                - q1y * q1z * s012 + qperpw * qperpx * s012
                + qperpy * qperpz * s012 + q1x * q1x * s022
                + q1y * q1y * s022 - qperpx * qperpx * s022
                - qperpy * qperpy * s022 - q1w * q1y * s102
                + q1x * q1z * s102 + qperpw * qperpy * s102
                - qperpx * qperpz * s102 + q1w * q1x * s112
                + q1y * q1z * s112 - qperpw * qperpx * s112
                - qperpy * qperpz * s112 - q1x * q1x * s122
                - q1y * q1y * s122 + qperpx * qperpx * s122
                + qperpy * qperpy * s122
                - float(2.0) * q1y * qperpw * s002 * theta
                + float(2.0) * q1z * qperpx * s002 * theta
                - float(2.0) * q1w * qperpy * s002 * theta
                + float(2.0) * q1x * qperpz * s002 * theta
                + float(2.0) * q1x * qperpw * s012 * theta
                + float(2.0) * q1w * qperpx * s012 * theta
                + float(2.0) * q1z * qperpy * s012 * theta
                + float(2.0) * q1y * qperpz * s012 * theta
                - float(4.0) * q1x * qperpx * s022 * theta
                - float(4.0) * q1y * qperpy * s022 * theta),
        };
        let c3_2 = DerivativeTerm {
            kc: float(0.0),
            kx: -float(2.0) * (-(q1w * qperpy * s000) + q1x * qperpz * s000 + q1x * qperpw * s010
                + q1w * qperpx * s010 - float(2.0) * q1x * qperpx * s020
                + q1w * qperpy * s100 - q1x * qperpz * s100
                - q1x * qperpw * s110 - q1w * qperpx * s110
                + q1z * (qperpx * s000 + qperpy * s010 - qperpx * s100 - qperpy * s110)
                + float(2.0) * q1x * qperpx * s120
                + q1y * (qperpz * s010 - float(2.0) * qperpy * s020 + qperpw * (-s000 + s100)
                    - qperpz * s110 + float(2.0) * qperpy * s120)) * theta,
            ky: -float(2.0) * (-(q1w * qperpy * s001) + q1x * qperpz * s001 + q1x * qperpw * s011
                + q1w * qperpx * s011 - float(2.0) * q1x * qperpx * s021
                + q1w * qperpy * s101 - q1x * qperpz * s101
                - q1x * qperpw * s111 - q1w * qperpx * s111
                + q1z * (qperpx * s001 + qperpy * s011 - qperpx * s101 - qperpy * s111)
                + float(2.0) * q1x * qperpx * s121
                + q1y * (qperpz * s011 - float(2.0) * qperpy * s021 + qperpw * (-s001 + s101)
                    - qperpz * s111 + float(2.0) * qperpy * s121)) * theta,
            kz: -float(2.0) * (-(q1w * qperpy * s002) + q1x * qperpz * s002 + q1x * qperpw * s012
                + q1w * qperpx * s012 - float(2.0) * q1x * qperpx * s022
                + q1w * qperpy * s102 - q1x * qperpz * s102
                - q1x * qperpw * s112 - q1w * qperpx * s112
                + q1z * (qperpx * s002 + qperpy * s012 - qperpx * s102 - qperpy * s112)
                + float(2.0) * q1x * qperpx * s122
                + q1y * (qperpz * s012 - float(2.0) * qperpy * s022 + qperpw * (-s002 + s102)
                    - qperpz * s112 + float(2.0) * qperpy * s122)) * theta,
        };
        let c4_2 = DerivativeTerm {
            kc: float(0.0),
            kx: q1w * qperpy * s000 - q1x * qperpz * s000 - q1x * qperpw * s010
                - q1w * qperpx * s010 + float(2.0) * q1x * qperpx * s020
                - q1w * qperpy * s100 + q1x * qperpz * s100
                + q1x * qperpw * s110 + q1w * qperpx * s110
                - float(2.0) * q1x * qperpx * s120
                - float(2.0) * qperpw * qperpy * s000 * theta
                + float(2.0) * qperpx * qperpz * s000 * theta
                - float(2.0) * q1w * q1x * s010 * theta
                + float(2.0) * qperpw * qperpx * s010 * theta
                + float(2.0) * qperpy * qperpz * s010 * theta
                + float(2.0) * q1x * q1x * s020 * theta
                + float(2.0) * q1y * q1y * s020 * theta
                - float(2.0) * qperpx * qperpx * s020 * theta
                - float(2.0) * qperpy * qperpy * s020 * theta
                + q1z * (-(qperpx * s000) - qperpy * s010 + qperpx * s100 + qperpy * s110
                    - float(2.0) * q1x * s000 * theta)
                + q1y * (-(qperpz * s010) + float(2.0) * qperpy * s020 + qperpw * (s000 - s100)
                    + qperpz * s110 - float(2.0) * qperpy * s120
                    + float(2.0) * q1w * s000 * theta
                    - float(2.0) * q1z * s010 * theta),
            ky: q1w * qperpy * s001 - q1x * qperpz * s001 - q1x * qperpw * s011
                - q1w * qperpx * s011 + float(2.0) * q1x * qperpx * s021
                - q1w * qperpy * s101 + q1x * qperpz * s101
                + q1x * qperpw * s111 + q1w * qperpx * s111
                - float(2.0) * q1x * qperpx * s121
                - float(2.0) * qperpw * qperpy * s001 * theta
                + float(2.0) * qperpx * qperpz * s001 * theta
                - float(2.0) * q1w * q1x * s011 * theta
                + float(2.0) * qperpw * qperpx * s011 * theta
                + float(2.0) * qperpy * qperpz * s011 * theta
                + float(2.0) * q1x * q1x * s021 * theta
                + float(2.0) * q1y * q1y * s021 * theta
                - float(2.0) * qperpx * qperpx * s021 * theta
                - float(2.0) * qperpy * qperpy * s021 * theta
                + q1z * (-(qperpx * s001) - qperpy * s011 + qperpx * s101 + qperpy * s111
                    - float(2.0) * q1x * s001 * theta)
                + q1y * (-(qperpz * s011) + float(2.0) * qperpy * s021 + qperpw * (s001 - s101)
                    + qperpz * s111 - float(2.0) * qperpy * s121
                    + float(2.0) * q1w * s001 * theta
                    - float(2.0) * q1z * s011 * theta),
            kz: q1w * qperpy * s002 - q1x * qperpz * s002 - q1x * qperpw * s012
                - q1w * qperpx * s012 + float(2.0) * q1x * qperpx * s022
                - q1w * qperpy * s102 + q1x * qperpz * s102
                + q1x * qperpw * s112 + q1w * qperpx * s112
                - float(2.0) * q1x * qperpx * s122
                - float(2.0) * qperpw * qperpy * s002 * theta
                + float(2.0) * qperpx * qperpz * s002 * theta
                - float(2.0) * q1w * q1x * s012 * theta
                + float(2.0) * qperpw * qperpx * s012 * theta
                + float(2.0) * qperpy * qperpz * s012 * theta
                + float(2.0) * q1x * q1x * s022 * theta
                + float(2.0) * q1y * q1y * s022 * theta
                - float(2.0) * qperpx * qperpx * s022 * theta
                - float(2.0) * qperpy * qperpy * s022 * theta
                + q1z * (-(qperpx * s002) - qperpy * s012 + qperpx * s102 + qperpy * s112
                    - float(2.0) * q1x * s002 * theta)
                + q1y * (-(qperpz * s012) + float(2.0) * qperpy * s022 + qperpw * (s002 - s102)
                    + qperpz * s112 - float(2.0) * qperpy * s122
                    + float(2.0) * q1w * s002 * theta
                    - float(2.0) * q1z * s012 * theta),
        };
        let c5_2 = DerivativeTerm {
            kc: float(0.0),
            kx: float(2.0) * (qperpw * qperpy * s000 - qperpx * qperpz * s000 + q1y * q1z * s010
                - qperpw * qperpx * s010 - qperpy * qperpz * s010
                - q1y * q1y * s020 + qperpx * qperpx * s020
                + qperpy * qperpy * s020 + q1x * q1z * (s000 - s100)
                - qperpw * qperpy * s100 + qperpx * qperpz * s100
                + q1w * (q1y * (-s000 + s100) + q1x * (s010 - s110))
                - q1y * q1z * s110 + qperpw * qperpx * s110
                + qperpy * qperpz * s110 + q1y * q1y * s120
                - qperpx * qperpx * s120 - qperpy * qperpy * s120
                + q1x * q1x * (-s020 + s120)) * theta,
            ky: float(2.0) * (qperpw * qperpy * s001 - qperpx * qperpz * s001 + q1y * q1z * s011
                - qperpw * qperpx * s011 - qperpy * qperpz * s011
                - q1y * q1y * s021 + qperpx * qperpx * s021
                + qperpy * qperpy * s021 + q1x * q1z * (s001 - s101)
                - qperpw * qperpy * s101 + qperpx * qperpz * s101
                + q1w * (q1y * (-s001 + s101) + q1x * (s011 - s111))
                - q1y * q1z * s111 + qperpw * qperpx * s111
                + qperpy * qperpz * s111 + q1y * q1y * s121
                - qperpx * qperpx * s121 - qperpy * qperpy * s121
                + q1x * q1x * (-s021 + s121)) * theta,
            kz: float(2.0) * (qperpw * qperpy * s002 - qperpx * qperpz * s002 + q1y * q1z * s012
                - qperpw * qperpx * s012 - qperpy * qperpz * s012
                - q1y * q1y * s022 + qperpx * qperpx * s022
                + qperpy * qperpy * s022 + q1x * q1z * (s002 - s102)
                - qperpw * qperpy * s102 + qperpx * qperpz * s102
                + q1w * (q1y * (-s002 + s102) + q1x * (s012 - s112))
                - q1y * q1z * s112 + qperpw * qperpx * s112
                + qperpy * qperpz * s112 + q1y * q1y * s122
                - qperpx * qperpx * s122 - qperpy * qperpy * s122
                + q1x * q1x * (-s022 + s122)) * theta,
        };

        Self {
            c1: [c1_0, c1_1, c1_2],
            c2: [c2_0, c2_1, c2_2],
            c3: [c3_0, c3_1, c3_2],
            c4: [c4_0, c4_1, c4_2],
            c5: [c5_0, c5_1, c5_2],
        }
    }
}
