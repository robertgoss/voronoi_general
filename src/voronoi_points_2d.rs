use std::ops::Add;
use euclid::default::*;
use euclid::{point2, vec2};
use json::JsonValue;
use crate::draw::SVG;

#[derive(Clone)]
struct VoronoiEdge {
    point : Point2D<f64>,
    dir : Vector2D<f64>,
    min_dist : f64,
    min_t : Option<f64>,
    max_t : Option<f64>
}

enum TrimmedEdge {
    Keep,
    Filter,
    Trimmed(VoronoiEdge)
}

pub struct VoronoiGraph {
    source_points : Vec<Point2D<f64>>,
    edges : Vec<VoronoiEdge>
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
            source_points : Vec::new(),
            edges : Vec::new()
        }
    }
    fn add_source_point(&mut self, point : Point2D<f64>) {
        // Trim existing
        self.edges.retain_mut(
            |edge| match edge.trim(&point) {
                TrimmedEdge::Keep => true,
                TrimmedEdge::Filter => false,
                TrimmedEdge::Trimmed(new_edge) => {
                    *edge = new_edge;
                    true
                }
            }
        );
        // Add new edges
        for pt in &self.source_points {
            let mut edge = VoronoiEdge::two_points(pt, &point);
            if let Some(edge) = edge.trim_points(&self.source_points) {
                self.edges.push(edge);
            }
        }
        // Add the point
        self.source_points.push(point);
    }

    pub fn draw(&self, output : &mut SVG) {
        for pt in &self.source_points {
            output.draw_point(&pt)
        }
        for edge in &self.edges {
            edge.draw(output)
        }
    }
}


impl VoronoiEdge {
    fn two_points(pt1 : &Point2D<f64>, pt2 : &Point2D<f64>) -> VoronoiEdge {
        let dir : Vector2D<f64> = *pt2 - *pt1;
        VoronoiEdge {
            point: pt1.lerp(*pt2, 0.5),
            dir: vec2(dir.y, -dir.x),
            min_dist: pt1.distance_to(*pt2) * 0.5,
            min_t: None,
            max_t: None,
        }
    }

    fn trim_points(&self, points : &Vec<Point2D<f64>>) -> Option<VoronoiEdge> {
        let mut trimmed = Some(self.clone());
        for point in points {
            if let Some(edge) = &trimmed {
                match edge.trim(&point) {
                    TrimmedEdge::Keep => {},
                    TrimmedEdge::Filter => {
                        trimmed = None
                    },
                    TrimmedEdge::Trimmed(new_edge) => {
                        trimmed = Some(new_edge)
                    }
                }
            }
        }
        trimmed
    }

    fn trim(&self, point : &Point2D<f64>) -> TrimmedEdge {
        TrimmedEdge::Keep
    }


    fn start(&self) -> Option<Point2D<f64>> {
        self.min_t.map(
            |t| self.point + (self.dir * t)
        )
    }

    fn end(&self) -> Option<Point2D<f64>> {
        self.max_t.map(
            |t| self.point + (self.dir * t)
        )
    }

    fn draw(&self, output : &mut SVG) {
        if let Some(start) = self.start() {
            if let Some(end) = self.end() {
                output.draw_line(&start, &end, "red");
            } else {
                output.draw_ray(&start, &self.dir, "red");
            }
        } else {
            if let Some(end) = self.end() {
                let dir = -self.dir;
                output.draw_ray(&end, &dir, "red");
            } else {
                output.draw_ray(&self.point, &self.dir, "red");
                let dir = -self.dir;
                output.draw_ray(&self.point, &dir, "red");
            }
        }
    }
}