pub mod common;
use crate::common::{parse_type_to_ruby, read_file};

#[test]
fn it_renders_simple_type_correctly() {
    assert_eq!(read_file("a_type.rb"), parse_type_to_ruby("a.graphql"));
}

#[test]
fn it_renders_type_with_multiple_fields() {
    assert_eq!(read_file("b_type.rb"), parse_type_to_ruby("b.graphql"));
}

#[test]
fn it_renders_type_with_argument() {
    assert_eq!(read_file("c_type.rb"), parse_type_to_ruby("c.graphql"));
}
