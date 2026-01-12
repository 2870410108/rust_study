use crossterm::event::{self, Event, KeyCode, read};
use std::time::Duration;

//方向枚举定义
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    Quit,
    Restart,
}

impl Direction {
    //方向冲突检测函数，如果新方向与当前方向冲突则返回true，否则返回false
    fn is_opposite(&self, other: Direction) -> bool {
        matches!(
            (self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
                | (Direction::Right, Direction::Left)
        )
    }

    //方向安全更新
    pub fn apply(self, next: Direction) -> Option<Direction> {
        // 1. 首先排除非方向性的指令
        match next {
            Direction::Up | Direction::Down | Direction::Left | Direction::Right => {
                // 2. 再进行原有的冲突检测
                if self.is_opposite(next) || self == next {
                    None
                } else {
                    Some(next)
                }
            }
            _ => None, // 如果是 Restart 或 Quit，直接视为无效转向
        }
    }
    //向量转换
    pub fn to_vec(self) -> (i32, i32) {
        match self {
            //up,down对应画布的Y轴
            //left,right对应画布的X轴
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
            _ => (0, 0),
        }
    }
}

//输入设备特征定义
pub trait InputDevice {
    fn direction_check(&mut self) -> Option<Direction>;
}

pub struct KeyboardInput;

impl InputDevice for KeyboardInput {
    fn direction_check(&mut self) -> Option<Direction> {
        let mut last_dir = None;
        // 只要缓冲区有东西，就一直读，读到没东西为止
        while event::poll(Duration::from_millis(0)).ok()? {
            if let Event::Key(key) = read().ok()? {
                let dir = match key.code {
                    KeyCode::Char('w') | KeyCode::Up => Some(Direction::Up),
                    KeyCode::Char('s') | KeyCode::Down => Some(Direction::Down),
                    KeyCode::Char('a') | KeyCode::Left => Some(Direction::Left),
                    KeyCode::Char('d') | KeyCode::Right => Some(Direction::Right),
                    KeyCode::Char('q') => Some(Direction::Quit),
                    KeyCode::Char('r') => Some(Direction::Restart),
                    _ => None,
                };
                if dir.is_some() {
                    last_dir = dir;
                }
            }
        }
        last_dir
    }
}
