# 设计文档模板 README（面向人类工程师与编程Agent协作）

本仓库是一套“以文件为单位”的设计文档模板，结合 DDD + BDD 的实践，帮助在轻工具场景（如 Zed 文本对话）下，与 LLM 高效协作完成从治理→战略→战术→质量→交付到编码落地的全流程设计与实现。

本模板强调结果契约与最小流程：
- 文件为单位：每次只对一个文件进行生成或修订。
- 先全貌、后细化：先生成整文件初稿，再逐条/逐章修订。
- 规则内嵌：每个目录有 AGENT.md 定义产出结构与对话节奏；每个文件顶部有“文件级契约注释”。
- 就地变更：仅在文件末尾“## 变更”记录关键改动摘要，避免文档膨胀。
- 设计到实现闭环：文档条目转 Dev Task Card；以小上下文、强边界对接编程Agent，落实契约、指标与验收。

目录结构
- 0-governance/：治理层（术语、约束、决策、风险）
- 1-strategic/：战略层（愿景、子域/上下文、上下文关系、业务语境）
- 2-tactical/：战术层（每个上下文的领域模型、用例、事件）
- 3-quality/：质量层（BDD 抽象层 Gherkin 特性文件）
- 4-delivery/：交付层（架构文字版、契约版本块、测试计划、运行手册）
- 每级目录均有 AGENT.md 说明“结果契约 + 协作节奏 + 拆分规范”

快速开始（5步）
1) 初始化骨架
- 阅读根目录 AGENT.md（Meta-Doc）：了解全库协作规则。
- 阅读各目录 AGENT.md：了解该层产出与节奏。
- 骨架文件均含“文件级契约注释”与“示例文件壳”。

2) 生成治理最小集（0-governance）
- 生成四个文件的整文件初稿：
  - glossary.md（≥6条术语；每条含 术语/定义/正例/反例/易混词）
  - constraints.md（≥4条约束；含可能影响的文件/环节）
  - decisions.md（≥2条决策；含选项与影响落点）
  - risks.md（≥3条风险；含可观测触发条件）
- 原则：具体、可判定、可追溯。

3) 生成战略骨架（1-strategic）
- vision、domain-map、context-map、ubiquitous-language 各生成整稿：
  - domain-map 至少3个子域和2–4个候选上下文条目
  - context-map 至少3条关系与“一句话策略+风险点”

4) 选择一个上下文开展战术设计（2-tactical/bounded-contexts/）
- 依次生成整稿：domain-model.md、use-cases.md、events.md。
- 逐“聚合小节/用例章节/事件条目”补强（每轮只改一段）。

5) 建立质量与交付闭环（3-quality、4-delivery）
- 3-quality/features：按类别生成 BDD 抽象层初稿（performance/reliability/security 各2–3场景或总计3–6场景）。
- 4-delivery：生成 architecture、contracts、testing-plan、runbook 整稿，并逐块完善：
  - contracts：将用例与 BDD Then 映射为接口/事件的版本块与观测信号
  - testing-plan：从 BDD 场景落地验收/契约/性能/韧性/安全测试项
  - runbook：SLO/SLI/告警阈值与 BDD/契约一致

与 LLM 协作的基本节奏
- 文件为单位：
  - 初稿：要求“仅输出该文件内容”，按文件契约结构生成整稿；文末追加“[v1.0 - YYYY-MM-DD] 首版”。
  - 修订：复制需要修改的条目/章节到对话，要求“仅输出该段修订版”，便于直接替换。
- 复杂主题拆分：
  - 当单文件 >150–200 行或主题发散，拆为同构文件（如 glossary-order.md、contracts-order.md），原文件保留“参见”占位。
- 语言与质量前置：
  - 新术语/约束/决策/风险先在 0-governance 更新，再推进下游文件，保持一致性。

各层使用要点与检查清单
- 0-governance
  - 术语：卡片≤6行，含正反例与易混词。
  - 约束：指向具体影响落点（文件/环节）。
  - 决策：先列选项再下结论；影响落点明确。
  - 风险：触发条件可观测（阈值/比例/时间窗/事件）。
- 1-strategic
  - 愿景：包含“本阶段不做”范围。
  - 子域：核心/支撑/通用标注，避免跨域混淆。
  - 关系：每条“一句话策略+风险点”。
- 2-tactical
  - 聚合：职责、不变量、事务边界、外部交互。
  - 用例：前置/主流程/异常≥2/边界/结果；建议在结果写观测信号名。
  - 事件：名称/含义/触发/订阅；必要时写幂等/顺序关注点。
- 3-quality（BDD 抽象层）
  - 约束：抽象层禁止 UI 步骤（click/input/see 等），仅业务语言；UI 仅放在 e2e-*.feature。
  - Then 必含可测阈值/审计事件/观测信号。
  - Scenario Outline + Examples 用于档位/阈值参数化。
- 4-delivery
  - 契约版本块：名称/版本/字段/来源用例/观测信号/兼容性；新增字段的向后兼容策略明确。
  - 测试计划：从 BDD 场景落地；Expected 可判定；有追溯链接。
  - 运行手册：SLO/SLI/告警阈值与 BDD/契约一致；排障提供“最短诊断路径”。

变更记录（强制最小规范）
- 所有文件在末尾“## 变更”仅追加单行摘要：
  - [vX.Y - YYYY-MM-DD] 范围/动机/变更/影响/验证
- 需求/任务等过程追踪保留在项目管理系统，不复制到设计文档。

常见陷阱与规避
- 一次生成过多 → 只对单文件、单段进行生成或修订。
- 用词漂移 → 新术语先更新 0-governance/glossary。
- BDD 写成 UI 脚本 → 抽象层禁止 UI 步骤；UI 仅在 e2e-*.feature。
- 契约与用例脱节 → 版本块中必须写 Source Use Case；用例结果建议写观测信号名。
- 指标名不统一 → BDD Then、contracts Observability、runbook SLO/SLI/Alerting 使用同一命名。

与编程Agent的对接
目的：将设计产物转换为可执行的实现任务，控制上下文大小与实现边界，确保代码与文档一致。

A. 交接对象 → Dev Task Card
以下“文件级条目”会转换为编程Agent的开发工单：
- 用例章节（2-tactical/use-cases.md 中单条用例）
- 契约版本块（4-delivery/contracts.md 中单个接口/事件版本块）
- 事件条目（2-tactical/events.md 中单个事件）
- BDD 场景（3-quality/features/*.feature 中单个 Scenario/Outline）
每张工单包含：
- Source：源文件与定位（锚点），及其关联（聚合小节、事件、BDD 场景/测试项）
- Intent（做什么）：目标行为/状态变化/集成动作（来自用例与契约）
- Constraints（边界）：实现层（领域/应用/适配）的限制；质量约束（阈值/幂等/顺序/安全要点）
- Contracts（输入/输出）：字段、错误语义、兼容性策略
- Tests（验收口径）：链接到 BDD 场景与 testing-plan 项（至少验收+契约+性能/韧性之一）
- Stubs & Interfaces：需要实现或调用的端口/仓储/网关抽象
- Observability：指标/日志/审计事件名（与 BDD/Runbook 一致）
- Done Definition：编译、单元/契约测试、埋点、沙箱自测通过

B. 编程Agent“上下文打包”（Prompt Context Package）
为每张 Dev Task Card 准备最小上下文包：
- 必选片段：对应用例/契约/事件/BDD 场景原文片段（仅该段）+ 关联聚合小节（职责/不变量/事务边界/外部交互）
- 术语卡（3–5条）：来自 0-governance/glossary，与本任务相关
- 约束卡（1–2条）：安全/合规/性能等显著约束
- 接口签名/代码骨架：目标层接口/端口/DTO/事件类（或建议签名）
- BDD 断言映射：Then 的阈值与观测信号名，用于埋点与测试断言
注意：只提供必要且充分的信息，保持小上下文。

C. 实现层次与代码落点（Clean Architecture）
- 领域层（Domain）：实体/值对象/聚合；不依赖基础设施；暴露抽象端口
- 应用层（Use Case）：编排与事务；调用端口；不含外部细节
- 适配/基础设施（Adapters/Infra）：DB/HTTP/队列实现；面向端口实现
编程Agent在工单中必须声明“当前实现层”；不得跨层修改；跨层需求另开工单。

D. 从 BDD 到代码与测试
- Given → 测试前置/夹具/依赖注入
- When → 用例服务调用或 API/消息触发
- Then → 断言响应/状态/事件/指标阈值（名称与时间窗一致）
- Outline → 表驱动参数化用例或负载档位
抽象层 Feature 保持业务语言；Step 层映射到 API/Service/Message；UI 步骤仅限 e2e-*.feature。

E. 最小质量护栏（编码期）
- 依赖倒置与接口优先；小 PR（每次仅一个功能单元）
- 单元/契约测试与属性测试（如适用）优先
- 指标埋点命名与 BDD/Runbook 一致；提交附“映射清单”（本PR覆盖的 use-case/contract/feature 场景ID）

F. Dev Task Card 模板（示例）
- Source
  - use-cases.md#UC-ORDER-CREATE（order 上下文）
  - contracts.md#POST-/orders v1
  - performance.feature#峰值下单延迟
- Intent
  - 实现“创建订单”应用服务与 API 端点；延迟目标满足 p95<250ms
- Constraints
  - 实现应用层与 API 适配层；领域层不改
  - 依赖 IInventoryChecker、IOrderRepository；幂等入队
- Contracts
  - Request: { items[], userId }；Response: { orderId, status }；兼容性：严格
- Tests
  - AT-ORDER-01、CT-ORDERS-POST-01、PT-ORDER-PEAK-01
- Stubs & Interfaces
  - IInventoryChecker.Check(ids)、IOrderRepository.Save(order)
- Observability
  - order_create_latency_p95、queue_ticket_issue_latency
- Done
  - 单元/契约/基准测试通过；指标打点齐全；沙箱联调通过

PR 与评审要点
- 是否遵循目标文件的“文件级契约注释”（结构/字段/节奏）？
- 是否仅修改了目标条目/章节？是否补了“## 变更”摘要？
- 新术语/约束是否已回填到 0-governance？
- BDD Then 是否为“可测阈值/观测/审计事件”而非空泛描述？
- PR 描述中是否附“映射清单”（用例/契约/场景ID）与“完成定义自检结果”？

FAQ
- 为什么不把需求/任务写进设计文档？
  - 为避免文档膨胀与噪音；过程追踪在项目管理系统；文档仅保留被认可的当前设计与必要变更摘要。
- 如何管理跨文件的大改？
  - 先在 0-governance/decisions 加一条简版决策（含影响落点），再逐文件按“文件为单位”修订对应条目，同时补各自“## 变更”摘要。
- BDD 的 UI 步骤写在哪里？
  - 在 3-quality/features/e2e-*.feature；抽象层 Feature 禁止 UI 术语。

版本约定
- 首版：所有文件“## 变更”包含 [v1.0 - YYYY-MM-DD] 首版。
- 小改：递增 v1.1、v1.2；结构性拆分或重大策略变更可升 v2.0，并在 README 顶部简述变更主题（可选）。

到这里可以按目录顺序启动：治理→战略→选上下文做战术→质量（BDD 抽象层）→交付（契约/测试/运行）→发 Dev Task Card 给编程Agent按层实现。保持“单文件/单条目（或单章节）/单工单”的节奏，就能在极简流程中稳步构建清晰、可审、可演进、可实现的完整设计与代码。
