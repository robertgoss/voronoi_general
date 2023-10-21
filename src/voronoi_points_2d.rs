use euclid::default::*;
use euclid::{vec2};
use crate::draw::SVG;

type UnitVector2D<S> = Vector2D<S>;

type PointId = u32;

#[derive(Clone)]
struct VoronoiEdge {
    point : Point2D<f64>,
    dir : UnitVector2D<f64>,
    min_dist : f64,
    points : [PointId; 2],
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
    pub fn new() -> VoronoiGraph {
        VoronoiGraph {
            source_points : Vec::new(),
            edges : Vec::new()
        }
    }
    pub fn add_source_point(&mut self, point : Point2D<f64>) {
        let new_id = self.source_points.len() as PointId;
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
        for (id, pt) in self.source_points.iter().enumerate() {
            let edge = VoronoiEdge::two_points(
                pt,
                &point,
                id as PointId,
                new_id
            );
            if let Some(trimmed_edge) = edge.trim_points(&self.source_points) {
                self.edges.push(trimmed_edge);
            }
        }
        // Add the point
        self.source_points.push(point);
    }

    pub fn draw(&self, output : &mut SVG) {
        for pt in &self.source_points {
            output.draw_point(&pt, "black")
        }
        for edge in &self.edges {
            edge.draw(output)
        }
    }
}


impl VoronoiEdge {
    fn two_points(pt1: &Point2D<f64>, pt2: &Point2D<f64>, id1: PointId, id2: PointId) -> VoronoiEdge {
        let dir: Vector2D<f64> = (*pt2 - *pt1).normalize();
        VoronoiEdge {
            point: pt1.lerp(*pt2, 0.5),
            dir: vec2(dir.y, -dir.x),
            points: [id1, id2],
            min_dist: pt1.distance_to(*pt2) * 0.5,
            min_t: None,
            max_t: None,
        }
    }

    fn trim_points(&self, points: &Vec<Point2D<f64>>) -> Option<VoronoiEdge> {
        let mut trimmed = Some(self.clone());
        for (index, point) in points.iter().enumerate() {
            let id = index as PointId;
            if self.points[0] == id || self.points[1] == id {
                continue
            }
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

    fn trim(&self, point: &Point2D<f64>) -> TrimmedEdge {
        let diff = *point - self.point;
        let t_nearest = self.dir.dot(diff);
        let t_nearest_sq = t_nearest * t_nearest;
        let perp_dir = vec2(self.dir.y, -self.dir.x);
        let d_nearest = perp_dir.dot(diff);
        let d_nearest_sq = d_nearest * d_nearest;
        let min_dist_sq = self.min_dist * self.min_dist;
        let cut_t = (t_nearest_sq + d_nearest_sq - min_dist_sq) / (2.0 * t_nearest);
        if t_nearest > 0.0 {
            self.cut_max(cut_t)
        } else {
            self.cut_min(cut_t)
        }
    }

    fn cut_max(&self, t: f64) -> TrimmedEdge {
        if let Some(min_t) = self.min_t {
            if t <= min_t {
                return TrimmedEdge::Filter
            }
        }
        if let Some(max_t) = self.max_t {
            if t >= max_t {
                return TrimmedEdge::Keep
            }
        }
        let mut edge = self.clone();
        edge.max_t = Some(t);
        TrimmedEdge::Trimmed(edge)
    }

    fn cut_min(&self, t: f64) -> TrimmedEdge {
        if let Some(max_t) = self.max_t {
            if t >= max_t {
                return TrimmedEdge::Filter
            }
        }
        if let Some(min_t) = self.min_t {
            if t <= min_t {
                return TrimmedEdge::Keep
            }
        }
        let mut edge = self.clone();
        edge.min_t = Some(t);
        TrimmedEdge::Trimmed(edge)
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

    fn draw(&self, output: &mut SVG) {
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