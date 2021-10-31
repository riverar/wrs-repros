use reader::TypeInclude;

fn main() {
    let reader = reader::TypeReader::get_mut();
    reader.types.namespaces.values_mut().for_each(|tree| tree.types.values_mut().for_each(|v| v.include = TypeInclude::Full));

    let root = reader.types.get_namespace("TestNamespace").unwrap();
    let tokens = gen::gen_source_file(".", root);
    println!("{}", tokens.into_string());
}