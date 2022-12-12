pub fn wait(delay: &str) {
    let mut child = std::process::Command::new("sleep")
        .arg(delay)
        .spawn()
        .unwrap();
    let _result = child.wait().unwrap();
}