use crate::lexial::Lexer;

pub(crate) mod lexial;
pub(crate) mod token;

fn main() {
    let input = r#"
    int x = 2;
    // sajkd hasklf j
void DAQ_signal_handler_IO ( int status )
{
    wait_flag = FALSE;
}

int main() {
print("hello word");
}
    "#
    .to_string();
    let mut lexer = Lexer::new(input);

    while !lexer.is_end() {
        let token = lexer.next_token();
        println!("{:?}", token);
    }
}
