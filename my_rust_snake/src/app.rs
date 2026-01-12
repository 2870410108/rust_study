use crate::game::game_direction::{Direction, InputDevice};
use crate::game::{self, game_display, snake};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{Clear, ClearType},
};
use std::io::stdout;
use std::{thread, time::Duration};
pub fn run() {
    //初始化封装成闭包或内部函数，方便重置
    let setup = || {
        (
            game::game_direction::KeyboardInput,
            game::GameState::new(),
            snake::Snake::new(),
            game_display::Canvas::new(),
        )
    };

    let (mut input_device, mut game_state, mut snake, mut canvas) = setup();

    while game_state.get_game_is_runing() {
        //统一获取输入，避免多次调用导致的状态不一致
        let input = input_device.direction_check();

        //处理挂起（游戏结束/暂停）状态
        if game_state.get_game_is_suspend() {
            canvas.render_canvas(&snake, &mut game_state);
            //如果是重新开始，则恢复游戏初始化状态
            if let Some(Direction::Restart) = input {
                let (i, g, s, c) = setup();
                input_device = i;
                game_state = g;
                snake = s;
                canvas = c;
                //清除一次屏幕
                execute!(
                    stdout(),
                    Clear(ClearType::All), // 清除所有字符
                    MoveTo(0, 0)           // 光标归位
                )
                .unwrap();
            }
            if let Some(Direction::Quit) = input {
                game_state.set_game_is_runing(false);
            }
            thread::sleep(Duration::from_millis(250));
            continue; // 跳过本次循环后续逻辑
        }

        // 正常游戏逻辑：处理退出和移动
        match input {
            Some(Direction::Quit) => {
                game_state.set_game_is_runing(false);
                continue;
            }
            Some(dir) => {
                if let Some(new_dir) = snake.get_head_direction().apply(dir) {
                    snake.set_head_direction(new_dir);
                }
            }
            None => {}
        }

        // 5. 更新状态与渲染
        snake.snake_move(
            &mut game_state,
            canvas.get_canvas_width(),
            canvas.get_canvas_height(),
        );
        canvas.render_canvas(&snake, &mut game_state);

        thread::sleep(Duration::from_millis(250));
    }

    // 6. 游戏退出后的收尾
    finalize_game(&mut game_state);
}

fn finalize_game(game_state: &mut game::GameState) {
    game_state.save_historic_score();
    println!("正在退出！");
    thread::sleep(Duration::from_millis(3000));
}
