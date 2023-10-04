pub fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    if ignore_case {
        return ignoring_case(query, contents);
    }
    case_sensitive(query, contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn when_not_ignoring_case() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape?";
        assert_eq!(
            search(query, contents, false),
            case_sensitive(query, contents)
        );
    }

    #[test]
    fn when_ignoring_case() {
        let query = "duCt";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape?";
        assert_eq!(
            search(query, contents, true),
            ignoring_case(query, contents)
        );
    }
}

fn case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];

    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod case_sensitive_tests {

    use super::case_sensitive;

    #[test]
    fn query_not_found() {
        let query = "foobar";
        let contents = "Rust:
safe, fast, productive.
Pick three.";

        let empty_vector: Vec<&str> = vec![];
        assert_eq!(empty_vector, case_sensitive(query, contents));
    }

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Duct tape?";

        assert_eq!(
            vec!["safe, fast, productive."],
            case_sensitive(query, contents)
        );
    }
}

fn ignoring_case<'a>(cased_query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = vec![];
    let query = cased_query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}

#[cfg(test)]
mod ignoring_case_tests {

    use super::ignoring_case;

    #[test]
    fn query_not_found() {
        let query = "foobar";
        let contents = "Rust:
safe, fast, productive.
Pick three.";

        let empty_vector: Vec<&str> = vec![];
        assert_eq!(empty_vector, ignoring_case(query, contents));
    }

    #[test]
    fn one_result() {
        let query = "rUsT";
        let contents = "Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], ignoring_case(query, contents));
    }
}
