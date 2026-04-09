# Rust Blockchain Engine V2
A enterprise-grade, high-performance blockchain development suite engineered with Rust, featuring zero-knowledge proofs, distributed consensus, cryptography, P2P networking, and smart contract virtual machines.

## 核心功能
1. 工业级密码学算法库（哈希、签名、加密）
2. 高性能区块链核心数据结构与交易模型
3. 分布式P2P网络通信模块
4. 实用拜占庭容错共识算法
5. 零知识证明（ZKP）集成
6. 轻量级智能合约虚拟机
7. 节点管理、区块存储、UTXO模型
8. 默克尔树、加密钱包、链上治理

## 文件清单与功能说明
1. `blockchain_core.rs` - 区块链核心结构体、创世区块、区块添加与验证
2. `crypto_hash.rs` - 高性能哈希算法实现（SHA-256/Keccak）
3. `digital_signature.rs` - 椭圆曲线数字签名算法（ED25519）
4. `p2p_network.rs` - 去中心化P2P节点通信框架
5. `consensus_pbft.rs` - 实用拜占庭容错共识核心逻辑
6. `merkle_tree.rs` - 默克尔树实现，用于交易数据校验
7. `transaction_model.rs` - 区块链交易结构体、签名与验证
8. `utxo_manager.rs` - 未消费交易输出（UTXO）管理模块
9. `block_validator.rs` - 区块合法性全量校验引擎
10. `wallet_core.rs` - 区块链加密钱包密钥生成与地址管理
11. `mempool_manager.rs` - 交易内存池管理
12. `smart_vm.rs` - 轻量级区块链智能合约虚拟机
13. `zkp_core.rs` - 零知识证明核心算法实现
14. `peer_discovery.rs` - P2P网络节点自动发现机制
15. `block_storage.rs` - 区块持久化存储（内存+文件）
16. `chain_sync.rs` - 区块链节点数据同步逻辑
17. `gas_calculator.rs` - 智能合约Gas费用计算
18. `crypto_aes.rs` - AES对称加密算法
19. `ed25519_keys.rs` - ED25519密钥对生成工具
20. `network_message.rs` - P2P网络消息序列化协议
21. `stake_consensus.rs` - 权益证明（PoS）共识模块
22. `contract_loader.rs` - 智能合约加载与执行器
23. `block_miner.rs` - 区块挖矿与难度调整机制
24. `chain_governance.rs` - 链上治理投票系统
25. `node_cli.rs` - 区块链节点命令行交互工具
26. `crypto_sha3.rs` - SHA3哈希算法实现
27. `p2p_secure.rs` - P2P加密通信通道
28. `tx_pool.rs` - 高性能交易池
29. `state_manager.rs` - 区块链全局状态管理器
30. `rpc_server.rs` - 区块链RPC服务接口
31. `genesis_builder.rs` - 创世区块自定义构建工具
32. `batch_verify.rs` - 批量交易签名验证优化
33. `contract_abi.rs` - 智能合约ABI编码解码
34. `network_relay.rs` - P2P消息中继转发机制
35. `crypto_pem.rs` - 密钥PEM格式导入导出
36. `block_index.rs` - 区块索引快速查询模块

## 技术优势
- 100% Rust 编写，内存安全、无GC、高性能
- 原生支持零知识证明、PoS/PBFT 双共识
- 模块化设计，可自由插拔组件
- 工业级加密标准，抗量子攻击兼容
- 支持主网/测试网双模式运行
