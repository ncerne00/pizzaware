extern crate embed_resource;

fn main() {
    embed_resource::compile("src/resources/resources.rc", embed_resource::NONE).manifest_optional().unwrap();
}