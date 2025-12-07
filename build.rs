fn main() {
    colosseum_build::push_commit_hash("COLOSSEUM_ENGINE_COMMIT");
    colosseum_build::push_build_time("COLOSSEUM_ENGINE_BUILD_TIME");

    println!("cargo::rerun-if-changed=src/graphics/managed_objects/material/shader/lit.hlsl");
    println!("cargo::rerun-if-changed=src/graphics/managed_objects/material/shader/unlit.hlsl");
}
