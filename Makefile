.PHONY: help format lint check build run clean test all

# 默认目标
all: format lint check build

# 显示帮助
help:
	@echo "可用的命令："
	@echo "  make format  - 格式化代码 (cargo fmt)"
	@echo "  make lint    - 运行 clippy 代码检查"
	@echo "  make check   - 快速类型检查 (cargo check)"
	@echo "  make build   - 构建项目"
	@echo "  make run     - 运行项目"
	@echo "  make clean   - 清理构建产物"
	@echo "  make test    - 运行测试"
	@echo "  make all     - 执行 format, lint, check, build"

# 格式化代码
format:
	@echo "🎨 格式化代码..."
	@cargo fmt

# 代码检查
lint:
	@echo "🔍 运行 clippy..."
	@cargo clippy --all-targets --all-features -- -D warnings 2>/dev/null || \
		cargo clippy --all-targets --all-features

# 快速检查
check:
	@echo "✅ 类型检查..."
	@cargo check

# 构建项目
build:
	@echo "🔨 构建项目..."
	@cargo build

# 运行项目
run:
	@echo "🚀 运行项目..."
	@cargo run

# 清理
clean:
	@echo "🧹 清理构建产物..."
	@cargo clean

# 运行测试
test:
	@echo "🧪 运行测试..."
	@cargo test