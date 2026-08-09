# ChatGPT Provider 集

通过 CDP 驱动已登录的 ChatGPT（GPT 图像生成）网页生成图片，与 Gemini 共用同一台浏览器。每个配方复用其**忠于风格**的图像提示词（有 `chatgpt` 覆盖则用覆盖，否则用 `gemini` 提示词）。

## 使用契约

- 需要一个已登录、开启图像生成的 ChatGPT 会话（Plus/Go）在 CDP Chrome 上。
- 选定配方后，用该配方的提示词，把 `{scene}` 替换成当前画面描述。
- **禁止**再套一层全局 `photorealistic, cinematic` 包装；那会覆盖风格（Berkshire 事故根因）。
- ChatGPT 图片来自 oaiusercontent.com（跨域），canvas 抓取常被 CORS 污染；抓不到时用元素截图兜底。
- 提示词保持 text-free；中文标题/字幕在后期用 PIL/ffmpeg 叠加。

## 每个配方的 ChatGPT 提示词

### `clean-doc` -> style `clean-docs` (gemini fallback)

```text
Generate one clean 16:9 horizontal explainer illustration. White or very light background, a few simple geometric shapes, clear hierarchy, restrained palette (mostly black lines with one orange accent for the main path). Flat, minimal, no marketing gloss. No text, no words, no logos, no watermark. Scene: {scene}
```

### `tech-diagram` -> style `technical-minimal` (gemini fallback)

```text
Generate one clean 16:9 horizontal technical explainer diagram. White background, precise boxes, arrows, layers, shelves and storage objects, clear boundaries. Black lines, orange for the write/main path, blue for read/state. Accuracy over decoration; do not invent components. Only unavoidable short labels. No text, no words, no logos, no watermark. Scene: {scene}
```

### `code-review` -> style `review-minimal` (gemini fallback)

```text
Generate one 16:9 horizontal code-review illustration. Neutral light background, one red risk/bad-path point and one blue fix/verify point, the bad path emphasized. Flat, minimal, black lines. No large code blocks, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `article-sketch` -> style `editorial-sketch` (gemini fallback)

```text
Generate one 16:9 horizontal editorial sketch illustration. Hand-drawn feel, slightly wobbly lines, large empty space, one concrete physical metaphor, sparse labels. Black ink lines with one restrained accent color. No PPT look, no decorative-only art, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `product-explainer` -> style `product-explainer` (gemini fallback)

```text
Generate one 16:9 horizontal product-explainer illustration. Quiet SaaS/docs feel, a few simplified abstract feature blocks, calm restrained palette. Flat, clean, generous empty space. No fake UI detail, no landing-page hero, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `whiteboard-story` -> style `whiteboard-comic` (gemini fallback)

```text
Generate one 16:9 horizontal whiteboard comic. Black marker line art on white, 2-4 simple panels, a few arrows and action lines, one problem and one result. Not cute, no long handwriting. No text, no words, no logos, no watermark. Scene: {scene}
```

### `doodle-light` -> style `simple-doodle` (gemini fallback)

```text
Generate one 16:9 horizontal simple doodle illustration for a knowledge video. White background, marker-style lines, a few simple objects, lots of empty space, 1-2 accent colors. Not messy, not childish. No text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `sticker-layers` -> style `sticker-layer` (gemini fallback)

```text
Generate one 16:9 horizontal flat sticker-style illustration with clear layering. Flat sticker objects with clean outlines, distinct foreground/midground/background so elements can pop in as animation layers. White background, restrained palette. No glossy 3D, no busy collage, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `clay-metaphor` -> style `soft-clay-cartoon` (gemini fallback)

```text
Generate one 16:9 horizontal soft clay/toy-like cartoon illustration. Rounded soft clay objects, simple lighting, big clear shapes, little text. Not candy-colored, not photographic, not a busy scene. No text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `software-news` -> style `cartoon-explainer` (gemini fallback)

```text
Generate one 16:9 horizontal flat 2D cartoon-explainer illustration. White background, black hand-drawn outlines, an original friendly cartoon worker character with a blue cap and blue overalls (original design, NOT a fixed IP, NOT a cutesy mascot). Use only blue and orange; orange blocks/arrows = data/action/flow. Flat 2D, one clear core action, subject 40-60% of canvas, clean space for later subtitles. No text, no words, no logos, no watermark. Scene: {scene}
```

### `consumer-coupon` -> style `coupon-worker-comic` (gemini fallback)

```text
Generate one 16:9 horizontal flat 2D comic illustration. White or very light background, thick black hand-drawn outlines, original yellow coupon/receipt/bean-shaped worker characters, short limbs, dot eyes, exaggerated action (NOT Minions: no goggles, no blue overalls, no capsule body, no single big eye, no denim). One core action about pricing/coupons/subscriptions; props carry the story; at most a few short labels. No poster title, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `finance-news` -> style `yellow-worker-comic` (gemini fallback)

```text
Generate one 16:9 horizontal flat 2D comic illustration. White or very light background, thick black hand-drawn outlines, original yellow worker characters with short limbs and dot eyes, exaggerated action (NOT Minions: no goggles, no blue overalls, no capsule body, no single big eye, no denim, no Gru/Minions look, no 3D toy render). One core business action (carrying cash into servers, building a data center, pulling fiber, turning a price dial, sorting company cards, pushing away noisy headlines, inflating a valuation balloon). Controlled palette: black lines, orange for action/flow, blue for system/state, red only for risk. Subject 50-70% of canvas, clean space for later title/subtitle. No text, no words, no logos, no watermark. Scene: {scene}
```

### `infra-meme` -> style `mambo-meme-comic` (gemini fallback)

```text
Generate one 16:9 horizontal flat 2D meme-rhythm illustration. White or very light background, thick black outlines, original abstract little characters or system components doing a repeatable beat action (requests wobble into a queue, a service shakes under pressure, a Kafka log conveyor carries events one slot at a time). Big empty space, main action 50-70%, a few high-saturation red/yellow/blue beat accents. The technical mechanism (queue, log, consumer, offset, cache) must stay readable. No existing meme/IP/emoji, no nightclub dancing, no danmaku, no text paragraphs. No text, no words, no logos, no watermark. Scene: {scene}
```

### `honglou-keyframe` -> style `honglou-retro-flat-anime` (gemini fallback)

```text
Generate one 16:9 full-bleed flat retro 2D TV-anime cel keyframe, no border, no margins. Simplified original Qing-era-inspired characters with readable adult faces, thick clean outlines, large flat color shapes, limited shading, painted old-TV-animation backgrounds (classical Chinese garden, mansion, pavilion, corridor, chamber, banquet, pond, moonlight, lanterns reduced to clear shape language). Do not copy any real actor, existing anime, TV adaptation, or IP character. Not chibi, not 3D/Pixar/clay, not photographic, no modern clothing or devices. No in-image title, subtitle, text, logo or watermark. Scene: {scene}
```
