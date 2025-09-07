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

use gettextrs::gettext;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};

use crate::config::VERSION;
use crate::{YetanotheraibrowserWindow, YetanotheraibrowserPreferences};

mod imp {
    use super::*;

    #[derive(Debug, Default)]
    pub struct YetanotheraibrowserApplication {}

    #[glib::object_subclass]
    impl ObjectSubclass for YetanotheraibrowserApplication {
        const NAME: &'static str = "YetanotheraibrowserApplication";
        type Type = super::YetanotheraibrowserApplication;
        type ParentType = gtk::Application;
    }

    impl ObjectImpl for YetanotheraibrowserApplication {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_gactions();
            obj.set_accels_for_action("app.quit", &["<primary>q"]);
        }
    }

    impl ApplicationImpl for YetanotheraibrowserApplication {
        // We connect to the activate callback to create a window when the application
        // has been launched. Additionally, this callback notifies us when the user
        // tries to launch a "second instance" of the application. When they try
        // to do that, we'll just present any existing window.
        fn activate(&self) {
            let application = self.obj();
            // Get the current window or create one if necessary
            let window = application.active_window().unwrap_or_else(|| {
                let window = YetanotheraibrowserWindow::new(&*application);
                window.upcast()
            });

            // Ask the window manager/compositor to present the window
            window.present();
        }
    }

    impl GtkApplicationImpl for YetanotheraibrowserApplication {}
    }

glib::wrapper! {
    pub struct YetanotheraibrowserApplication(ObjectSubclass<imp::YetanotheraibrowserApplication>)
        @extends gio::Application, gtk::Application, 
        @implements gio::ActionGroup, gio::ActionMap;
}

impl YetanotheraibrowserApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .property("resource-base-path", "/page/newlevel/yaab")
            .build()
    }

    fn setup_gactions(&self) {
        let quit_action = gio::ActionEntry::builder("quit")
            .activate(move |app: &Self, _, _| app.quit())
            .build();
        let about_action = gio::ActionEntry::builder("about")
            .activate(move |app: &Self, _, _| app.show_about())
            .build();
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |app: &Self, _, _| app.show_preferences())
            .build();
        self.add_action_entries([quit_action, about_action, preferences_action]);
    }

    fn show_about(&self) {
        let window = self.active_window().unwrap();
        let about = gtk::AboutDialog::builder()
            .transient_for(&window)
            .modal(true)
            .program_name("yetanotheraibrowser")
            .logo_icon_name("page.newlevel.yaab")
            .version(VERSION)
            .authors(vec!["Andrew Abela"])
            // Translators: Replace translator-credits with your name/username, and optionally an email or URL.
            .translator_credits(&gettext("translator-credits"))
            .copyright("© 2025 Andrew Abela")
            .license_type(gtk::License::MitX11)
            .build();

        about.present();
    }

    fn show_preferences(&self) {
        let window = self.active_window().unwrap();
        let preferences = YetanotheraibrowserPreferences::new(&window);
        preferences.present();
    }
}
