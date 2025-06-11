use reqwest; // 用于发送 HTTP 请求
use scraper::{Html, Selector}; // 用于解析 HTML
use std::error::Error;
use url::{Url, Position}; // 用于 URL 处理

#[tokio::main] // 使用 Tokio 作为异步运行时
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 定义起始 URL
    let start_url = "https://www.rust-lang.org/";
    println!("🚀 开始爬取: {}", start_url);

    // 2. 发送 HTTP GET 请求
    let response = reqwest::get(start_url).await?;
    println!("✅ 收到响应 - 状态码: {}", response.status());

    // 3. 获取 HTML 内容
    let body = response.text().await?;
    println!("📄 获取到 HTML 内容 (长度: {} 字节)", body.len());

    // 4. 解析 HTML 并提取链接
    let links = extract_links(&body, start_url)?;
    println!("🔗 找到 {} 个链接:", links.len());

    // 5. 打印前10个链接
    for (i, link) in links.iter().take(10).enumerate() {
        println!("{}. {}", i + 1, link);
    }

    Ok(())
}

/// 从 HTML 内容中提取所有有效链接
fn extract_links(html: &str, base_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    // 解析基础 URL
    let base = Url::parse(base_url)?;
    
    // 创建 HTML 解析器
    let document = Html::parse_document(html);
    // 创建选择器查找所有 <a> 标签
    let selector = Selector::parse("a").unwrap();
    
    let mut links = Vec::new();

    // 遍历所有 <a> 元素
    for element in document.select(&selector) {
        // 获取 href 属性值
        if let Some(href) = element.value().attr("href") {
            // 处理相对路径并标准化 URL
            if let Ok(absolute_url) = base.join(href) {
                // 确保是 HTTP/HTTPS 协议
                if absolute_url.scheme() == "http" || absolute_url.scheme() == "https" {
                    // 获取完整 URL 字符串
                    let full_url = absolute_url[..Position::AfterPath].to_string();
                    links.push(full_url);
                }
            }
        }
    }

    // 去重并排序
    links.sort();
    links.dedup();

    Ok(links)
}