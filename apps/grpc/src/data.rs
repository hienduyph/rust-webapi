use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Feature {
    location: Location,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    latitude: i32,
    longitude: i32,
}

pub fn load() -> Vec<crate::routeguide::Feature> {
    let file = std::fs::File::open("db.json").expect("Failed t open data file");
    let decoded: Vec<Feature> =
        serde_json::from_reader(&file).expect("failed to deserialize features");
    decoded
        .into_iter()
        .map(|feature| crate::routeguide::Feature {
            name: feature.name,
            location: Some(crate::routeguide::Point {
                longitude: feature.location.longitude,
                latitude: feature.location.latitude,
            }),
        })
        .collect()
}
