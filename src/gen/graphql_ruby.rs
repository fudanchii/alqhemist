use tera::{Context, Tera};

use graphql::schema::{Field, InputValue, ObjectType, Type, TypeDefinition, Value};
use heck::SnakeCase;
use lazy_static::lazy_static;
use regex::Regex;

use std::collections::BTreeMap;

static OBJECT_TEMPLATE: &str = r#"# DO NOT EDIT! THIS FILE IS AUTO-GENERATED FROM A GRAPHQL SCHEMA
module Types
  class {{object_typename}} < Types::BaseObject
{% if description %}    description <<-DESC
    {{description}}
    DESC{% endif %}
{% if rb_includes %}    include {{objectname}}Methods{% endif %}
{% if implements_interfaces %}{% for interface in implements_interfaces %}
    implements {{interface}}
{% endfor %}{% endif %}
{% for field in fields %}
    field :{{field.name}}, {{field.type}}, null: {{field.nullable}}{{field.description_decl}}{{field.arguments_decl}}
{% endfor %}
  end
end
"#;

lazy_static! {
    static ref RE: Regex = Regex::new("\n{2,}").unwrap();
}

pub fn mutate_type(typedef: &mut TypeDefinition, context: &mut BTreeMap<String, String>) -> String {
    let result = match typedef {
        TypeDefinition::Object(ot) => render_object_type(ot, context),
        _ => "".to_owned(),
    };

    RE.replace_all(&result, "\n").to_string()
}

fn render_object_type(
    objdef: &mut ObjectType,
    type_context: &mut BTreeMap<String, String>,
) -> String {
    let mut tpl_ctx = Context::new();

    type_context.insert(objdef.name.clone(), format!("{}Type", objdef.name));

    tpl_ctx.insert("objectname", &objdef.name);
    tpl_ctx.insert("object_typename", type_context.get(&objdef.name).unwrap());
    tpl_ctx.insert("fields", &to_type_fields(&objdef.fields, type_context));

    if objdef.description.is_some() {
        tpl_ctx.insert("description", objdef.description.as_ref().unwrap());
    }

    for dir in &objdef.directives {
        if dir.name == "rb_includes" {
            tpl_ctx.insert("rb_includes", &true);
        }
    }

    Tera::one_off(OBJECT_TEMPLATE, &tpl_ctx, false).unwrap()
}

#[derive(Default)]
struct ParsedField {
    typ: String,
    nullable: bool,
}

fn to_type_fields(
    fields: &Vec<Field>,
    type_context: &mut BTreeMap<String, String>,
) -> Vec<BTreeMap<&'static str, String>> {
    let mut vec_map = Vec::new();
    for field in fields {
        let mut map = BTreeMap::new();
        let parsed_field = resolve_type(&field.field_type, type_context);

        map.insert("name", field.name.clone().to_snake_case());
        map.insert("type", parsed_field.typ);
        map.insert("nullable", parsed_field.nullable.to_string());
        map.insert("description_decl", resolve_description(field));
        map.insert("arguments_decl", resolve_arguments(field, type_context));

        vec_map.push(map);
    }
    vec_map
}

fn resolve_type(ftype: &Type, type_context: &mut BTreeMap<String, String>) -> ParsedField {
    let mut result: ParsedField = Default::default();
    match ftype {
        Type::NamedType(name) => {
            result.typ = type_context.get(name).unwrap_or(name).to_string();
            result.nullable = true;
        }
        Type::ListType(inner_type) => {
            let inner_parsed_field = resolve_type(&*inner_type, type_context);
            result.typ = format!(
                "[{}{}]",
                inner_parsed_field.typ,
                if inner_parsed_field.nullable {
                    ", null: true".to_string()
                } else {
                    "".to_string()
                }
            );
            result.nullable = true;
        }
        Type::NonNullType(inner_type) => {
            result.typ = resolve_type(&*inner_type, type_context).typ;
            result.nullable = false;
        }
    }
    result
}

fn resolve_description(field: &Field) -> String {
    match &field.description {
        None => "".to_string(),
        Some(s) => format!(
            r#",
      description: "{}""#,
            s.to_string()
        ),
    }
}

fn resolve_arguments(field: &Field, type_context: &mut BTreeMap<String, String>) -> String {
    if field.arguments.len() == 0 {
        return "".to_string();
    }

    let arg_list = (&field.arguments)
        .into_iter()
        .map(|arg| {
            let parsed_arg = resolve_type(&arg.value_type, type_context);
            let default_value = resolve_default_value(&arg.default_value);
            format!(
                r#"argument :{}, {}, required: {}{}"#,
                arg.name, parsed_arg.typ, !parsed_arg.nullable, default_value
            )
        })
        .collect::<Vec<String>>()
        .join("\n");

    format!(
        r#" do
      {}
    end"#,
        arg_list
    )
}

fn resolve_default_value(val: &Option<Value>) -> String {
    if val.is_none() {
        return "".to_string();
    }

    format!(", default_value: {}", resolve_value(val.as_ref().unwrap()))
}

fn resolve_value(val: &Value) -> String {
    match val {
        Value::Variable(n) => n.to_string(),
        Value::Int(n) => format!("{}", n.as_i64().unwrap()),
        Value::Float(f) => format!("{}", f),
        Value::String(s) => format!(r#""{}""#, s),
        Value::Boolean(b) => format!("{}", b),
        Value::Null => "nil".to_string(),
        Value::Enum(e) => format!(r#""{}""#, e),
        Value::List(v) => format!(
            "[{}]",
            v.into_iter()
                .map(|value| resolve_value(value))
                .collect::<Vec<String>>()
                .join(", ")
        ),
        Value::Object(obj) => format!(
            "{{{}}}",
            obj.into_iter()
                .map(|(k, v)| format!("{}: {}", k, resolve_value(v)))
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}
