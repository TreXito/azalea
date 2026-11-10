use std::io::Cursor;

use azalea_entity::particle::Particle;
use azalea_protocol::{
    packets::game::{ClientboundGamePacket, c_level_particles::RandomizationType},
    read::deserialize_packet,
};

#[test]
fn mc26_3_level_particles_particle_is_written_first() {
    // Packet 48, followed by Mojang's 26.3 field order:
    // particle, two booleans, doubles, six floats, count, randomization type.
    let mut raw = vec![48, 0, 1, 0];
    for value in [1.5_f64, 2.5, 3.5] {
        raw.extend_from_slice(&value.to_be_bytes());
    }
    for value in [0.1_f32, 0.2, 0.3, 0.4, 0.5, 0.6] {
        raw.extend_from_slice(&value.to_be_bytes());
    }
    raw.push(7);
    raw.push(2);

    let packet = deserialize_packet::<ClientboundGamePacket>(&mut Cursor::new(raw.as_slice()))
        .expect("26.3 particle packet should decode");
    let p = match packet {
        ClientboundGamePacket::LevelParticles(p) => p,
        other => panic!("expected level particles, got {other:?}"),
    };

    assert_eq!(p.particle, Particle::AngryVillager);
    assert!(p.override_limiter);
    assert!(!p.always_show);
    assert_eq!((p.x, p.y, p.z), (1.5, 2.5, 3.5));
    assert_eq!(
        (
            p.x_dist,
            p.y_dist,
            p.z_dist,
            p.x_max_speed,
            p.y_max_speed,
            p.z_max_speed,
        ),
        (0.1, 0.2, 0.3, 0.4, 0.5, 0.6)
    );
    assert_eq!(p.count, 7);
    assert_eq!(
        p.randomization_type,
        RandomizationType::AlternativeWithSpeed
    );
}
