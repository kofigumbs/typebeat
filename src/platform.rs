use std::future::Future;
use std::path::PathBuf;

use directories::UserDirs;
use rfd::AsyncFileDialog;

/// Tracks the last directory/file where the user saved/opened a .typebeat file
pub struct Location {
    directory: Option<PathBuf>,
    file_name: Option<String>,
}

impl From<PathBuf> for Location {
    fn from(value: PathBuf) -> Location {
        Location {
            directory: Some(value.parent().unwrap().to_path_buf()),
            file_name: Some(value.file_name().unwrap().to_string_lossy().to_string()),
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            directory: UserDirs::new().and_then(|x| x.audio_dir().map(PathBuf::from)),
            file_name: None,
        }
    }
}

impl Location {
    pub fn file_dialog(&self) -> AsyncFileDialog {
        let mut dialog = AsyncFileDialog::new().add_filter("typebeat", &["typebeat"]);
        if let Some(directory) = self.directory.as_ref() {
            dialog = dialog.set_directory(directory);
        }
        if let Some(file_name) = self.file_name.as_deref() {
            dialog = dialog.set_file_name(file_name);
        }
        dialog
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn save_file(location: &Location) -> impl Future<Output = Option<PathBuf>> {
    let task = location.file_dialog().save_file();
    async { task.await.map(|file| PathBuf::from(file.path())) }
}

/// Run Future in a scheduled Promise
#[cfg(target_arch = "wasm32")]
pub fn execute<F, T>(f: F)
where
    T: Future<Output = ()> + 'static,
    F: FnOnce() -> T,
{
    wasm_bindgen_futures::spawn_local(f());
}

/// Run Future in another thread
#[cfg(not(target_arch = "wasm32"))]
pub fn execute<F, T>(f: F)
where
    T: Future<Output = ()>,
    F: FnOnce() -> T + Send + 'static,
{
    std::thread::spawn(move || futures::executor::block_on(f()));
}
