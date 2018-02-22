#[macro_use]
extern crate graphql_derive;

trait GraphQL {
    fn query() -> String;
}

#[cfg(test)]
mod tests {
    use super::GraphQL;

    #[derive(GraphQL)]
    #[serde(rename_all = "camelCase")]
    struct Repository {
        name: String,
        #[serde(rename = "xxx")] url: String,
        fork_count: i32,
    }

    #[test]
    fn it_works() {
        assert_eq!(
            Repository::query(),
            "{
name
url
fork_count
}
"
        );
    }
}
