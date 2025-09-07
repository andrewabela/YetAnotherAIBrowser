/* MIT License
 *
 * Copyright (c) 2025 Andrew Abela
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use glib::clone;
use std::thread;
use std::sync::mpsc;
use crate::genAi::llm::logic;
use webkit6::WebView;
use webkit6::prelude::*;

const SYS_PROMPT: &str = "As an advanced AI web browser, your core directive is to create a highly representative and functionally illustrative web page that emulates the visual design, interactive patterns, and content structure observed at the provided URL. Your output must be a singular, complete HTML document, optimized for direct rendering in a standard wide-screen desktop web browser, encompassing all necessary components for a production-ready and high-fidelity web experience. This creation is fundamentally an independent demonstration, focusing intently on capturing the essence and functionality of the original's user-facing presentation and a precise replication of its interactive flow, rather than a direct copy of its underlying source code or proprietary assets. Crucially, you must not generate any copyrighted content, proprietary assets, or sensitive information, nor should the output be used for deceptive purposes or to infringe upon intellectual property rights. Within this singular HTML output, embed all CSS styling rules directly within a <style> tag, ensuring a comprehensive and precise emulation of the original site's entire aesthetic. This includes meticulous attention to every detail of typography (fonts, sizes, weights, line heights, font-display properties, fallbacks), comprehensive color palettes (primary, secondary, accent, text, background, hover states), precise spacing (margins, padding, gaps, letter-spacing, word-spacing for optimal readability), and robust responsive design principles, guaranteeing optimal display and usability across all device types and screen orientations through careful application of media queries, flexible units like rem and em, and a mobile-first or desktop-first approach as implied by the original. Furthermore, meticulously reproduce visual nuances such as distinct box-shadows, subtle text-shadows, complex background gradients, smooth transitions for interactive elements, accurate hover states, clear focus states for accessibility, and the subtle use of pseudo-elements (e.g., ::before, ::after) for decorative or functional purposes. Similarly, integrate all JavaScript functionalities directly into a <script> tag, enabling interactive elements such as dynamic navigation menus (e.g., dropdowns, accordions, off-canvas menus, sticky headers), sophisticated content loaders (e.g. tabbed content), complex form validations (e.g., client-side input checking, real-time feedback, error messages), interactive image carousels with navigation controls and auto-play options, embedded video players (represented by placeholder thumbnails and controls), and any other observed user interface widgets to behave with exact functional similarity to the reference site, ensuring event listeners are correctly applied to simulate user interaction and dynamic content updates without actual server-side calls. Assume comprehensive access to all textual content, including articles, headlines, body paragraphs, detailed captions, and user comments; generate contextually appropriate, coherent, and valid-looking placeholder information where specific textual details are not explicitly provided or discernible, ensuring a consistent tone, style, length, and subject matter that authentically aligns with the original's perceived content. Specifically, if the emulated site is a blog or article page, aim to generate approximately 1000 words of relevant content. If it's a forum or discussion board, aim to generate around 20 distinct posts or comments. For search results pages or e-commerce/shopping websites, aim to generate at least 20 items or product listings. For all visual assets, including images, icons, and background graphics, you must use http://localhost:2345/generate_image?description=<detailed image description (around 45 tokens/(words))> for high-quality image URLs that accurately convey the original content's visual essence, maintaining correct aspect ratios, placement, and overall visual balance. All hyperlinks should be present, styled correctly, and functional; you must ensure all href attributes point to full, complete, and valid external-looking URLs, including the root domain, path, and any parameters (e.g., https://www.example.com/page/subpage?query=term#section-id). Absolutely NO standalone # or internal anchors (e.g., href='#section') are permitted for any link. Do not use localhost or 127.0.0.1, and do not generate any relative links, even in the nav bar. Furthermore, if the original page contains a search bar, its functionality must be emulated such that submitting a query opens a new page with a full valid URL (domain/path?parameters#whatever) corresponding to the search results, accurately mimicking the original site's navigation structure and intended user journey. The page layout must be accurately reproduced, adhering to all observed grid systems (e.g., CSS Grid), flexbox arrangements, absolute/relative/fixed positioning attributes, and Z-index layering, ensuring proper document flow, visual hierarchy, and element stacking order. Every interactive component, including buttons, input fields, dropdowns, sliders, and complex widgets, must be present, styled authentically, and functionally representative of their counterparts on the reference site, including their accessible states (e.g., disabled, active, hover, focus). Incorporate best practices for modern web development, including appropriate semantic HTML5 elements for logical structure (e.g., <header>, <nav>, <main>, <article>, <section>, <aside>, <footer>, <figure>, <figcaption>), essential accessibility attributes (e.g., ARIA roles, labels, tabindex for keyboard navigation, clear visual focus indicators, simulated alt text for all generated images, and adherence to perceived color contrast ratios), and standard meta tags for document description, viewport control, character encoding, and basic SEO/social sharing (e.g., <title>, <meta name='description'>, Open Graph tags for social media previews). The ultimate objective is to deliver an HTML page that, when rendered, provides a comprehensive, authentic, and seamless browsing experience that demonstrates the appearance and front-end functionality of the user-provided URL, without relying on the original's proprietary code or assets, and without generating any content that could be considered a direct copy or infringement. Your response must be only the complete HTML code, without any additional text, explanation or indicators you are typing html (like html), the output is assumed as HTML there is no need to define it as html using html. TL;DR; Reproduce a complete HTML document that emulates the provided URL. This document must replicate (but not copy) the original's visual design, functionality, and content, using embedded CSS and JavaScript. All content must be in full and contains relevant information that a user would expect from the provided URL.";

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/page/newlevel/yaab/window.ui")]
    pub struct YetanotheraibrowserWindow {
    // Template widgets
    #[template_child]
    pub url_field: TemplateChild<gtk::Entry>,
    #[template_child]
    pub go_btn: TemplateChild<gtk::Button>,
    #[template_child]
    pub response_view: TemplateChild<WebView>,
    #[template_child]
    pub content_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub loading_spinner: TemplateChild<gtk::Spinner>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for YetanotheraibrowserWindow {
        const NAME: &'static str = "YetanotheraibrowserWindow";
        type Type = super::YetanotheraibrowserWindow;
        type ParentType = gtk::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for YetanotheraibrowserWindow {}
    impl WidgetImpl for YetanotheraibrowserWindow {}
    impl WindowImpl for YetanotheraibrowserWindow {}
    impl ApplicationWindowImpl for YetanotheraibrowserWindow {}
}

glib::wrapper! {
    pub struct YetanotheraibrowserWindow(ObjectSubclass<imp::YetanotheraibrowserWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl YetanotheraibrowserWindow {
    pub fn new<P: IsA<gtk::Application>>(application: &P) -> Self {
        let window: Self = glib::Object::builder()
            .property("application", application)
            .build();

        let action = gio::SimpleAction::new("open-temp-preferences", None);
        action.connect_activate(clone!(@weak window => move |_, _| {
            let settings_window = gtk::Window::builder()
                .title("Preferences")
                .default_width(550)
                .default_height(450)
                .transient_for(&window)
                .modal(true)
                .build();
            settings_window.present();
        }));
        window.add_action(&action);

        window.setup_handlers();
        window.configure_network_timeout();
        window
    }

    fn setup_handlers(&self) {
        let imp = self.imp();
        // Activate on Enter in entry
        imp.url_field.connect_activate(clone!(@weak self as win => move |e| {
            win.start_stream(e.text().to_string());
        }));
        // Click on Go button
        imp.go_btn.connect_clicked(clone!(@weak self as win => move |_| {
            let text = win.imp().url_field.text().to_string();
            win.start_stream(text);
        }));
        
        // Intercept URL clicks from the WebView
        imp.response_view.connect_decide_policy(clone!(@weak self as win => @default-return false, move |webview, decision, decision_type| {
            use webkit6::PolicyDecisionType;
            use webkit6::NavigationPolicyDecision;
            
            if decision_type == PolicyDecisionType::NavigationAction {
                if let Some(nav_decision) = decision.downcast_ref::<NavigationPolicyDecision>() {
                    if let Some(mut nav_action) = nav_decision.navigation_action() {
                        if let Some(request) = nav_action.request() {
                            if let Some(uri) = request.uri() {
                                let uri_str = uri.as_str();
                                
                                // Only intercept if it's a different URL (not the current page)
                                if let Some(current_uri) = webview.uri() {
                                    if uri_str != current_uri.as_str() {
                                        win.imp().url_field.set_text(uri_str);
                                        win.start_stream(uri_str.to_string());
                                        decision.ignore();
                                        return true;
                                    }
                                } else {
                                    // If no current URI, treat it as a new navigation
                                    win.imp().url_field.set_text(uri_str);
                                    win.start_stream(uri_str.to_string());
                                    decision.ignore();
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
            false
        }));

    }

    fn configure_network_timeout(&self) {
        let imp = self.imp();

        // Get the WebView from the TemplateChild and access its settings
        if let Some(settings) = webkit6::prelude::WebViewExt::settings(&*imp.response_view) {
            // WebKit has various timeout settings we can configure
            // These are internal timeouts that affect network operations

            // Enable developer extras for better debugging
            settings.set_enable_developer_extras(true);

            // Configure other WebKit settings that might affect network behavior
            settings.set_enable_javascript(true);
            settings.set_enable_media(false);

            println!("WebView settings configured for extended network operations");
        }

        // Note: For more advanced network timeout configuration,
        // we may need to use WebKit's NetworkSession API or FFI
        // to access the underlying SoupSession directly

        println!("Network timeout configuration initialized");
    }

    fn start_stream(&self, prompt: String) {
        if prompt.trim().is_empty() { return; }
        
        // Show loading spinner
        self.imp().content_stack.set_visible_child_name("loading");
        self.imp().loading_spinner.set_spinning(true);
        
        let (tx, rx) = mpsc::channel::<Result<String, String>>();
        let prompt_clone = prompt.clone();
        
        thread::spawn(move || {
            let client = logic::ClientKind::new_default();
            let result = if let Ok(client) = client {
                let model = crate::config::get_model_name();
                match client.prompt(&model, &prompt_clone, SYS_PROMPT) {
                    Ok(response) => Ok(response),
                    Err(e) => Err(format!("Failed to fetch response from the model: {}", e)),
                }
            } else {
                Err("Failed to initialize the LLM client".to_string())
            };
            let _ = tx.send(result);
        });
        
        let window_weak = self.downgrade();
        glib::idle_add_local(move || {
            if let Some(window) = window_weak.upgrade() {
                match rx.try_recv() {
                    Ok(result) => {
                        let imp = window.imp();
                        imp.loading_spinner.set_spinning(false);
                        
                        match result {
                            Ok(response) => {
                                imp.response_view.load_html(&response, None);
                            }
                            Err(error_msg) => {
                                let error_html = format!("<html><body><h3>Error</h3><p>{}</p></body></html>", error_msg);
                                imp.response_view.load_html(&error_html, None);
                            }
                        }
                        imp.content_stack.set_visible_child_name("webview");
                        Continue(false)
                    }
                    Err(mpsc::TryRecvError::Empty) => Continue(true),
                    Err(mpsc::TryRecvError::Disconnected) => {
                        let imp = window.imp();
                        imp.loading_spinner.set_spinning(false);
                        let error_html = "<html><body><h3>Error</h3><p>Connection to LLM was lost</p></body></html>";
                        imp.response_view.load_html(error_html, None);
                        imp.content_stack.set_visible_child_name("webview");
                        Continue(false)
                    }
                }
            } else {
                Continue(false)
            }
        });
    }
}
