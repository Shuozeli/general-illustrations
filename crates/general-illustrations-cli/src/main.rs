use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, anyhow, bail};
use clap::{Parser, Subcommand, ValueEnum};
use general_illustrations_ark::ArkImageProvider;
use general_illustrations_core::{
    AspectRatio, ImageGenerationRequest, ImageProvider, OutputFormat,
};
use general_illustrations_minimax::MinimaxImageProvider;
use general_illustrations_skill_renderer::write_skill;
use general_illustrations_skill_spec::SkillSpec;

#[derive(Debug, Parser)]
#[command(version, about = "Generate illustrations through provider adapters")]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Generate(GenerateArgs),
    Providers,
    Skill(SkillArgs),
    Recipe(RecipeArgs),
}

#[derive(Debug, Parser)]
struct GenerateArgs {
    #[arg(long, value_enum, default_value_t = ProviderArg::Minimax)]
    provider: ProviderArg,
    #[arg(long)]
    prompt: Option<String>,
    #[arg(long)]
    prompt_file: Option<PathBuf>,
    #[arg(long, default_value = "16:9")]
    aspect_ratio: String,
    #[arg(long, value_enum, default_value_t = OutputFormatArg::Png)]
    output_format: OutputFormatArg,
    #[arg(long, default_value = "generated")]
    output_prefix: String,
    #[arg(long, default_value = "out")]
    output_dir: PathBuf,
}

#[derive(Debug, Parser)]
struct SkillArgs {
    #[command(subcommand)]
    command: SkillCommand,
}

#[derive(Debug, Parser)]
struct RecipeArgs {
    #[command(subcommand)]
    command: RecipeCommand,
}

#[derive(Debug, Subcommand)]
enum RecipeCommand {
    Prompt(RecipePromptArgs),
    Schema(RecipeSchemaArgs),
}

#[derive(Debug, Parser)]
struct RecipePromptArgs {
    #[arg(long)]
    spec: PathBuf,
    #[arg(long)]
    recipe: String,
    #[arg(long)]
    data: PathBuf,
}

#[derive(Debug, Parser)]
struct RecipeSchemaArgs {
    #[arg(long)]
    spec: PathBuf,
    #[arg(long)]
    recipe: String,
    #[arg(long)]
    out: Option<PathBuf>,
}

#[derive(Debug, Subcommand)]
enum SkillCommand {
    Validate(SkillValidateArgs),
    Render(SkillRenderArgs),
}

#[derive(Debug, Parser)]
struct SkillValidateArgs {
    #[arg(long)]
    spec: PathBuf,
}

#[derive(Debug, Parser)]
struct SkillRenderArgs {
    #[arg(long)]
    spec: PathBuf,
    #[arg(long)]
    out: PathBuf,
    #[arg(long)]
    copy_assets: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ProviderArg {
    Minimax,
    Ark,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormatArg {
    Png,
    Jpeg,
}

impl From<OutputFormatArg> for OutputFormat {
    fn from(value: OutputFormatArg) -> Self {
        match value {
            OutputFormatArg::Png => OutputFormat::Png,
            OutputFormatArg::Jpeg => OutputFormat::Jpeg,
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Generate(args) => generate(args),
        Command::Providers => {
            println!("minimax");
            println!("ark");
            Ok(())
        }
        Command::Skill(args) => skill(args),
        Command::Recipe(args) => recipe(args),
    }
}

fn skill(args: SkillArgs) -> Result<()> {
    match args.command {
        SkillCommand::Validate(args) => validate_skill(args),
        SkillCommand::Render(args) => render_skill(args),
    }
}

fn recipe(args: RecipeArgs) -> Result<()> {
    match args.command {
        RecipeCommand::Prompt(args) => recipe_prompt(args),
        RecipeCommand::Schema(args) => recipe_schema(args),
    }
}

fn recipe_schema(args: RecipeSchemaArgs) -> Result<()> {
    let spec = read_skill_spec(&args.spec)?;
    let (recipe, _style, _composition_pattern, template) =
        find_recipe_components(&spec, &args.recipe)?;
    let variable_names = general_illustrations_skill_spec::template_variables(&template.body);

    let mut properties = serde_json::Map::new();
    for variable in &variable_names {
        properties.insert(
            variable.clone(),
            serde_json::Value::Object(
                [(
                    "type".to_string(),
                    serde_json::Value::String("string".to_string()),
                )]
                .into_iter()
                .collect(),
            ),
        );
    }

    let schema = serde_json::Value::Object(
        [
            (
                "$schema".to_string(),
                serde_json::Value::String(
                    "https://json-schema.org/draft/2020-12/schema".to_string(),
                ),
            ),
            (
                "title".to_string(),
                serde_json::Value::String(format!("{} recipe data schema", recipe.id)),
            ),
            (
                "description".to_string(),
                serde_json::Value::String(format!("{}. {}", recipe.name, recipe.description)),
            ),
            (
                "type".to_string(),
                serde_json::Value::String("object".to_string()),
            ),
            (
                "additionalProperties".to_string(),
                serde_json::Value::Bool(true),
            ),
            (
                "required".to_string(),
                serde_json::Value::Array(
                    variable_names
                        .iter()
                        .map(|name| serde_json::Value::String(name.clone()))
                        .collect(),
                ),
            ),
            (
                "properties".to_string(),
                serde_json::Value::Object(properties),
            ),
        ]
        .into_iter()
        .collect(),
    );

    let schema_text =
        serde_json::to_string_pretty(&schema).context("failed to serialize schema to JSON")?;

    match args.out {
        Some(path) => {
            fs::write(&path, schema_text)
                .with_context(|| format!("failed to write {}", path.display()))?;
            println!("wrote {}", path.display());
        }
        None => println!("{schema_text}"),
    }

    Ok(())
}

fn recipe_prompt(args: RecipePromptArgs) -> Result<()> {
    let spec = read_skill_spec(&args.spec)?;
    let (recipe, style, composition_pattern, template) =
        find_recipe_components(&spec, &args.recipe)?;

    let mut values = read_prompt_data(&args.data)?;

    values
        .entry("recipe_id".to_string())
        .or_insert(recipe.id.clone());
    values
        .entry("recipe_name".to_string())
        .or_insert(recipe.name.clone());
    values
        .entry("recipe_description".to_string())
        .or_insert(recipe.description.clone());
    values
        .entry("style_id".to_string())
        .or_insert(recipe.style_id.clone());
    values
        .entry("style_name".to_string())
        .or_insert(style.name.clone());
    values
        .entry("style_use_when".to_string())
        .or_insert(style.use_when.clone());
    values
        .entry("style_drawing_rule".to_string())
        .or_insert(style.drawing_rule.clone());
    values
        .entry("style_avoid".to_string())
        .or_insert(style.avoid.clone());
    values
        .entry("style_tags".to_string())
        .or_insert(style.tags.join(" / "));
    values
        .entry("composition_pattern_id".to_string())
        .or_insert(recipe.composition_pattern_id.clone());
    values
        .entry("composition_pattern_name".to_string())
        .or_insert(composition_pattern.name.clone());
    values
        .entry("composition_pattern_use_when".to_string())
        .or_insert(composition_pattern.use_when.clone());
    values
        .entry("composition_pattern_drawing_rule".to_string())
        .or_insert(composition_pattern.drawing_rule.clone());
    values
        .entry("composition_pattern_tags".to_string())
        .or_insert(composition_pattern.tags.join(" / "));
    values
        .entry("prompt_template_id".to_string())
        .or_insert(recipe.prompt_template_id.clone());
    values
        .entry("prompt_template_name".to_string())
        .or_insert(template.name.clone());

    for (key, value) in &recipe.default_variables {
        values.entry(key.clone()).or_insert_with(|| value.clone());
    }

    let prompt = general_illustrations_skill_spec::render_prompt(&template.body, &values)
        .map_err(|error| anyhow!("failed to render prompt template: {error}"))?;

    println!("{}", prompt);
    Ok(())
}

fn find_recipe_components<'a>(
    spec: &'a SkillSpec,
    recipe_id: &'a str,
) -> Result<(
    &'a general_illustrations_skill_spec::RecipeSpec,
    &'a general_illustrations_skill_spec::StyleSpec,
    &'a general_illustrations_skill_spec::CompositionPatternSpec,
    &'a general_illustrations_skill_spec::PromptTemplateSpec,
)> {
    let recipe = spec
        .recipes
        .iter()
        .find(|recipe| recipe.id == recipe_id)
        .ok_or_else(|| anyhow!("missing recipe: {}", recipe_id))?;

    let style = spec
        .styles
        .iter()
        .find(|style| style.id == recipe.style_id)
        .ok_or_else(|| {
            anyhow!(
                "recipe {} references unknown style {}",
                recipe.id,
                recipe.style_id
            )
        })?;

    let composition_pattern = spec
        .composition_patterns
        .iter()
        .find(|pattern| pattern.id == recipe.composition_pattern_id)
        .ok_or_else(|| {
            anyhow!(
                "recipe {} references unknown composition pattern {}",
                recipe.id,
                recipe.composition_pattern_id
            )
        })?;

    let template = spec
        .prompt_templates
        .iter()
        .find(|template| template.id == recipe.prompt_template_id)
        .ok_or_else(|| {
            anyhow!(
                "recipe {} references unknown prompt template {}",
                recipe.id,
                recipe.prompt_template_id
            )
        })?;

    Ok((recipe, style, composition_pattern, template))
}

fn read_prompt_data(path: &Path) -> Result<HashMap<String, String>> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;

    let value: serde_json::Value = serde_json::from_str(&contents)
        .with_context(|| format!("failed to parse {} as JSON", path.display()))?;

    let obj = value.as_object().ok_or_else(|| {
        anyhow!("prompt data must be a JSON object (a map of template variables to values)")
    })?;

    let mut values = HashMap::new();

    for (key, value) in obj {
        let rendered = match value {
            serde_json::Value::String(text) => text.clone(),
            serde_json::Value::Array(values) => values
                .iter()
                .map(render_prompt_value)
                .collect::<Result<Vec<_>, _>>()?
                .join(" / "),
            other => other.to_string(),
        };

        values.insert(key.clone(), rendered);
    }

    Ok(values)
}

fn render_prompt_value(value: &serde_json::Value) -> Result<String> {
    match value {
        serde_json::Value::String(text) => Ok(text.clone()),
        serde_json::Value::Null => Ok(String::new()),
        serde_json::Value::Bool(boolean) => Ok(boolean.to_string()),
        serde_json::Value::Number(number) => Ok(number.to_string()),
        serde_json::Value::Array(values) => values
            .iter()
            .map(render_prompt_value)
            .collect::<Result<Vec<_>, _>>()
            .map(|parts| parts.join(" / ")),
        _ => Err(anyhow!(
            "prompt value must be string, bool, number, null, or a list of these values"
        )),
    }
}

fn validate_skill(args: SkillValidateArgs) -> Result<()> {
    let spec = read_skill_spec(&args.spec)?;
    spec.validate()?;
    println!("valid: {}", spec.name);
    Ok(())
}

fn render_skill(args: SkillRenderArgs) -> Result<()> {
    let spec = read_skill_spec(&args.spec)?;
    let asset_base_dir = args
        .spec
        .parent()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    write_skill(&spec, &args.out, &asset_base_dir, args.copy_assets)
        .with_context(|| format!("failed to render skill to {}", args.out.display()))?;
    println!("{}", args.out.display());
    Ok(())
}

fn read_skill_spec(path: &Path) -> Result<SkillSpec> {
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    serde_json::from_str(&contents).with_context(|| format!("failed to parse {}", path.display()))
}

fn generate(args: GenerateArgs) -> Result<()> {
    let prompt = match (args.prompt, args.prompt_file) {
        (Some(prompt), None) => prompt,
        (None, Some(path)) => fs::read_to_string(&path)
            .with_context(|| format!("failed to read prompt file {}", path.display()))?,
        (Some(_), Some(_)) => bail!("use either --prompt or --prompt-file, not both"),
        (None, None) => bail!("missing --prompt or --prompt-file"),
    };

    let request = ImageGenerationRequest {
        prompt,
        aspect_ratio: parse_aspect_ratio(&args.aspect_ratio),
        output_format: args.output_format.into(),
        n: 1,
    };

    fs::create_dir_all(&args.output_dir)
        .with_context(|| format!("failed to create {}", args.output_dir.display()))?;

    let provider: Box<dyn ImageProvider> = match args.provider {
        ProviderArg::Minimax => {
            let api_key = env::var("MINIMAX_API_KEY").context("MINIMAX_API_KEY must be set")?;
            Box::new(MinimaxImageProvider::new(api_key))
        }
        ProviderArg::Ark => {
            let api_key = env::var("DOUBAO_ARK_AGENT_PLAN_API_KEY")
                .or_else(|_| env::var("ARK_AGENT_PLAN_API_KEY"))
                .context("DOUBAO_ARK_AGENT_PLAN_API_KEY or ARK_AGENT_PLAN_API_KEY must be set")?;
            let provider = ArkImageProvider::new(api_key);
            let provider = match env::var("ARK_AGENT_PLAN_IMAGE_ENDPOINT") {
                Ok(endpoint) => provider.with_endpoint(endpoint),
                Err(_) => provider,
            };
            let provider = match env::var("ARK_AGENT_PLAN_IMAGE_MODEL") {
                Ok(model) => provider.with_model(model),
                Err(_) => provider,
            };
            Box::new(provider)
        }
    };

    let images = provider.generate(&request)?;
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("system time before UNIX_EPOCH")?
        .as_secs();

    for (index, image) in images.iter().enumerate() {
        let path = args.output_dir.join(format!(
            "{}-{timestamp}-{index}.{}",
            args.output_prefix,
            image.output_format.extension()
        ));
        fs::write(&path, &image.bytes)
            .with_context(|| format!("failed to write {}", path.display()))?;
        println!("{}", path.display());
    }

    Ok(())
}

fn parse_aspect_ratio(value: &str) -> AspectRatio {
    match value {
        "1:1" => AspectRatio::Square,
        "16:9" => AspectRatio::Landscape16x9,
        "9:16" => AspectRatio::Portrait9x16,
        other => AspectRatio::Custom(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_aspect_ratios() {
        // Arrange
        let values = ["1:1", "16:9", "9:16"];

        // Act
        let parsed = values.map(parse_aspect_ratio);

        // Assert
        assert_eq!(
            parsed,
            [
                AspectRatio::Square,
                AspectRatio::Landscape16x9,
                AspectRatio::Portrait9x16
            ]
        );
    }

    #[test]
    fn preserves_custom_aspect_ratio() {
        // Arrange
        let value = "4:3";

        // Act
        let parsed = parse_aspect_ratio(value);

        // Assert
        assert_eq!(parsed, AspectRatio::Custom(value.to_string()));
    }
}
