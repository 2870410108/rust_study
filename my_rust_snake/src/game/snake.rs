use crate::game::{GameState, game_direction::Direction};
use rand::Rng;
use std::collections::VecDeque;
use std::{thread, time::Duration};
//第一，定义蛇的数据结构，以及初始化蛇
#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    //坐标结构体
    pub x: i32,
    pub y: i32,
}
pub struct Snake {
    // 蛇的身体，front() = 头，back() = 尾
    body: VecDeque<Position>,
    // 蛇头位置
    head_position: Position,
    // 蛇头方向
    head_direction: Direction,
    // 蛇的长度
    snake_body_length: usize,
    //食物坐标
    snak_food_position: Position,
}
impl Snake {
    //初始化蛇
    pub fn new() -> Self {
        let head_position = Position { x: 5, y: 5 };
        let snake_body_length = 3;
        let mut snake = Snake {
            body: VecDeque::new(),
            head_direction: Direction::Right,
            head_position,
            snake_body_length,
            snak_food_position: Position { x: 5, y: 8 },
        };
        //初始化蛇身每一个部分的位置
        for i in 0..(snake_body_length as i32) {
            let pos = Position {
                x: head_position.x - i,
                y: head_position.y,
            };
            snake.body.push_back(pos);
        }
        snake
    }
    //预判下一步蛇头位置
    pub fn predicted_position(
        &self,
        snake_head_current_position: Position,
        snake_head_current_direction: Direction,
    ) -> Position {
        let (x, y) = snake_head_current_direction.to_vec();
        Position {
            x: x + snake_head_current_position.x,
            y: y + snake_head_current_position.y,
        }
    }
    //定义蛇的碰撞检测逻辑
    pub fn is_out_of_bounds(
        &self,
        snake_head_next_position: &Position,
        canvas_width: usize,
        canvas_height: usize,
    ) -> bool {
        snake_head_next_position.x < 0
            || snake_head_next_position.x >= canvas_width as i32
            || snake_head_next_position.y < 0
            || snake_head_next_position.y >= canvas_height as i32
    }
    pub fn is_touch_self_body(
        &self,
        snake_head_next_position: &Position,
        body: &VecDeque<Position>,
    ) -> bool {
        // contains 会逐个比较 body 中的元素是否等于 snake_head_current_position
        // 前提是这个元素得实现可比较的特性
        body.contains(snake_head_next_position)
    }
    //定义移动一次蛇的方发
    pub fn snake_move(
        &mut self,
        game_state: &mut GameState,
        canvas_width: usize,
        canvas_height: usize,
    ) {
        let next_position = self.predicted_position(self.head_position, self.head_direction);
        //判断是否符合移动条件
        if self.is_out_of_bounds(&next_position, canvas_width, canvas_height) {
            game_state.set_game_is_suspend(true);
            println!("您操控的蛇出界！如果重新开始游戏请按下：'r'");
            thread::sleep(Duration::from_millis(3000));
        } else if self.is_touch_self_body(&next_position, &self.body) {
            game_state.set_game_is_suspend(true);
            println!("您操控的蛇与自身碰撞！如果重新开始游戏请按下：'r'");
            thread::sleep(Duration::from_millis(3000));
        } else {
            self.body.push_front(next_position);
            if next_position == self.snak_food_position {
                self.snake_body_length += 1;
                self.generate_food(canvas_width, canvas_height);
            } else {
                self.body.pop_back();
            }
            //更新位置
            self.set_head_position(next_position);
        }
    }

    //定义实物的生成逻辑
    fn generate_food(&mut self, canvas_width: usize, canvas_height: usize) {
        let mut rng = rand::thread_rng();
        let mut snak_food_position = Position {
            x: rng.gen_range(1..canvas_width) as i32,
            y: rng.gen_range(1..canvas_height) as i32,
        };
        loop {
            //食物保证不生成在蛇身体的坐标上
            if self.body.contains(&snak_food_position) {
                snak_food_position = Position {
                    x: rng.gen_range(1..canvas_width) as i32,
                    y: rng.gen_range(1..canvas_height) as i32,
                };
            } else {
                self.snak_food_position = snak_food_position;
                break;
            }
        }
    }

    //获取蛇的数据结构队列
    pub fn get_snake_body(&self) -> &VecDeque<Position> {
        &self.body
    }
    pub fn get_head_direction(&self) -> Direction {
        self.head_direction
    }
    pub fn set_head_direction(&mut self, new_direction: Direction) {
        self.head_direction = new_direction;
    }
    // pub fn get_head_position(&self) -> Position {
    //     self.head_position
    // }
    pub fn set_head_position(&mut self, new_position: Position) {
        self.head_position = new_position;
    }
    pub fn get_snake_body_length(&self) -> usize {
        self.snake_body_length
    }
    // pub fn set_snake_body_length(&mut self, new_length: usize) {
    //     self.snake_body_length = new_length;
    // }
    pub fn get_snak_food_position(&self) -> Position {
        self.snak_food_position
    }
}
