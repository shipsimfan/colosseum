fn main() {
    colosseum_build::push_commit_hash("COLOSSEUM_ENGINE_COMMIT");
    colosseum_build::push_build_time("COLOSSEUM_ENGINE_BUILD_TIME");

    println!("cargo::rerun-if-changed=src/graphics/shader/default_unlit.hlsl");
}
