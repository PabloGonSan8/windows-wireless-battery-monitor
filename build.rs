fn main() {
    // Icon + manifest (PerMonitorV2 DPI, Common Controls v6, Win10 supportedOS for layered child windows).
    embed_resource::compile("app.rc", embed_resource::NONE).manifest_required().unwrap();
}
