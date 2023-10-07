use euclid::default::*;
use euclid::point2;
use json::JsonValue;
use crate::draw::SVG;

pub struct VoronoiGraph {
    source_points : Vec<Point2D<f64>>
}

impl VoronoiGraph {
    pub fn from_json(input : &JsonValue) -> Option<VoronoiGraph> {
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
                point[0].as_f64().unwrap_or(0.0),
                point[1].as_f64().unwrap_or(0.0)
            );
            graph.add_source_point(pt);
        }
        Some(graph)
    }

    fn new() -> VoronoiGraph {
        VoronoiGraph {
            source_points : Vec::new()
        }
    }

    fn add_source_point(&mut self, point : Point2D<f64>) {
        self.source_points.push(point);
    }

    pub fn draw(&self, output : &mut SVG) {
        for pt in &self.source_points {
            output.draw_point(pt)
        }
    }
}