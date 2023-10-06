use std::env;
use std::fs;

mod voronoi_points_2d;

fn main() {
    let in_path = env::args().nth(1).unwrap_or("input.json".to_string());
    let out_path = env::args().nth(2).unwrap_or("input.json".to_string());
    let in_file = fs::read_to_string(&in_path).expect(
        &format!("Could not read {}", &in_path)
    );
    let input = json::parse(&in_file).expect(
        &format!("Could not parse {}", &in_path)
    );
    println!("Parsed {}", &in_path);
    let scenario = input["scenario"].as_str().expect("Missing scenario key");
    if scenario != "voronoi_points_2d" {
        println!("Unknown scenario {}", scenario);
    }
    let voronoi = voronoi_points_2d::VoronoiGraph::from_json(&input);
    voronoi.draw(&out_path)
}
