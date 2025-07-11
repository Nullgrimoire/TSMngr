fn main() {
    // Only embed resources on Windows
    if cfg!(target_os = "windows") {
        embed_resource::compile("tsmngr.rc", std::iter::empty::<&str>());
    }
}
