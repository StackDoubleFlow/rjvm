use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use zip::ZipArchive;

struct JarFile {
    archive: ZipArchive<Cursor<Vec<u8>>>,
    manifest: JarManifest,
}

struct JarManifest {}

impl JarFile {
    fn read(path: PathBuf) -> JarFile {
        let data = fs::read(path).unwrap();
        let archive = ZipArchive::new(Cursor::new(data)).unwrap();

        JarFile {
            archive,
            manifest: JarManifest {},
        }
    }
}
