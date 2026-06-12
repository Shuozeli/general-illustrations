use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkillSpec {
    pub schema_version: u32,
    pub name: String,
    pub display_name: String,
    pub short_description: String,
    pub description: String,
    pub default_prompt: String,
    pub positioning: PositioningSpec,
    pub style_dna: StyleDnaSpec,
    pub references: ReferenceSpec,
    pub workflow: Vec<WorkflowStepSpec>,
    pub styles: Vec<StyleSpec>,
    pub composition_patterns: Vec<CompositionPatternSpec>,
    pub prompt_templates: Vec<PromptTemplateSpec>,
    pub qa: QaSpec,
    #[serde(default)]
    pub examples: Vec<ExampleAssetSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PositioningSpec {
    pub title: String,
    pub body: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleDnaSpec {
    pub one_liner: String,
    pub must: Vec<String>,
    pub colors: Vec<String>,
    pub text_rules: Vec<String>,
    pub never: Vec<String>,
    pub aesthetic_direction: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceSpec {
    pub intro: String,
    pub items: Vec<ReferenceItemSpec>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceItemSpec {
    pub path: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkflowStepSpec {
    pub title: String,
    pub body: Vec<String>,
    #[serde(default)]
    pub bullets: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StyleSpec {
    pub id: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub name: String,
    pub use_when: String,
    pub drawing_rule: String,
    pub avoid: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompositionPatternSpec {
    pub id: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub name: String,
    pub use_when: String,
    pub drawing_rule: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptTemplateSpec {
    pub id: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub name: String,
    pub body: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QaSpec {
    pub must_pass: Vec<String>,
    pub failure_signals: Vec<String>,
    pub iteration_rules: Vec<IterationRuleSpec>,
    pub delivery_judgment: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IterationRuleSpec {
    pub problem: String,
    pub fix: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExampleAssetSpec {
    pub id: String,
    pub source_path: String,
    pub target_path: String,
    pub caption: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("invalid skill spec: {problems}")]
pub struct ValidationError {
    problems: ValidationProblems,
}

impl ValidationError {
    pub fn problems(&self) -> &[String] {
        self.problems.as_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidationProblems(Vec<String>);

impl ValidationProblems {
    fn as_slice(&self) -> &[String] {
        &self.0
    }
}

impl std::fmt::Display for ValidationProblems {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0.join("; "))
    }
}

impl SkillSpec {
    pub fn validate(&self) -> Result<(), ValidationError> {
        let mut problems = Vec::new();

        if self.schema_version != 1 {
            problems.push(format!(
                "schema_version must be 1, got {}",
                self.schema_version
            ));
        }

        if !is_kebab_case(&self.name) {
            problems.push("name must be lowercase kebab-case".to_string());
        }

        if self.display_name.trim().is_empty() {
            problems.push("display_name must not be empty".to_string());
        }

        if self.short_description.trim().is_empty() {
            problems.push("short_description must not be empty".to_string());
        }

        if self.description.trim().is_empty() {
            problems.push("description must not be empty".to_string());
        }

        let skill_reference = format!("${}", self.name);
        if !self.default_prompt.contains(&skill_reference) {
            problems.push(format!(
                "default_prompt must mention {skill_reference} so agents can invoke the skill"
            ));
        }

        if self.positioning.body.is_empty() {
            problems.push("positioning.body must contain at least one paragraph".to_string());
        }

        if self.style_dna.must.is_empty() {
            problems.push("style_dna.must must contain at least one item".to_string());
        }

        if self.references.items.is_empty() {
            problems.push("references.items must contain at least one reference".to_string());
        }

        if self.workflow.is_empty() {
            problems.push("workflow must contain at least one step".to_string());
        }

        if self.styles.is_empty() {
            problems.push("styles must contain at least one style".to_string());
        }
        validate_unique_ids(
            "styles",
            self.styles.iter().map(|style| &style.id),
            &mut problems,
        );
        for style in &self.styles {
            validate_tags(&format!("styles.{}", style.id), &style.tags, &mut problems);
        }

        if self.composition_patterns.is_empty() {
            problems.push("composition_patterns must contain at least one pattern".to_string());
        }
        validate_unique_ids(
            "composition_patterns",
            self.composition_patterns.iter().map(|pattern| &pattern.id),
            &mut problems,
        );
        for pattern in &self.composition_patterns {
            validate_tags(
                &format!("composition_patterns.{}", pattern.id),
                &pattern.tags,
                &mut problems,
            );
        }

        if self.prompt_templates.is_empty() {
            problems.push("prompt_templates must contain at least one template".to_string());
        }
        validate_unique_ids(
            "prompt_templates",
            self.prompt_templates.iter().map(|template| &template.id),
            &mut problems,
        );
        for template in &self.prompt_templates {
            validate_tags(
                &format!("prompt_templates.{}", template.id),
                &template.tags,
                &mut problems,
            );
        }

        if self.qa.must_pass.is_empty() {
            problems.push("qa.must_pass must contain at least one item".to_string());
        }

        if self.qa.failure_signals.is_empty() {
            problems.push("qa.failure_signals must contain at least one item".to_string());
        }

        validate_unique_ids(
            "examples",
            self.examples.iter().map(|example| &example.id),
            &mut problems,
        );

        if problems.is_empty() {
            Ok(())
        } else {
            Err(ValidationError {
                problems: ValidationProblems(problems),
            })
        }
    }
}

fn validate_tags(label: &str, tags: &[String], problems: &mut Vec<String>) {
    let mut seen = HashSet::new();

    for tag in tags {
        if !is_kebab_case(tag) {
            problems.push(format!("{label}.tags.{tag} must be lowercase kebab-case"));
        }

        if !seen.insert(tag) {
            problems.push(format!("{label}.tags.{tag} is duplicated"));
        }
    }
}

fn validate_unique_ids<'a>(
    label: &str,
    ids: impl Iterator<Item = &'a String>,
    problems: &mut Vec<String>,
) {
    let mut seen = HashSet::new();

    for id in ids {
        if !is_kebab_case(id) {
            problems.push(format!("{label}.{id} must be lowercase kebab-case"));
        }

        if !seen.insert(id) {
            problems.push(format!("{label}.{id} is duplicated"));
        }
    }
}

fn is_kebab_case(value: &str) -> bool {
    let mut previous_was_dash = false;
    let mut saw_alphanumeric = false;

    for character in value.chars() {
        match character {
            'a'..='z' | '0'..='9' => {
                previous_was_dash = false;
                saw_alphanumeric = true;
            }
            '-' if saw_alphanumeric && !previous_was_dash => previous_was_dash = true,
            _ => return false,
        }
    }

    saw_alphanumeric && !previous_was_dash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_spec() {
        // Arrange
        let spec = valid_spec();

        // Act
        let result = spec.validate();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn rejects_missing_skill_reference_in_default_prompt() {
        // Arrange
        let mut spec = valid_spec();
        spec.default_prompt = "Generate an illustration.".to_string();

        // Act
        let error = spec.validate().unwrap_err();

        // Assert
        assert!(
            error.problems().contains(
                &"default_prompt must mention $example-skill so agents can invoke the skill"
                    .to_string()
            )
        );
    }

    #[test]
    fn rejects_duplicate_style_ids() {
        // Arrange
        let mut spec = valid_spec();
        spec.styles.push(spec.styles[0].clone());

        // Act
        let error = spec.validate().unwrap_err();

        // Assert
        assert!(
            error
                .problems()
                .contains(&"styles.clean-docs is duplicated".to_string())
        );
    }

    #[test]
    fn rejects_invalid_tags() {
        // Arrange
        let mut spec = valid_spec();
        spec.styles[0].tags = vec!["Video".to_string(), "Video".to_string()];

        // Act
        let error = spec.validate().unwrap_err();

        // Assert
        assert!(
            error
                .problems()
                .contains(&"styles.clean-docs.tags.Video must be lowercase kebab-case".to_string())
        );
    }

    fn valid_spec() -> SkillSpec {
        SkillSpec {
            schema_version: 1,
            name: "example-skill".to_string(),
            display_name: "Example Skill".to_string(),
            short_description: "Example illustration recipes".to_string(),
            description: "Generate structured illustration skills.".to_string(),
            default_prompt: "Use $example-skill for this document.".to_string(),
            positioning: PositioningSpec {
                title: "核心定位".to_string(),
                body: vec!["为正文生成解释图。".to_string()],
            },
            style_dna: StyleDnaSpec {
                one_liner: "清爽、可读。".to_string(),
                must: vec!["16:9 横版。".to_string()],
                colors: vec!["橙色：主路径。".to_string()],
                text_rules: vec!["短标注。".to_string()],
                never: vec!["不要 PPT。".to_string()],
                aesthetic_direction: vec!["要清楚。".to_string()],
            },
            references: ReferenceSpec {
                intro: "按任务需要读取。".to_string(),
                items: vec![ReferenceItemSpec {
                    path: "references/styles.md".to_string(),
                    description: "预置风格。".to_string(),
                }],
            },
            workflow: vec![WorkflowStepSpec {
                title: "消化正文".to_string(),
                body: vec!["先理解内容。".to_string()],
                bullets: vec!["核心观点是什么".to_string()],
            }],
            styles: vec![StyleSpec {
                id: "clean-docs".to_string(),
                tags: vec!["article".to_string(), "docs".to_string()],
                name: "clean-docs".to_string(),
                use_when: "文档配图。".to_string(),
                drawing_rule: "白底短标注。".to_string(),
                avoid: "营销海报。".to_string(),
            }],
            composition_patterns: vec![CompositionPatternSpec {
                id: "workflow".to_string(),
                tags: vec!["flow".to_string()],
                name: "Workflow 流程".to_string(),
                use_when: "输入到输出。".to_string(),
                drawing_rule: "左到右。".to_string(),
            }],
            prompt_templates: vec![PromptTemplateSpec {
                id: "single-image".to_string(),
                tags: vec!["image".to_string()],
                name: "生图提示词模板".to_string(),
                body: "Generate one image.".to_string(),
            }],
            qa: QaSpec {
                must_pass: vec!["是 16:9 横版。".to_string()],
                failure_signals: vec!["太像 PPT。".to_string()],
                iteration_rules: vec![IterationRuleSpec {
                    problem: "太复杂".to_string(),
                    fix: "删节点".to_string(),
                }],
                delivery_judgment: "一秒看懂。".to_string(),
            },
            examples: Vec::new(),
        }
    }
}
