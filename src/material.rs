use crate::Vec3;

#[derive(Debug)]
pub struct Material {
    pub albedo: Vec3,
    pub specular: Vec3,
    pub specularity: f64,
    pub emissive: Vec3,
    pub roughness: f64,
    pub fresnel_0: f64,
    pub transparency: f64,
    pub n: f64,
}

impl Material {
    pub const DEFAULT_MAT: Material = Material {
        albedo: Vec3::ZERO,
        emissive: Vec3::ZERO,
        roughness: 0.2,
        specular: Vec3::ZERO,
        specularity: 0.0,
        fresnel_0: 0.70,
        transparency: 0.0,
        n: 1.3,
    };

    pub const MIRROR: Material = Material {
        albedo: Vec3::ONE,
        emissive: Vec3::ZERO,
        roughness: 0.0,
        specular: Vec3::ZERO,
        specularity: 0.0,
        fresnel_0: 1.0,
        transparency: 0.0,
        n: 0.0,
    };

    pub const GLOSSY: Material = Material {
        roughness: 0.5,
        ..Material::MIRROR
    };

    pub const DIFFUSE: Material = Material {
        roughness: 1.0,
        ..Material::MIRROR
    };

    pub const TOMATO: Material = Material {
        albedo: Vec3 {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        },
        specular: Vec3::ONE,
        specularity: 0.1,
        fresnel_0: 0.8,
        ..Material::DIFFUSE
    };

    pub const GLASS: Material = Material {
        fresnel_0: 0.0,
        transparency: 1.0,
        n: 1.2,
        ..Material::MIRROR
    };

    pub const FRESNEL_GLASS: Material = Material {
        fresnel_0: 0.1,
        ..Material::GLASS
    };

    pub const WHITE_LIGHT: Material = Material {
        albedo: Vec3::ZERO,
        specular: Vec3::ZERO,
        specularity: 0.0,
        emissive: Vec3 {
            x: 1.5,
            y: 1.5,
            z: 1.5,
        },
        roughness: 0.0,
        fresnel_0: 0.0,
        transparency: 0.0,
        n: 0.0,
    };

    pub const N_AIR: f64 = 1.0;

    pub fn default() -> &'static Material {
        &Material::DEFAULT_MAT
    }
}
