use std::fs::{self, File};
use std::io::{self};
use zip::write::FileOptions;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let base_path: &Path = Path::new("C:\\Windows\\System32\\winevt\\Logs");
    let zip_file: File = File::create("windows_events.zip")?;
    let mut zip: zip::ZipWriter<File> = zip::ZipWriter::new(zip_file);

    let options: FileOptions = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    match fs::read_dir(base_path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path: PathBuf = entry.path();
                        if path.is_file() {
                            let name: &str = path.strip_prefix(base_path)
                                .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("Error en strip_prefix: {}", e)))?
                                .to_str()
                                .unwrap();
                            
                            zip.start_file(name, options)?;
                            let mut f: File = File::open(&path)?;
                            io::copy(&mut f, &mut zip)?;
                        }
                    }
                    Err(e) => eprintln!("Error reading entry: {}", e),
                }
            }
        }
        Err(e) => eprintln!("Error reading directory: {}", e),
    }
    zip.finish()?;
    Ok(())
}
