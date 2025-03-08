.PHONY: build test run clean clippy fmt

# 默认构建目标
all: build

# 编译项目
build:
	@cargo build

# 运行测试
test:
	@cargo test

start-server: build
	@echo "启动区块链节点到后台..."
	@MINE_INTERVAL=30 AUTO_MINE=1 cargo run -- start & \
	echo $$! > server.pid
	@sleep 1  # 等待进程初始化
	@if ! ps -p $$(cat server.pid) > /dev/null; then \
		echo "服务进程启动失败"; \
		rm -f server.pid; \
		exit 1; \
	fi
	@for i in {1..10}; do \
		curl -s http://localhost:8080/blocks >/dev/null && exit 0 || sleep 3; \
	done; \
	echo "服务启动超时"; exit 1

stop-server:
	@if [ -f server.pid ]; then \
		kill -9 $$(cat server.pid) 2>/dev/null || true; \
		while lsof -i :8080 | grep LISTEN; do sleep 1; done; \
		rm -f server.pid; \
	fi

test-api: stop-server
	@sleep 2  # 等待端口释放
	@make start-server
	@echo "等待服务就绪..."
	@echo "等待服务就绪..."
	@trap 'make stop-server' EXIT
	@echo "运行API测试..."
	@echo "测试区块列表接口..."
	curl -sf -o /dev/null -w '%{http_code}' http://localhost:8080/blocks | grep -q 200
	curl -s http://localhost:8080/blocks | jq -e '. | length >= 1'
	@echo "测试区块详情接口..."
	curl -sf -o /dev/null -w '%{http_code}' http://localhost:8080/blocks/0 | grep -q 200
	@echo "验证链有效性..."
	curl -s -X POST http://localhost:8080/sync -H 'Content-Type: application/json' -d @blockchain.json | jq -e '.valid' | grep -q true

# 启动节点服务
start:
	@MINE_INTERVAL=30 AUTO_MINE=1 cargo run -- start

# 运行程序（支持带空格的参数）
run:
	@cargo run -- "$(ARGS)"

# 清理构建文件
clean:
	@cargo clean

# 代码风格检查
clippy:
	@cargo clippy --all-targets

# 代码格式化
fmt:
	@cargo fmt --all

# 显示帮助信息
help:
	@echo "可用命令:"
	@echo "  build     - 编译项目"
	@echo "  test      - 运行测试"
	@echo "  test-api  - 运行API接口测试"
	@echo "  start     - 启动节点服务"
	@echo "  clean     - 清理构建文件"
	@echo "  clippy    - 代码风格检查"
	@echo "  fmt       - 代码格式化"
	@echo "  help      - 显示帮助信息"
	@echo "  start-core - 启动区块链核心服务"
	@echo "  start-api  - 启动API服务"
start-core: build
	@echo "启动区块链核心服务..."
	@AUTO_MINE=1 cargo run --bin core &
	echo $$! > core.pid

start-api: build
	@echo "启动API服务..."
	@cargo run --bin api &
	echo $$! > api.pid

stop-all:
	@if [ -f core.pid ]; then kill -9 $$(cat core.pid) && rm core.pid; fi
	@if [ -f api.pid ]; then kill -9 $$(cat api.pid) && rm api.pid; fi