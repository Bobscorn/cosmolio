use std::{error::Error, marker::PhantomData, fmt::Display};

use bevy::{asset::{AssetLoader, AsyncReadExt}, prelude::*};
use serde::Deserialize;

pub trait AssetExtensions
{
    fn extensions() -> &'static [&'static str];
}

#[derive(Default)]
pub struct RonAssetLoader<T: Asset + for<'b> Deserialize<'b> + AssetExtensions>
{
    pub real_t: PhantomData<T>,
}

#[derive(Debug)]
pub enum RonAssetLoadError
{
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
}

impl From<std::io::Error> for RonAssetLoadError
{
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for RonAssetLoadError
{
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl Display for RonAssetLoadError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            Self::Io(e) => f.write_fmt(format_args!("Io error: {}", e)),
            Self::Ron(e) => f.write_fmt(format_args!("Ron error: {}", e))
        }
    }
}

impl Error for RonAssetLoadError {}

impl<T: Asset + for<'b> Deserialize<'b> + AssetExtensions> AssetLoader for RonAssetLoader<T>
{
    type Asset = T;
    type Settings = ();
    type Error = RonAssetLoadError;

    fn extensions(&self) -> &[&str] {
        T::extensions()
    }

    fn load<'c>(
            &'c self,
            reader: &'c mut bevy::asset::io::Reader,
            _settings: &'c Self::Settings,
            _load_context: &'c mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'c, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<T>(&bytes)?.into();
            Ok(custom_asset)
        })
    }
}
