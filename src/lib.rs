use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::transform::TransformSystem;

pub mod prelude {
    pub use crate::GlobalTransform2;
    pub use crate::SpatialBundle2;
    pub use crate::SpriteBundle2;
    pub use crate::SpriteSheetBundle2;
    pub use crate::Transform2;
    pub use crate::Transform2Plugin;
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect, Component)]
#[reflect(Component, Default, PartialEq)]
pub struct Transform2 {
    pub translation: Vec2,
    pub depth: f32,
    pub rotation: f32,
    pub scale: Vec2,
}

impl Transform2 {
    pub const IDENTITY: Self = Self {
        translation: Vec2::ZERO,
        depth: 0.0,
        rotation: 0.0,
        scale: Vec2::ONE,
    };

    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_translation_depth(translation: Vec2, depth: f32) -> Self {
        Self {
            translation,
            depth,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_rotation(rotation: f32) -> Self {
        Self {
            rotation,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vec2) -> Self {
        Self {
            scale,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn from_depth(depth: f32) -> Self {
        Self {
            depth,
            ..Self::IDENTITY
        }
    }

    #[inline]
    #[must_use]
    pub const fn with_translation(mut self, translation: Vec2) -> Self {
        self.translation = translation;
        self
    }

    #[inline]
    #[must_use]
    pub const fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    #[inline]
    #[must_use]
    pub const fn with_depth(mut self, depth: f32) -> Self {
        self.depth = depth;
        self
    }

    #[inline]
    #[must_use]
    pub const fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for Transform2 {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl std::fmt::Display for Transform2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ T[{}, {}], D[{}], R[{}], S[{}, {}] }}",
            self.translation.x,
            self.translation.y,
            self.depth,
            self.rotation,
            self.scale.x,
            self.scale.y
        )
    }
}

impl From<&Transform2> for Transform {
    fn from(transform_2: &Transform2) -> Self {
        Self {
            translation: transform_2.translation.extend(transform_2.depth),
            rotation: Quat::from_rotation_z(-transform_2.rotation),
            scale: transform_2.scale.extend(1.0),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Reflect, Component)]
#[reflect(Component, Default, PartialEq)]
pub struct GlobalTransform2(Transform2);

impl GlobalTransform2 {
    #[must_use]
    #[inline]
    pub fn transform2(&self) -> &Transform2 {
        &self.0
    }

    #[must_use]
    #[inline]
    pub fn translation(&self) -> Vec2 {
        self.transform2().translation
    }

    #[must_use]
    #[inline]
    pub fn rotation(&self) -> f32 {
        self.transform2().rotation
    }

    #[must_use]
    #[inline]
    pub fn scale(&self) -> Vec2 {
        self.transform2().scale
    }
}

impl std::fmt::Display for GlobalTransform2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<&GlobalTransform> for GlobalTransform2 {
    fn from(gtf: &GlobalTransform) -> Self {
        let (scale, rotation, translation) = gtf.to_scale_rotation_translation();
        GlobalTransform2(Transform2 {
            translation: translation.xy(),
            depth: translation.z,
            rotation: -rotation.to_euler(EulerRot::ZYX).0,
            scale: scale.xy(),
        })
    }
}

#[derive(Default, Bundle)]
pub struct SpatialBundle2 {
    pub transform2: Transform2,
    #[bundle]
    pub spatial_bundle: SpatialBundle,
}

impl SpatialBundle2 {
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            transform2: Transform2::from_translation(translation),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Bundle)]
pub struct SpriteBundle2 {
    pub transform2: Transform2,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}

#[derive(Clone, Default, Bundle)]
pub struct SpriteSheetBundle2 {
    pub transform2: Transform2,
    #[bundle]
    pub sprite_sheet_bundle: SpriteSheetBundle,
}

fn update_transform_from_transform2(
    mut transforms: Query<(&Transform2, &mut Transform), Changed<Transform2>>,
) {
    transforms.for_each_mut(|(tf2, mut tf)| {
        *tf = tf2.into();
    })
}

fn update_global_transform_2(
    mut global_transforms: Query<
        (&GlobalTransform, &mut GlobalTransform2),
        Changed<GlobalTransform>,
    >,
) {
    global_transforms.for_each_mut(|(gtf, mut gtf2)| {
        *gtf2 = gtf.into();
    });
}

pub struct Transform2Plugin;

impl Plugin for Transform2Plugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Transform2>()
            .register_type::<GlobalTransform2>()
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                update_transform_from_transform2.before(TransformSystem::TransformPropagate),
            )
            .add_startup_system_to_stage(
                StartupStage::PostStartup,
                update_global_transform_2.after(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_transform_from_transform2.before(TransformSystem::TransformPropagate),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_global_transform_2.after(TransformSystem::TransformPropagate),
            );
    }
}
