#[macro_export]
macro_rules! parser_chain {
    ($($parser:expr),*; $input:ident, $use_all_input:ident) => {
        $(let result = $parser($input);
        if let Ok((input, _)) = result {
            if input.is_empty() || !$use_all_input {
                return result;
            }
        })*
    }
}
