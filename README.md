# hmbird_controller

自主掌控「风驰」性能开关的轻量级守护模块。  
无需云控、无需 Root、无需数据库——只要内核支持 hmbird，即可让指定应用自动进入高性能模式。

---

## 1. 功能一览
- **零依赖**：不连云端、不读数据库，单机运行。  
- **自动切换**：监听顶层应用，匹配配置文件后即时 `scx_enable=1`。  
- **心跳守护**：防止被系统或应用意外关闭。  
- **一键配置**：修改 `app_config.toml` → 重启服务生效。  

---

## 2. 快速开始
### 2.1 下载
| 渠道   | 链接                                                                 |
|--------|----------------------------------------------------------------------|
| 蓝云   | [wwp.lanzoup.com/iUMSR3blhtgd](https://wwp.lanzoup.com/iUMSR3blhtgd) |
| GitHub | [releases](https://github.com/reigadegr/hmbird_controller/releases) |

---

3. 技术栈

组件	用途	
Rust	高性能、内存安全	
tokio	异步运行时，低开销事件循环	
mimalloc	替代系统 malloc，降低碎片	
dumpsys-rs	快速获取顶层应用，减少 JNI 开销	

---

4. 兼容性
- 已测试：OnePlus 13（ColorOS 16，Android 16）  
- 理论支持：内核含 `hmbird` 子系统的任意设备  
- 其他机型：欢迎提 Issue 补充测试结果。

---

5. 许可证
MIT © 2025 reigadegr

---

免责声明：本工具仅用于学习与研究，修改系统参数可能造成游戏性能劣化。请于下载后24小时内删除
