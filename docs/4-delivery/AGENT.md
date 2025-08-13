# Delivery AGENT

目标
- 把战术设计和质量场景落到“可交付”的最小集合：架构视图（C4文字版）、对外/对内契约（接口与事件的版本块）、测试计划（验收/契约/基准/混沌/安全）、运行手册（可观测性与运维操作）。
- 协作以“文件”为单位：每个文件先整稿初版，再逐章/逐条修订；复杂主题按上下文或能力拆分为同构文件。
- 与上游对齐：用例→契约映射、BDD Then→测试项与观测信号、上下文关系→集成策略（同步/异步/ACL/反腐层）。

文件清单
- architecture.md：C4 Context/Container 文字版（只写必要视图与关键决策）。
- contracts.md：接口/事件的“版本块”清单（名称+字段+来源用例+观测信号+兼容性）。
- testing-plan.md：验收/契约/基准/混沌/安全的测试项（从 BDD 特性文件落地）。
- runbook.md：运行手册（配置、启动、SLO/SLI、告警、排障基本流程）。

文件级结果契约与节奏
- architecture.md
  - 结构
    - Context（一句话系统使命；主要外部参与者/系统）
    - Containers（关键容器：API/服务/消息/存储；职责与技术要点）
    - Integration Strategy（来自 1-strategic/context-map：同步/异步/ACL/反腐层）
    - Key Decisions（3–5条含动机/权衡/影响的简条，引用 governance/decisions.md）
  - 节奏
    - 首轮：整文件初稿（精简四节）
    - 迭代：逐节修订（优先补 Integration 与 Key Decisions）
  - 质量性质
    - 与战略上下文关系一致；与战术用例/事件一致；关键决策有影响落点
- contracts.md
  - 表达单位：版本块（Version Block）
  - 结构（接口）
    - Name:  （如 POST /orders）
    - Version: vX.Y（Date）
    - Request: 字段名与要点（无需穷举校验规则）
    - Response: 字段名与要点
    - Source Use Case: 关联用例ID
    - Observability: 指标/日志/事件名（从 BDD Then 对应）
    - Compatibility: 兼容性说明（向后兼容/弃用计划）
  - 结构（事件）
    - Name:
    - Version: vX.Y（Date）
    - Payload: 字段清单
    - Emitter: 来源上下文/聚合
    - Consumers: 示例订阅方
    - Observability: 事件计数/滞留/延迟信号名
    - Ordering/Idempotency: 顺序/幂等说明（如适用）
  - 节奏
    - 首轮：整文件初稿（2–4个关键接口或事件的版本块）
    - 迭代：每轮仅新增/修订一个版本块；只输出该块
  - 质量性质
    - 每个版本块可追溯到用例与BDD场景；包含至少1个观测信号；兼容性清晰
- testing-plan.md
  - 结构
    - Acceptance Tests（对应 2-tactical/use-cases 的主/异常/边界）
    - Contract Tests（接口与事件的契约约束）
    - Performance/Load（从 3-quality/features/performance.feature 提取）
    - Resilience/Chaos（从 reliability.feature 提取：故障/降级/补偿/恢复）
    - Security（从 security.feature 提取：鉴权/授权/审计）
    - Traceability（可选：列出 Scenario → Test ID 对照）
  - 每条测试项字段
    - ID/Title、Preconditions、Steps/Stimulus、Expected（可含阈值）、Links（Use Case/Feature/Contract）、Notes（工具或数据集提示）
  - 节奏
    - 首轮：整文件初稿（每类各2–3条代表项）
    - 迭代：逐条新增/修订；只输出该测试项
  - 质量性质
    - 每类至少1条；Expected 含可判定阈值或断言；有追溯链接
- runbook.md
  - 结构
    - Configuration（关键配置/开关位）
    - Startup/Shutdown（步骤与健康检查端点）
    - SLO/SLI（从 BDD Then映射：指标名/目标/时间窗）
    - Alerting（阈值/触发条件/初步处置）
    - Operations（常见场景：扩容/降级/回退/补偿/数据修复）
    - Troubleshooting（按症状→可能原因→诊断步骤→缓解）
  - 节奏
    - 首轮：整文件初稿（每节1–2段即可）
    - 迭代：逐节补强（尤其是 SLO/Alerting 的指标名与阈值）
  - 质量性质
    - 指标名与 contracts/testing-plan/BDD 一致；处置步骤可执行且最短路径明确

抽象验收性质（DoD-Properties）
- 一致性：术语与 0-governance/glossary 对齐；上下文关系与 1-strategic/context-map 一致；用例/事件与 2-tactical 对齐；阈值/信号与 3-quality BDD 对齐。
- 完备性（阶段目标）：contracts 含≥2个版本块（接口/事件）；testing-plan 覆盖≥3类测试；runbook 包含 SLO/Alerting 基线；architecture 包含 Integration 与 Key Decisions。
- 可证伪性：testing-plan 的 Expected 可判定；contracts 的兼容性与版本策略可被挑战（是否需要弃用计划）；runbook 的告警阈值能触发演练。
- 可操作性：contracts→testing-plan 有直接测试项；runbook 的指标/告警与 contracts/BDD 的信号名一致，可被监控平台实现。

对话节奏（文件为单位）
- 初稿阶段：对四个文件逐一生成整文件初稿（仅输出目标文件内容），并在文末追加“## 变更”首条“v1.0 首版”。
- 修订阶段：每轮仅修改一个章节（architecture/runbook）或一个条目（一个契约版本块/一个测试项）；在指令中粘贴原段落，模型仅返回该段更新。
- 扩展/拆分：当 contracts 或 testing-plan 过长，按上下文或能力拆分为 contracts-.md、testing-plan-.md；原文件保留“参见”占位与索引。

与上下游的对齐提示
- 从 2-tactical/use-cases：每个关键用例至少映射一个接口或事件版本块；异常/边界用例在 testing-plan 中体现。
- 从 3-quality/features（BDD 抽象层）：将 Then 的阈值与审计/观测信号名写入 contracts 与 runbook；performance/reliability/security 场景转化为测试项。
- 向后影响：当契约字段变化或事件新增，回到 governance/decisions 增一条简版决策；必要时在 1-strategic/context-map 更新关系策略备注。

文件顶部契约注释（建议粘贴到每个文件开头）
- architecture.md
  - 本文件提供 C4 Context/Container 文字版与集成策略、关键决策简条；先整稿后逐节修订；变更写文末。
- contracts.md
  - 本文件按“版本块”维护接口/事件；每轮仅新增/修订一个版本块；需包含 Source Use Case 与 Observability；变更写文末。
- testing-plan.md
  - 本文件按测试类别维护测试项；先整稿后逐项修订；Expected 必含可判定断言；变更写文末。
- runbook.md
  - 本文件维护配置/启动/健康检查/SLO/告警/运维与排障；指标名与阈值需与 BDD/契约一致；先整稿后逐节修订；变更写文末。

---

示例文件壳（可直接落库）

- architecture.md
  # Architecture (C4–Text)
  ## Context
  - 系统使命：一句话
  - 外部参与者/系统：用户、支付网关、库存服务…

  ## Containers
  - API：接入层；鉴权/限流；协议与技术要点
  - Order Service：领域应用服务；聚合边界
  - Messaging：事件总线/主题；持久化策略
  - Storage：主存储/读模型；关键分区/索引

  ## Integration Strategy
  - 订单→支付：异步事件（PaymentRequested/PaymentSettled）
  - 订单→库存：同步RPC + 幂等防重
  - 反腐层：对接遗留库存系统字段差异

  ## Key Decisions
  - [YYYY-MM-DD] 选择事件驱动对账（见 governance/decisions.md）
    动机：削峰与解耦；影响：contracts/events 新增两事件
  - …

  ## 变更
  - [v1.0 - YYYY-MM-DD] 首版

- contracts.md
  # Contracts (Versioned)

  ## APIs
  ### POST /orders
  - Version: v1 (YYYY-MM-DD)
  - Request: { items[], userId }
  - Response: { orderId, status }
  - Source Use Case: UC-ORDER-CREATE
  - Observability: order_create_latency_p95
  - Compatibility: 新增字段向后兼容，弃用计划见下

  ### POST /orders
  - Version: v1.1 (YYYY-MM-DD)
  - Response+: queueToken（超时排队）
  - Source Use Case: UC-ORDER-CREATE（异常：支付网关超时）
  - Observability: queue_ticket_issue_latency
  - Compatibility: 向后兼容；客户端可选读取

  ## Events
  ### OrderCreated
  - Version: v1 (YYYY-MM-DD)
  - Payload: { orderId, items[], createdAt }
  - Emitter: Order Service
  - Consumers: Inventory, Marketing
  - Observability: order_created_events_count
  - Ordering/Idempotency: 每订单一次；幂等键=orderId

  ## 变更
  - [v1.0 - YYYY-MM-DD] 首版

- testing-plan.md
  # Testing Plan

  ## Acceptance
  - AT-ORDER-01 创建订单（主流程）
    Preconditions: 用户已登录；有库存
    Steps: 创建订单
    Expected: 返回待支付；产生 OrderCreated
    Links: UC-ORDER-CREATE；contracts: POST /orders v1

  ## Contract
  - CT-ORDERS-POST-01 字段与兼容性
    Preconditions: -
    Steps: 校验请求/响应字段；v1与v1.1 兼容
    Expected: v1.1 新增 queueToken 不影响老客户端
    Links: POST /orders v1, v1.1

  ## Performance/Load
  - PT-ORDER-PEAK-01 峰值延迟
    Preconditions: QPS=2,000；支付网关延迟200–500ms
    Steps: 施加负载，执行下单
    Expected: p95250ms 连续15m 告警；库存依赖错误率>10% 告警；审计延迟>5s 告警

  ## Operations
  - 扩容：水平扩展 API 与队列消费者
  - 降级：开启排队开关，降低重试次数
  - 回退：禁用 v1.1 字段回退策略
  - 补偿：触发 order_compensation 扫描与重放

  ## Troubleshooting
  - 症状：下单延迟升高
    诊断：查看 order_create_latency_p95、队列长度、库存依赖错误率
    缓解：限流、临时提升消费者数、开启降级

  ## 变更
  - [v1.0 - YYYY-MM-DD] 首版

---

使用建议
- 初稿：对四个文件逐一生成整稿；严格遵循文件契约；只输出该文件内容与“v1.0 首版”变更条目。
- 修订：复制需要修改的“版本块/测试项/章节”到对话中，请求“仅输出该段修订版”。
- 扩展：contracts 或 testing-plan 变长时按上下文拆分为 contracts-order.md、testing-plan-order.md；原文件保留索引与“参见”占位。
