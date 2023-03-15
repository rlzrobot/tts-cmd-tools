use std::{
    env,
    fs::{self, DirEntry},
    path::PathBuf,
};
use std::process::Command;
fn main() {
    println!("Hello, world!");
    let current_dir = env::current_dir().expect("获取当前目录失败");
    println!("当前目录为:{:?}", current_dir);
    for entry in fs::read_dir(current_dir).expect("列目录文件失败") {
        let entry = entry.expect("获取文件失败");
        let path = entry.path();
        if is_txt_file(&entry) {
            let file_prefix = get_file_prefix(&path);
            println!("文件名:{:?}", get_file_prefix(&path));
            execute_tts_cmd(&file_prefix);
        }
    }
}

fn is_txt_file(file_entry: &DirEntry) -> bool {
    let path = file_entry.path();
    path.is_file() && path.extension().is_some() && path.extension().unwrap() == "txt"
}

fn get_file_prefix(path: &PathBuf) -> String {
    path.file_name()
        .map(|x| -> String {
            let file_prefix = x
                .to_str()
                .map(|str| str.split(".").collect::<Vec<&str>>())
                .map(|v| String::from(v[0]))
                .expect("文件名称规范不正确解析错误");
            file_prefix
        })
        .expect("文件名称规范不正确解析错误")
}

fn execute_tts_cmd(file_prefix:&String) {
    //edge-tts --rate=-30% --voice zh-TW-HsiaoChenNeural --file "署雷公.txt" --write-media 署雷公.mp3
    //edge-tts --rate=-30% --voice zh-HK-HiuMaanNeural --file "署雷公.txt" --write-media 署雷公_gd.mp3
    Command::new("edge-tts")
        .arg("--rate=-30%")
        .arg("--voice")
        .arg("zh-HK-HiuMaanNeural")
        .arg("--file")
        .arg(format!("{}.txt",file_prefix))
        .arg("--write-media")
        .arg(format!("{}_gd.mp3",file_prefix))
        //.spawn()
        .status()
        .expect("生成粤语的内容文件失败");

    Command::new("edge-tts")
        .arg("--rate=-30%")
        .arg("--voice")
        .arg("zh-TW-HsiaoChenNeural")
        .arg("--file")
        .arg(format!("{}.txt",file_prefix))
        .arg("--write-media")
        .arg(format!("{}.mp3",file_prefix))
        //.spawn()
        .status()
        .expect("生成粤语的内容文件失败");
}