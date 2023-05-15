use cooltraption_simulation::components::Position;
use sfml::{
    graphics::{
        CircleShape, Color, Drawable, RectangleShape, RenderStates, RenderTarget, RenderWindow,
        Shape, Transformable,
    },
    window::{Event, Key, Style}, system::Vector2f,
};



struct Bullet<'s> {
    head: CircleShape<'s>,
    torso: RectangleShape<'s>,
}

impl<'s> Bullet<'s> {
    pub fn new() -> Self {
        let mut head = CircleShape::new(50.0, 50);
        head.set_position((100.0, 100.0));
        head.set_fill_color(Color::RED);
        let mut torso = RectangleShape::with_size((100., 200.).into());
        torso.set_position((100.0, 150.0));
        torso.set_fill_color(Color::BLUE);

        Self { head, torso }
    }
}

// Implement the Drawable trait for our custom drawable.
impl<'s> Drawable for Bullet<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        render_target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        render_target.draw(&self.head);
        render_target.draw(&self.torso)
    }
}

pub struct Renderer<I: Iterator<Item = Vec<Position>>> {
    positions_generator: I,
}

impl<I: Iterator<Item = Vec<Position>>> Renderer<I> {
    pub fn new(generator: I) -> Self{
        Self { positions_generator: generator }
    }

    pub fn render(mut self) {
        let mut window = RenderWindow::new(
            (1920, 1080),
            "Custom drawable",
            Style::CLOSE,
            &Default::default(),
        );
        window.set_vertical_sync_enabled(true);

        let _bullet = Bullet::new();
        loop {
            while let Some(event) = window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => return,
                    _ => {}
                }
            }
            //window.draw(&bullet);
            let mut circle = CircleShape::new(20.0, 512);
            if let Some(positions) = self.positions_generator.next(){
                window.clear(Color::BLACK);
                for pos in positions{
                    let sfml_pos = Vector2f::new(pos.x.0.to_num(), pos.y.0.to_num());
                    circle.set_position(sfml_pos);
                    window.draw(&circle);
                }
            }
            window.display();
        }
    }
}
