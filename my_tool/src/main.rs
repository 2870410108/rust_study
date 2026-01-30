#![allow(clippy::collapsible_if)]

// 引入处理日期和本地时间的库
use chrono::{Datelike, Local};
// 引入正则表达式库，用于匹配和提取文本内容
use regex::Regex;
// 引入文件系统操作、文件读写相关模块
use std::fs::{self, File};
// 引入流操作、读写、拷贝及标准输出控制模块
use std::io::{Cursor, Read, Write, copy, stdout};
// 引入外部命令调用模块，用于执行 PowerShell
use std::process::Command;
// 引入目录遍历库，用于递归或单层搜索文件
use walkdir::WalkDir;
// 引入 Zip 压缩包处理库，用于解压和重构 .docx 文件
use zip::{ZipArchive, ZipWriter, write::FileOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 获取当前本地时间
    let now = Local::now();
    // 格式化表格内使用的日期（如: 1.23）
    let today_table = format!("{}.{}", now.month(), now.day());
    // 格式化文件名使用的日期（如: 2026.01.23）
    let today_filename = now.format("%Y.%m.%d").to_string();

    // --- 1. 寻找最新的参考文件 ---
    let mut latest_file = String::new();
    let mut latest_time = std::time::SystemTime::UNIX_EPOCH;

    for entry in WalkDir::new(".")
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let name = entry.file_name().to_string_lossy();

        if name.starts_with("~$")
            || !name.to_lowercase().ends_with(".docx")
            || !name.contains("机动部")
        {
            continue;
        }

        if let Ok(m) = entry.metadata()
            && let Ok(time) = m.modified()
        {
            if time > latest_time {
                latest_time = time;
                latest_file = name.to_string();
            }
        }
    }

    if latest_file.is_empty() {
        println!("❌ 在程序根目录下未找到[机动部....工作日清.docx]相关的文件");
        wait_for_exit();
        return Ok(());
    }

    // --- 2. 处理文件内容 ---
    let file_bytes = fs::read(&latest_file)?;
    let mut xml_data = String::new();
    {
        let mut archive = ZipArchive::new(Cursor::new(&file_bytes))?;
        let mut doc_xml = archive.by_name("word/document.xml")?;
        doc_xml.read_to_string(&mut xml_data)?;
    }

    // 正则提取：识别日期 (x.xx)
    let re_date = Regex::new(r"<w:t>(\d{1,2}\.\d{1,2})</w:t>")?;
    let old_date = re_date
        .captures(&xml_data)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .unwrap_or_else(|| "未知".into());

    // 正则提取：识别工作内容 (1.xxxx)
    let re_work = Regex::new(r"<w:t>(1\.[^<]*[\u4e00-\u9fa5][^<]*)</w:t>")?;
    let old_work = re_work
        .captures(&xml_data)
        .map(|c| c.get(1).unwrap().as_str().to_string())
        .unwrap_or_else(|| "未识别到工作内容".into());

    // --- 重点修复：动态识别姓名 ---
    // 匹配“机动部”后面紧跟的 1-10 个非数字字符，直到遇到年份数字 (20xx)
    let re_name = Regex::new(r"机动部([^\d]{1,10})\d{4}")?;
    let user_name = re_name
        .captures(&latest_file)
        .map(|c| c.get(1).unwrap().as_str().trim().to_string())
        .unwrap_or_else(|| "成员".into());

    // --- 预览与交互 ---
    let line = "🌟".repeat(27);
    println!("\n{}", line);
    println!("🔍 识别到日清文件: {}", latest_file);
    println!("📅 识别到汇报日期: {}", old_date);
    println!("📝 识别到主要工作: {}", old_work);
    println!("{}", line);

    println!("\n🎉 准备生成今日文档 ({})", today_filename);
    print!("⌨️  请输入今日工作 (直接[回车]沿用旧内容): 1. ");
    stdout().flush()?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let input = input.trim();

    let final_work = if input.is_empty() {
        old_work.clone()
    } else if input.starts_with("1.") {
        input.to_string()
    } else {
        format!("1.{}", input)
    };

    let mut final_xml = xml_data.replace(
        &format!("<w:t>{}</w:t>", old_date),
        &format!("<w:t>{}</w:t>", today_table),
    );
    final_xml = final_xml.replace(
        &format!("<w:t>{}</w:t>", old_work),
        &format!("<w:t>{}</w:t>", final_work),
    );

    println!("🗑️  正在清理旧档: {}...", latest_file);
    let _ = fs::remove_file(&latest_file);

    // --- 3. 生成新文件 ---
    // 使用动态提取的姓名 user_name
    let out_name = format!("机动部{}{}工作日清.docx", user_name, today_filename);
    let out_file = File::create(&out_name)?;
    let mut writer = ZipWriter::new(out_file);

    let mut archive = ZipArchive::new(Cursor::new(&file_bytes))?;
    for i in 0..archive.len() {
        let mut inner = archive.by_index(i)?;
        let name = inner.name().to_string();
        writer.start_file(
            &name,
            FileOptions::default().compression_method(inner.compression()),
        )?;

        if name == "word/document.xml" {
            writer.write_all(final_xml.as_bytes())?;
        } else {
            copy(&mut inner, &mut writer)?;
        }
    }

    writer.finish()?;
    println!("\n✨ 任务完美达成！");
    println!("✅ 旧文件已删除");
    println!("✅ 新文件已生成: {}", out_name);

    // --- 4. 复制到剪切板 ---
    if let Ok(abs_path) = std::env::current_dir().map(|p| p.join(&out_name)) {
        let script = format!("Set-Clipboard -Path '{}'", abs_path.to_string_lossy());
        let output = Command::new("powershell")
            .args(["-Command", &script])
            .output();

        match output {
            Ok(out) if out.status.success() => {
                println!("📋 文件已成功复制到剪切板，可直接粘贴到工作群！");
            }
            _ => println!("⚠️ 文件已生成，但自动复制到剪切板失败。"),
        }
    }

    wait_for_exit();
    Ok(())
}

fn wait_for_exit() {
    println!("\n按 [回车键] 退出程序...");
    let mut temp = String::new();
    let _ = std::io::stdin().read_line(&mut temp);
}
