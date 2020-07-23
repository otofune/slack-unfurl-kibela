use super::types::{GraphQLQueryResponse, Result};

pub fn parse_query<'a, T>(val: &'a str) -> Result<T>
where
    T: serde::Deserialize<'a>,
{
    let res: GraphQLQueryResponse<T> = serde_json::from_str(&val)?;
    match res {
        GraphQLQueryResponse::Ok { data } => Ok(data),
        GraphQLQueryResponse::Err { errors } => {
            let errors: Vec<String> = errors.into_iter().map(|m| m.message).collect();
            // into をやめたい (適当すぎる Result<T> を使うのやめて固有のエラーにしたい)
            Err(format!(
                "GraphQL server returns error: {}",
                errors.as_slice().join(", ")
            )
            .into())
        }
    }
}

#[cfg(test)]
mod parse_query_tests {
    use super::super::types::GraphQLQueryResponse;

    #[test]
    fn it_must_return_joined_description_when_error() {
        let s = serde_json::json!({
            "errors": [
                {
                    "message": "いっこめのエラー"
                },
                {
                    "message": "second error"
                }
            ]
        });
        let s = serde_json::to_string(&s).unwrap();
        let result = super::parse_query::<GraphQLQueryResponse<()>>(&s);
        assert_eq!(result.is_err(), true);
        assert_eq!(
            result.err().unwrap().to_string(),
            "GraphQL server returns error: いっこめのエラー, second error"
        );
    }

    #[test]
    fn it_must_return_result_when_not_json() {
        let s = "invalid";
        let result = super::parse_query::<GraphQLQueryResponse<()>>(&s);
        assert_eq!(result.is_err(), true);
    }
}
