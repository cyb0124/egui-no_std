use alloc::{borrow::ToOwned, rc::Rc, string::String};
use core::{cell::RefCell, mem::size_of};
use egui::{
    load::{BytesPoll, ImageLoadResult, ImageLoader, ImagePoll, LoadError, SizeHint},
    ColorImage,
};
use hashbrown::HashMap;

type Entry = Result<Rc<ColorImage>, String>;

#[derive(Default)]
pub struct SvgLoader {
    cache: RefCell<HashMap<(String, SizeHint), Entry>>,
}

impl SvgLoader {
    pub const ID: &'static str = egui::generate_loader_id!(SvgLoader);
}

impl ImageLoader for SvgLoader {
    fn id(&self) -> &str {
        Self::ID
    }

    fn load(&self, ctx: &egui::Context, uri: &str, size_hint: SizeHint) -> ImageLoadResult {
        let uri = uri.to_owned();

        let mut cache = self.cache.borrow_mut();
        // We can't avoid the `uri` clone here without unsafe code.
        if let Some(entry) = cache.get(&(uri.clone(), size_hint)).cloned() {
            match entry {
                Ok(image) => Ok(ImagePoll::Ready { image }),
                Err(err) => Err(LoadError::Loading(err)),
            }
        } else {
            match ctx.try_load_bytes(&uri) {
                Ok(BytesPoll::Ready { bytes, .. }) => {
                    let result = crate::image::load_svg_bytes_with_size(&bytes, Some(size_hint))
                        .map(Rc::new);
                    cache.insert((uri, size_hint), result.clone());
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
        self.cache.borrow_mut().retain(|(u, _), _| u != uri);
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
