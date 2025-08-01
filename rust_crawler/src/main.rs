use reqwest;
use scraper::{Html, Selector};
use std::error::Error;
use url::{Url, Position};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 1. 定义起始 URL
    let start_url = "https://www.rust-lang.org/";
    println!("🚀 开始爬取: {}", start_url);

    // 2. 获取起始页内容
    let response = reqwest::get(start_url).await?;
    let body = response.text().await?;
    
    // 3. 提取页面中的链接
    let links = extract_links(&body, start_url)?;
    println!("🔗 找到 {} 个链接", links.len());
    
    // 4. 限制爬取的链接数量（避免过多请求）
    let links_to_crawl = links.iter().take(5).cloned().collect();
    
    // 5. 并发爬取链接
    println!("\n开始并发爬取...");
    crawl_concurrently(links_to_crawl).await;
    
    println!("\n所有爬取任务完成！");
    Ok(())
}

/// 并发爬取多个 URL
async fn crawl_concurrently(urls: Vec<String>) {
    let mut tasks = vec![];
    
    for url in urls {
        // 为每个 URL 创建一个异步任务
        let task = tokio::spawn(async move {
            // 发送请求
            match reqwest::get(&url).await {
                Ok(response) => {
                    let status = response.status();
                    // 获取内容长度
                    let content_length = match response.text().await {
                        Ok(text) => text.len(),
                        Err(_) => 0,
                    };
                    
                    // 返回爬取结果
                    (url, status, content_length)
                }
                Err(e) => {
                    // 请求失败时返回错误信息
                    (url, reqwest::StatusCode::BAD_REQUEST, 0)
                }
            }
        });
        
        tasks.push(task);
    }
    
    // 等待所有任务完成并收集结果
    let mut results = vec![];
    for task in tasks {
        match task.await {
            Ok(result) => results.push(result),
            Err(_) => println!("任务执行失败"),
        }
    }
    
    // 打印爬取结果
    println!("\n爬取结果:");
    println!("{:<45} | {:<10} | {}", "URL", "状态码", "内容长度");
    println!("{:-<80}", "");
    
    for (url, status, length) in results {
        println!("{:<45} | {:<10} | {} 字节", url, status, length);
    }
}

/// 从 HTML 内容中提取所有有效链接
fn extract_links(html: &str, base_url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let base = Url::parse(base_url)?;
    let document = Html::parse_document(html);
    let selector = Selector::parse("a").unwrap();
    
    let mut links = Vec::new();

    for element in document.select(&selector) {
        if let Some(href) = element.value().attr("href") {
            if let Ok(absolute_url) = base.join(href) {
                if absolute_url.scheme() == "http" || absolute_url.scheme() == "https" {
                    let full_url = absolute_url[..Position::AfterPath].to_string();
                    links.push(full_url);
                }
            }
        }
    }

    links.sort();
    links.dedup();
    Ok(links)
}