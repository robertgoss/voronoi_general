use euclid::default::*;
use euclid::vec2;
use crate::draw::SVG;
use num_rational::Ratio;

type PointId = u32;

fn int_dot(v1 : Vector2D<i32>, v2 : Vector2D<i32>) -> i64 {
    (v1.x as i64 * v2.x as i64) + (v1.y as i64 * v2.y as i64)
}

#[derive(Clone)]
struct VoronoiEdge {
    double_point : Point2D<i32>,
    mul_dir : Vector2D<i32>,
    points : [PointId; 2],
    min_t : Option<Ratio<i64>>,
    max_t : Option<Ratio<i64>>
}

enum TrimmedEdge {
    Keep,
    Filter,
    Trimmed(VoronoiEdge)
}

pub struct VoronoiGraph {
    source_points : Vec<Point2D<i32>>,
    edges : Vec<VoronoiEdge>
}

impl VoronoiGraph {
    pub fn new() -> VoronoiGraph {
        VoronoiGraph {
            source_points : Vec::new(),
            edges : Vec::new()
        }
    }
    pub fn add_source_point(&mut self, point : Point2D<i32>) {
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
            output.draw_point(&pt.cast::<f64>(), "black")
        }
        for edge in &self.edges {
            edge.draw(output)
        }
    }
}


impl VoronoiEdge {
    fn two_points(pt1: &Point2D<i32>, pt2: &Point2D<i32>, id1: PointId, id2: PointId) -> VoronoiEdge {
        let dir: Vector2D<i32> = *pt2 - *pt1;
        VoronoiEdge {
            double_point: *pt1 + pt2.to_vector(),
            mul_dir: vec2(dir.y, -dir.x),
            points: [id1, id2],
            min_t: None,
            max_t: None,
        }
    }

    fn trim_points(&self, points: &Vec<Point2D<i32>>) -> Option<VoronoiEdge> {
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

    fn trim(&self, point: &Point2D<i32>) -> TrimmedEdge {
        let diff = (*point * 2) - self.double_point;
        let t_nearest = int_dot(self.mul_dir, diff);
        let t_nearest_sq = t_nearest * t_nearest;
        let mul_perp_dir = vec2(self.mul_dir.y, -self.mul_dir.x);
        let d_nearest = int_dot(mul_perp_dir,diff);
        let d_nearest_sq = d_nearest * d_nearest;
        let min_dist_sq = int_dot(self.mul_dir,self.mul_dir);
        let min_dist_quart = min_dist_sq * min_dist_sq;
        if t_nearest == 0 {
            if d_nearest_sq > min_dist_sq {
                return TrimmedEdge::Keep
            } else {
                return TrimmedEdge::Filter
            }
        }
        let cut_t = Ratio::new(
            t_nearest_sq + d_nearest_sq - min_dist_quart,
            4 * t_nearest * min_dist_sq
        );
        if t_nearest > 0 {
            self.cut_max(cut_t)
        } else {
            self.cut_min(cut_t)
        }
    }

    fn cut_max(&self, t: Ratio<i64>) -> TrimmedEdge {
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

    fn cut_min(&self, t: Ratio<i64>) -> TrimmedEdge {
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
        let point = self.double_point.cast::<f64>() * 0.5;
        let dir = self.mul_dir.cast::<f64>();
        self.min_t.map(
            |t| point + (dir * (*t.numer() as f64 / *t.denom() as f64))
        )
    }

    fn end(&self) -> Option<Point2D<f64>> {
        let point = self.double_point.cast::<f64>() * 0.5;
        let dir = self.mul_dir.cast::<f64>();
        self.max_t.map(
            |t| point + (dir * (*t.numer() as f64 / *t.denom() as f64))
        )
    }

    fn draw(&self, output: &mut SVG) {
        let dir = self.mul_dir.cast::<f64>();
        let norm_dir = dir.normalize();
        if let Some(start) = self.start() {
            if let Some(end) = self.end() {
                output.draw_line(&start, &end, "red");
            } else {
                output.draw_ray(&start, &norm_dir, "red");
            }
        } else {
            if let Some(end) = self.end() {
                let inv_norm_dir = -norm_dir;
                output.draw_ray(&end, &inv_norm_dir, "red");
            } else {
                let point = self.double_point.cast::<f64>() * 0.5;
                output.draw_ray(&point, &norm_dir, "red");
                let inv_norm_dir = -norm_dir;
                output.draw_ray(&point, &inv_norm_dir, "red");
            }
        }
    }
}











#[cfg(test)]
mod tests {
    use euclid::point2;
    use super::*;

    #[test]
    fn test_basic_trim() {
        let p1 : Point2D<i32> = point2(-10, 0);
        let p2 : Point2D<i32> = point2(0, 0);
        let p3 : Point2D<i32> = point2(5, 5);

        // Test the 3 possible lines trimmed against the third point are correct
        let e12 = VoronoiEdge::two_points(&p1, &p2, 1, 2);
        let tr12 = e12.trim(&p3);
        if let TrimmedEdge::Trimmed(te12) = tr12 {
            assert!(te12.min_t.is_some());
            assert!(te12.max_t.is_none());
            let v = te12.min_t.unwrap();
            assert_eq!(v, Ratio::new(-1,1));
        } else {
            // Should have trimmed
            assert!(false);
        }
        let e13 = VoronoiEdge::two_points(&p1, &p3, 1, 3);
        let tr13 = e13.trim(&p2);
        if let TrimmedEdge::Trimmed(te13) = tr13 {
            assert!(te13.min_t.is_none());
            assert!(te13.max_t.is_some());
            let v = te13.max_t.unwrap();
            assert_eq!(v, Ratio::new(-1,2));
        } else {
            // Should have trimmed
            assert!(false);
        }
        let e23 = VoronoiEdge::two_points(&p2, &p3, 2, 3);
        let tr23 = e23.trim(&p1);
        if let TrimmedEdge::Trimmed(te23) = tr23 {
            assert!(te23.min_t.is_some());
            assert!(te23.max_t.is_none());
            let v = te23.min_t.unwrap();
            assert_eq!(v, Ratio::new(-3,2));
        } else {
            // Should have trimmed
            assert!(false);
        }
    }
}