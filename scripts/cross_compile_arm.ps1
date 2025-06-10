# ARM 交叉编译自动化脚本
# 使用方法: .\scripts\cross_compile_arm.ps1

param(
    [Parameter(Mandatory=$false)]
    [ValidateSet("armv7", "aarch64", "auto")]
    [string]$Architecture = "auto",
    
    [Parameter(Mandatory=$false)]
    [string]$NasIP = ""
)

# 颜色输出函数
function Write-ColorOutput($ForegroundColor) {
    $fc = $host.UI.RawUI.ForegroundColor
    $host.UI.RawUI.ForegroundColor = $ForegroundColor
    if ($args) {
        Write-Output $args
    }
    $host.UI.RawUI.ForegroundColor = $fc
}

Write-ColorOutput Green "🛡️ TradeAlert ARM 交叉编译脚本"
Write-ColorOutput Green "=================================="

# 检查是否在项目根目录
if (!(Test-Path "Cargo.toml")) {
    Write-ColorOutput Red "❌ 错误：请在项目根目录运行此脚本"
    exit 1
}

Write-ColorOutput Yellow "📋 检查环境..."

# 检查 Rust 安装
if (!(Get-Command "rustc" -ErrorAction SilentlyContinue)) {
    Write-ColorOutput Red "❌ 未找到 Rust 安装"
    Write-ColorOutput Yellow "请先安装 Rust: https://rustup.rs/"
    exit 1
}

$rustVersion = rustc --version
Write-ColorOutput Green "✅ Rust 版本: $rustVersion"

# 检查 cross 工具
if (!(Get-Command "cross" -ErrorAction SilentlyContinue)) {
    Write-ColorOutput Yellow "📦 安装 cross 交叉编译工具..."
    cargo install cross
    if ($LASTEXITCODE -ne 0) {
        Write-ColorOutput Red "❌ cross 安装失败"
        exit 1
    }
}

Write-ColorOutput Green "✅ cross 工具已就绪"

# 确定目标架构
$targetArch = ""
$targetTriple = ""

if ($Architecture -eq "auto" -and $NasIP -ne "") {
    Write-ColorOutput Yellow "🔍 自动检测 NAS 架构..."
    
    # 尝试通过 SSH 检测架构（需要先配置 SSH）
    Write-ColorOutput Yellow "💡 如果已配置 SSH，将尝试自动检测架构"
    Write-ColorOutput Yellow "   否则请手动指定 -Architecture armv7 或 -Architecture aarch64"
    
    $sshTest = ssh -o ConnectTimeout=5 -o BatchMode=yes tradealert@$NasIP "uname -m" 2>$null
    if ($LASTEXITCODE -eq 0) {
        if ($sshTest -eq "armv7l") {
            $targetArch = "armv7"
            $targetTriple = "armv7-unknown-linux-gnueabihf"
        } elseif ($sshTest -eq "aarch64") {
            $targetArch = "aarch64"
            $targetTriple = "aarch64-unknown-linux-gnu"
        }
        Write-ColorOutput Green "✅ 检测到架构: $sshTest ($targetArch)"
    } else {
        Write-ColorOutput Yellow "⚠️ 无法自动检测，请手动指定架构"
        Write-ColorOutput Yellow "   使用 -Architecture armv7 或 -Architecture aarch64"
        exit 1
    }
} else {
    switch ($Architecture) {
        "armv7" {
            $targetArch = "armv7"
            $targetTriple = "armv7-unknown-linux-gnueabihf"
        }
        "aarch64" {
            $targetArch = "aarch64" 
            $targetTriple = "aarch64-unknown-linux-gnu"
        }
        default {
            Write-ColorOutput Red "❌ 请指定有效的架构: -Architecture armv7 或 -Architecture aarch64"
            exit 1
        }
    }
}

Write-ColorOutput Yellow "🎯 编译目标: $targetTriple"

# 添加编译目标
Write-ColorOutput Yellow "📦 添加编译目标..."
rustup target add $targetTriple

# 执行交叉编译
Write-ColorOutput Yellow "🔨 开始交叉编译..."
Write-ColorOutput Yellow "   这可能需要几分钟时间..."

cross build --release --target $targetTriple --bin trade_alert_rust

if ($LASTEXITCODE -ne 0) {
    Write-ColorOutput Red "❌ 编译失败"
    exit 1
}

# 检查编译结果
$binaryPath = "target\$targetTriple\release\trade_alert_rust"
if (Test-Path $binaryPath) {
    $fileSize = (Get-Item $binaryPath).Length
    $fileSizeMB = [math]::Round($fileSize / 1MB, 2)
    Write-ColorOutput Green "✅ 编译成功！"
    Write-ColorOutput Green "   文件位置: $binaryPath"
    Write-ColorOutput Green "   文件大小: $fileSizeMB MB"
} else {
    Write-ColorOutput Red "❌ 编译文件未找到"
    exit 1
}

# 生成上传脚本
$uploadScript = @"
# ARM 二进制文件上传脚本
# 生成时间: $(Get-Date)

# 请替换为您的 NAS IP 地址
`$NAS_IP = "$NasIP"

if (`$NAS_IP -eq "" -or `$NAS_IP -eq "your-nas-ip") {
    Write-Host "请先编辑此脚本，设置正确的 NAS IP 地址" -ForegroundColor Red
    exit 1
}

Write-Host "📤 上传到 NAS..." -ForegroundColor Yellow

# 上传二进制文件
scp "$binaryPath" tradealert@`$NAS_IP:/volume1/apps/trade-alert/

# 上传静态文件
scp -r templates tradealert@`$NAS_IP:/volume1/apps/trade-alert/
scp -r static tradealert@`$NAS_IP:/volume1/apps/trade-alert/

if (`$LASTEXITCODE -eq 0) {
    Write-Host "✅ 上传完成！" -ForegroundColor Green
    Write-Host "🔗 请SSH连接到NAS继续配置: ssh tradealert@`$NAS_IP" -ForegroundColor Yellow
} else {
    Write-Host "❌ 上传失败" -ForegroundColor Red
}
"@

$uploadScriptPath = "scripts\upload_to_nas_$targetArch.ps1"
$uploadScript | Out-File -FilePath $uploadScriptPath -Encoding UTF8

Write-ColorOutput Green "📋 下一步操作："
Write-ColorOutput Yellow "1. 如需上传到NAS，运行: .\$uploadScriptPath"
Write-ColorOutput Yellow "2. 确保已在NAS上配置SSH和创建用户"
Write-ColorOutput Yellow "3. 上传完成后，SSH连接到NAS进行最终配置"

Write-ColorOutput Green "🎉 交叉编译完成！" 