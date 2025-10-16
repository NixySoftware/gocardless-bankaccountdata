use std::{fs, path::Path};

use toml_edit::{DocumentMut, Formatted, Item, Key, Table, Value};

const PACKAGE: &str = "gocardless-bankaccountdata";

fn main() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Directory should have parent.")
        .join("packages")
        .join(PACKAGE)
        .join("Cargo.toml");

    let mut document = fs::read_to_string(&path)
        .expect("Manifest should be read.")
        .parse::<DocumentMut>()
        .expect("Manifest should be valid TOML.");

    let mut formatted_name = Formatted::new(PACKAGE.to_owned());
    formatted_name.decor_mut().set_suffix("\n");

    let mut workspace_table =
        Table::from_iter([(Key::new("workspace"), Value::Boolean(Formatted::new(true)))]);
    workspace_table.set_implicit(true);
    workspace_table.set_dotted(true);

    *document
        .get_mut("package")
        .expect("Manifest should have package item.") = Item::Table(Table::from_iter([
        (Key::new("name"), Item::Value(Value::String(formatted_name))),
        (Key::new("authors"), Item::Table(workspace_table.clone())),
        (Key::new("edition"), Item::Table(workspace_table.clone())),
        (Key::new("license"), Item::Table(workspace_table.clone())),
        (Key::new("repository"), Item::Table(workspace_table.clone())),
        (Key::new("version"), Item::Table(workspace_table.clone())),
    ]));

    let item = document
        .get_mut("dependencies")
        .expect("Manifest should have dependencies item.");

    *item
        .get_mut("reqwest-middleware")
        .expect("Dependency bon should exist.")
        .get_mut("version")
        .expect("Dependency should have version.") =
        Item::Value(Value::String(Formatted::new("^0.4".to_owned())));

    *item
        .get_mut("bon")
        .expect("Dependency bon should exist.")
        .get_mut("version")
        .expect("Dependency should have version.") =
        Item::Value(Value::String(Formatted::new("^3.6".to_owned())));

    document
        .get_mut("features")
        .expect("Manifest should have features item.")
        .as_table_mut()
        .expect("Features should be a table.")
        .decor_mut()
        .set_prefix("\n");

    fs::write(&path, document.to_string()).expect("Manifest should be written.");
}
