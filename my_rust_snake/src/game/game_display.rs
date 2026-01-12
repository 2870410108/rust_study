/*
在这个文件里面将会实现游戏画面的渲染功能
第一步 定义一个画布,使用一个一维动态数组和数学定位（Index = (y * W) + x）
y(行号)：代表你前面有多少个“完整的行”。
W(宽度)：每一行包含的元素个数。
x (列号)：代表你在当前这一行往后偏移了多少个位置。

*/
use crate::game::GameState;
use crate::game::snake::{self, Position};
use crossterm::{cursor, execute};
use std::io::{Write, stdout};
pub struct Canvas {
    canvas_width: usize,
    canvas_height: usize,
    buffer: Vec<char>,
}
impl Canvas {
    pub fn new() -> Self {
        let canvas_width = 40;
        let canvas_height = 15;

        Canvas {
            canvas_width,
            canvas_height,
            buffer: vec![' '; canvas_width * canvas_height],
        }
    }
    pub fn clear(&mut self) {
        self.buffer.fill(' ');
    }

    pub fn set_canvas_disply_char(&mut self, display_char: char, char_position: &Position) {
        // (行坐标 * 总宽度) + 列坐标
        let index = (char_position.y as usize) * self.canvas_width + (char_position.x as usize);
        // 确保索引不会超出 buffer 实际长度
        if index < self.buffer.len() {
            self.buffer[index] = display_char;
        }
    }
    pub fn render_canvas(&mut self, snake: &snake::Snake, game_state: &mut GameState) {
        let mut stdout = stdout();

        // 1. 数据准备：将逻辑状态同步到 Canvas 的 buffer
        self.prepare_buffer(snake);

        // 2. 环境设置：将光标复位到 (0,0) 并隐藏
        // 使用 queue! 暂存指令，最后一次性 flush
        execute!(stdout, cursor::MoveTo(0, 0), cursor::Hide).unwrap();

        // 3. 构造地图内容（保持一次性输出减少闪烁）
        let mut frame = String::with_capacity(2048);
        self.draw_map_to_string(&mut frame);

        // 4. 计算分数并构造 UI 文本
        let current_score = (snake.get_snake_body_length() as i32 - 3).max(0);
        game_state.set_game_score(current_score as usize);

        self.draw_ui_to_string(&mut frame, game_state);

        // 5. 一次性打印，极致顺滑
        print!("{}", frame);
        stdout.flush().unwrap();
    }

    /// 专门负责填充 Buffer 的内部逻辑
    fn prepare_buffer(&mut self, snake: &snake::Snake) {
        self.clear(); // 清空旧数据

        // 写入蛇身
        for (i, pos) in snake.get_snake_body().iter().enumerate() {
            let symbol = if i == 0 { 'O' } else { '■' };
            self.set_canvas_disply_char(symbol, pos);
        }

        // 写入食物
        self.set_canvas_disply_char('$', &snake.get_snak_food_position());
    }

    /// 专门负责构造地图边框和内容
    fn draw_map_to_string(&self, frame: &mut String) {
        let wall_h = "▄".repeat(self.canvas_width * 2);
        let wall_f = "▀".repeat(self.canvas_width * 2);

        // 上边框
        frame.push_str(&format!("◆{}◆\n", wall_h));

        for y in 0..self.canvas_height {
            frame.push('█'); // 左边框
            for x in 0..self.canvas_width {
                frame.push(self.buffer[y * self.canvas_width + x]);
                frame.push(' '); // 间隔，让显示更方正
            }
            frame.push_str("█\n"); // 右边框
        }

        // 下边框
        frame.push_str(&format!("◆{}◆\n", wall_f));
    }

    /// 专门负责构造下方的说明文字
    fn draw_ui_to_string(&self, frame: &mut String, state: &GameState) {
        frame.push_str("--------------------------------\n");
        frame.push_str(&format!(
            "历史最高分数：{}\n",
            state.get_game_historic_score()
        ));
        frame.push_str(&format!("当前分数：{}\n", state.get_game_score()));
        frame.push_str("操作说明: WSAD 移动, Q 退出!\n");
        frame.push_str("---------------------------------\n");
    }
    pub fn get_canvas_width(&self) -> usize {
        self.canvas_width
    }
    // pub fn set_canvas_width(&mut self, new_width: usize) {
    //     self.canvas_width = new_width;
    // }
    pub fn get_canvas_height(&self) -> usize {
        self.canvas_height
    }
    // pub fn set_canvas_height(&mut self, new_height: usize) {
    //     self.canvas_height = new_height;
    // }
}
