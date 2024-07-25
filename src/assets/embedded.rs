//! Simplified version of https://github.com/vleue/bevy_embedded_assets

use std::{
    io::Read,
    path::{Path, PathBuf},
    pin::Pin,
    task::Poll,
};

use bevy::{
    asset::io::{
        AssetReader,
        AssetReaderError,
        AssetSource,
        AssetSourceId,
        ErasedAssetReader,
        PathStream,
        Reader,
    },
    prelude::*,
    tasks::futures_lite::{AsyncRead, AsyncSeek, Stream},
    utils::HashMap,
};
use include_dir::{include_dir, Dir};

const ASSET_DIR: Dir = include_dir!("assets");

pub(crate) fn plugin(app: &mut App) {
    if app.is_plugin_added::<AssetPlugin>() {
        error!("The embedded asset plugin must come before bevy's AssetPlugin");
    }
    app.register_asset_source(
        AssetSourceId::Default,
        AssetSource::build().with_reader(move || {
            Box::new(EmbeddedAssetReader::new(AssetSource::get_default_reader(
                ASSET_DIR.path().to_str().unwrap_or("assets").into(),
            )))
        }),
    );
}

/// A wrapper around the raw bytes of an asset
#[derive(Default, Debug, Clone, Copy)]
pub struct DataReader(pub &'static [u8]);

impl AsyncRead for DataReader {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> Poll<std::io::Result<usize>> {
        let read = self.get_mut().0.read(buf);
        Poll::Ready(read)
    }
}

impl AsyncSeek for DataReader {
    fn poll_seek(
        self: Pin<&mut Self>,
        _: &mut std::task::Context<'_>,
        _pos: std::io::SeekFrom,
    ) -> Poll<std::io::Result<u64>> {
        Poll::Ready(Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Seek not supported",
        )))
    }
}

/// A wrapper around directories to read them
struct DirReader(Vec<PathBuf>);

impl Stream for DirReader {
    type Item = PathBuf;

    fn poll_next(
        self: Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        Poll::Ready(this.0.pop())
    }
}

/// A custom asset reader to search embedded assets first (and fall back to the
/// asset folder otherwise)
struct EmbeddedAssetReader {
    loaded: HashMap<&'static Path, &'static [u8]>,
    fallback: Box<dyn ErasedAssetReader>,
}

impl EmbeddedAssetReader {
    fn new(
        mut fallback: impl FnMut() -> Box<dyn ErasedAssetReader> + Send + Sync + 'static,
    ) -> Self {
        let mut reader = Self {
            loaded: HashMap::default(),
            fallback: fallback(),
        };
        // Preload all files in the asset directory
        load_assets_rec(&ASSET_DIR, &mut reader);
        reader
    }
}

fn load_assets_rec(dir: &'static Dir, reader: &mut EmbeddedAssetReader) {
    for file in dir.files() {
        debug!("Embedding asset: '{}'", file.path().display());
        reader.loaded.insert(file.path(), file.contents());
    }
    for dir in dir.dirs() {
        load_assets_rec(dir, reader);
    }
}

// Here we implement the `AssetReader` trait from bevy, which lets us switch the
// default reader for our own, automating the handling of the embedded://
// namespace and allowing us to use the same code regardless of where the method
impl AssetReader for EmbeddedAssetReader {
    async fn read<'a>(&'a self, path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        if ASSET_DIR.contains(path) {
            return self
                .loaded
                .get(path)
                .map(|b| -> Box<Reader> { Box::new(DataReader(b)) })
                .ok_or_else(|| AssetReaderError::NotFound(path.to_path_buf()));
        }
        warn!("Asset read failed for '{}', using fallback", path.display());
        self.fallback.read(path).await
    }

    async fn read_meta<'a>(&'a self, path: &'a Path) -> Result<Box<Reader<'a>>, AssetReaderError> {
        let meta_path = path.to_path_buf().with_added_extension(".meta");
        if ASSET_DIR.contains(&meta_path) {
            return self
                .loaded
                .get(&*meta_path)
                .map(|b| -> Box<Reader> { Box::new(DataReader(b)) })
                .ok_or_else(|| AssetReaderError::NotFound(path.to_path_buf()));
        }
        self.fallback.read(path).await
    }

    async fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> Result<Box<PathStream>, AssetReaderError> {
        if ASSET_DIR.contains(path) {
            let paths: Vec<_> = self
                .loaded
                .keys()
                .filter(|p| p.starts_with(path))
                .map(|t| t.to_path_buf())
                .collect();
            return Ok(Box::new(DirReader(paths)));
        }
        warn!("Dir read failed for '{}', using fallback", path.display());
        self.fallback.read_directory(path).await
    }

    async fn is_directory<'a>(&'a self, path: &'a Path) -> Result<bool, AssetReaderError> {
        let base = path.join("");
        Ok(self
            .loaded
            .keys()
            .any(|p| p.starts_with(&base) && p != &path))
    }
}
