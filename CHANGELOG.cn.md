[English Version](./CHANGELOG.md) | 中文版

# 变更日志 —— Rust SDK (`nps-rs`)

格式参考 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

在 NPS 达到 v1.0 稳定版之前，套件内所有仓库同步使用同一个预发布版本号。

---

## [1.0.0-alpha.20] — 发布候选，尚未发布

- 有边界实验：结果资源/授权、重复schema批次、本地任务交接和易失节点转移；各语言支持范围不同，未承诺六SDK完整运行时一致性。
- 保留与等价HTTPgzip及普通工具的真实测量；不宣称固定token、费用或速度收益。
- 候选使用统一版本、本地CI与独立包验证；完整生产能力、beta和实际发布保持后续门槛。

## [Unreleased]

- 按当前安全公告更新锁定的 Quinn/rustls/webpki/根证书/Tokio 依赖；已有锁文件的使用者也须更新自己的解析依赖图。

## [1.0.0-alpha.19] —— 2026-09-05

### 新增

- 新增公开的 `nps-conformance::alpha19` 策略入口，并可执行全部 47 条共享 NCP/NWP/NIP/NDP/NOP 加固向量，覆盖负向路径与 fixture 之外的边界分支。

### 变更

- 将 Node L2 扩展到 v0.7/38 cases，纳入当前 AaaS L2-01..L2-07；L2-01..05
  为强制项，L2-06/07 记 `na` 时必须携带有理由的 message。
- 将 Node L2 manifest 对齐 v0.6 与全部 31 个 case；validation 现会强制校验
  summary 完整性、整族适用性以及 single-/multi-Anchor HA 互斥约束。
- 注册 alpha.19 新增的协议错误码及其规范 NPS 状态映射。

## [1.0.0-alpha.18] —— 2026-08-15

### 新增

- 新增官方有状态 LLM context DTO、mutex 保护的进程内 store 与 Action Server coordinator，覆盖 owner 隔离、CAS reservation、生命周期 action、真异步执行、取消和 19 个共享一致性向量。

### 变更

- 跨 SDK 统一 unary request correlation、LLM usage 记账、严格有状态请求校验、任务所有权、provider 授权、超时处理与晚到结果拒绝。

## [1.0.0-alpha.17] —— 2026-08-02

### 新增

- 将参考服务端能力移植到 Rust SDK：NCP 原生传输、NWP action/complex/memory 节点与双向 bridge、NIP CA 服务与完整校验，以及 NOP 编排。
- 实现共享的 NCP 0.11、NWP 0.20、NIP 0.13、NDP 0.12 与 NOP 0.9
  可移植 Profile，并执行语言无关一致性 fixture。

### 变更

- 在整个 workspace 启用干净的 `cargo fmt` 与严格 Clippy 检查。
- 升级 `quinn-proto` 和 `rustls-webpki`，修复当前 RustSec 漏洞。

## [1.0.0-alpha.16] —— 2026-07-23

### 变更

- 套件级 alpha.16 源码同步与协议兼容性更新。

## [1.0.0-alpha.15] —— 2026-06-28

### 变更

- 套件级 alpha.15 同步：对齐包元数据、当前 README / 版本 banner、分发源树以及 release-prep 说明到 NPS-Dev。
- 承载源事实树中的 NCP Tier-3 BinaryVector、入站 NWP Bridge server 加固、NIP canonical trust/revoke，以及 NDP discovery canonical-form 对齐。

## [1.0.0-alpha.14] —— 2026-06-26

### Added

- `nps_nip::ca_client::NipCaClient`：远程 NIP CA 的类型化客户端，覆盖 discovery、CRL、agent/node 注册、X.509 注册、续签、撤销和校验。
- `nps_nwp::NwpNativeNodeServer`：native-mode NWP 服务端 helper，用于在已建立的 NCP stream 上分发 QueryFrame/ActionFrame。
- `nps-conformance` crate 及 `nps-sdk` re-export：TC-N1/TC-N2 一致性用例目录、manifest 构造器和校验器，用于 CI/自认证流程。

---

## [1.0.0-alpha.11] —— 2026-05-28

### 新增

#### nps-nop
- `DagNode` compensation、policy/aggregate 常量、TaskState、cross-cluster delegate 与 AlignStream Ack/Nak。

#### nps-ndp
- security profile、registry enforcement、AnnounceFrame 字段，以及 typed GraphNode/GraphEdge/GraphFrame §5 结构。

#### nps-nip
- `IdentReputationPolicyHint`、`IdentFrame::reputation_policy` 与 `ocsp_staple`。

#### nps-nwp
- 带 subscription ID、filter、heartbeat、max-events 与 cursor 的 `SubscribeFrame`。

---

## [1.0.0-alpha.2] —— 2026-04-19

### Changed

- 版本升级至 `1.0.0-alpha.2`，与套件同步。除版本对齐外无功能变更。
- 88 tests 全绿。

### 涵盖模块

- nps-core / nps-ncp / nps-nwp / nps-nip / nps-ndp / nps-nop / nps-sdk

---

## [1.0.0-alpha.1] —— 2026-04-10

作为 NPS 套件 `v1.0.0-alpha.1` 的一部分首次公开 alpha。

[1.0.0-alpha.2]: https://github.com/LabAcacia/nps/releases/tag/v1.0.0-alpha.2
[1.0.0-alpha.1]: https://github.com/LabAcacia/nps/releases/tag/v1.0.0-alpha.1
