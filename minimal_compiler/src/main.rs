mod lexer;
use lexer::tokenize;
fn main() {
    let input = "int main(void) { return 2; }";
    let tokens = tokenize(input);
    println!("{:?}", tokens);
}
