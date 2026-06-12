# 生图提示词模板

每张图单独生成。根据正文内容替换变量，不要把多张图拼在一起。

## 生图提示词模板

Tags: image / general / article / video

```text
Generate one standalone 16:9 horizontal Chinese article or knowledge-video illustration.

Visual style:
{选择的预置风格：clean-docs / technical-minimal / review-minimal / editorial-sketch / product-explainer / simple-doodle / cartoon-explainer / coupon-worker-comic / yellow-worker-comic / whiteboard-comic / sticker-layer / soft-clay-cartoon}

Visual DNA:
Clean readable composition. White or very light background. One core idea only.
Sparse labels. Strong empty space. Clear foreground/midground/background if the
image is video-ready. Use orange for main action, blue for system/read/state,
red only for warning/problem/result.

Theme:
{正文配图主题}

Structure type:
{Workflow / 系统局部 / 前后对比 / 角色状态 / 概念隐喻 / 方法分层 / 地图路线 / 小漫画分镜 / 动作循环场景}

Core idea:
{这张图要表达的核心意思}

Composition:
{具体画面：主体在哪里、正在发生什么、主要物件是什么、信息如何流动}

Suggested elements:
{元素1} / {元素2} / {元素3} / {元素4}

Short labels:
{标注词1} / {标注词2} / {标注词3} / {标注词4} / {可选标注词5}

Constraints:
One image explains only one core structure. Keep the main subject around 40%-60%
of the canvas. Preserve enough blank space. Use at most 3-8 short labels. Do not
write a title in the top-left corner. Do not write the structure type on the
image. Do not make it a formal PPT slide, dense architecture, stock art, or
commercial illustration. Do not copy prior examples; invent a fresh visual
metaphor for this specific content.
```

## 原创黄色小人动作漫画短视频画面模板

Tags: video / business / finance / tech-news / comic

```text
Generate one 16:9 horizontal knowledge-video frame in the yellow-worker-comic style.

Topic:
{财经、商业、消费、科技、公司新闻、AI 基建、供应链、监管、估值或商业模式主题}

Core mechanism:
{这张图要解释的机制，例如 capital expenditure / pricing power / price war / margin pressure / valuation demand / regulatory risk / cash-flow lock-in}

Main action:
{只选一个动作：building a data center / carrying cash into servers / pulling fiber cables / turning a price dial / sorting company cards / pushing away noisy headlines / inflating a valuation balloon / comparing revenue and margin piles}

Composition:
White or very light background. One large foreground action with original yellow worker characters. The main character or object should occupy 50%-70% of the canvas. Leave clean space for subtitles and later title overlay. Use black hand-drawn outlines, flat yellow bodies, small limbs, tiny dot eyes, simple props, and visible motion lines. Make it feel like an action beat from a smart business or finance explainer, not a poster.

Short labels only:
{0-4 readable labels such as AI CAPEX / FIBER / MARGIN / PRICE WAR / RISK / CASH FLOW; labels are optional and should be skipped when overlay text will be added later}

Animation beats:
{beat 1} -> {beat 2} -> {beat 3}; the still image should imply these beats clearly.

Constraints:
This must be original character design, not Minions. No goggles, no blue overalls, no capsule bodies, no one-eyed mascot, no denim, no Gru/Minions-style design, no 3D toy rendering. Avoid coupon-only props unless the topic is actually about coupons. Avoid large title text, left-side text blocks, Markdown-like explanations, screenshots, UI mockups, and dense labels. The story must be readable through action first and text second.
```

## 技术图提示词

Tags: image / technical / systems / diagram

```text
Generate one clean 16:9 technical explainer diagram.

Topic:
{技术主题}

Entities:
{实体列表}

Flow:
{写入路径 / 读取路径 / 状态变化 / 风险路径}

Highlight:
{最重要的边界、tradeoff 或机制}

Style:
technical-minimal, clean-docs, or whiteboard-comic.

Labels only:
{短标注列表}

Constraints:
Use precise boxes, arrows, shelves, layers, or paths. Do not invent components.
Do not add long paragraphs. Make it useful for a technical blog or video.
```

## 图像编辑提示：去掉左上角标题

Tags: edit / cleanup / text-removal

```text
Edit the provided image. Remove only the title "{要删除的文字}" and its underline
from the top-left corner. Fill that area with the same clean background. Preserve
everything else exactly: objects, labels, paths, line style, composition, aspect
ratio, and image quality. Do not add any new text or objects.
```

## 图像编辑提示：降低幼稚感

Tags: edit / style-adjustment / cartoon

```text
Regenerate this illustration with the same core meaning and layout, but make it
less cute and more like a clean technical explainer. Keep rounded simplified
objects, but reduce toy-like expressions, bright candy colors, and mascot energy.
Use sparse labels and a clean white background.
```
