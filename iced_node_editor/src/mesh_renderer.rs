use iced::advanced::graphics::mesh::{Indexed, SolidVertex2D};
use iced::{Point, Size};

pub trait MeshRenderer {
    fn draw_buffers(&mut self, buffers: Indexed<SolidVertex2D>);
}

impl<Theme> MeshRenderer for iced::Renderer<Theme> {
    fn draw_buffers(&mut self, buffers: Indexed<SolidVertex2D>) {
        let min = buffers
            .vertices
            .iter()
            .fold(Point::new(f32::MAX, f32::MAX), |min, v| {
                Point::new(min.x.min(v.position[0]), min.y.min(v.position[1]))
            });

        let max = buffers
            .vertices
            .iter()
            .fold(Point::new(f32::MIN, f32::MIN), |max, v| {
                Point::new(max.x.max(v.position[0]), max.y.max(v.position[1]))
            });

        let size = Size::new(max.x - min.x, max.y - min.y);

        if size.width >= 1.0 && size.height >= 1.0 {
            self.draw_mesh(iced::advanced::graphics::Mesh::Solid { buffers, size });
        }
    }
}
