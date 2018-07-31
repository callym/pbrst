use cgmath::prelude::*;
use shrinkwraprs::Shrinkwrap;

use crate::prelude::*;
use crate::math::*;
use crate::bxdf::{ Bsdf, TransportMode };
use crate::primitive::Primitive;
use crate::shape::Shape;
use super::{ BaseInteraction, Shading };

#[derive(Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SurfaceInteraction<'a> {
    #[shrinkwrap(main_field)]
    pub interaction: BaseInteraction,
    pub uv: Point2f,
    pub dudx: Float,
    pub dvdx: Float,
    pub dudy: Float,
    pub dvdy: Float,
    pub dpdx: Vector3f,
    pub dpdy: Vector3f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal,
    pub dndv: Normal,
    pub shape: Option<&'a dyn Shape>,
    pub primitive: Option<&'a dyn Primitive>,
    pub shading: Shading,
    pub bsdf: Option<Bsdf>,
    pub bssrdf: Option<()>,
}

impl SurfaceInteraction<'a> {
    #[cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]
    pub fn new(
        p: Point3f,
        p_err: Vector3f,
        uv: Point2f,
        wo: Vector3f,
        dpdu: Vector3f,
        dpdv: Vector3f,
        dndu: Normal,
        dndv: Normal,
        time: Float,
        shape: Option<&'a dyn Shape>,
        primitive: Option<&'a dyn Primitive>,
    ) -> Self {
        let mut n: Normal = dpdu.cross(dpdv).normalize().into();

        // adjust normal based on orientation & handiness
        if shape.map_or(false, |s| s.reverse_orientation() ^ s.transform_swaps_handedness()) {
            n = (-*n).into();
        }

        let interaction = BaseInteraction::new(p, n, p_err, wo, time, None);

        // init shading geom from true geom
        let shading = Shading {
            n,
            dpdu,
            dpdv,
            dndu,
            dndv,
        };

        Self {
            interaction,
            uv,
            dudx: float(0.0),
            dvdx: float(0.0),
            dudy: float(0.0),
            dvdy: float(0.0),
            dpdx: Vector3f::zero(),
            dpdy: Vector3f::zero(),
            dpdu,
            dpdv,
            dndu,
            dndv,
            shape,
            primitive,
            shading,
            bsdf: None,
            bssrdf: None,
        }
    }

    pub fn set_shading_geometry(&mut self, dpdus: Vector3f, dpdvs: Vector3f, dndus: Normal, dndvs: Normal, orientation_is_authoritative: bool) {
        // compute shading.n for Self
        let mut n: Normal = dpdus.cross(dpdvs).normalize().into();

        // adjust normal based on orientation & handiness
        if self.shape.as_ref().map_or(false, |s| s.reverse_orientation() ^ s.transform_swaps_handedness()) {
            *n *= float(-1.0);
        }

        if orientation_is_authoritative {
            self.shading.n = n.face_forward(self.shading.n);
        } else {
            self.shading.n = self.shading.n.face_forward(n);
        }

        // initialize shading partial derivative values
        self.shading.dpdu = dpdus;
        self.shading.dpdv = dpdvs;
        self.shading.dndu = dndus;
        self.shading.dndv = dndvs;
    }

    pub fn compute_scattering_functions(&mut self, ray: &RayDifferential, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) {
        self.compute_differentials(ray);

        match &self.primitive {
            Some(primitive) => {
                *self = primitive.compute_scattering_functions(self.clone(), arena, mode, allow_multiple_lobes);
            },
            None => (),
        }
    }

    pub fn compute_differentials(&mut self, ray: &RayDifferential) {
        if ray.has_differentials() {
            // these are guaranteed to be Some(_) because
            // of the ray.has_differentials() call.
            let rx = ray.x.unwrap();
            let ry = ray.y.unwrap();

            let n = self.n.unwrap();

            // we assume that the surface is locally flat in respect to the sampling rate
            // compute auxiliary intersection points with the surface plane
            let p = self.p.into_vector();
            let d = (*n).dot(p);

            let tx = -((*n).dot(rx.origin.into_vector()) - d) / (*n).dot(rx.direction);
            let px = rx.origin + rx.direction * tx;

            let ty = -((*n).dot(ry.origin.into_vector()) - d) / (*n).dot(ry.direction);
            let py = ry.origin + ry.direction * ty;

            self.dpdx = (px - p).into_vector();
            self.dpdy = (py - p).into_vector();

            // compute (u, v) offsets at auxiliary points
            // choose two dimensions to use for ray offset computation
            let mut dim = [Dim::X; 2];
            if n.x.abs() > n.y.abs() && n.x.abs() > n.z.abs() {
                dim[0] = Dim::Y;
                dim[1] = Dim::Z;
            } else if n.y.abs() > n.z.abs() {
                dim[0] = Dim::X;
                dim[1] = Dim::Z;
            } else {
                dim[0] = Dim::X;
                dim[1] = Dim::Y;
            }

            // init A, Bx, By matrices
            let a = [
                [ self.dpdu[dim[0] as usize], self.dpdv[dim[0] as usize] ],
                [ self.dpdu[dim[1] as usize], self.dpdv[dim[1] as usize] ],
            ];

            let bx = [
                px[dim[0] as usize] - p[dim[0] as usize],
                px[dim[1] as usize] - p[dim[1] as usize],
            ];

            let by = [
                py[dim[0] as usize] - p[dim[0] as usize],
                py[dim[1] as usize] - p[dim[1] as usize],
            ];

            if let Some((dudx, dvdx)) = solve_linear_system_2x2(a, bx) {
                self.dudx = dudx;
                self.dvdx = dvdx;
            } else {
                self.dudx = float(0.0);
                self.dvdx = float(0.0);
            }

            if let Some((dudy, dvdy)) = solve_linear_system_2x2(a, by) {
                self.dudy = dudy;
                self.dvdy = dvdy;
            } else {
                self.dudy = float(0.0);
                self.dvdy = float(0.0);
            }
        } else {
            self.dudx = float(0.0);
            self.dvdx = float(0.0);
            self.dudy = float(0.0);
            self.dvdy = float(0.0);
            self.dpdx = Vector3f::zero();
            self.dpdy = Vector3f::zero();
        }
    }

    pub fn le(&self, _ray: &Vector3f) -> Spectrum {
        if let Some(prim) = &self.primitive {
            if let Some(_area) = prim.get_area_light() {
                unimplemented!()
            }
        }

        Spectrum::new(0.0)
    }
}
