pub fn generate_ids(options: &Vec<(String, String)>, selected_names: &Vec<String>) -> Vec<String> {
    let selected_ids = options
        .iter()
        .filter(|(_, name)| selected_names.contains(name))
        .map(|(id, _)| id.clone())
        .collect();

    selected_ids
}