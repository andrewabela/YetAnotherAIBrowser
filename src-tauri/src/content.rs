use reqwest::Client;
use serde::{Deserialize, Serialize};

// Request payload structure
#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

// Response payload structure
#[derive(Deserialize, Debug)]
struct OllamaResponse {
    response: String
}

pub async fn process_url(url: String) -> String {
    let ollama_url: &'static str = "http://localhost:11434/api/generate";
    let _model_name: &'static str = "deepseek-coder-v2:16b";

    let client: Client = Client::new();

    let prompt: String = format!("As an advanced AI web browser, your core directive is to create a highly representative and functionally illustrative web page that emulates the visual design, interactive patterns, and content structure observed at the provided URL: {}. Your output must be a singular, complete HTML document, optimized for direct rendering in a standard web browser, encompassing all necessary components for a production-ready and high-fidelity web experience. This creation is fundamentally an independent demonstration, focusing intently on capturing the essence and functionality of the original's user-facing presentation and a precise replication of its interactive flow, rather than a direct copy of its underlying source code or proprietary assets. Crucially, you must not generate any copyrighted content, proprietary assets, or sensitive information, nor should the output be used for deceptive purposes or to infringe upon intellectual property rights. Within this singular HTML output, embed all CSS styling rules directly within a <style> tag, ensuring a comprehensive and precise emulation of the original site's entire aesthetic. This includes meticulous attention to every detail of typography (fonts, sizes, weights, line heights), comprehensive color palettes, precise spacing (margins, padding, gaps), and robust responsive design principles, guaranteeing optimal display and usability across all device types and screen orientations. Similarly, integrate all JavaScript functionalities directly into a <script> tag, enabling interactive elements such as dynamic navigation menus, content loaders, complex form validations, image carousels, video players, and any other observed user interface widgets to behave with functional similarity to the reference site. Assume comprehensive access to all textual content, including articles, headlines, body paragraphs, detailed captions, and user comments; generate contextually appropriate, coherent, and valid-looking placeholder information where specific textual details are not explicitly provided or discernible. For all visual assets, including images, icons, and background graphics, for high-quality placeholder image URLs that accurately convey the original content's visual essence, maintaining correct aspect ratios, placement, and overall visual balance use https://robohash.org/<image description>. All hyperlinks should be present, styled correctly, and functional, pointing to valid-looking placeholder URLs or internal anchors, accurately mimicking the original site's navigation structure and intended user journey. The page layout must be accurately reproduced, adhering to all observed grid systems, flexbox arrangements, absolute/relative positioning attributes, and Z-index layering. Every interactive component, including buttons, input fields, dropdowns, sliders, and complex widgets, must be present, styled authentically, and functionally representative of their counterparts on the reference site. Incorporate best practices for modern web development, including appropriate semantic HTML5 elements for logical structure, essential accessibility attributes, and standard meta tags for document description, viewport control, and character encoding. The ultimate objective is to deliver an HTML page that, when rendered, provides a comprehensive, authentic, and seamless browsing experience that demonstrates the appearance and front-end functionality of {}, without relying on the original's proprietary code or assets, and without generating any content that could be considered a direct copy or infringement. Your response must be only the complete HTML code, without any additional text or explanation.", url, url);
    let request_payload: OllamaRequest = OllamaRequest {
        model: _model_name.to_string(),
        prompt: prompt.to_string(),
        stream: false,
    };

    match client.post(ollama_url)
        .json(&request_payload)
        .send()
        .await {
        Ok(response) => {
            if response.status().is_success() {
                // --- Parse and Display the Response ---
                match response.json::<OllamaResponse>().await {
                    Ok(ollama_response) => {
                        println!("\n🤖 Response:\n{}", ollama_response.response);
                        return format!("<div>{}</div>", ollama_response.response);
                    }
                    Err(e) => {
                        eprintln!("❌ Error: Failed to parse JSON response: {}", e);
                    }
                }
            } else {
                eprintln!("❌ Error: Request failed with status: {}", response.status());
                if let Ok(error_text) = response.text().await {
                    eprintln!("Error details: {}", error_text);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Error: Failed to send request to Ollama: {}", e);
            eprintln!("Please ensure your Ollama instance is running at {}", ollama_url);
        }
    }

    format!("Processed URL: <h2>{}</h2> (err)", url)
}
