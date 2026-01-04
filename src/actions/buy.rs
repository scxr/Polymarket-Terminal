fn parse_string_list(s: &str) -> Vec<String> {
    s.trim_matches(|c| c == '[' || c == ']')
        .split(',')
        .map(|item| item.trim().trim_matches('"').to_string())
        .collect()
}
pub fn buy_yes(clob_ids: String, option: &str) {
    let clob_ids_parsed = parse_string_list(&clob_ids);
    println!("{:?}", clob_ids_parsed);
    println!("{}", clob_ids);
    if option == "Yes" {
        let opt = clob_ids_parsed.get(0);
        println!("Option: {:?}", opt.unwrap());
    }
    println!("buy yes");
}