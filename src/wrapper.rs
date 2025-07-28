use asset_importer_rs_core::{
    AiExport, AiImporter, AiImporterInfo, DataExporter, DataLoader, ExportProperties,
};
use asset_importer_rs_scene::AiScene;
use std::path::Path;

use crate::error::{AiExporterError, AiImporterError};

pub struct AiImportWrapper<T: AiImporter>
where
    T::Error: Into<AiImporterError>,
{
    inner: T,
}

impl<T: AiImporter> AiImportWrapper<T>
where
    T::Error: Into<AiImporterError>,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: AiImporter> AiImporterInfo for AiImportWrapper<T>
where
    T::Error: Into<AiImporterError>,
{
    fn info(&self) -> asset_importer_rs_core::AiImporterDesc {
        self.inner.info()
    }
}

impl<T: AiImporter> AiImporter for AiImportWrapper<T>
where
    T::Error: Into<AiImporterError>,
{
    type Error = AiImporterError;

    fn can_read_dyn<'data>(&self, path: &Path, loader: &DataLoader<'_>) -> bool {
        self.inner.can_read_dyn(path, loader)
    }

    fn read_file_dyn(&self, path: &Path, loader: &DataLoader<'_>) -> Result<AiScene, Self::Error> {
        self.inner.read_file_dyn(path, loader).map_err(Into::into)
    }
}

pub struct AiExportWrapper<T: AiExport>
where
    T::Error: Into<AiExporterError>,
{
    inner: T,
}

impl<T: AiExport> AiExportWrapper<T>
where
    T::Error: Into<AiExporterError>,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }

    pub fn into_inner(self) -> T {
        self.inner
    }

    pub fn inner(&self) -> &T {
        &self.inner
    }

    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: AiExport> AiExport for AiExportWrapper<T>
where
    T::Error: Into<AiExporterError>,
{
    type Error = AiExporterError;

    fn export_file_dyn(
        &self,
        scene: &AiScene,
        path: &Path,
        properties: &ExportProperties,
        exporter: &DataExporter<'_>,
    ) -> Result<(), Self::Error> {
        self.inner
            .export_file_dyn(scene, path, properties, exporter)
            .map_err(Into::into)
    }
}
