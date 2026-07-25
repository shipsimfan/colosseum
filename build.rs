const SHADER_FILES: &[&str] = &["src/render/render_objects/fixed/fullscreen-quad.slang"];

fn main() {
    colosseum_build::push_branch("COLOSSEUM_ENGINE_BRANCH");
    colosseum_build::push_commit_hash("COLOSSEUM_ENGINE_COMMIT");
    colosseum_build::push_build_time("COLOSSEUM_ENGINE_BUILD_TIME");

    for shader in SHADER_FILES {
        println!("cargo::rerun-if-changed={}", shader);
    }
}
