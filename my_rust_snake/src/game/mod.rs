pub mod game_direction;
pub mod game_display;
pub mod snake;

use std::fs;

pub struct GameState {
    game_is_runing: bool,
    game_is_suspend: bool,
    game_score: usize,
    game_historic_score: usize,
}

impl GameState {
    const SCORE_FILE: &'static str = "game_data.txt";
    const KEY_NAME: &'static str = "historic_score";

    /// 1. 初始化：在创建 GameState 时自动从文件加载历史最高分
    pub fn new() -> Self {
        let mut historic_score = 0;

        // 尝试读取文件
        if let Ok(content) = fs::read_to_string(Self::SCORE_FILE) {
            // 逐行解析，寻找 historic_score:XX
            for line in content.lines() {
                if let Some((key, value)) = line.split_once(':')
                    && key.trim() == Self::KEY_NAME
                {
                    historic_score = value.trim().parse::<usize>().unwrap_or(0);
                    break;
                }
            }
        }

        GameState {
            game_is_runing: true,
            game_is_suspend: false,
            game_score: 0,
            game_historic_score: historic_score,
        }
    }

    /// 2. 修改与保存：只修改对应行，保留其他行内容
    pub fn save_historic_score(&mut self) {
        // 逻辑：只有当前分数突破记录才触发写入，减少磁盘 IO
        if self.game_score <= self.game_historic_score {
            println!("很遗憾！您没能突破历史最高分！！！",);
            return;
        }
        println!("哇塞！您突破了历史最高分，正在为您保存游戏得分！",);
        self.game_historic_score = self.game_score;

        // 读取现有内容，如果文件不存在则默认为空字符串
        let content = fs::read_to_string(Self::SCORE_FILE).unwrap_or_default();

        // 将文件内容转为行列表，同时过滤掉可能因编辑器产生的空白行
        let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();

        let mut found = false;
        let new_line = format!("{}: {}", Self::KEY_NAME, self.game_historic_score);

        // 遍历寻找并替换
        for line in lines.iter_mut() {
            if let Some((key, _)) = line.split_once(':')
                && key.trim() == Self::KEY_NAME
            {
                *line = new_line.clone();
                found = true;
                break;
            }
        }

        // 如果没找到该键（例如新玩家），则追加到末尾
        if !found {
            lines.push(new_line);
        }

        // 拼接回字符串，并在末尾加上换行符，符合标准文本文件规范
        let mut final_content = lines.join("\n");
        final_content.push('\n');

        // 覆写回磁盘
        if let Err(e) = fs::write(Self::SCORE_FILE, final_content) {
            eprintln!("保存游戏数据失败: {}", e);
        }
    }
    pub fn get_game_is_runing(&self) -> bool {
        self.game_is_runing
    }
    pub fn set_game_is_runing(&mut self, new_state: bool) {
        self.game_is_runing = new_state
    }
    pub fn get_game_is_suspend(&self) -> bool {
        self.game_is_suspend
    }
    pub fn set_game_is_suspend(&mut self, new_state: bool) {
        self.game_is_suspend = new_state
    }
    pub fn get_game_score(&self) -> usize {
        self.game_score
    }
    pub fn set_game_score(&mut self, new_score: usize) {
        self.game_score = new_score
    }
    pub fn get_game_historic_score(&self) -> usize {
        self.game_historic_score
    }
}
