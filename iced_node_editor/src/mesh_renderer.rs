use iced::{Point};
use iced_graphics::triangle::{ColoredVertex2D, Mesh2D};

pub trait MeshRenderer {
    fn draw_mesh(&mut self, mesh: Mesh2D<ColoredVertex2D>);
}

impl<B, T> MeshRenderer for iced_graphics::Renderer<B, T>
where
    B: iced_graphics::backend::Backend,
{
    fn draw_mesh(&mut self, mesh: Mesh2D<ColoredVertex2D>) {
        let min = mesh
            .vertices
            .iter()
            .fold(Point::new(f32::MAX, f32::MAX), |min, v| {
                Point::new(min.x.min(v.position[0]), min.y.min(v.position[1]))
            });

        let max = mesh
            .vertices
            .iter()
            .fold(Point::new(f32::MIN, f32::MIN), |max, v| {
                Point::new(max.x.max(v.position[0]), max.y.max(v.position[1]))
            });

        let size = iced_graphics::Size::new(max.x - min.x, max.y - min.y);
        
        if size.width >= 1.0 && size.height >= 1.0 {
            self.draw_primitive(iced_graphics::Primitive::SolidMesh { buffers: mesh, size });
        }
    }
}
