# QA Checklist

## 必过项

- 是 16:9 横版。
- 背景干净。
- 选择了一个明确风格。
- 一张图只讲一个核心结构。
- 没有复刻旧案例构图，而是为当前内容生成了新隐喻。
- 画面清爽、有创意、有意思。
- 文章和文档配图主体不超过画面约 60%；短视频动作画面可以到 70%，但必须保留字幕区。
- 标注少、短、能读。
- 短视频画面优先通过动作表达，不靠左侧大字或大段文字解释。
- coupon-worker-comic 必须是原创黄色票券/收据小工人，不能出现小黄人式护目镜、蓝色背带裤、胶囊身体或单眼大眼。
- yellow-worker-comic 必须是原创黄色小人/小工人动作漫画，不能出现真实小黄人 IP 元素，也不能被优惠券场景绑死。
- honglou-retro-flat-anime 必须是原创红楼梦复古电视动画关键帧：full-bleed、无白边、无图片内字幕/标题、原创人物脸和服饰，不借用影视演员或现成 IP。
- honglou-retro-flat-anime 源图只负责关键帧；封面黄红大字、开头 burn 字幕、底部字幕、BGM 和 TTS 不属于图片 Recipe。
- honglou-retro-flat-anime 必须为后期 Ken Burns/Editly 动效预留安全边界：人物脸、手和关键道具不要贴边，底部字幕区保持干净。
- honglou-retro-flat-anime 用于连续视频时，相邻 5-10 秒内不能重复同一张源图或几乎相同构图；每个 beat 要换动作、镜头距离、光线、人物姿态或象征物。
- honglou-period-drama-painterly 必须是原创红楼梦真人转绘感关键帧：成人脸、演员式表演、戏剧光影、full-bleed、无白边、无图片内字幕/标题。
- honglou-period-drama-painterly 可以有成熟真人绘画感，但不能像真实照片、电视剧截图、具体演员脸、影视服装复刻或游戏国风海报。
- honglou-period-drama-painterly 必须为后期 Ken Burns/Editly 动效预留安全边界：人物脸、手和关键道具不要贴边，底部字幕区和中央动态文字区保持干净。
- mambo-meme-comic 必须是原创中文抽象梗感动作画面，不能复刻赛马娘、哈基米、翻车鱼之歌、现成表情包、歌词、角色或音频视觉。
- mambo-meme-comic 的节奏感不能牺牲技术可读性；系统组件、队列、日志、consumer、offset 等机制元素必须能看懂。
- story-infra-kafka-mambo-technical 必须保持 Kafka 机制准确：消息流、partition、offset、consumer、broker、replica、ISR、retry 或 dedupe 等核心对象要能看懂，不能只是泛泛的可爱手绘。
- story-infra-kafka-mambo-technical 必须给后期封面大字和底部字幕留干净空间，图片内不能放大标题、字幕或长段说明。
- 风格和用途匹配：文章、技术文档、review、短视频不能混用错。
- 橙色只用于主路径或动作。
- 红色只用于重点、问题、提醒或结果。
- 蓝色只用于补充说明、读路径或系统状态。

## 失败信号

出现以下情况，重生成或局部编辑：

- 左上角有“流程图 / 系统架构图 / Workflow / Roadmap”等类型标题。
- 画面像 PPT、课程课件、正式流程图。
- 元素太多、箭头太多、节点太多。
- 文字变成大段解释。
- 背景有复杂纹理、渐变、噪点或光污染。
- 真实 UI 截图或科技感界面。
- 中文错字严重或标注不可读。
- 卡通风格变得太幼稚、太可爱、太像儿童玩具。
- 黄色小人短视频画面像真实小黄人、玩具吉祥物、海报封面或文字卡片，而不是原创动作漫画。
- 红楼梦关键帧像现代国风海报、游戏立绘、精致概念艺术、白色 slide canvas、真人剧照、现成影视演员脸，或图片里直接写了大标题/字幕。
- 红楼梦真人转绘感关键帧变成真实照片、电视剧截图、具体演员脸、仙侠游戏海报、3D 写实渲染，或图片里直接写了大标题/字幕。
- 红楼梦关键帧人物脸贴边、字幕区被关键道具占满，后期左右移动或缩放时会裁掉脸、手或诗意物件。
- 红楼梦连续视频在开头或相邻短段落反复使用同一张图，导致观众感觉画面卡住或素材重复。
- 曼波梗画面像现成 IP、表情包截图、夜店舞蹈、满屏弹幕或纯搞笑乱图。
- 曼波梗画面只剩节奏感，看不出真实技术机制或系统状态。
- Kafka story-infra 画面变成普通小黑装饰图、PPT 架构图、密集 dashboard，或把 Kafka 机制画错。
- 短视频画面没有明确动作 beat，后续很难做动画。
- 技术图发明了不存在的组件。
- 和 assets/examples/ 里的旧案例构图过于相似。

## 迭代方法

- 太普通：加入一个具体物理动作和低科技物件。
- 太复杂：删节点，只保留一个动作和 3-5 个短标注。
- 太可爱：强调 clean technical explainer、not cute、not mascot、not childish。
- 太 PPT：去掉标题、边框、整齐网格和过多箭头，改成场景或物件。
- 太像旧案例：保留核心意思，换掉主物件和动作。
- 文字错：优先局部编辑；错得多就重生成并减少标注数量。
- 不适合视频：加大主体，减少文字，留字幕区，明确前中后景。
- yellow-worker-comic 太像真实小黄人 IP：去掉护目镜、蓝色背带裤、胶囊身体、单眼大眼和 3D 玩具感，改成扁平原创黄色小人或小工人。
- yellow-worker-comic 太依赖文字：删掉左侧文字块和大标题，把机制改成一个动作：搭建服务器、拉光纤、搬资金、筛选公司、推开噪音或追逐风险信号。
- honglou-retro-flat-anime 太像现代国风海报或概念艺术：压低细节密度，改成 1980s/1990s 电视动画 cel 感：粗线、大色块、少纹理、少头饰、少餐具、少背景细节。
- honglou-retro-flat-anime 出现白边或图片内文字：明确 full-bleed、no border、no margins、no white slide canvas，并删除图片内标题/字幕；封面标题和底部字幕只在视频 render 阶段添加。
- honglou-retro-flat-anime 相邻画面重复：保留同一人物 recipe，但重画为新的 story beat：换镜头距离、人物姿态、主要道具、光源方向或场景位置。前 5-10 秒尤其不要 A-B-A 式快速回到同一张图。
- honglou-retro-flat-anime 不适合后期动效：重画时加入 foreground/midground/background，给脸和关键道具留 8%-12% 安全边界，底部留字幕区，并明确 intended motion: slow push-in / pan / zoom-out。
- honglou-period-drama-painterly 太像照片或电视剧截图：强调 painterly illustration、original actor-like face、no real actor likeness、no TV still，并保留绘画笔触和非摄影皮肤。
- honglou-period-drama-painterly 太像国风游戏海报：删掉仙侠光效、盔甲、夸张体积光和复杂服饰，改成荣国府书房、祠堂、走廊、账册、灯影和克制成人表演。
- honglou-period-drama-painterly 不适合后期动效：加入明确前景/中景/背景和 8%-12% 安全边界，底部字幕区和中央动态文字区保持干净。
- mambo-meme-comic 太像现成梗或 IP：删掉任何可识别角色、歌词、猫猫梗、赛马娘/哈基米/翻车鱼视觉，改成原创抽象系统组件和循环动作。
- mambo-meme-comic 看不懂机制：减少无关搞笑元素，明确画出请求、队列、日志传送带、consumer、offset 或故障状态，并只保留 0-4 个短标签。
- story-infra-kafka-mambo-technical 太泛或太装饰：把画面改成一个具体 Kafka 故障窗口或消息流动作：蓝色消息 token 经过 partition log、offset marker、consumer、broker/replica、dedupe gate 或 DB ledger，删除装饰性角色。
- story-infra-kafka-mambo-technical 机制不准确：重画核心对象关系，明确 partition、offset、consumer group、leader/follower、ISR、ack、retry 或 idempotency key 的真实边界；不要用全局队列或万能 exactly-once 隐喻偷换概念。

## 交付判断

高质量图应该让读者 1 秒内看懂核心结构，并且愿意把它放进文章或短视频里。如果第一眼像模板页，而不是为当前内容专门画的一张解释图，就不合格。
