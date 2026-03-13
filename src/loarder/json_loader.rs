// чтения файла и парсирование
use std::fs;
use crate::graph::Graph;
pub fn load_graph(path : &str) -> Graph {
    let data = fs::read_to_string(path).expect("невозможно прочитать файл");// чтение файла json 
    let graph: Graph = serde_json::from_str(&data).expect("ошибка парсинга");// обект graph принимает данные из полученого текста в обЬекте res и переводит вид данных в структуру Graph 
    
    graph
}