use euclid::point2;
use json::JsonValue;
use rand_chacha::ChaCha8Rng;
use rand::prelude::*;
use crate::voronoi_points_2d::VoronoiGraph;

pub fn make_scenario(input : &JsonValue) -> Option<VoronoiGraph> {
    let name = input["scenario"].as_str()?;
    match name {
        "points" => points_from_json(input),
        "random" => random_from_json(input),
        _ => None
    }
}

fn points_from_json(input: &JsonValue) -> Option<VoronoiGraph> {
    let points = &input["points"];
    if !points.is_array() {
        return None
    }
    let mut graph = VoronoiGraph::new();
    for point in points.members() {
        if !point.is_array() || point.len() != 2 {
            return None
        }
        let pt = point2(
            point[0].as_i32().unwrap_or(0),
            point[1].as_i32().unwrap_or(0)
        );
        graph.add_source_point(pt);
    }
    Some(graph)
}

fn random_from_json(input: &JsonValue) -> Option<VoronoiGraph> {
    let num = input["number"].as_usize()?;
    let seed = input["seed"].as_u64().unwrap_or(0);
    let mut graph = VoronoiGraph::new();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    for _ in 0..num {
        let i : i32 = rng.gen();
        let j : i32 = rng.gen();
        let x = (i % 100);
        let y = (j % 100);
        graph.add_source_point(point2(x, y));
    }
    Some(graph)
}