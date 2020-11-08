use crate::Position;

#[derive(Clone, Debug)]
pub struct Node<T> {
    position: Position,
    pub node: T,
}

impl<T> Node<T> {
    pub fn new(position: Position, node: T) -> Self {
        Self { position, node }
    }

    pub fn get_position(&self) -> Position {
        self.position.clone()
    }
}
