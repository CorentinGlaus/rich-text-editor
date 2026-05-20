#[macro_export]
macro_rules! texture_bytes {
    ($path:expr) => {
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/assets/textures/",
            $path
        ))
    };
}

// TODO: Remove the decoding from the hot path. Move it to a different thread.
#[macro_export]
macro_rules! get_image_bytes {
    ($bytes:expr) => {{
        let image = image::load_from_memory($bytes).expect("Error loading image");

        let diffuse_rgba = image.to_rgba8();
        let dimensions = (image.width(), image.height());
        (diffuse_rgba, dimensions)
    }};
}
