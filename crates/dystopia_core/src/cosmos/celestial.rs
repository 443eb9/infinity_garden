use bevy::{
    math::DVec2,
    prelude::{Component, Deref, DerefMut, Resource},
};
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct ShowOrbits(bool);

#[derive(Resource)]
pub struct Cosmos {
    pub bodies: Vec<CelestialBodyData>,
    pub orbits: Vec<Orbit>,
}

/// The index of this body in cosmos.
///
/// You can fetch the detailed data using this index.
#[derive(Component, Debug, Default, Deref)]
pub struct BodyIndex(usize);

impl BodyIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

/// The index of this orbit in cosmos.
///
/// You can fetch the detailed data using this index.
#[derive(Component, Debug, Default, Deref)]
pub struct OrbitIndex(usize);

impl OrbitIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
}

/// Marker struct for stars.
#[derive(Component, Debug, Default)]
pub struct Star;

/// Marker struct for planets.
#[derive(Component, Debug, Default)]
pub struct Planet;

// Marker struct for moons.
#[derive(Component, Debug, Default)]
pub struct Moon;

/// All celestial dynamic data for a body.
#[derive(Debug)]
pub struct CelestialBodyData {
    pub pos: DVec2,
    pub mass: f64,
    pub radius: f64,
}

/// The type of a main sequence star.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarType {
    #[default]
    O,
    B,
    A,
    F,
    G,
    K,
    M,
}

/// Detailed class of a main sequence star.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StarClass {
    pub ty: StarType,
    pub sub_ty: u32,
    pub index: u32,
}

/// The type of a main sequence star.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BodyType {
    #[default]
    Rocky,
    GasGiant,
    IceGiant,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Orbit {
    pub initial_progress: f64,
    pub center_id: usize,
    pub center: DVec2,
    pub radius: f64,
    pub sidereal_period: u64,
    pub rotation_period: u64,
}
