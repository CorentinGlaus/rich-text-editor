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
