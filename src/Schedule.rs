use std::fs;
use std::process::Command;
pub fn open_schedule(mmd_path: &str) {
    let mmd = fs::read_to_string(mmd_path).expect("файла нет!");
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="ru-Ru">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Граф</title>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@10/dist/mermaid.min.js"></script>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Визуализация графа</h1>
            <p>Интерактивная схема связей между узлами</p>
        </div>
        <div class="graph-container">
            <pre class="mermaid">
{}
            </pre>
        </div>
        <div class="footer">
            Сгенерировано Graph Constructor | Mermaid.js
        </div>
    </div>
    <script>
        mermaid.initialize({{
            startOnLoad: true,
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
