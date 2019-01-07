extern crate graphql_parser as graphql;
extern crate lazy_static;

use graphql::schema::Definition;

use std::collections::BTreeMap;
use std::io::Read;

pub mod gen;
use crate::gen::graphql_ruby::mutate_type;

pub fn transmute(schema: &str) -> Result<(), Error> {
    let mut contents = String::new();
    let mut schema_context = BTreeMap::new();
    let mut file = std::fs::File::open(schema)?;

    schema_context.insert("Int".to_string(), "Integer".to_string());

    let ast = {
        file.read_to_string(&mut contents)?;
        graphql::parse_schema(&contents)?
    };

    for def in ast.definitions {
        match def {
            Definition::TypeDefinition(mut o) => {
                println!("{}", mutate_type(&mut o, &mut schema_context))
            }
            _ => println!(""),
        }
    }

    Ok(())
}

pub struct Error(String);

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error(format!("{}", e))
    }
}

impl From<graphql::schema::ParseError> for Error {
    fn from(e: graphql::schema::ParseError) -> Self {
        Error(format!("{}", e))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
