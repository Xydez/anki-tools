#[derive(thiserror::Error, Debug)]
pub enum DbError {
    #[error("failed to extract apkg file")]
    ZipError(zip::result::ZipError),
    #[error("failed to decompress collection.anki21b")]
    DecompressError(std::io::Error),
}

/// Extract an `apkg` file; this uses the modern Anki format and will not work for older Anki
/// versions.
pub fn extract_archive<R>(reader: R) -> Result<Vec<u8>, DbError>
where
    R: std::io::Read + std::io::Seek,
{
    let mut archive = zip::read::ZipArchive::new(reader).map_err(DbError::ZipError)?;

    let collection_compressed = archive
        .by_name("collection.anki21b")
        .map_err(DbError::ZipError)?;

    let collection = {
        let mut dst = Vec::new();

        zstd::stream::copy_decode(collection_compressed, &mut dst)
            .map_err(DbError::DecompressError)?;

        dst
    };

    Ok(collection)
}
