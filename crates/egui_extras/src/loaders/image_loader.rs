use alloc::{rc::Rc, string::String};
use core::{cell::RefCell, mem::size_of};
use egui::{
    load::{BytesPoll, ImageLoadResult, ImageLoader, ImagePoll, LoadError, SizeHint},
    ColorImage,
};
use hashbrown::HashMap;

type Entry = Result<Rc<ColorImage>, String>;

#[derive(Default)]
pub struct ImageCrateLoader {
    cache: RefCell<HashMap<String, Entry>>,
}

impl ImageCrateLoader {
    pub const ID: &'static str = egui::generate_loader_id!(ImageCrateLoader);
}

fn is_unsupported_mime(mime: &str) -> bool {
    // TODO(emilk): use https://github.com/image-rs/image/pull/2038 when new `image` crate is released.
    mime.contains("svg")
}

impl ImageLoader for ImageCrateLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, ctx: &egui::Context, uri: &str, _: SizeHint) -> ImageLoadResult {
        // three stages of guessing if we support loading the image:
        // 1. URI extension
        // 2. Mime from `BytesPoll::Ready`
        // 3. image::guess_format

        let mut cache = self.cache.borrow_mut();
        if let Some(entry) = cache.get(uri).cloned() {
            match entry {
                Ok(image) => Ok(ImagePoll::Ready { image }),
                Err(err) => Err(LoadError::Loading(err)),
            }
        } else {
            match ctx.try_load_bytes(uri) {
                Ok(BytesPoll::Ready { bytes, mime, .. }) => {
                    // (2 and 3)
                    if mime.as_deref().is_some_and(is_unsupported_mime)
                        || image::guess_format(&bytes).is_err()
                    {
                        return Err(LoadError::NotSupported);
                    }

                    let result = crate::image::load_image_bytes(&bytes).map(Rc::new);
                    cache.insert(uri.into(), result.clone());
                    match result {
                        Ok(image) => Ok(ImagePoll::Ready { image }),
                        Err(err) => Err(LoadError::Loading(err)),
                    }
                }
                Ok(BytesPoll::Pending { size }) => Ok(ImagePoll::Pending { size }),
                Err(err) => Err(err),
            }
        }
    }

    fn forget(&self, uri: &str) {
        let _ = self.cache.borrow_mut().remove(uri);
    }

    fn forget_all(&self) {
        self.cache.borrow_mut().clear();
    }

    fn byte_size(&self) -> usize {
        self.cache
            .borrow()
            .values()
            .map(|result| match result {
                Ok(image) => image.pixels.len() * size_of::<egui::Color32>(),
                Err(err) => err.len(),
            })
            .sum()
    }
}
