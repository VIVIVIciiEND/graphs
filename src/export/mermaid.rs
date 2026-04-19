pub fn export_mer(paths: &Vec<Vec<String>> )-> String {
    let mut result = String::from("graph TD;\n");
    
    for path in paths {
        if path.len() <2{
            continue ; 
        }
        for i in 0..path.len()-1 {
            let from = &path[i]; 
            let to = &path[i+1]; 
            result.push_str(&format!("{} --> {};\n" , from , to)); 
        }
    }
    result
}
