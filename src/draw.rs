use std::fs::File;
use std::io::{Result, Write};
use euclid::default::{Box2D, Point2D};
use euclid::Size2D;

pub struct SVG {
    size : Option<Box2D<f64>>,
    lines : Vec<String>
}

impl SVG {
    pub fn new() -> SVG {
        SVG {
            lines : vec!(),
            size : None
        }
    }

    pub fn write(&mut self, path : &str) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(b"<svg version=\"1.1\"\n")?;
        file.write_all(b"     width=\"1024\" height=\"1024\"\n")?;
        file.write_all(b"     xmlns=\"http://www.w3.org/2000/svg\"\n")?;
        let view_str = self.view_str();
        if view_str.len() > 0 {
            file.write_all(b"\n     ")?;
            file.write_all(&view_str.as_bytes())?;
        }
        file.write_all(b">\n")?;
        for line in &self.lines {
            file.write_all(&line.as_bytes())?;
            file.write_all(b"\n")?;
        }
        file.write_all(b"</svg>")?;
        Ok(())
    }

    fn view_str(&self) -> String {
        match self.size {
            None => "".to_string(),
            Some(bound_box) => {
                let outer_box =
                    bound_box.inflate(0.5, 0.5)
                             .scale(1.2, 1.2);
                format!(
                    " viewBox=\"{} {} {} {}\"",
                    outer_box.min.x,
                    outer_box.min.y,
                    outer_box.width(),
                    outer_box.height()
                )
            }
        }
    }

    fn expand_size(&mut self, pt : &Point2D<f64>) {
        let pt_box = Box2D::from_origin_and_size(
            *pt,
            Size2D::new(2.0,2.0)
        );
        match &mut self.size {
            None => {
                self.size = Some(pt_box)
            },
            Some(current_box) => {
                *current_box = current_box.union(&pt_box)
            }
        }
    }

    pub fn draw_point(&mut self, pt : &Point2D<f64>) {
        self.expand_size(pt);
        println!("Box {:?}", self.size);
        self.lines.push(
            format!("  <circle cx=\"{}\" cy=\"{}\" r=\"2\"/>", pt.x, pt.y)
        )
    }
}