# 需求文档管理脚本
# 用于管理本地Git版本控制但不上传GitHub的需求文档

param(
    [Parameter(Position=0)]
    [ValidateSet("status", "commit", "log", "backup", "check", "help")]
    [string]$Action = "help",
    
    [Parameter(Position=1)]
    [string]$Message = ""
)

$RequirementPath = "docs/Requirement"
$BackupPath = "backup/requirements"

function Show-Help {
    Write-Host "📋 需求文档管理脚本" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "用法: .\manage_requirements.ps1 <action> [message]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "可用操作:" -ForegroundColor Green
    Write-Host "  status   - 查看需求文档状态"
    Write-Host "  commit   - 提交需求文档变更"
    Write-Host "  log      - 查看需求文档提交历史"
    Write-Host "  backup   - 备份需求文档"
    Write-Host "  check    - 检查文档完整性"
    Write-Host "  help     - 显示此帮助信息"
    Write-Host ""
    Write-Host "示例:" -ForegroundColor Yellow
    Write-Host "  .\manage_requirements.ps1 status"
    Write-Host "  .\manage_requirements.ps1 commit '更新A股策略需求'"
    Write-Host "  .\manage_requirements.ps1 backup"
}

function Show-Status {
    Write-Host "📊 需求文档状态检查" -ForegroundColor Cyan
    Write-Host ""
    
    # 检查需求文档目录是否存在
    if (-not (Test-Path $RequirementPath)) {
        Write-Host "❌ 需求文档目录不存在: $RequirementPath" -ForegroundColor Red
        return
    }
    
    # 显示Git状态
    Write-Host "🔍 Git状态:" -ForegroundColor Green
    git status $RequirementPath --porcelain
    
    # 统计文档数量
    $docCount = (Get-ChildItem "$RequirementPath\*.md" | Measure-Object).Count
    Write-Host ""
    Write-Host "📈 文档统计:" -ForegroundColor Green
    Write-Host "  需求文档总数: $docCount"
    
    # 检查是否有未提交的变更
    $changes = git status $RequirementPath --porcelain
    if ($changes) {
        Write-Host "⚠️  有未提交的变更" -ForegroundColor Yellow
    } else {
        Write-Host "✅ 所有变更已提交" -ForegroundColor Green
    }
}

function Commit-Requirements {
    param([string]$CommitMessage)
    
    Write-Host "💾 提交需求文档变更" -ForegroundColor Cyan
    Write-Host ""
    
    if (-not $CommitMessage) {
        $CommitMessage = Read-Host "请输入提交信息"
    }
    
    if (-not $CommitMessage) {
        Write-Host "❌ 提交信息不能为空" -ForegroundColor Red
        return
    }
    
    # 添加所有需求文档变更
    git add $RequirementPath
    
    # 提交变更
    $fullMessage = "需求文档: $CommitMessage"
    git commit -m $fullMessage
    
    Write-Host "✅ 需求文档已提交到本地Git" -ForegroundColor Green
    Write-Host "📝 提交信息: $fullMessage" -ForegroundColor Gray
    Write-Host ""
    Write-Host "⚠️  注意: 需求文档不会被推送到GitHub (受.gitignore保护)" -ForegroundColor Yellow
}

function Show-Log {
    Write-Host "📜 需求文档提交历史" -ForegroundColor Cyan
    Write-Host ""
    
    # 显示需求文档的提交历史
    git log --oneline --graph --decorate $RequirementPath | Select-Object -First 20
    
    Write-Host ""
    Write-Host "💡 提示: 使用 'git log $RequirementPath' 查看完整历史" -ForegroundColor Gray
}

function Backup-Requirements {
    Write-Host "💾 备份需求文档" -ForegroundColor Cyan
    Write-Host ""
    
    # 创建备份目录
    if (-not (Test-Path $BackupPath)) {
        New-Item -ItemType Directory -Path $BackupPath -Force | Out-Null
    }
    
    # 生成备份文件名
    $timestamp = Get-Date -Format "yyyyMMdd_HHmmss"
    $backupFile = "$BackupPath\requirements_backup_$timestamp.zip"
    
    # 创建ZIP备份
    try {
        Compress-Archive -Path "$RequirementPath\*" -DestinationPath $backupFile -Force
        Write-Host "✅ 需求文档已备份到: $backupFile" -ForegroundColor Green
        
        # 显示备份文件信息
        $backupInfo = Get-Item $backupFile
        Write-Host "📊 备份信息:" -ForegroundColor Gray
        Write-Host "  文件大小: $([math]::Round($backupInfo.Length / 1KB, 2)) KB"
        Write-Host "  创建时间: $($backupInfo.CreationTime)"
        
        # 清理旧备份（保留最近5个）
        $oldBackups = Get-ChildItem "$BackupPath\requirements_backup_*.zip" | 
                     Sort-Object CreationTime -Descending | 
                     Select-Object -Skip 5
        
        if ($oldBackups) {
            $oldBackups | Remove-Item -Force
            Write-Host "🧹 已清理 $($oldBackups.Count) 个旧备份文件" -ForegroundColor Gray
        }
        
    } catch {
        Write-Host "❌ 备份失败: $($_.Exception.Message)" -ForegroundColor Red
    }
}

function Check-Requirements {
    Write-Host "🔍 需求文档完整性检查" -ForegroundColor Cyan
    Write-Host ""
    
    # 检查必需文档是否存在
    $requiredDocs = @(
        "README.md",
        "1.1-PRD_MASTER.md",
        "REQUIREMENT_ID_REGISTRY.md"
    )
    
    $missingDocs = @()
    foreach ($doc in $requiredDocs) {
        $docPath = Join-Path $RequirementPath $doc
        if (-not (Test-Path $docPath)) {
            $missingDocs += $doc
        }
    }
    
    if ($missingDocs) {
        Write-Host "❌ 缺少必需文档:" -ForegroundColor Red
        $missingDocs | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
    } else {
        Write-Host "✅ 所有必需文档都存在" -ForegroundColor Green
    }
    
    # 检查文档编号规范
    Write-Host ""
    Write-Host "📋 文档编号检查:" -ForegroundColor Green
    
    $docs = Get-ChildItem "$RequirementPath\*.md" | Where-Object { $_.Name -match '^\d+\.\d+-' }
    $docCount = ($docs | Measure-Object).Count
    Write-Host "  标准编号文档: $docCount 个"
    
    # 检查Git忽略状态
    Write-Host ""
    Write-Host "🔒 Git忽略状态检查:" -ForegroundColor Green
    
    $gitStatus = git check-ignore $RequirementPath 2>$null
    if ($gitStatus) {
        Write-Host "  ✅ 需求文档目录已被Git忽略（不会上传到GitHub）" -ForegroundColor Green
    } else {
        Write-Host "  ⚠️  需求文档目录未被Git忽略" -ForegroundColor Yellow
        Write-Host "     请检查.gitignore文件配置" -ForegroundColor Yellow
    }
}

# 主执行逻辑
switch ($Action.ToLower()) {
    "status" { Show-Status }
    "commit" { Commit-Requirements -CommitMessage $Message }
    "log" { Show-Log }
    "backup" { Backup-Requirements }
    "check" { Check-Requirements }
    "help" { Show-Help }
    default { Show-Help }
} 