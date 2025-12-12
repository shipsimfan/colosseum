fn main() {
    colosseum_build::push_commit_hash("COLOSSEUM_ENGINE_COMMIT");
    colosseum_build::push_build_time("COLOSSEUM_ENGINE_BUILD_TIME");

    println!("cargo::rerun-if-changed=src/graphics/managed_objects/material/shader/lit.hlsl");
    println!("cargo::rerun-if-changed=src/graphics/managed_objects/material/shader/unlit.hlsl");

    println!("cargo::rerun-if-changed=src/graphics/context/post_processing/color_correction.hlsl");
    println!("cargo::rerun-if-changed=src/graphics/context/post_processing/fxaa.hlsl");
    println!("cargo::rerun-if-changed=src/graphics/context/post_processing/render_scale.hlsl");
    println!("cargo::rerun-if-changed=src/graphics/context/post_processing/vertex_shader.hlsl");
}
