use std::fs;
use std::process::Command;
pub fn open_schedule(mmd_path: &str) {
    let mmd = fs::read_to_string(mmd_path).expect("файла нет!");
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="ru">
<head>
    <meta charset="UTF-8">
    <title>Граф</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
</head>
<body>
    <h1>Граф</h1>

    <pre class="mermaid">{mmd1}</pre>

    <script>
        mermaid.initialize({{
            startOnLoad: true
        }});
    </script>
</body>
</html>"#,
        mmd1 = mmd
    );
    let t_html = "graph.html";
    fs::write(t_html, html).expect("ошибка html");
    Command::new("cmd").args(&["/C", "start", t_html]).spawn();
}
