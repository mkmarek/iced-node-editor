use iced::advanced::graphics::mesh::{self, Indexed, SolidVertex2D};
use iced::{Point, Rectangle, Size, Transformation};

pub trait MeshRenderer {
    fn draw_buffers(&mut self, buffers: Indexed<SolidVertex2D>);
}

impl<T> MeshRenderer for T
where
    T: mesh::Renderer,
{
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
            self.draw_mesh(mesh::Mesh::Solid {
                buffers,
                transformation: Transformation::IDENTITY,
                clip_bounds: Rectangle::new(min, size.max((2.0, 2.0).into())),
            });
        }
    }
}
