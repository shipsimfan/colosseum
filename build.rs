const SHADER_FILES: &[&str] = &[
    "src/render/render_objects/fixed/fullscreen-quad.slang",
    "src/render/frame_graph/nodes/fxaa/fxaa.slang",
    "src/render/frame_graph/nodes/procedural_sky/procedural-sky.slang",
    "src/render/frame_graph/nodes/quantization/quantization.slang",
    "src/render/frame_graph/nodes/tone_map/tone-map.slang",
    "src/update/render_objects/new/lit-opaque.slang",
    "src/update/render_objects/new/unlit-opaque.slang",
];

fn main() {
    colosseum_build::push_branch("COLOSSEUM_ENGINE_BRANCH");
    colosseum_build::push_commit_hash("COLOSSEUM_ENGINE_COMMIT");
    colosseum_build::push_build_time("COLOSSEUM_ENGINE_BUILD_TIME");

    for shader in SHADER_FILES {
        println!("cargo::rerun-if-changed={}", shader);
    }
}
