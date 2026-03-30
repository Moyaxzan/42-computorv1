pub fn parse_argument(arg: &String) -> (bool, &'static str) {
    if arg.len() == 2 {
        return (false, "Arg.len = 2");
    }
    return (true, "Ok");
}
