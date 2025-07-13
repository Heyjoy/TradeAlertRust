// 用户会话管理
(function() {
    'use strict';
    
    // 生成唯一用户ID
    function generateUserId() {
        const timestamp = Date.now();
        const random = Math.random().toString(36).substr(2, 9);
        return `user_${timestamp}_${random}`;
    }
    
    // 获取或创建用户ID
    function getUserId() {
        let userId = localStorage.getItem('trade_alert_user_id');
        if (!userId) {
            userId = generateUserId();
            localStorage.setItem('trade_alert_user_id', userId);
            console.log('创建新用户ID:', userId);
        }
        return userId;
    }
    
    // 全局设置用户ID
    window.TradeAlert = window.TradeAlert || {};
    window.TradeAlert.userId = getUserId();
    
    // 修改所有API请求以包含用户ID
    const originalFetch = window.fetch;
    window.fetch = function(url, options = {}) {
        // 只拦截API请求
        if (url.startsWith('/api/')) {
            options.headers = options.headers || {};
            options.headers['X-User-Id'] = window.TradeAlert.userId;
        }
        return originalFetch.call(this, url, options);
    };
    
    // 修改jQuery AJAX请求（如果使用jQuery）
    if (window.jQuery) {
        jQuery.ajaxPrefilter(function(options, originalOptions, jqXHR) {
            if (options.url && options.url.startsWith('/api/')) {
                jqXHR.setRequestHeader('X-User-Id', window.TradeAlert.userId);
            }
        });
    }
    
    // 演示模式检测和提示
    function checkDemoMode() {
        // 通过检查URL参数或其他方式检测演示模式
        const urlParams = new URLSearchParams(window.location.search);
        if (urlParams.get('demo') === 'true' || window.location.hostname.includes('demo')) {
            showDemoBanner();
        }
    }
    
    // 显示演示模式横幅
    function showDemoBanner() {
        const banner = document.createElement('div');
        banner.className = 'alert alert-info alert-dismissible fade show demo-banner';
        banner.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 9999;
            margin: 0;
            border-radius: 0;
            text-align: center;
        `;
        banner.innerHTML = `
            <div class="container">
                <span class="me-2">🔬</span>
                <strong>演示模式</strong> - 这是一个演示环境，您的数据不会被保存，且与其他用户隔离
                <span class="ms-2">用户ID: ${window.TradeAlert.userId.substr(0, 12)}...</span>
                <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
            </div>
        `;
        
        // 插入到页面顶部
        document.body.insertBefore(banner, document.body.firstChild);
        
        // 为页面内容添加顶部边距
        document.body.style.paddingTop = '60px';
    }
    
    // 页面加载完成后执行
    document.addEventListener('DOMContentLoaded', function() {
        checkDemoMode();
        
        // 在开发者工具中显示用户信息
        console.log('TradeAlert 用户会话信息:');
        console.log('- 用户ID:', window.TradeAlert.userId);
        console.log('- 会话时间:', new Date().toLocaleString());
        
        // 添加用户信息到页面（可选）
        const userInfo = document.getElementById('user-info');
        if (userInfo) {
            userInfo.textContent = `用户: ${window.TradeAlert.userId.substr(0, 8)}...`;
        }
    });
    
    // 导出工具函数
    window.TradeAlert.getUserId = getUserId;
    window.TradeAlert.showDemoBanner = showDemoBanner;
    
})();