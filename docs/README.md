# AI辅助DDD软件工程模板

欢迎使用这个通用模板！它将领域驱动设计（DDD）的核心原则与AI深度融合，演进传统软件工程实践到AI时代。通过目录级规则机制（ai_rules.md），AI工具（如Claude、GPT集成插件或自定义脚本）可以自动适应不同阶段的思维模式，自治或引导生成文档、模型、代码和测试，从而减少手动工作、设计偏差和审查负担。

此模板适用于任何复杂软件系统（如企业应用或AI产品），强调DDD的通用语言（Ubiquitous Language）、界限上下文（Bounded Contexts）和聚合（Aggregates）。它不绑定特定编程语言或工具，但假设使用Git版本控制和Markdown/YAML格式。

## 模板概述
- **核心思路**：DDD流程分为5个阶段目录，每个目录有ai_rules.md（指导AI行为）。AI加载规则链（全局 > 阶段 > 子目录），生成输出。早期阶段（discovery/modeling）侧重AI引导人类协作；后期（implementation/testing/evolution）侧重AI自治执行。
- **益处**：
  - 自动化70-80%非代码工作。
  - 确保Clean Architecture和DDD一致性（e.g., 分层、不可变式验证）。
  - 支持演进：通过evolution/记录变更，保持文档活性和系统健康。
- **前提**：安装AI工具（e.g., VS Code with Claude extension）；项目置于Git仓库。

### 目录结构
```
project-root/
├── README.md  # 本文件：流程指南
├── ai_rules.md  # 全局规则（system-prompt）
├── 1-discovery/  # 需求发现（探索事件/上下文）
│   ├── ai_rules.md
│   └── ... (e.g., events_storm.md)
├── 2-modeling/  # 领域建模（聚合/不变式）
│   ├── ai_rules.md
│   └── ... (e.g., aggregates/order_aggregate.yaml)
├── 3-implementation/  # 代码实现（分层编码）
│   ├── ai_rules.md
│   └── ... (e.g., src/order_service.py)
├── 4-testing/  # 测试验证（单元/集成）
│   ├── ai_rules.md
│   └── ... (e.g., unit_tests/order_aggregate_test.py)
└── 5-evolution/  # 演进维护（变更记录/优化）
    ├── ai_rules.md
    └── ... (e.g., changes/change_log.md)
```

## DDD设计流程指南
DDD开发是一个迭代过程，从问题域探索到解决方案实现，再到持续演进。模板将它映射为线性阶段（可循环），强调输入/输出依赖：每个阶段的输出作为下一个的输入。最佳实践：从小迭代开始（e.g., 一个子域），使用AI生成草案，人类审阅/反馈。整个流程可与敏捷结合（e.g., sprint回顾时运行evolution）。

### 步骤1: 准备与启动
- **在哪里开始**：根目录，阅读本README和ai_rules.md理解全局规则。
- **与AI协作**：配置AI工具加载ai_rules.md（e.g., 通过插件）。输入方式：创建空文件，提供初始提示（e.g., "分析订单域需求"）。
- **最佳实践**：定义项目通用语言（在根目录添加ubiquitous_language.md）。初始化Git仓库。

### 步骤2: Discovery阶段（需求收集与发现）
- **定位**：探索问题域，识别事件、痛点和界限上下文。输入：业务PRD/访谈。
- **与AI协作**：在1-discovery/创建文件（e.g., events_storm.md），输入业务描述；AI引导脑暴（生成草案 + 澄清提问），人类反馈迭代。
- **完成文档**：事件列表（events_storm.md）、痛点表格（pain_points.md）、上下文映射（bounded_contexts.yaml）。
- **作为输入到下一步**：这些文档提供事件基础和边界定义。
- **最佳实践**：从小事件风暴开始；如果复杂，添加子目录（如events_storming/）细化。输出覆盖至少3个子域视角。时间：1-2天。

### 步骤3: Modeling阶段（领域建模）
- **定位**：构建战术模型，包括聚合、实体和不变式。输入：discovery输出（e.g., 事件列表）。
- **与AI协作**：在2-modeling/创建文件（e.g., aggregates/order_aggregate.yaml），输入上游文档；AI引导严谨设计（生成草案 + 验证提问），人类确认业务规则。
- **完成文档**：模型YAML（e.g., aggregate definitions with methods/invariants）。
- **作为输入到下一步**：这些模型作为代码契约。
- **最佳实践**：聚焦核心域；添加子目录（如aggregates/）分离实体。验证不变式通过伪代码模拟。时间：2-3天。

### 步骤4: Implementation阶段（实现与编码）
- **定位**：转化为分层代码，遵守Clean Architecture。输入：modeling模型。
- **与AI协作**：在3-implementation/创建文件（e.g., src/order_service.py），输入上游YAML；AI自治生成（先规划层分配，然后代码），人类审阅计划/输出。
- **完成文档**：代码文件（.py）、BDD spec（.feature）。
- **作为输入到下一步**：这些代码作为测试目标。
- **最佳实践**：先规划职责边界；添加子目录（如src/）组织层。确保幂等性和事件驱动。时间：3-5天。

### 步骤5: Testing阶段（测试与验证）
- **定位**：验证模型/实现，包括不变式和边界。输入：implementation代码。
- **与AI协作**：在4-testing/创建文件（e.g., unit_tests/order_aggregate_test.py），输入上游代码；AI自治生成（先规划覆盖，然后测试），人类运行/审阅报告。
- **完成文档**：测试脚本（.py）、覆盖报告（.md）。
- **作为输入到下一步**：这些测试作为演进基准。
- **最佳实践**：使用BDD for行为测试；添加子目录（如unit_tests/）分离类型。目标覆盖>80%。时间：2-4天。

### 步骤6: Evolution阶段（演进与维护）
- **定位**：记录迭代、变更原因，并优化。输入：全流程输出 + 新需求/Git diff。
- **与AI协作**：在5-evolution/创建文件（e.g., changes/change_log.md），输入变更细节；AI自治分析（生成记录 + 建议），人类决策重构。
- **完成文档**：变更日志（.md）、优化报告（.yaml）。
- **循环回流程**：输出反馈到早期阶段（e.g., 更新modeling模型），启动新迭代。
- **最佳实践**：按时间序组织（e.g., changes/子文件夹）；记录'why'（原因）和影响。定期运行（e.g., sprint末）。时间：1天/迭代。

### 完整流程循环
1. **启动**：Discovery -> Modeling（探索到模型）。
2. **构建**：Implementation -> Testing（代码到验证）。
3. **迭代**：Evolution -> 反馈到Discovery/其他（记录变更，重启循环）。
- **总时间估计**：初次MVP 1-2周；后续迭代1周。
- **关键依赖**：始终使用相对路径引用输入（e.g., [../1-discovery/events_storm.md]）；Git commit每个输出。

## 最佳实践
- **AI协作技巧**：早期阶段多反馈（e.g., "澄清这个不变式"）；后期审阅AI输出（e.g., 层计划）。如果AI卡住，检查规则继承。
- **工具推荐**：VS Code + AI插件；Git for版本控制；Markdown编辑器 for文档。
- **常见陷阱避免**：保持文档简洁（90%）、设计一致性。

## 如何开始
1. Clone此仓库。
2. 在1-discovery/创建第一个文件，输入需求，触发AI。
3. 跟随步骤推进，审阅/合并输出。
4. 贡献：Fork并PR改进规则/示例。

此模板桥接传统DDD与AI实践。如果问题，查看ai_rules.md或issue tracker。Happy designing！
