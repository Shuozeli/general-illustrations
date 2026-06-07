use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
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
            Ok(())
        }
        Command::Skill(args) => skill(args),
    }
}

fn skill(args: SkillArgs) -> Result<()> {
    match args.command {
        SkillCommand::Validate(args) => validate_skill(args),
        SkillCommand::Render(args) => render_skill(args),
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
