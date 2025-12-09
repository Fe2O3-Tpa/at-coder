use std::io;

fn main() {
    let q: usize = read_buffer().parse::<usize>().unwrap();
    let mut query_list: Vec<Vec<usize>> = Vec::new();
    for _ in 0..q {
        query_list.push(
            read_buffer_vec()
                .iter()
                .map(|t| t.parse().unwrap())
                .collect(),
        );
    }
    let mut bag: Vec<usize> = Vec::new();
    for i in query_list {
        if i[0] == 1 {
            bag.push(i[1]);
        } else {
            let mut minimum_place = 0;
            for i in 1..bag.len() {
                if bag[minimum_place] > bag[i] {
                    minimum_place = i;
                }
            }
            println!("{}", bag[minimum_place].to_string());
            bag.remove(minimum_place);
        }
    }
}

fn read_buffer_vec() -> Vec<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().split(" ").map(|s| s.to_string()).collect()
}

fn read_buffer() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
    buffer.trim().to_string()
}
