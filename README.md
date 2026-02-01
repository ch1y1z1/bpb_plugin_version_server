# 运行指南

## 环境要求

- Rust 工具链（cargo）
- 建议系统：Linux/macOS/Windows

## 快速开始

### 1. 编译并运行

```bash
# 编译
cargo build --release

# 运行
./target/release/bpb_plugin_version_server
```

或直接使用 cargo 运行：

```bash
cargo run
```

默认监听地址：`0.0.0.0:3000`

---

## 命令行参数

```
Usage: bpb_plugin_version_server [OPTIONS]

Options:
  -p, --port <PORT>            监听端口 (默认: 3000)
  -d, --data-file <DATA_FILE>  数据文件路径 (默认: data.json)
  -h, --help                   打印帮助信息
  -V, --version                打印版本号
```

### 示例

```bash
# 使用默认端口 3000
./bpb_plugin_version_server

# 指定端口
./bpb_plugin_version_server -p 8080
./bpb_plugin_version_server --port 8080

# 指定数据文件
./bpb_plugin_version_server -d /var/lib/version-server/data.json
./bpb_plugin_version_server --data-file /path/to/data.json

# 组合使用
./bpb_plugin_version_server -p 8080 -d /path/to/data.json
```

### 帮助信息

```bash
./bpb_plugin_version_server --help
```

---

## 日志管理

### 前台运行（控制台输出）

```bash
cargo run
```

### 后台运行 + 日志文件

**Linux/macOS:**

```bash
# 方式1：nohup
nohup cargo run > server.log 2>&1 &

# 方式2：使用 systemd（生产环境推荐）
# 创建服务文件 /etc/systemd/system/version-server.service
```

**使用 systemd 服务示例：**

创建 `/etc/systemd/system/version-server.service`：

```ini
[Unit]
Description=Version Server
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/path/to/your/project
ExecStart=/path/to/your/project/target/release/bpb_plugin_version_server
Restart=always
Environment=PORT=3000
StandardOutput=append:/var/log/version-server.log
StandardError=append:/var/log/version-server.log

[Install]
WantedBy=multi-user.target
```

启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable version-server
sudo systemctl start version-server
sudo systemctl status version-server
```

### Docker 运行

**Dockerfile:**

```dockerfile
FROM rust:1.75 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/bpb_plugin_version_server /app/
COPY --from=builder /app/data.json /app/
EXPOSE 3000
CMD ["./bpb_plugin_version_server"]
```

运行：

```bash
docker build -t version-server .
docker run -d -p 3000:3000 -v $(pwd)/data.json:/app/data.json version-server
```

---

## 生产环境建议

### 1. 使用反向代理（Nginx）

```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    location / {
        proxy_pass http://127.0.0.1:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}
```

### 2. 配置环境变量

创建 `.env` 文件：

```bash
PORT=3000
DATA_FILE=/var/lib/version-server/data.json
RUST_LOG=info
```

### 3. 日志轮转

使用 `logrotate` 管理日志：

```bash
# /etc/logrotate.d/version-server
/var/log/version-server.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 644 www-data www-data
    sharedscripts
    postrotate
        systemctl reload version-server
    endscript
}
```

### 4. 监控检查

```bash
# 健康检查
curl -f http://localhost:3000/get/test || exit 1
```

---

## 数据备份

数据文件 `data.json` 会自动创建在当前目录，建议定期备份：

```bash
# 备份脚本
#!/bin/bash
cp data.json data.json.backup.$(date +%Y%m%d)
```

---

## 常见问题

**Q: 端口被占用？**
```bash
# 查找占用端口的进程
lsof -i :3000
# 或
netstat -tlnp | grep 3000

# 终止进程
kill -9 <PID>
```

**Q: 外网无法访问？**
- 检查防火墙：`sudo ufw allow 3000`
- 确认 bind 地址为 `0.0.0.0` 而非 `127.0.0.1`
- 检查云服务器安全组规则

**Q: 权限不足？**
```bash
# 给执行权限
chmod +x target/release/bpb_plugin_version_server
```
