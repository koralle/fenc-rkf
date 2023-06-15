use crate::terminator::Terminator;

pub fn replace_terminator(text: &str, terminator: &Terminator) -> String {
    if terminator == &Terminator::LF {
        text.replace("\r\n", "\n")
    } else {
        todo!("not implemented yet")
    }
}

#[cfg(test)]
mod tests {
    use crate::eol::replace_terminator;
    use crate::terminator::Terminator;

    #[test]
    fn it_works() {
        let query_text = "Hello, world\r\n";
        let expected = "Hello, world\n";
        let actual = replace_terminator(query_text, &Terminator::LF);

        assert_eq!(actual, expected);
    }
}
