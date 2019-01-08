pub mod common;
use crate::common::{parse_type_to_ruby, read_file};
use std::collections::BTreeMap;

fn create_context() -> BTreeMap<String, String> {
    let mut context = BTreeMap::new();
    context.insert("Int".to_string(), "Integer".to_string());
    context
}

#[test]
fn it_renders_simple_type_correctly() {
    let mut ctx = create_context();
    assert_eq!(
        read_file("a_type.rb"),
        parse_type_to_ruby("a.graphql", &mut ctx)
    );
}

#[test]
fn it_renders_type_with_multiple_fields() {
    let mut ctx = create_context();
    assert_eq!(
        read_file("b_type.rb"),
        parse_type_to_ruby("b.graphql", &mut ctx)
    );
}

#[test]
fn it_renders_type_with_argument() {
    let mut ctx = create_context();
    assert_eq!(
        read_file("c_type.rb"),
        parse_type_to_ruby("c.graphql", &mut ctx)
    );
}

#[test]
fn it_renders_type_with_list_field() {
    let mut ctx = create_context();
    ctx.insert(
        "ServantClass".to_string(),
        "Types::ServantClassEnum".to_string(),
    );
    assert_eq!(
        read_file("d_type.rb"),
        parse_type_to_ruby("d.graphql", &mut ctx)
    );
}
