use std::path::PathBuf;

const EXTENSIONS: [&str; 6] = ["heic", "jpeg", "jpg", "jxl", "png", "webp"];

pub trait ImageManipulation {
    fn is_image(&self) -> Option<PathBuf>;
}

impl ImageManipulation for PathBuf {
    fn is_image(&self) -> Option<PathBuf> {
        let extensions = EXTENSIONS;
        for extension in extensions {
            let r = match self.extension() {
                Some(v) => v.eq(extension),
                None => false 
            };

            if r {
                return Some(self.to_owned());
            }
        }

        None
    }
}