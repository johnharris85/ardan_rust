use openapiv3::OpenAPI;

fn main() {
    let data = include_str!("openapi.json");
    let openapi: OpenAPI = serde_json::from_str(data).expect("Could not deserialize input");
    //println!("{:?}", openapi);
    for (path_name, path_item) in openapi.paths.paths {
        println!("{path_name:?}");
    };
}