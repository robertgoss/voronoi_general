use std::env;
use std::fs;

mod voronoi_points_2d;
mod draw;
mod scenario;

fn main() {
    let in_path = env::args().nth(1).unwrap_or("input.json".to_string());
    let out_path = env::args().nth(2).unwrap_or("output.svg".to_string());
    let in_file = fs::read_to_string(&in_path).expect(
        &format!("Could not read {}", &in_path)
    );
    let mut out_file = draw::SVG::new();
    let input = json::parse(&in_file).expect(
        &format!("Could not parse {}", &in_path)
    );
    println!("Parsed {}", &in_path);
    let voronoi = scenario::make_scenario(&input).expect(
        "Could not load / calculate"
    );
    voronoi.draw(&mut out_file);
    out_file.write(&out_path).expect(
        &format!("Could not write to {}", &out_path)
    )
}
