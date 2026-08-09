//! Golden test: render the committed `specs/general-illustrations.json` and
//! compare every generated file against the committed `skill/general-illustrations`
//! directory. This is the anti-drift guard -- it fails if the source-of-truth
//! spec and the generated Markdown ever diverge (the exact class of bug where
//! `styles.md` lost a style's `Tags` line while the JSON still had it).
//!
//! When this test fails, re-run:
//!   cargo run -p general-illustrations-cli -- skill render \
//!     --spec specs/general-illustrations.json --out skill/general-illustrations

use std::fs;
use std::path::PathBuf;

use general_illustrations_skill_renderer::render_skill;
use general_illustrations_skill_spec::SkillSpec;

fn repo_root() -> PathBuf {
    // crate dir is <root>/crates/general-illustrations-skill-renderer
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn load_spec() -> SkillSpec {
    let path = repo_root().join("specs/general-illustrations.json");
    let contents = fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read {}: {e}", path.display()));
    serde_json::from_str(&contents)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", path.display()))
}

#[test]
fn committed_spec_is_valid() {
    load_spec().validate().expect("committed spec must validate");
}

#[test]
fn rendered_skill_matches_committed_markdown() {
    let spec = load_spec();
    let rendered = render_skill(&spec).expect("render");
    let skill_dir = repo_root().join("skill/general-illustrations");

    let mut mismatches = Vec::new();
    for file in &rendered.files {
        let committed_path = skill_dir.join(&file.path);
        match fs::read_to_string(&committed_path) {
            Ok(committed) if committed == file.contents => {}
            Ok(_) => mismatches.push(format!("content drift: {}", file.path.display())),
            Err(_) => mismatches.push(format!("missing committed file: {}", file.path.display())),
        }
    }

    assert!(
        mismatches.is_empty(),
        "generated skill diverges from committed skill/general-illustrations \
         (re-run `skill render`):\n{}",
        mismatches.join("\n")
    );
}

#[test]
fn every_gemini_prompt_is_recipe_faithful() {
    let spec = load_spec();
    assert!(!spec.recipes.is_empty(), "expected recipes in the spec");
    for recipe in &spec.recipes {
        let prompt = recipe.providers.gemini.prompt.to_lowercase();
        assert!(
            !prompt.contains("photorealistic") && !prompt.contains("cinematic"),
            "recipe {} reintroduced a photoreal/cinematic Gemini wrapper (Berkshire regression)",
            recipe.id
        );
        assert!(
            recipe.providers.gemini.prompt.contains("{scene}"),
            "recipe {} Gemini prompt is missing the {{scene}} placeholder",
            recipe.id
        );
    }
}
