pub fn wait(delay: &str) {
    let mut child = std::process::Command::new("sleep")
        .arg(delay)
        .spawn()
        .unwrap();
    let _result = child.wait().unwrap();
}

pub fn clear_screen() {
    print!("{esc}[?25l{esc}[2J{esc}[1;1H", esc = 27 as char);
}