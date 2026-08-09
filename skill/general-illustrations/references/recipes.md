# 配方目录

配方把类别、风格、默认构图、提示词模板和 provider 提示词绑成一个可选单元。选一个配方 id，即可拿到风格 + 构图 + 模板 + Gemini 提示词，不必手动配对。

## 文章 / 技术文档 (`article-docs`)

### clean-doc (`clean-doc`)

Tags: article / docs

适合：文章、README、产品解释、普通文章配图。

- 风格 style：`clean-docs`
- 默认构图 composition：`concept-metaphor`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one clean 16:9 horizontal explainer illustration. White or very light background, a few simple geometric shapes, clear hierarchy, restrained palette (mostly black lines with one orange accent for the main path). Flat, minimal, no marketing gloss. No text, no words, no logos, no watermark. Scene: {scene}
```

### tech-diagram (`tech-diagram`)

Tags: technical / systems

适合：系统结构、RPC、数据库、LSM Tree、架构边界、数据流。

- 风格 style：`technical-minimal`
- 默认构图 composition：`system-slice`
- 提示词模板 template：`technical-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one clean 16:9 horizontal technical explainer diagram. White background, precise boxes, arrows, layers, shelves and storage objects, clear boundaries. Black lines, orange for the write/main path, blue for read/state. Accuracy over decoration; do not invent components. Only unavoidable short labels. No text, no words, no logos, no watermark. Scene: {scene}
```

### code-review (`code-review`)

Tags: code-review / risk

适合：代码 review、bug path、风险传播、测试缺口、before/after。

- 风格 style：`review-minimal`
- 默认构图 composition：`before-after`
- 提示词模板 template：`technical-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal code-review illustration. Neutral light background, one red risk/bad-path point and one blue fix/verify point, the bad path emphasized. Flat, minimal, black lines. No large code blocks, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### article-sketch (`article-sketch`)

Tags: article / editorial / sketch

适合：文章观点、方法论、抽象隐喻、认知转折。

- 风格 style：`editorial-sketch`
- 默认构图 composition：`concept-metaphor`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal editorial sketch illustration. Hand-drawn feel, slightly wobbly lines, large empty space, one concrete physical metaphor, sparse labels. Black ink lines with one restrained accent color. No PPT look, no decorative-only art, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### product-explainer (`product-explainer`)

Tags: product / saas / explainer

适合：产品能力、feature 介绍、工具说明、docs intro。

- 风格 style：`product-explainer`
- 默认构图 composition：`layered-method`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal product-explainer illustration. Quiet SaaS/docs feel, a few simplified abstract feature blocks, calm restrained palette. Flat, clean, generous empty space. No fake UI detail, no landing-page hero, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### whiteboard-story (`whiteboard-story`)

Tags: engineering / teaching / comic / whiteboard

适合：工程故事、bug 解释、before/after、失败到成功、教学分镜。

- 风格 style：`whiteboard-comic`
- 默认构图 composition：`comic-panels`
- 提示词模板 template：`technical-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal whiteboard comic. Black marker line art on white, 2-4 simple panels, a few arrows and action lines, one problem and one result. Not cute, no long handwriting. No text, no words, no logos, no watermark. Scene: {scene}
```

## 轻量短视频 (`video-light`)

### doodle-light (`doodle-light`)

Tags: video / doodle / lightweight

适合：短视频知识图、轻量技术解释、简笔画。

- 风格 style：`simple-doodle`
- 默认构图 composition：`concept-metaphor`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal simple doodle illustration for a knowledge video. White background, marker-style lines, a few simple objects, lots of empty space, 1-2 accent colors. Not messy, not childish. No text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### sticker-layers (`sticker-layers`)

Tags: video / sticker / layered

适合：短视频动画拆层、组件介绍、流程分解。

- 风格 style：`sticker-layer`
- 默认构图 composition：`layered-method`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal flat sticker-style illustration with clear layering. Flat sticker objects with clean outlines, distinct foreground/midground/background so elements can pop in as animation layers. White background, restrained palette. No glossy 3D, no busy collage, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### clay-metaphor (`clay-metaphor`)

Tags: video / clay / metaphor

适合：概念隐喻、视频插图、轻松技术解释。

- 风格 style：`soft-clay-cartoon`
- 默认构图 composition：`concept-metaphor`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal soft clay/toy-like cartoon illustration. Rounded soft clay objects, simple lighting, big clear shapes, little text. Not candy-colored, not photographic, not a busy scene. No text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

## 角色动作漫画 (`video-character-comic`)

### software-news (`software-news`)

Tags: video / software / cartoon / character

适合：软件新闻、AI 工程、平台、开发者工具解释视频。

- 风格 style：`cartoon-explainer`
- 默认构图 composition：`comic-panels`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal flat 2D cartoon-explainer illustration. White background, black hand-drawn outlines, an original friendly cartoon worker character with a blue cap and blue overalls (original design, NOT a fixed IP, NOT a cutesy mascot). Use only blue and orange; orange blocks/arrows = data/action/flow. Flat 2D, one clear core action, subject 40-60% of canvas, clean space for later subtitles. No text, no words, no logos, no watermark. Scene: {scene}
```

- 软件新闻解释频道默认配方；蓝帽蓝背带小工人 + 蓝橙双色。

### consumer-coupon (`consumer-coupon`)

Tags: video / consumer / comic / character / coupon

适合：优惠券、订阅、外卖平台、会员价、动态定价、消费心理短视频。

- 风格 style：`coupon-worker-comic`
- 默认构图 composition：`action-loop-scenes`
- 提示词模板 template：`single-image`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal flat 2D comic illustration. White or very light background, thick black hand-drawn outlines, original yellow coupon/receipt/bean-shaped worker characters, short limbs, dot eyes, exaggerated action (NOT Minions: no goggles, no blue overalls, no capsule body, no single big eye, no denim). One core action about pricing/coupons/subscriptions; props carry the story; at most a few short labels. No poster title, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### finance-news (`finance-news`)

Tags: video / finance / business / tech-news / comic / character

适合：财经、公司新闻、财报解释、AI 基建、供应链、监管、估值、商业模式视频。

- 风格 style：`yellow-worker-comic`
- 默认构图 composition：`action-loop-scenes`
- 提示词模板 template：`yellow-worker-video-frame`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal flat 2D comic illustration. White or very light background, thick black hand-drawn outlines, original yellow worker characters with short limbs and dot eyes, exaggerated action (NOT Minions: no goggles, no blue overalls, no capsule body, no single big eye, no denim, no Gru/Minions look, no 3D toy render). One core business action (carrying cash into servers, building a data center, pulling fiber, turning a price dial, sorting company cards, pushing away noisy headlines, inflating a valuation balloon). Controlled palette: black lines, orange for action/flow, blue for system/state, red only for risk. Subject 50-70% of canvas, clean space for later title/subtitle. No text, no words, no logos, no watermark. Scene: {scene}
```

- 财经/财报视频默认配方（美投侃新闻风）。
- 不要再套全局 photorealistic/cinematic 包装 —— 那是 Berkshire 事故根因。

## 梗感节奏动画 (`video-meme-motion`)

### infra-meme (`infra-meme`)

Tags: video / tech / meme / comic / infrastructure

适合：Kafka 堆积、下单链路卡死、consumer 追 offset、Redis 热点、慢查询等系统机制。

- 风格 style：`mambo-meme-comic`
- 默认构图 composition：`action-loop-scenes`
- 提示词模板 template：`mambo-meme-video-frame`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 horizontal flat 2D meme-rhythm illustration. White or very light background, thick black outlines, original abstract little characters or system components doing a repeatable beat action (requests wobble into a queue, a service shakes under pressure, a Kafka log conveyor carries events one slot at a time). Big empty space, main action 50-70%, a few high-saturation red/yellow/blue beat accents. The technical mechanism (queue, log, consumer, offset, cache) must stay readable. No existing meme/IP/emoji, no nightclub dancing, no danmaku, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

## 古典文学关键帧 (`literary-period`)

### honglou-keyframe (`honglou-keyframe`)

Tags: video / anime / literary / honglou

适合：红楼梦 / 古典文学短视频关键帧、连续分镜、演员表演感镜头。

- 风格 style：`honglou-retro-flat-anime`
- 默认构图 composition：`state-scenes`
- 提示词模板 template：`honglou-anime-video-frame`

Gemini 提示词（把 `{scene}` 换成当前画面）：

```text
Generate one 16:9 full-bleed flat retro 2D TV-anime cel keyframe, no border, no margins. Simplified original Qing-era-inspired characters with readable adult faces, thick clean outlines, large flat color shapes, limited shading, painted old-TV-animation backgrounds (classical Chinese garden, mansion, pavilion, corridor, chamber, banquet, pond, moonlight, lanterns reduced to clear shape language). Do not copy any real actor, existing anime, TV adaptation, or IP character. Not chibi, not 3D/Pixar/clay, not photographic, no modern clothing or devices. No in-image title, subtitle, text, logo or watermark. Scene: {scene}
```
