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
use glib::ControlFlow;
use crate::genAi::llm::logic;

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
    pub response_view: TemplateChild<gtk::TextView>,
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

        // Temporary inline preferences window action using new-style clone! syntax.
        // If an application-level preferences window exists, this can be removed.
        let action = gio::SimpleAction::new("open-temp-preferences", None);
        action.connect_activate(clone!(
            #[weak]
            window,
            move |_, _| {
                let settings_window = gtk::Window::builder()
                    .title("Preferences")
                    .default_width(550)
                    .default_height(450)
                    .transient_for(&window)
                    .modal(true)
                    .build();
                settings_window.present();
            }
        ));
        window.add_action(&action);

        window.setup_handlers();
        window
    }

    fn setup_handlers(&self) {
        let imp = self.imp();
        // Activate on Enter in entry
        imp.url_field.connect_activate(clone!(
            #[weak(rename_to=win)]
            self,
            move |e| {
                win.start_stream(e.text().to_string());
            }
        ));
        // Click on Go button
        imp.go_btn.connect_clicked(clone!(
            #[weak(rename_to=win)]
            self,
            move |_| {
                let text = win.imp().url_field.text().to_string();
                win.start_stream(text);
            }
        ));
    }

    fn start_stream(&self, prompt: String) {
        if prompt.trim().is_empty() { return; }
        let buffer = self.imp().response_view.buffer();
        buffer.set_text("");
        let (tx, rx) = mpsc::channel::<String>();
        let prompt_clone = prompt.clone();
        thread::spawn(move || {
            let _ = logic::stream_prompt(&prompt_clone, |c| { let _ = tx.send(c.to_string()); });
        });
        let win_weak = self.downgrade();
        glib::idle_add_local(move || {
            if let Some(win) = win_weak.upgrade() {
                match rx.try_recv() {
                    Ok(chunk) => {
                        let buf = win.imp().response_view.buffer();
                        let mut end_iter = buf.end_iter();
                        buf.insert(&mut end_iter, &chunk);
                        return ControlFlow::Continue;
                    }
                    Err(mpsc::TryRecvError::Empty) => return ControlFlow::Continue,
                    Err(mpsc::TryRecvError::Disconnected) => return ControlFlow::Break,
                }
            }
            ControlFlow::Break
        });
    }
}
