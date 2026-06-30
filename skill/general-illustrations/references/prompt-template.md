# 生图提示词模板

每张图单独生成。根据正文内容替换变量，不要把多张图拼在一起。

## 生图提示词模板

Tags: image / general / article / video

```text
Generate one standalone 16:9 horizontal Chinese article or knowledge-video illustration.

Visual style:
{选择的预置风格：clean-docs / technical-minimal / review-minimal / editorial-sketch / product-explainer / simple-doodle / cartoon-explainer / honglou-retro-flat-anime / honglou-period-drama-painterly / coupon-worker-comic / yellow-worker-comic / mambo-meme-comic / story-infra-kafka-mambo-technical / whiteboard-comic / sticker-layer / soft-clay-cartoon}

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

## 红楼梦现代日本电影感动漫视频关键帧模板

Tags: video / cartoon / anime / literary / honglou

```text
Generate one 16:9 full-bleed Hongloumeng / 红楼梦 literary video keyframe in the honglou-retro-flat-anime style.

Video / series context:
{视频主题与段落位置，例如 晴雯撕扇 / 秦可卿托梦 / 宝钗黛玉咏絮词 / 第七十六回凹晶馆联诗。If this belongs to a multi-image video, specify the previous and next beats so this frame is visually distinct.}

Story beat:
{一个具体情节或评论判断。Use one clear dramatic action or emotional beat only, not a general poster.}

Character bible:
{本视频连续人物设定。For each recurring character, define age, slender anime face shape, eye shape, hair silhouette, robe color family, posture, temperament, and emotional role. Keep this consistent across frames.}

Characters in this frame:
{本镜头人物和主次。State who is the main face. Use original Hongloumeng characters with modern 2D Japanese cinematic anime cues: slender oval face, delicate jawline, natural expressive eyes that are not oversized, small readable nose and mouth, clean hair silhouette, youthful but not childlike proportions, restrained acting, visible thin line art, flat cel shading, not CGI. Qing-era-inspired clothing and hair should remain simplified and readable. Use black or dark brown hair for all Hongloumeng characters; no red, pink, blue, purple, or fantasy-colored hair. Do not copy any real actor, existing anime, TV adaptation, film director, film, or IP character.}

Environment:
{garden / mansion / pavilion / corridor / chamber / banquet hall / pond / moonlit courtyard / Longcui Nunnery, simplified as clean contemporary anime background shapes. Use symbolic props from the source scene: fan, fan ribs, lantern, curtain, medicine bowl, empty chair, table, moon reflection, cold pond, crane shadow, bamboo stool, account books, trunks, doors, rocks, flowers, or catkins.}

Mood:
{清爽电影感 / 轻喜剧 / 短暂温情 / 尴尬体面 / 家族衰败 / 诗意寒意 / 命运预告 / 短暂温情后转冷}

Camera and motion readiness:
{wide / medium / close。State intended motion: slow push-in / left-to-right pan / right-to-left pan / slight zoom-out / parallax-ready foreground-midground-background. Keep faces, hands and key props away from edges with 8%-12% safe margin. Leave a clean lower subtitle area.}

Composition:
{具体画面。Use clear staging, not a crowded poster. For adjacent frames, change camera distance, posture, main prop, light direction, or setting so the same image does not repeat in the first 5-10 seconds.}

Style:
Modern 2D Japanese cinematic anime keyframe adapted to Hongloumeng. Slender clean character faces, delicate jawline, natural expressive eyes, small readable nose and mouth, visible crisp thin line art, flat cel shading, soft cinematic lighting, luminous interior light or airy outdoor depth, refined but readable old mansion/garden backgrounds. It should read as hand-drawn animation, not CGI, doll render, oil painting, or photorealistic skin. Full-bleed image, no border, no margins, no white slide canvas.

Text policy:
No in-image title text, no subtitles, no captions, no logos, no watermarks. Final video subtitles, opening burn text, and yellow/red cover typography are added by the render stage, not inside the generated source image.

Hard negatives:
No Xiaohei / black-blob explainer mascot. No whiteboard or PPT slide. No round Doraemon-like child face, no baby face, no overly round face, no giant anime eyes, no cute toddler proportions, no Chinese traditional animation movie look, no Lotus Lantern / Havoc in Heaven style, no realistic live-action drama still, no photorealism, no modern guofeng poster art, no game splash art, no ink-wash illustration, no red/pink/blue/purple hair, no fantasy-colored hair, no 3D, CGI, Pixar-like, toy, porcelain doll, clay, glossy render, oil-paint skin, modern clothing, electric lights, microphones, cameras, UI, game interface, dense hair ornaments, dense embroidery, dense tableware. No direct Makoto Shinkai, Your Name, Doraemon, Cardcaptor Sakura, existing anime, borrowed copyrighted characters, existing actor faces, signature costumes, signature shots, or IP props.
```

## 红楼梦真人转绘感视频关键帧模板

Tags: video / literary / honglou / period-drama / painterly

```text
Generate one 16:9 full-bleed Hongloumeng / 红楼梦 literary video keyframe in the honglou-period-drama-painterly style.

Video / series context:
{视频主题与段落位置，例如 贾政父子 / 秦可卿托梦 / 贾雨村官场 / 贾府衰败 / 王熙凤理家。If this belongs to a multi-image video, specify previous and next beats so this frame is visually distinct.}

Story beat:
{一个具体情节或评论判断。Use one clear dramatic action or emotional beat only, not a general poster.}

Character bible:
{本视频连续人物设定。For each recurring character, define age, mature face shape, eye shape, brow/nose/mouth, beard or hair silhouette, robe color family, posture, temperament, and emotional role. Keep this consistent across frames. Do not copy any existing actor.}

Characters in this frame:
{本镜头人物和主次。State who is the main face. Use original Hongloumeng characters rendered like Chinese period-drama actors translated into painterly illustration: mature adult facial structure, natural-sized almond eyes, readable nose and mouth, restrained acting, specific but original features, dignified posture, Qing-era-inspired robes.}

Environment:
{ancestral hall / study / Rongguo mansion corridor / red-pillared courtyard / Grand View Garden threshold / yamen-like official room / moonlit chamber / family hall. Use props such as ledgers, carved screens, incense smoke, official hat stand, lantern, bamboo shadows, tablets without readable text, doors, account books, empty chair, garden rocks.}

Mood:
{家族压力 / 官场阴影 / 父子隔阂 / 盛极而衰 / 冷静悲剧 / 托梦寒意 / 体面下面的疲惫}

Camera and motion readiness:
{wide / medium / close。State intended motion: slow push-in / left-to-right pan / right-to-left pan / slight zoom-out / parallax-ready foreground-midground-background. Keep faces, hands and symbolic props away from edges with 8%-12% safe margin. Leave a clean lower subtitle area and a clean central area for dynamic text.}

Composition:
{具体画面。Use clear dramatic staging, not a crowded poster. For adjacent frames, change camera distance, posture, main prop, light direction, or setting so the same image does not repeat in the first 5-10 seconds.}

Style:
Refined Chinese period-drama painterly illustration for Hongloumeng video. Mature actor-like but original faces, restrained expressions, soft painterly brushwork, cinematic lantern and moonlight, deep red mansion shadows, aged wood, official dark blue, muted gold, bamboo and incense atmosphere. It should read as a high-quality illustrated drama keyframe, not a real photo, not a TV screenshot, not anime, not CGI. Full-bleed image, no border, no margins, no white slide canvas.

Text policy:
No in-image title text, no subtitles, no captions, no logos, no watermarks, no readable plaques or calligraphy. Final video subtitles, opening burn text, and yellow/red cover typography are added by the render stage, not inside the generated source image.

Hard negatives:
No real actor likeness, no TV adaptation still, no copied costume, no direct film frame, no photorealistic photo skin, no camera grain, no pores, no modern guofeng game splash art, no fantasy armor, no xianxia magic light, no 3D/CGI/Pixar/toy/clay/glossy render, no chibi, no childlike round face, no giant anime eyes, no modern clothing, electric lights, microphones, cameras, UI, dense hair ornaments, dense embroidery, dense tableware, readable Chinese characters, calligraphy, or false text.
```

## 曼波抽象梗短视频画面模板

Tags: video / tech / meme / comic / infrastructure

```text
Generate one 16:9 horizontal Chinese knowledge-video frame in the mambo-meme-comic style.

Topic:
{技术主题或工程故事主题，例如 Kafka 消息堆积 / 下单链路卡死 / consumer 追 offset / Redis 热点 key / 数据库慢查询}

Core mechanism:
{这张图必须讲清的机制，不能只搞笑，例如 append-only log / queue decoupling / backpressure / consumer lag / retry storm / cache miss}

Meme action loop:
{2-4 个循环动作 beat，例如 requests wobble into a queue -> service shakes under pressure -> Kafka log conveyor carries events -> consumers catch up by offset}

Composition:
White or very light background. Flat 2D, thick black outlines, original abstract characters or system components, no existing meme/IP characters. One large looping action scene should occupy 50%-70% of the canvas. Use small motion lines, repeated poses, bouncing arrows, beat marks, and a few saturated red/yellow/blue accents to create a catchy Chinese short-video meme rhythm. Leave clean subtitle/title space. The technical entities must remain recognizable: topic/log/consumer/offset/database/service should be visually distinct when used.

Short labels only:
{0-4 short readable labels such as ORDER / KAFKA LOG / OFFSET / LAG / DB / RETRY; skip labels if overlay text will be added later}

Constraints:
Do not copy Uma Musume, Hakimi, Manbo song visuals, existing meme screenshots, lyrics, cats, known characters, or audio-specific references. Do not make a dance-club scene, real dancers, anime fan art, chaotic bullet comments, low-resolution meme collage, or pure nonsense. The joke energy is allowed only if the infrastructure mechanism is still clear.
```

## Kafka story-infra 技术短视频画面模板

Tags: video / tech / kafka / infrastructure / comic / series

```text
Generate one 16:9 horizontal Chinese knowledge-video frame in the story-infra-kafka-mambo-technical style.

Episode / story beat:
{Kafka episode and segment beat, for example duplicate consumption after crash / early offset commit / ISR election / consumer group rebalance / retry dead letter queue}

Core Kafka mechanism:
{The exact mechanism this frame must explain. Be technically accurate: partition log, offset, consumer group, broker leader/follower, ISR, producer ack, retry, dedupe, idempotency key, or compaction.}

Story objects:
{Choose 3-6 concrete objects: order blocks, blue envelopes/tokens, append-only log conveyor, partition lanes, offset markers, consumer workers, broker boxes, replica ledgers, dedupe gate, database ledger, risk alarm, stuck worker, replay arrow.}

Composition:
White or very light background. Flat hand-drawn infrastructure comic, thick black outlines, simple flat fills, strong empty space. One clear Kafka mechanism scene should occupy 50%-70% of the canvas, with clean title space for a large cover overlay and clean lower subtitle space. Use blue envelopes/tokens/dots for event flow and system state. Use red/orange only for warning, pressure, failure, cracks, risky windows, or blocked paths. Arrows and motion lines should show message flow, offset movement, retry/replay, or failure windows. The image must read as an infrastructure story frame, not decoration.

Text policy:
Prefer no real in-image text. If labels are necessary, use only 0-4 very short labels such as ORDER / OFFSET / RETRY / DB / ACK / ISR. Do not put a title, subtitle, paragraph, markdown, or PPT label block inside the image; video render will add cover title and subtitles.

Hard negatives:
No PPT slide, no formal architecture diagram, no dense dashboard, no UI screenshot, no stock art, no commercial illustration, no 3D/Pixar/toy/clay/glossy render, no dark tech background, no decorative gradient, no cute mascot-centric Xiaohei image. Characters or operators may appear only if they act on the Kafka mechanism. Do not draw Kafka incorrectly: partition is not a single global FIFO for all keys, offset is not the message body, followers do not compete to write, and exactly-once should not be shown as business-wide magic.
```

## 原创黄色小人动作漫画短视频画面模板

Tags: video / business / finance / tech-news / comic / character

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
