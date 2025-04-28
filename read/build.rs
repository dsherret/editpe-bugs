fn main() {
  embed_resource::compile("read-manifest.rc", embed_resource::NONE).manifest_optional().unwrap();
}