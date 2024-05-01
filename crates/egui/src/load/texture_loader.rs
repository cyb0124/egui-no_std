use super::*;

#[derive(Default)]
pub struct DefaultTextureLoader {
    cache: RefCell<HashMap<(String, TextureOptions), TextureHandle>>,
}

impl TextureLoader for DefaultTextureLoader {
    fn id(&self) -> &str {
        crate::generate_loader_id!(DefaultTextureLoader)
    }

    fn load(
        &self,
        ctx: &Context,
        uri: &str,
        texture_options: TextureOptions,
        size_hint: SizeHint,
    ) -> TextureLoadResult {
        let mut cache = self.cache.borrow_mut();
        if let Some(handle) = cache.get(&(uri.into(), texture_options)) {
            let texture = SizedTexture::from_handle(handle);
            Ok(TexturePoll::Ready { texture })
        } else {
            match ctx.try_load_image(uri, size_hint)? {
                ImagePoll::Pending { size } => Ok(TexturePoll::Pending { size }),
                ImagePoll::Ready { image } => {
                    let handle = ctx.load_texture(uri, image, texture_options);
                    let texture = SizedTexture::from_handle(&handle);
                    cache.insert((uri.into(), texture_options), handle);
                    Ok(TexturePoll::Ready { texture })
                }
            }
        }
    }

    fn forget(&self, uri: &str) {
        self.cache.borrow_mut().retain(|(u, _), _| u != uri);
    }

    fn forget_all(&self) {
        self.cache.borrow_mut().clear();
    }

    fn end_frame(&self, _: usize) {}

    fn byte_size(&self) -> usize {
        self.cache
            .borrow()
            .values()
            .map(|texture| texture.byte_size())
            .sum()
    }
}
