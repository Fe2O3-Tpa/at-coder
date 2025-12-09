use std::io;

fn main() {
    let q = read_buffer().parse::<usize>().unwrap();

    let mut query_list: Vec<Vec<usize>> = Vec::new();
    for _ in 0..q {
        query_list.push(
            read_buffer_vec()
                .iter()
                .map(|t| t.parse().unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    let mut mati_menu: Vec<usize> = Vec::new();
    for i in query_list {
        if i[0] == 1 {
            mati_menu.push(i[1]);
        }
        if i[0] == 2 {
            println!("{}", mati_menu[0].to_string());
            mati_menu.remove(0);
        }
    }
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}