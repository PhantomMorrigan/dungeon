#![deny(missing_docs)]
//! A simple Game written in bevy

use bevy_hanabi::HanabiPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

use crate::prelude::*;

/// Utility Re-exports
pub mod prelude {
    pub use avian3d::prelude::*;
    pub use bevy::prelude::*;
    pub use serde::*;
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PanOrbitCameraPlugin,
            PhysicsPlugins::default(),
            HanabiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

/// System spawning initial camera and such
fn setup(mut commands: Commands) {
    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.5, 5.0)),
        PanOrbitCamera::default(),
    ));
}
