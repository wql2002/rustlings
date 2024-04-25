// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.


fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // trim frontend
    let mut front_end = 0;
    let mut back_end = input.len();
    for c in input.chars() {
        if c == ' ' {
            front_end += 1;
        } else {
            break;
        }
    }
    // trim backend
    for c in input.chars().rev() {
        if c == ' ' {
            back_end -= 1;
        } else {
            break;
        }
    }
    input[front_end..back_end].to_string()
}

fn compose_me(input: &str) -> String {
<<<<<<< HEAD:exercises/strings/strings3.rs
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_owned() + " world!"
=======
    // TODO: Add " world!" to the string! There are multiple ways to do this!
    ???
>>>>>>> forked/main:exercises/09_strings/strings3.rs
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars", "balloons")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
