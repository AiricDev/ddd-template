# Quality AGENT

目标
- 用 BDD 的 Gherkin 特性文件表达非功能质量需求（性能、可靠性、安全、成本等），统一“业务可读、可验证”的语言。
- 明确采用“业务语言抽象层”的流派：Feature/Scenario 用通用业务语言描述行为与阈值，不写具体UI操作；技术实现映射放在 Step 层（API/服务/消息/UI）。
- 协作以“文件”为单位：每个 .feature 文件先整稿，再逐场景修订；必要时按类别或上下文拆分。

约束（关键要求，优先级最高）
- 禁止在抽象层 Feature/Scenario 中使用具体UI动作/控件术语（如 click、input、see、页面元素名）。仅在 e2e-*.feature 中允许出现 UI 步骤。
- Given/When/Then 必须使用领域通用语言与上下文（聚合/用例/事件/业务动作），不得依赖具体界面细节。
- Then 必须包含“可测阈值/可观测信号/审计事件”的断言，不得使用“更快/更安全”这类不可判定描述。

目录结构
- features/
  - performance.feature（抽象层）
  - reliability.feature（抽象层）
  - security.feature（抽象层）
  - cost.feature（可选，抽象层）
  - e2e-*.feature（仅在确有必要的端到端UI验收场景，数量受控）
- glue/
  - mapping.md（可选：Scenario → 用例/契约/观测信号/Step绑定渠道）
  - steps/
    - api-steps.md（抽象步骤 → API 调用）
    - service-steps.md（抽象步骤 → 服务层/事件注入）
    - ui-steps.md（仅 e2e 场景使用）

文件级结果契约与节奏
- 任一抽象层 .feature（如 performance.feature）
  - 结构（Gherkin）
    - Feature: （一句话业务意图或SLO/SLA）
    - Background:（可选）公共上下文（负载/依赖状态/数据规模/开关位）
    - Scenario:
      - Given …（业务/系统上下文，如“存在活动商品与有效库存”“网关延迟分布为200–500ms”）
      - When …（业务动作或系统事件，如“发生‘用户创建订单’的业务操作”“注入库存依赖超时”）
      - Then …（行为与阈值断言，如“p95150–200行或主题混杂时，按“类别/上下文/链路”拆分：如 performance-order.feature、security-public-apis.feature；原文件保留“参见”占位。

与上下游的映射
- 2-tactical/use-cases.md：将核心用例主动作映射为 When；异常/边界转为可靠性场景。
- 4-delivery/contracts.md 与 testing-plan.md：将 Then 中的指标与审计事件命名，写入契约/测试计划；必要时添加“基准/混沌/安全测试”占位。
- glue/mapping.md：维护 “Scenario → 用例/契约/观测信号/Step绑定渠道(API/Service/Message/UI)” 的最小追溯表。

文件顶部契约注释（粘贴到每个 .feature 顶部，以注释保留）

```
# 本文件为抽象层 BDD 特性文件（禁止使用UI具体操作术语，如 click/input/see）。
# Given/When/Then 仅使用业务语言与系统上下文；Then 必含可测阈值/观测信号/审计事件断言。
# 先整文件初稿（2–3场景），后逐场景修订；每轮仅改一个场景块；变更记录写在文件底部“## 变更”。
```

---

示例文件壳（可直接放入 features/）

- performance.feature
  Feature: 性能与容量目标（下单链路）
    目的：促销高峰保持下单体验的响应性

    Background:
      Given 存在活动商品与有效库存
      And 支付网关延迟分布为 200–500ms（偶发）

    Scenario: 峰值下单延迟与排队反馈
      Given 下单请求速率为 2,000 QPS
      When 发生“用户创建订单”的业务操作
      Then 下单接口 p95 延迟应小于 250ms
      And 排队票据返回延迟应小于 100ms
      And 产生观测信号 order_create_latency_p95, queue_ticket_issue_latency（按分钟聚合）

    Scenario Outline: 负载档位与目标延迟
      Given 下单请求速率为  QPS
      When 发生“用户创建订单”的业务操作
      Then 下单接口 p95 延迟应小于  ms
      Examples:
        | qps  | p95_ms |
        | 500  | 150    |
        | 1000 | 200    |
        | 2000 | 250    |

  # 变更
  # - [v1.0 - YYYY-MM-DD] 首版

- reliability.feature
  Feature: 可靠性与故障恢复（订单-库存依赖）
    目的：依赖故障时维持下单成功与补偿有序

    Scenario: 库存服务超时的降级与补偿
      Given 库存服务在 1 分钟内错误率为 20%
      And 单次库存调用超时阈值为 1s
      When 发生“用户创建订单”的业务操作
      Then 系统应降级为排队并返回可重试提示
      And 故障期间总体下单成功率（含排队）应不低于 95%
      And 产生补偿任务事件 order_compensation_created
      And 观测信号 inventory_dependency_error_rate, order_degraded_success_ratio 被采集

  # 变更
  # - [v1.0 - YYYY-MM-DD] 首版

- security.feature
  Feature: 最小权限与审计（外部订单查询）
    目的：仅授权客户端可访问订单查询，并留存审计

    Scenario: 基于 Scope 的授权控制与审计落库
      Given 外部客户端使用 OAuth2 调用订单查询能力，携带 scope orders:read
      When 发生“查询订单详情”的业务操作
      Then 授权通过率应为 100%，越权拒绝率应为 100%
      And 审计事件 order_read_audit 在 5s 内落库
      And 观测信号 orders_read_authorized_ratio, authz_denied_count, audit_event_latency 被采集

  # 变更
  # - [v1.0 - YYYY-MM-DD] 首版

- 可选：e2e-checkout.feature（仅在确有端到端UI验收需求时）
  Feature: 端到端结账旅程（UI）
    说明：此文件允许使用UI步骤，仅用于关键旅程的验收。抽象场景仍以抽象层 Feature 为准。

    Scenario: 用户完成结账
      Given 用户已登录并选定商品
      When 用户在结账页提交订单
      Then 页面展示“下单成功”提示并显示订单号
      And 产生审计事件 checkout_success_audit

- glue/mapping.md（可选）
  # 场景映射（Scenario → 用例/契约/观测/Step绑定）
  - performance.feature/峰值下单延迟与排队反馈
    - 用例：2-tactical/.../use-cases.md#UC-ORDER-CREATE
    - 契约：4-delivery/contracts.md#POST-/orders
    - 观测：order_create_latency_p95, queue_ticket_issue_latency
    - 绑定：API（POST /orders），Service（OrderApplicationService.createOrder）

  - reliability.feature/库存服务超时的降级与补偿
    - 用例：UC-ORDER-CREATE（异常：库存失败）
    - 事件：events.md#OrderQueued, OrderCompensationCreated
    - 观测：inventory_dependency_error_rate, order_degraded_success_ratio
    - 绑定：Service（故障注入），Message（补偿队列）

---

使用建议
- 生成初稿：针对每个 .feature 单独生成整稿，明确“禁止UI步骤”的约束；仅输出该文件内容与“## 变更”首条。
- 场景修订：复制目标 Scenario/Outline 到对话，要求仅输出该块的修订版；强调保持“抽象层，不含UI步骤”。
- 需要E2E时：新建 e2e-*.feature，UI步骤只写在此类文件；抽象层 Feature 不与其混排。
