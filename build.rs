fn main() {
    // Ensure openapi directory exists
    let openapi_path = std::path::Path::new("openapi/nomba_openapi.json");
    if !openapi_path.exists() {
        panic!("OpenAPI spec not found at {:?}", openapi_path);
    }

    // Tell cargo to rerun if the OpenAPI spec changes
    println!("cargo:rerun-if-changed=openapi/nomba_openapi.json");
}
