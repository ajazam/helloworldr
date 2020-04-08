pub fn text( a: &str, b : &str) -> String {
    let mut result = String::new();
    result.push_str(a);
    result.push_str(b); // comment
    result
}

fn main() {
    println!("{}", text("hello ", "world"));
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(text("hello ", "world"), "hello world")
    }
}
