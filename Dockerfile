FROM docker.m.daocloud.io/ubuntu:24.04

# 设置非交互模式，避免在安装过程中出现提示
ENV DEBIAN_FRONTEND=noninteractive

# 更新apt包列表并安装所需的工具
# apt-get update: 更新包索引
# apt-get install -y: 安装包，-y表示自动同意安装提示
#   build-essential: 包含了编译C/C++程序所需的基本工具，如gcc, g++和make
#   flex, bison: 内核编译过程中需要用到的词法分析器和语法分析器
#   libssl-dev: 编译内核时可能需要OpenSSL开发库（用于内核签名等）
#   libelf-dev: 用于处理ELF文件（可执行文件、共享库等）的开发库
#   dwarves: 提供了pahole工具，用于分析调试信息，有时内核构建会用到
#   bc: 命令行计算器，内核配置脚本会用到
#   rsync: 文件同步工具，有时在部署或管理内核模块时有用
#   perl: 内核构建脚本中会用到Perl脚本
#   linux-headers-$(uname -r): 可选，但通常有助于交叉编译环境或模块构建
#   fakeroot: 用于模拟root权限，便于构建打包
#   xz-utils: 用于处理XZ压缩格式的工具，内核通常以XZ压缩
#   rustup: 用于Rust 编译程序所需的基本工具。
#   python: 提供Python 开发环境；
RUN for file in /etc/apt/sources.list.d/*.sources; do \
    # 备份原始文件（可选）
    # cp "$file" "${file}.bak"; \
    # 使用sed替换 archive.ubuntu.com 和 security.ubuntu.com
    sed -i 's|ports.ubuntu.com|mirrors.aliyun.com|g' "$file" && \
    sed -i 's|ports.ubuntu.com|mirrors.aliyun.com|g' "$file"; \
    done && \
    apt-get update && \
    apt-get install -y \
    build-essential \
    gcc-arm-linux-gnueabihf \
    gcc-x86-64-linux-gnu \
    gcc-mingw-w64-x86-64 \
    g++-mingw-w64-x86-64 \
    flex \
    bison \
    libssl-dev \
    libelf-dev \
    dwarves \
    bc \
    rsync \
    fakeroot \
    vim \
    xz-utils \
    git \
    curl \
    kmod \
    cpio\
    python3 \
    rustup && \
    # Install rustup - the Rust toolchain installer
    # Optional: HTTP Proxy setting
    export http_proxy=http://192.168.2.7:1087  && \
    export https_proxy=http://192.168.2.7:1087 && \
    # Use non-interactive installation by passing '-y' to the rustup-init script
    # curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    \
    # Add Rust's cargo bin directory to PATH for subsequent commands in the same RUN layer
    # For global access in future sessions, you'd add this to a profile file (e.g., ~/.bashrc)
    # or ensure CMD or ENTRYPOINT handles it.
    # For now, it's enough for this Dockerfile's single RUN command to see Cargo.
    export PATH="/root/.cargo/bin:$PATH" && \
    \
    # Optional: Install a specific Rust toolchain if needed (e.g., stable, nightly)
    # rustup toolchain install stable && \
    rustup default stable && \
    rustup toolchain install 1.90-aarch64-unknown-linux-gnu  && \
    # rustup target install armv7-unknown-linux-gnueabihf
    rustup target add armv7-unknown-linux-gnueabihf && \
    # rustup target install x86_64-unknown-linux-gnu
    rustup target add x86_64-unknown-linux-gnu && \
    rustup target add x86_64-pc-windows-gnu && \
    \
    # Optional: Clean up Rustup's temporary files, if any
    rm -rf /root/.rustup/tmp && \
    \
    # 清理APT缓存，减小镜像大小
    apt-get clean && \
    # Create WORKDIR
    mkdir -p /export/coderepo && \
    rm -rf /var/lib/apt/lists/*

# Set default working directory (optional, but useful for subsequent operations)
WORKDIR /export

# Set PATH for future container sessions (makes 'cargo' available immediately)
ENV PATH="/root/.cargo/bin:$PATH"

# 提示用户镜像已准备好
CMD ["bash"]
