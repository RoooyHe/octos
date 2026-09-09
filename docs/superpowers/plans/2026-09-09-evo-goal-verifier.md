# 2026-09-09 evo-goal-verifier — 完成校验错误分类、有界恢复及证据账本

- Spec: `specs/task-evo-goal-verifier.spec.md` **v3**（agent-spec lint 100%，**29 scenarios**；v2 吸收外层复核 8 条，v3 吸收 GLM/k3 设计审查——双 APPROVE-WITH-CHANGES 收编）
- Native goal: `goal_01` (profile `octosfix`)
- 分支: `fix/evo-goal-verifier`；CARGO_TARGET_DIR=/Users/zhangalex/Work/Projects/FW/octos/target；CARGO_BUILD_JOBS=4；测试 --test-threads=8；无 cargo clean。

## v3 关键裁决（两 peer 分歧点）

- seam：自由函数（单次调用+纯函数解析 classify_verifier_reply）+ orchestrator wrapper `verify_goal_completion_bounded`（gate→attempt→per-attempt charge→status 式预算检查→重试→落账）。两 peer 一致。
- 解析规则采 k3 §2 最终版（fence 成对剥一层、反引号成对剥、|L|==1 且 L[0]==DONE（大小写不敏感保留现状）、NOT_DONE 前缀→InsufficientEvidence、其余含散文→InvalidResponse+原文截断回显）。
- 预算 gate 状态式（goal.status=="active" 才 attempt 2；charge 返回 Option<Value> 有 5 种 None 分支不可作判据）。
- 账本 outcome 五值（含 done）修正 v2 kind 硬伤；replay 不追加行不 charge；语义类永久阻断、infra 类 10min 冷却（防楔死）；digest=sha256 域分离+revision；fail-open。
- charge 迁 per-attempt 进 wrapper（charge_goal_verifier_usage 签名不动）。

## 步骤

1. [x] 合约：spec v3 + lint（100%，29 scenarios）。
2. [x] GLM/k3 双只读 peer 独立设计审查（design-glm.md / design-k3.md），先不互读。
3. [x] 定点互审收编：v3 合并裁决已落 spec；两 peer 结论一致度高，分歧（散文归类 InvalidResponse vs InsufficientEvidence）裁决为 InvalidResponse（k3：无法区分语义 vs 协议违规，归格式错更诚实）。
4. [ ] RED：按 spec 29 个场景写失败测试。
5. [ ] GREEN：分层实现（纯函数解析 → 自由函数重构 → wrapper → 账本 → 调用点迁移）。
6. [ ] 必跑验证：`cargo test -p octos-cli --features api --all-targets -- --test-threads=8`、`cargo clippy -p octos-cli --features api --all-targets -- -D warnings`、`cargo fmt --all -- --check`、spec lifecycle。
7. [ ] 独立实现审查（independent-glm.md / independent-k3.md）→ 互审（cross-glm.md / cross-k3.md），首审不覆盖。
8. [ ] 落 .octos/progress.md（真实 ids）+ 提交（仅本任务文件，禁 git add -A）+ 外层复验。

## 风险

- agent_orchestrator.rs 45k 行，改动需最小侵入（新类型放 goal_loop_runtime.rs，orchestrator 只加 wrapper + 改调用点）。
- 编译破坏面（k3 §6 实勘）：4 真实调用点 + goal_tool.rs 2016/2074 ignored 测试 + ~5 verifier 直测解构适配（20269/37829/2545 等）；maybe_complete_goal_from_model 全家 + ~14 处 Done 构造零改动（verdict 枚举不动的回报）。
- 存量 mock 返回空 content 的测试会看到 2 次调用（新重试语义）——RED 阶段排查点。
- budget_exhaustion 场景是唯一跨层（wrapper+charge+goal_update）集成测试，其余 gate 场景落 wrapper 层。
