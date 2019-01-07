use graphql_parser as graphql;

use self::graphql::schema::Definition;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;

use alqhemist::gen;

pub fn read_file(fname: &str) -> String {
    let mut f = File::open(format!("tests/fixtures/{}", fname)).unwrap();

    let mut contents = String::new();

    f.read_to_string(&mut contents).unwrap();

    contents
}

pub fn parse_type_to_ruby(fname: &str) -> String {
    let mut schema_context = BTreeMap::new();

    let contents = read_file(fname);

    let mut ast = graphql::parse_schema(&contents).unwrap();

    schema_context.insert("Int".to_string(), "Integer".to_string());

    match &mut ast.definitions[0] {
        Definition::TypeDefinition(o) => gen::graphql_ruby::mutate_type(o, &mut schema_context),
        _ => "".to_string(),
    }
}
