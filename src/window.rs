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
use webkit6::prelude::WebViewExt;

const SYS_PROMPT: &str = "You are a helpful assistant that responds to user queries with HTML code.";

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
