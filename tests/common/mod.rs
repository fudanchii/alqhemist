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

pub fn parse_type_to_ruby(fname: &str, mut ctx: &mut BTreeMap<String, String>) -> String {
    let contents = read_file(fname);

    let mut ast = graphql::parse_schema(&contents).unwrap();

    match &mut ast.definitions[0] {
        Definition::TypeDefinition(o) => gen::graphql_ruby::mutate_type(o, &mut ctx),
        _ => "".to_string(),
    }
}
