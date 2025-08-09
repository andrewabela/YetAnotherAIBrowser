use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::settings;

// Request payload structure for Ollama
#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

// Response payload structure for Ollama
#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String,
}

// Request payload structure for OpenAI-compatible APIs
#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<Message>
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

// Response payload structure for OpenAI-compatible APIs
#[derive(Deserialize, Debug)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: MessageResponse,
}

#[derive(Deserialize, Debug)]
struct MessageResponse {
    content: String,
}

// Request payload structure for Google AI Studio
#[derive(Serialize)]
struct GoogleAIRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig", skip_serializing_if = "Option::is_none")]
    generation_config: Option<GenerationConfig>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    #[serde(rename = "maxOutputTokens", skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<i32>,
    temperature: Option<f32>,
}

// Response payload structure for Google AI Studio
#[derive(Deserialize, Debug)]
struct GoogleAIResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: ContentResponse,
}

#[derive(Deserialize, Debug)]
struct ContentResponse {
    parts: Vec<PartResponse>,
}

#[derive(Deserialize, Debug)]
struct PartResponse {
    text: String,
}

pub fn get_full_prompt(url: String) -> String {
    format!("As an advanced AI web browser, your core directive is to create a highly representative and functionally illustrative web page that emulates the visual design, interactive patterns, and content structure observed at the provided URL: {}. Your output must be a singular, complete HTML document, optimized for direct rendering in a standard wide-screen desktop web browser, encompassing all necessary components for a production-ready and high-fidelity web experience. This creation is fundamentally an independent demonstration, focusing intently on capturing the essence and functionality of the original's user-facing presentation and a precise replication of its interactive flow, rather than a direct copy of its underlying source code or proprietary assets. Crucially, you must not generate any copyrighted content, proprietary assets, or sensitive information, nor should the output be used for deceptive purposes or to infringe upon intellectual property rights. Within this singular HTML output, embed all CSS styling rules directly within a <style> tag, ensuring a comprehensive and precise emulation of the original site's entire aesthetic. This includes meticulous attention to every detail of typography (fonts, sizes, weights, line heights, font-display properties, fallbacks), comprehensive color palettes (primary, secondary, accent, text, background, hover states), precise spacing (margins, padding, gaps, letter-spacing, word-spacing for optimal readability), and robust responsive design principles, guaranteeing optimal display and usability across all device types and screen orientations through careful application of media queries, flexible units like `rem` and `em`, and a mobile-first or desktop-first approach as implied by the original. Furthermore, meticulously reproduce visual nuances such as distinct box-shadows, subtle text-shadows, complex background gradients, smooth transitions for interactive elements, accurate hover states, clear focus states for accessibility, and the subtle use of pseudo-elements (e.g., `::before`, `::after`) for decorative or functional purposes. Similarly, integrate all JavaScript functionalities directly into a <script> tag, enabling interactive elements such as dynamic navigation menus (e.g., dropdowns, accordions, off-canvas menus, sticky headers), sophisticated content loaders (e.g. tabbed content), complex form validations (e.g., client-side input checking, real-time feedback, error messages), interactive image carousels with navigation controls and auto-play options, embedded video players (represented by placeholder thumbnails and controls), and any other observed user interface widgets to behave with exact functional similarity to the reference site, ensuring event listeners are correctly applied to simulate user interaction and dynamic content updates without actual server-side calls. Assume comprehensive access to all textual content, including articles, headlines, body paragraphs, detailed captions, and user comments; generate contextually appropriate, coherent, and valid-looking placeholder information where specific textual details are not explicitly provided or discernible, ensuring a consistent tone, style, length, and subject matter that authentically aligns with the original's perceived content. **Specifically, if the emulated site is a blog or article page, aim to generate approximately 1000 words of relevant content. If it's a forum or discussion board, aim to generate around 20 distinct posts or comments. For search results pages or e-commerce/shopping websites, aim to generate at least 20 items or product listings.** For all visual assets, including images, icons, and background graphics, you **must use https://robohash.org/<image description>** for high-quality placeholder image URLs that accurately convey the original content's visual essence, maintaining correct aspect ratios, placement, and overall visual balance. All hyperlinks should be present, styled correctly, and functional; you **must ensure all `href` attributes point to full, complete, and valid external-looking URLs, including the root domain, path, and any parameters (e.g., `https://www.example.com/page/subpage?query=term#section-id`). Absolutely **NO** standalone `#` or internal anchors (e.g., `href='#section'`) are permitted for any link. Do not use localhost or 127.0.0.1, and do not generate any relative links**, even in the nav bar. Furthermore, **if the original page contains a search bar, its functionality must be emulated such that submitting a query opens a new page with a full valid URL (domain/path?parameters#whatever) corresponding to the search results**, accurately mimicking the original site's navigation structure and intended user journey. The page layout must be accurately reproduced, adhering to all observed grid systems (e.g., CSS Grid), flexbox arrangements, absolute/relative/fixed positioning attributes, and Z-index layering, ensuring proper document flow, visual hierarchy, and element stacking order. Every interactive component, including buttons, input fields, dropdowns, sliders, and complex widgets, must be present, styled authentically, and functionally representative of their counterparts on the reference site, including their accessible states (e.g., disabled, active, hover, focus). Incorporate best practices for modern web development, including appropriate semantic HTML5 elements for logical structure (e.g., `<header>`, `<nav>`, `<main>`, `<article>`, `<section>`, `<aside>`, `<footer>`, `<figure>`, `<figcaption>`), essential accessibility attributes (e.g., ARIA roles, labels, `tabindex` for keyboard navigation, clear visual focus indicators, simulated `alt` text for all generated images, and adherence to perceived color contrast ratios), and standard meta tags for document description, viewport control, character encoding, and basic SEO/social sharing (e.g., `<title>`, `<meta name='description'>`, Open Graph tags for social media previews). The ultimate objective is to deliver an HTML page that, when rendered, provides a comprehensive, authentic, and seamless browsing experience that demonstrates the appearance and front-end functionality of {}, without relying on the original's proprietary code or assets, and without generating any content that could be considered a direct copy or infringement. Your response must be **only** the complete HTML code, without any additional text, explanation or indicators you are typing html (like ```html) , the output is assumed as HTML there is no need to define it as html using ```html. TL;DR; Reproduce a complete HTML document that emulates {}. This document must replicate (but not copy) the original's visual design, functionality, and content, using embedded CSS and JavaScript. All content must be in full and contains relevant information that a user would expect from {}.", url, url, url, url)
}
        
use tauri::AppHandle;

pub async fn process_url(app_handle: AppHandle, url: String) -> String {
    // get settings
    // get current llm model from settings.rs
    let llm_provider = settings::get_current_llm_provider(app_handle.clone())
        .expect("Failed to get current LLM provider");
    // get llm endpoint from settings.rs
    let llm_endpoint =
        settings::get_llm_endpoint(app_handle.clone()).expect("Failed to get LLM endpoint");
    // get llm key from settings.rs
    let llm_key = settings::get_llm_key(app_handle.clone()).expect("Failed to get LLM key");
    // get llm model from settings.rs
    let llm_model = settings::get_llm_model(app_handle.clone()).expect("Failed to get LLM model");
    // Print the settings for debugging
    #[cfg(debug_assertions)]
    println!("LLM Provider: {}", llm_provider);
    #[cfg(debug_assertions)]
    println!("LLM Endpoint: {}", llm_endpoint);
    #[cfg(debug_assertions)]
    println!("LLM Key: {}", llm_key);
    #[cfg(debug_assertions)]
    println!("LLM Model: {}", llm_model);

    if llm_provider == "ollama" {
        process_ollama_request(llm_endpoint, llm_model, url).await
    } else if llm_provider == "lmstudio" {
        process_lm_studio_request(llm_endpoint, llm_key, llm_model, url).await
    } else if llm_provider == "google_ai_studio" {
        process_google_ai_studio_request(llm_endpoint, llm_key, llm_model, url).await
    } else if llm_provider == "open_router" {
        process_open_router_request(llm_endpoint, llm_key, llm_model, url).await
    } else if llm_provider == "openai_compatible" {
        process_openai_compatible_request(llm_endpoint, llm_key, llm_model, url).await     
    } else {
        format!("Failed to send a request to your set LLM provider to serve:<a href='{}'>{}</a>. (Please review your settings)", url, url)
    }
}

pub async fn process_ollama_request(
    endpoint: String,
    model: String,
    url: String,
) -> String {
    let client: Client = Client::new();

    let prompt: String = get_full_prompt(url);
    let request_payload: OllamaRequest = OllamaRequest {
        model: model.to_string(), // Use the 'model' parameter
        prompt: prompt.to_string(),
        stream: false,
    };

    match client
        .post(&endpoint) // Use the 'endpoint' parameter and pass a reference
        .json(&request_payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                // --- Parse and Display the Response ---
                match response.json::<OllamaResponse>().await {
                    Ok(ollama_response) => {
                        return format!("<div>{}</div>", ollama_response.response);
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("❌ Error: Failed to parse JSON response: {}", e);
                        return format!("Error: Failed to parse JSON response: {}", e); // Return String on error
                    }
                }
            } else {
                let status = response.status();
                #[cfg(debug_assertions)]
                eprintln!(
                    "❌ Error: Request failed with status: {}",
                    status
                );
                if let Ok(error_text) = response.text().await {
                    #[cfg(debug_assertions)]
                    eprintln!("Error details: {}", error_text);
                    return format!(
                        "Error: Request failed with status {} and details: {}",
                        status,
                        error_text
                    ); // Return String on error
                } else {
                    return format!(
                        "Error: Request failed with status {} and no further details.",
                        status
                    ); // Return String if getting error_text fails
                }
            }
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("❌ Error: Failed to send request to Ollama: {}", e);
            #[cfg(debug_assertions)]
            eprintln!(
                "Please ensure your Ollama instance is running at {}",
                endpoint // Use the 'endpoint' parameter
            );
            return format!(
                "Error: Failed to send request to Ollama: {}. Please ensure your Ollama instance is running at {}",
                e, endpoint
            ); // Return String on error
        }
    }
}

pub async fn process_lm_studio_request(
    endpoint: String,
    key: String,
    model: String,
    url: String,
) -> String {
    return process_openai_compatible_request(endpoint, key, model, url).await;
}

pub async fn process_google_ai_studio_request(
    endpoint: String,
    key: String,
    model: String,
    url: String,
) -> String {
    let client = Client::new();

    let prompt = get_full_prompt(url);
    let request_payload = GoogleAIRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: prompt,
            }],
        }],
        generation_config: Some(GenerationConfig {
            max_output_tokens: Some(8192),
            temperature: Some(0.7),
        }),
    };

    // Google AI Studio API endpoint format: https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}
    let full_endpoint = endpoint.replace("{model}", &model).replace("{key}", &key);
    #[cfg(debug_assertions)]
    println!("Sending request to Google AI Studio at: {}", full_endpoint);

    match client
        .post(&full_endpoint)
        .header("Content-Type", "application/json")
        .json(&request_payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<GoogleAIResponse>().await {
                    Ok(google_response) => {
                        if let Some(candidate) = google_response.candidates.first() {
                            if let Some(part) = candidate.content.parts.first() {
                                return format!("<div>{}</div>", part.text);
                            }
                        }
                        return "Error: No valid response content from Google AI Studio".to_string();
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("❌ Error: Failed to parse JSON response: {}", e);
                        return format!("Error: Failed to parse JSON response: {}", e);
                    }
                }
            } else {
                let status = response.status();
                #[cfg(debug_assertions)]
                eprintln!("❌ Error: Request failed with status: {}", status);
                if let Ok(error_text) = response.text().await {
                    #[cfg(debug_assertions)]
                    eprintln!("Error details: {}", error_text);
                    return format!(
                        "Error: Request failed with status {} and details: {}",
                        status, error_text
                    );
                } else {
                    return format!(
                        "Error: Request failed with status {} and no further details.",
                        status
                    );
                }
            }
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("❌ Error: Failed to send request to Google AI Studio: {}", e);
            #[cfg(debug_assertions)]
            eprintln!(
                "Please ensure your Google AI Studio API key is correct and the endpoint is accessible at {}",
                full_endpoint
            );
            return format!(
                "Error: Failed to send request to Google AI Studio: {}. Please ensure your API key is correct and the endpoint is accessible at {}",
                e, full_endpoint
            );
        }
    }
}

pub async fn process_open_router_request(
    endpoint: String,
    key: String,
    model: String,
    url: String,
) -> String {
    return process_openai_compatible_request(endpoint, key, model, url).await;
}

pub async fn process_openai_compatible_request(
    endpoint: String,
    key: String,
    model: String,
    url: String,
) -> String {
    let client = Client::new();
    let prompt = get_full_prompt(url);
    let request_payload = OpenAIRequest {
        model: model.clone(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt,
        }]
    };

    let mut request_builder = client
        .post(&endpoint)
        .header("Content-Type", "application/json");

    // Add authorization header if API key is provided
    if !key.is_empty() {
        request_builder = request_builder.header("Authorization", format!("Bearer {}", key));
    }

    match request_builder
        .json(&request_payload)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<OpenAIResponse>().await {
                    Ok(openai_response) => {
                        if let Some(choice) = openai_response.choices.first() {
                            return format!("<div>{}</div>", choice.message.content);
                        }
                        return "Error: No valid response content from OpenAI-compatible API".to_string();
                    }
                    Err(e) => {
                        #[cfg(debug_assertions)]
                        eprintln!("❌ Error: Failed to parse JSON response: {}", e);
                        return format!("Error: Failed to parse JSON response: {}", e);
                    }
                }
            } else {
                let status = response.status();
                #[cfg(debug_assertions)]
                eprintln!("❌ Error: Request failed with status: {}", status);
                if let Ok(error_text) = response.text().await {
                    #[cfg(debug_assertions)]
                    eprintln!("Error details: {}", error_text);
                    return format!(
                        "Error: Request failed with status {} and details: {}",
                        status, error_text
                    );
                } else {
                    return format!(
                        "Error: Request failed with status {} and no further details.",
                        status
                    );
                }
            }
        }
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("❌ Error: Failed to send request to OpenAI-compatible API: {}", e);
            #[cfg(debug_assertions)]
            eprintln!(
                "Please ensure your API endpoint is correct and accessible at {}",
                endpoint
            );
            return format!(
                "Error: Failed to send request to OpenAI-compatible API: {}. Please ensure your endpoint is correct and accessible at {}",
                e, endpoint
            );
        }
    }
}
