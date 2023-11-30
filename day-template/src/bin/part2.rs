fn main() {
    println!("Part2!");
    let input = include_str!("./input2.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> String {
    "todo!()".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = process(
            ""
    );
        assert_eq!(result, "todo!()".to_string());
    }
}
