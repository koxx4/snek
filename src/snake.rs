use piston_window::color::{GRAY, GREEN};
use piston_window::math::{Matrix2d, Scalar};
use piston_window::rectangle::square;
use piston_window::{Graphics, rectangle, types};
use piston_window::types::{Color};
use crate::snake::SnakeBlockDirection::{Down, Left, Right, Up};

#[derive(PartialEq, Clone, Debug)]
pub enum SnakeBlockDirection {
    Right,
    Left,
    Up,
    Down
}

impl SnakeBlockDirection {

    pub fn has_different_axis_than(&self, other: &SnakeBlockDirection) -> bool {

        match self {
            Right | Left => {
                *other == Up || *other == Down
            }
            Up | Down => {
                *other == Left || *other == Right
            }
        }
    }
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

    pub fn get_current_position(&self) -> [Scalar; 2] {

        [self.outer_block[0], self.outer_block[1]]
    }
}

pub struct Snake {
    blocks: Vec<SnakeBlock>,
    single_block_size: Scalar,
    head_current_direction: SnakeBlockDirection
}

impl Snake {

    pub fn new(blocks_count: i32, block_size: Scalar, blocks_padding: Scalar) -> Snake {

        let mut snake_body: Vec<SnakeBlock> = Vec::new();

        for  i in 0..blocks_count  {
            snake_body.push(SnakeBlock::new(block_size * i as f64 + 30.0, 30.0, block_size, blocks_padding));
        }

        Snake {
            blocks: snake_body,
            single_block_size: block_size,
            head_current_direction: Right
        }
    }

    pub fn draw<G>(&self, transform: Matrix2d, graphics: &mut G)
    where G: Graphics {

        self.blocks
            .iter()
            .for_each(|block| block.draw(transform, graphics));
    }

    pub fn move_right(&mut self) {

        assert!(!self.blocks.is_empty());

        match self.head_current_direction {
            Right | Up | Down => {
                self.head_current_direction = Right;
                self.move_snake();
            },
            Left => {}
        }
    }

    pub fn move_left(&mut self) {

        assert!(!self.blocks.is_empty());

        match self.head_current_direction {
            Left | Up | Down => {
                self.head_current_direction = Left;
                self.move_snake();
            }
            Right => {}
        }
    }

    pub fn move_up(&mut self) {

        assert!(!self.blocks.is_empty());

        match self.head_current_direction {
            Right | Up | Left => {
                self.head_current_direction = Up;
                self.move_snake();
            }
            Down => {}
        }
    }

    pub fn move_down(&mut self) {

        assert!(!self.blocks.is_empty());

        match self.head_current_direction {
            Right | Down | Left => {
                self.head_current_direction = Down;
                self.move_snake();
            }
            Up => {}
        }
    }

    fn get_head(&mut self) -> &mut SnakeBlock {

        self.blocks
            .iter_mut()
            .last()
            .expect("Snake has no head")
    }

    fn get_head_position(&mut self) -> [Scalar; 2] {

        self.get_head().get_current_position()
    }

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