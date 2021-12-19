fn main() -> Result<(), String> {
    // for use in docker build where file changes can be wonky
    println!("cargo:rerun-if-env-changed=FORCE_REBUILD");

    let proto_defs = [
        "extensions/extensions.proto",
        "capabilities.proto",
        "expression.proto",
        "function.proto",
        "parameterized_types.proto",
        "plan.proto",
        "relations.proto",
        "type.proto",
        "type_expressions.proto",
    ];

    for def in &proto_defs {
        println!("cargo:rerun-if-changed=substrait/proto/substrait/{}", def);
    }

    let paths: Vec<String> = proto_defs
        .iter()
        .map(|s| format!("substrait/proto/substrait/{}", s))
        .collect();

    prost_build::compile_protos(&paths, &["substrait/proto"])
        .map_err(|e| format!("protobuf compilation failed: {}", e))
}
