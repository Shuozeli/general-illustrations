use std::fs;
use std::path::{Path, PathBuf};

use general_illustrations_skill_spec::SkillSpec;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedSkill {
    pub files: Vec<RenderedFile>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedFile {
    pub path: PathBuf,
    pub contents: String,
}

#[derive(Debug, Error)]
pub enum RenderError {
    #[error(transparent)]
    Validation(#[from] general_illustrations_skill_spec::ValidationError),
    #[error("failed to write {path}: {source}")]
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to copy asset from {source_path} to {target_path}: {source}")]
    CopyAsset {
        source_path: PathBuf,
        target_path: PathBuf,
        source: std::io::Error,
    },
}

pub fn render_skill(spec: &SkillSpec) -> Result<RenderedSkill, RenderError> {
    spec.validate()?;

    Ok(RenderedSkill {
        files: vec![
            RenderedFile {
                path: PathBuf::from("SKILL.md"),
                contents: normalize_contents(render_skill_md(spec)),
            },
            RenderedFile {
                path: PathBuf::from("agents/openai.yaml"),
                contents: normalize_contents(render_openai_yaml(spec)),
            },
            RenderedFile {
                path: PathBuf::from("references/style-dna.md"),
                contents: normalize_contents(render_style_dna(spec)),
            },
            RenderedFile {
                path: PathBuf::from("references/styles.md"),
                contents: normalize_contents(render_styles(spec)),
            },
            RenderedFile {
                path: PathBuf::from("references/composition-patterns.md"),
                contents: normalize_contents(render_composition_patterns(spec)),
            },
            RenderedFile {
                path: PathBuf::from("references/prompt-template.md"),
                contents: normalize_contents(render_prompt_templates(spec)),
            },
            RenderedFile {
                path: PathBuf::from("references/qa-checklist.md"),
                contents: normalize_contents(render_qa(spec)),
            },
        ],
    })
}

pub fn write_skill(
    spec: &SkillSpec,
    output_dir: &Path,
    asset_base_dir: &Path,
    copy_assets: bool,
) -> Result<(), RenderError> {
    let rendered = render_skill(spec)?;

    for file in rendered.files {
        let path = output_dir.join(file.path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|source| RenderError::Write {
                path: parent.to_path_buf(),
                source,
            })?;
        }
        fs::write(&path, file.contents).map_err(|source| RenderError::Write { path, source })?;
    }

    if copy_assets {
        for example in &spec.examples {
            let source_path = asset_base_dir.join(&example.source_path);
            let target_path = output_dir.join(&example.target_path);
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent).map_err(|source| RenderError::Write {
                    path: parent.to_path_buf(),
                    source,
                })?;
            }
            fs::copy(&source_path, &target_path).map_err(|source| RenderError::CopyAsset {
                source_path,
                target_path,
                source,
            })?;
        }
    }

    Ok(())
}

fn render_skill_md(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "---");
    push_line(&mut output, &format!("name: {}", spec.name));
    push_line(
        &mut output,
        &format!("description: {}", yaml_string(&spec.description)),
    );
    push_line(&mut output, "---");
    push_line(&mut output, "");
    push_line(&mut output, &format!("# {}", spec.display_name));
    push_line(&mut output, "");
    push_line(&mut output, &format!("## {}", spec.positioning.title));
    push_line(&mut output, "");
    for paragraph in &spec.positioning.body {
        push_paragraph(&mut output, paragraph);
    }
    push_line(&mut output, "## 先读这些参考");
    push_line(&mut output, "");
    push_paragraph(&mut output, &spec.references.intro);
    for item in &spec.references.items {
        push_line(
            &mut output,
            &format!("- `{}`：{}", item.path, item.description),
        );
    }
    push_line(&mut output, "");
    push_line(&mut output, "## 工作流");
    push_line(&mut output, "");
    for (index, step) in spec.workflow.iter().enumerate() {
        push_line(&mut output, &format!("### {}. {}", index + 1, step.title));
        push_line(&mut output, "");
        for paragraph in &step.body {
            push_paragraph(&mut output, paragraph);
        }
        for bullet in &step.bullets {
            push_line(&mut output, &format!("- {bullet}"));
        }
        push_line(&mut output, "");
    }
    push_line(&mut output, "## 输出口径");
    push_line(&mut output, "");
    push_line(
        &mut output,
        "生成前的策略输出要短而准。生成后的交付要包含：",
    );
    push_line(&mut output, "");
    push_line(&mut output, "- 生成了几张");
    push_line(&mut output, "- 每张图的用途");
    push_line(&mut output, "- 保存路径");
    push_line(&mut output, "- 哪些图最稳，哪些图是可选");
    push_line(&mut output, "");
    push_line(&mut output, "不要长篇解释风格理论；让图自己说话。");
    output
}

fn render_openai_yaml(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "interface:");
    push_line(
        &mut output,
        &format!("  display_name: {}", yaml_string(&spec.display_name)),
    );
    push_line(
        &mut output,
        &format!(
            "  short_description: {}",
            yaml_string(&spec.short_description)
        ),
    );
    push_line(
        &mut output,
        &format!("  default_prompt: {}", yaml_string(&spec.default_prompt)),
    );
    output
}

fn render_style_dna(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "# 风格 DNA");
    push_line(&mut output, "");
    push_line(&mut output, "## 一句话");
    push_line(&mut output, "");
    push_paragraph(&mut output, &spec.style_dna.one_liner);
    push_list_section(&mut output, "必须", &spec.style_dna.must);
    push_list_section(&mut output, "颜色", &spec.style_dna.colors);
    push_list_section(&mut output, "文字", &spec.style_dna.text_rules);
    push_list_section(&mut output, "绝对不要", &spec.style_dna.never);
    push_line(&mut output, "## 审美方向");
    push_line(&mut output, "");
    for paragraph in &spec.style_dna.aesthetic_direction {
        push_paragraph(&mut output, paragraph);
    }
    output
}

fn render_styles(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "# 预置风格");
    push_line(&mut output, "");
    push_line(&mut output, "选择一个风格即可，不要在一张图里混太多风格。");
    push_line(&mut output, "");

    for style in &spec.styles {
        push_line(&mut output, &format!("## {}", style.name));
        push_line(&mut output, "");
        push_line(&mut output, &format!("适合：{}", style.use_when));
        push_line(&mut output, "");
        push_line(&mut output, &format!("画法：{}", style.drawing_rule));
        push_line(&mut output, "");
        push_line(&mut output, &format!("避免：{}", style.avoid));
        push_line(&mut output, "");
    }

    output
}

fn render_composition_patterns(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "# 构图模式与原创规则");
    push_line(&mut output, "");
    push_line(&mut output, "## 基础结构类型");
    push_line(&mut output, "");
    push_line(&mut output, "选择一种结构即可，不要混太多。");
    push_line(&mut output, "");

    for pattern in &spec.composition_patterns {
        push_line(&mut output, &format!("### {}", pattern.name));
        push_line(&mut output, "");
        push_line(&mut output, &format!("适合：{}", pattern.use_when));
        push_line(&mut output, "");
        push_line(&mut output, &format!("画法：{}", pattern.drawing_rule));
        push_line(&mut output, "");
    }

    push_line(&mut output, "## 原创隐喻生成法");
    push_line(&mut output, "");
    push_line(&mut output, "每次都从当前内容重新发明隐喻，不能照搬旧图。");
    push_line(&mut output, "");
    push_line(&mut output, "### 三步");
    push_line(&mut output, "");
    push_line(
        &mut output,
        "1. 把抽象概念换成一个物理动作：收集、倾倒、压平、合并、过滤、跳过、折叠、分拣、沉淀、锁住。",
    );
    push_line(
        &mut output,
        "2. 把系统结构换成一个低科技物件：纸箱、托盘、抽屉、货架、滚筒、水管、邮筒、黑盒、账本、筛子、篮子。",
    );
    push_line(
        &mut output,
        "3. 让核心动作可见：谁在写入、谁在整理、哪里变快、哪里变慢、哪里是后台发生。",
    );
    push_line(&mut output, "");
    push_line(&mut output, "### 可用物件池");
    push_line(&mut output, "");
    push_line(
        &mut output,
        "- 纸箱、抽屉、托盘、货架、账本、漏斗、秤、邮筒、门、井、梯子、水管、线团、闸门、转盘、黑盒、打孔器、压面机、滚筒、篮子、标签贴。",
    );
    push_line(&mut output, "- 用时只选 1-3 个，不要堆满。");
    push_line(&mut output, "");
    push_line(&mut output, "## 反复刻规则");
    push_line(&mut output, "");
    push_line(
        &mut output,
        "不要默认打开或复刻 `assets/examples/`。这些图片只用于风格校准，例如线条密度、留白、颜色克制、文字量、图层清晰度。",
    );
    push_line(&mut output, "");
    push_line(
        &mut output,
        "除非用户明确说“照这张 / 复刻这个构图 / 用这个案例改”，否则不要直接复用旧构图。",
    );
    push_line(&mut output, "");
    push_line(
        &mut output,
        "同类主题也要换新隐喻。例如 LSM Tree 不一定总画货架，也可以画托盘、账本、滚筒、过滤器；RPC 不一定总画线缆，也可以画远程阀门、邮筒、遥控器。",
    );
    output
}

fn render_prompt_templates(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "# 生图提示词模板");
    push_line(&mut output, "");
    push_line(
        &mut output,
        "每张图单独生成。根据正文内容替换变量，不要把多张图拼在一起。",
    );
    push_line(&mut output, "");

    for template in &spec.prompt_templates {
        push_line(&mut output, &format!("## {}", template.name));
        push_line(&mut output, "");
        push_line(&mut output, "```text");
        push_line(&mut output, template.body.trim());
        push_line(&mut output, "```");
        push_line(&mut output, "");
    }

    output
}

fn render_qa(spec: &SkillSpec) -> String {
    let mut output = String::new();
    push_line(&mut output, "# QA Checklist");
    push_line(&mut output, "");
    push_line(&mut output, "## 必过项");
    push_line(&mut output, "");
    for item in &spec.qa.must_pass {
        push_line(&mut output, &format!("- {item}"));
    }
    push_line(&mut output, "");
    push_line(&mut output, "## 失败信号");
    push_line(&mut output, "");
    push_line(&mut output, "出现以下情况，重生成或局部编辑：");
    push_line(&mut output, "");
    for item in &spec.qa.failure_signals {
        push_line(&mut output, &format!("- {item}"));
    }
    push_line(&mut output, "");
    push_line(&mut output, "## 迭代方法");
    push_line(&mut output, "");
    for rule in &spec.qa.iteration_rules {
        push_line(&mut output, &format!("- {}：{}", rule.problem, rule.fix));
    }
    push_line(&mut output, "");
    push_line(&mut output, "## 交付判断");
    push_line(&mut output, "");
    push_line(&mut output, &spec.qa.delivery_judgment);
    output
}

fn push_paragraph(output: &mut String, paragraph: &str) {
    push_line(output, paragraph);
    push_line(output, "");
}

fn push_list_section(output: &mut String, title: &str, items: &[String]) {
    push_line(output, &format!("## {title}"));
    push_line(output, "");
    for item in items {
        push_line(output, &format!("- {item}"));
    }
    push_line(output, "");
}

fn push_line(output: &mut String, line: &str) {
    output.push_str(line);
    output.push('\n');
}

fn yaml_string(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

fn normalize_contents(contents: String) -> String {
    format!("{}\n", contents.trim_end())
}

#[cfg(test)]
mod tests {
    use general_illustrations_skill_spec::{
        CompositionPatternSpec, IterationRuleSpec, PositioningSpec, PromptTemplateSpec, QaSpec,
        ReferenceItemSpec, ReferenceSpec, SkillSpec, StyleDnaSpec, StyleSpec, WorkflowStepSpec,
    };

    use super::*;

    #[test]
    fn renders_skill_markdown_and_reference_files() {
        // Arrange
        let spec = valid_spec();

        // Act
        let rendered = render_skill(&spec).unwrap();

        // Assert
        assert_eq!(rendered.files.len(), 7);
        assert_eq!(rendered.files[0].path, PathBuf::from("SKILL.md"));
        assert!(rendered.files[0].contents.contains("name: example-skill"));
        assert!(
            rendered
                .files
                .iter()
                .any(|file| file.path.as_path() == Path::new("references/styles.md"))
        );
    }

    #[test]
    fn renders_openai_yaml_with_default_prompt() {
        // Arrange
        let spec = valid_spec();

        // Act
        let rendered = render_skill(&spec).unwrap();
        let yaml = rendered
            .files
            .iter()
            .find(|file| file.path.as_path() == Path::new("agents/openai.yaml"))
            .unwrap();

        // Assert
        assert!(
            yaml.contents
                .contains("default_prompt: \"Use $example-skill for this document.\"")
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
                name: "clean-docs".to_string(),
                use_when: "文档配图。".to_string(),
                drawing_rule: "白底短标注。".to_string(),
                avoid: "营销海报。".to_string(),
            }],
            composition_patterns: vec![CompositionPatternSpec {
                id: "workflow".to_string(),
                name: "Workflow 流程".to_string(),
                use_when: "输入到输出。".to_string(),
                drawing_rule: "左到右。".to_string(),
            }],
            prompt_templates: vec![PromptTemplateSpec {
                id: "single-image".to_string(),
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
