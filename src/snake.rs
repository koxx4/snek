use piston_window::color::{GRAY, GREEN};
use piston_window::math::{Matrix2d, Scalar, Vec2d};
use piston_window::rectangle::square;
use piston_window::{Graphics, rectangle, types};
use piston_window::types::{Color};
use crate::apple::SnakeBlockCount;
use crate::snake::SnakeBlockDirection::{Down, Left, Right, Up};

#[derive(Clone, Debug)]
pub enum SnakeBlockDirection {
    Right,
    Left,
    Up,
    Down
}

#[derive(Debug)]
struct SnakeBlock {
    inner_block: types::Rectangle,
    outer_block: types::Rectangle,
    inner_block_color: Color,
    outer_block_color: Color,
    inner_block_padding: f64
}

impl SnakeBlock {

    /// Creates new snake block composed of two rectangles. One of them
    /// is embed in another. To specify how much smaller the inner
    /// block a padding argument is used.
    pub fn new(x: Scalar, y: Scalar, size: Scalar, padding: Scalar) -> SnakeBlock {

        SnakeBlock {
            inner_block: square(x + padding, y + padding, size - padding * 2.0),
            outer_block: square(x, y, size),
            inner_block_padding: padding,
            inner_block_color: GREEN,
            outer_block_color: GRAY
        }
    }

    #[allow(dead_code)]
    pub fn shift(&mut self, x: Scalar, y: Scalar) {

        self.outer_block[0] += x;
        self.outer_block[1] += y;

        self.inner_block[0] += x;
        self.inner_block[1] += y;
    }

    pub fn move_to(&mut self, x: Scalar, y: Scalar) {

        self.outer_block[0] = x;
        self.outer_block[1] = y;

        self.inner_block[0] = self.outer_block[0] + self.inner_block_padding;
        self.inner_block[1] = self.outer_block[1] + self.inner_block_padding;
    }

    pub fn draw<G>(&self, transform: Matrix2d, graphics: &mut G)
        where G: Graphics
    {
        //Draw outer
        rectangle(self.outer_block_color, self.outer_block, transform, graphics);
        //Draw inner
        rectangle(self.inner_block_color, self.inner_block, transform, graphics);
    }

    pub fn get_current_position(&self) -> Vec2d {

        [self.outer_block[0], self.outer_block[1]]
    }
}

pub struct Snake {
    blocks: Vec<SnakeBlock>,
    single_block_size: Scalar,
    head_current_direction: SnakeBlockDirection,
    blocks_padding: Scalar
}

impl Snake {

    pub fn new(blocks_count: SnakeBlockCount, block_size: Scalar, blocks_padding: Scalar) -> Snake {

        let mut snake_body: Vec<SnakeBlock> = Vec::new();

        for  i in 0..blocks_count  {
            snake_body.push(SnakeBlock::new(block_size * i as f64, block_size, block_size, blocks_padding));
        }

        Snake {
            blocks: snake_body,
            single_block_size: block_size,
            head_current_direction: Right,
            blocks_padding
        }
    }

    pub fn draw<G>(&self, transform: Matrix2d, graphics: &mut G)
    where G: Graphics {

        self.blocks
            .iter()
            .for_each(|block| block.draw(transform, graphics));
    }

    pub fn get_length(&self) -> SnakeBlockCount {
        self.blocks.len()
    }

    pub fn move_in_current_direction(&mut self) {

        assert!(!self.blocks.is_empty());
        self.move_snake();
    }

    pub fn change_dir_to_right(&mut self) {

        match self.head_current_direction {
            Up | Down => self.head_current_direction = Right,
            _ => {}
        }
    }

    pub fn change_dir_to_left(&mut self) {

        match self.head_current_direction {
            Up | Down => self.head_current_direction = Left,
            _ => {}
        }
    }

    pub fn change_dir_to_up(&mut self) {

        match self.head_current_direction {
            Right | Left => self.head_current_direction = Up,
            _ => {}
        }
    }

    pub fn change_dir_to_down(&mut self) {

        match self.head_current_direction {
            Right | Left => self.head_current_direction = Down,
            _ => {}
        }
    }

    pub fn is_head_at_position(&self, position: &Vec2d) -> bool {

        let head_pos = self.get_head_position();

        head_pos[0] == position[0] && head_pos[1] == position[1]
    }

    pub fn grow(&mut self, count: SnakeBlockCount) {

        for i in 0..count {

            let block_in_front_pos = self.blocks[0].get_current_position();
            let new_block_pos = [block_in_front_pos[0] - self.single_block_size, block_in_front_pos[1]];

            self.blocks.insert(
                0,
                SnakeBlock::new(new_block_pos[0], new_block_pos[1], self.single_block_size, self.blocks_padding)
            );
        }
    }

    fn get_head(&self) -> &SnakeBlock {

        self.blocks
            .iter()
            .last()
            .expect("Snake has no head")
    }

    fn get_head_position(&self) -> [Scalar; 2] {

        self.get_head().get_current_position()
    }

    #[allow(dead_code)]
    fn move_block(block_size: Scalar, direction: &SnakeBlockDirection, block: &mut SnakeBlock) {

        match direction {
            Right => block.shift(block_size, 0.0),
            Left => block.shift(block_size, 0.0),
            Up => block.shift(0.0, block_size),
            Down => block.shift(0.0, block_size)
        }
    }

    fn move_snake(&mut self) {

        let head_pos = self.get_head_position();

        let mut next_block_position = match self.head_current_direction {
            Right => [head_pos[0] + self.single_block_size, head_pos[1]],
            Left => [head_pos[0] - self.single_block_size, head_pos[1]],
            Up => [head_pos[0], head_pos[1] - self.single_block_size],
            Down => [head_pos[0], head_pos[1] + self.single_block_size],
        };

        self.blocks
            .iter_mut()
            .rev()
            .for_each(|block| {

                let current_pos = block.get_current_position();
                block.move_to(next_block_position[0], next_block_position[1]);
                next_block_position = current_pos;
            });
    }
}