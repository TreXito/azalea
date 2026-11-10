use azalea_buf::AzBuf;
use azalea_entity::particle::Particle;
use azalea_protocol_macros::ClientboundGamePacket;

#[derive(AzBuf, ClientboundGamePacket, Clone, Debug, PartialEq)]
pub struct ClientboundLevelParticles {
    /// 26.3 puts the particle options before all the spawn fields.
    pub particle: Particle,
    pub override_limiter: bool,
    pub always_show: bool,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub x_dist: f32,
    pub y_dist: f32,
    pub z_dist: f32,
    pub x_max_speed: f32,
    pub y_max_speed: f32,
    pub z_max_speed: f32,
    #[var]
    pub count: u32,
    pub randomization_type: RandomizationType,
}

#[derive(AzBuf, Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum RandomizationType {
    #[default]
    Default = 0,
    Alternative = 1,
    AlternativeWithSpeed = 2,
}
