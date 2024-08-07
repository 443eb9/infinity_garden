//! Configs about the game, generally some constants including units, some presets etc.

use bevy::{
    asset::{Asset, AssetServer, Assets, Handle},
    log::info,
    prelude::{Commands, Res, ResMut, Resource, World},
};
use serde::de::DeserializeOwned;

use crate::assets::manifest::Manifest;

#[derive(Resource)]
pub struct RawConfigHandle<C: RawConfig> {
    pub handle: Handle<C>,
}

/// The config waiting for being processed before inserted as [`Self::Processed`].
/// Structs implements this trait should start with `Raw`, except:
///
/// It is also possible that [`Self::Processed`] is exactly [`Self`], which means
/// the config don't need process, and the struct name don't need to start with
/// `Raw`.
pub trait RawConfig: Asset + Clone + DeserializeOwned + Sized {
    type Processed: Resource + From<Self>;

    const PATH: &'static str;

    fn load(world: &mut World) {
        info!("Start loading config: {}", Self::PATH);

        let handle = world.resource::<AssetServer>().load::<Self>(Self::PATH);

        world.insert_resource(RawConfigHandle { handle });
    }

    fn process(&self) -> Self::Processed {
        (*self).clone().into()
    }
}

pub fn process_raw_config_when_finish_loading<C: RawConfig>(
    mut command: Commands,
    handle: Option<Res<RawConfigHandle<C>>>,
    assets: Res<Assets<C>>,
    mut manifest: ResMut<Manifest>,
) {
    if let Some(handle) = handle {
        if let Some(assets) = assets.get(&handle.handle) {
            command.insert_resource(assets.process());
            manifest.finish::<C>();
        }
    }
}
