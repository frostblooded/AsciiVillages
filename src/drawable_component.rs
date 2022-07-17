use bevy::ecs::component::Component;

#[derive(Component, Debug)]
pub struct DrawableComponent {
    pub symbol: char,
}

impl DrawableComponent {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
}
