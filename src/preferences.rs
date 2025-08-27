use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{gio, glib};
use crate::config::{self, LlmModelProvider};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/page/newlevel/yaab/preferences.ui")]
    pub struct YetanotheraibrowserPreferences {
        #[template_child]
        pub provider_dropdown: TemplateChild<gtk::DropDown>,
        #[template_child]
        pub model_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub model_suggestions_box: TemplateChild<gtk::FlowBox>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for YetanotheraibrowserPreferences {
        const NAME: &'static str = "YetanotheraibrowserPreferences";
        type Type = super::YetanotheraibrowserPreferences;
        type ParentType = gtk::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for YetanotheraibrowserPreferences {}
    impl WidgetImpl for YetanotheraibrowserPreferences {}
    impl WindowImpl for YetanotheraibrowserPreferences {}
}

glib::wrapper! {
    pub struct YetanotheraibrowserPreferences(ObjectSubclass<imp::YetanotheraibrowserPreferences>)
        @extends gtk::Widget, gtk::Window,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl YetanotheraibrowserPreferences {
    pub fn new<P: IsA<gtk::Window>>(parent: &P) -> Self {
        let preferences: Self = glib::Object::builder().build();
        preferences.set_transient_for(Some(parent));
        preferences.set_modal(true);
        preferences.setup();
        preferences
    }

    fn setup(&self) {
        let imp = self.imp();

        // Initialize provider combo
        let provider = config::get_model_provider();
    // Map provider to index
    let idx = match provider { LlmModelProvider::Ollama => 0, LlmModelProvider::LmStudio => 1 };
    imp.provider_dropdown.set_selected(idx);

        // Initialize model entry
        imp.model_entry.set_text(&config::get_model_name());

    // Populate suggestions initially
    self.refresh_suggestions();

    // Provider change -> update setting and refresh
        let obj_weak = self.downgrade();
        imp.provider_dropdown.connect_selected_notify(move |dd| {
            if let Some(obj) = obj_weak.upgrade() {
                let selected = dd.selected();
                let provider = if selected == 1 { LlmModelProvider::LmStudio } else { LlmModelProvider::Ollama };
                config::set_model_provider(provider);
        obj.refresh_suggestions();
            }
        });

        // Model entry change (on focus-out or activate we persist)
        imp.model_entry.connect_activate(|entry| {
            let text = entry.text().to_string();
            if !text.trim().is_empty() { config::set_model_name(&text); }
        });
        imp.model_entry.connect_changed(|entry| {
            let text = entry.text();
            if text.len() > 3 { config::set_model_name(&text); }
        });

    }

    fn refresh_suggestions(&self) {
        use crate::genAi::llm::logic::ClientKind;
        let imp = self.imp();
        // Clear previous children
    // Remove existing children safely
    while let Some(child) = imp.model_suggestions_box.first_child() { imp.model_suggestions_box.remove(&child); }
        // Always show base suggestions for Ollama provider
        let provider = config::get_model_provider();
        let mut suggestions: Vec<String> = vec![];
        if let Ok(client) = ClientKind::new_default() {
            if let Ok(mut models) = client.list_models() { suggestions.append(&mut models); }
        }
        // Add curated suggestions
        if matches!(provider, LlmModelProvider::Ollama) {
            for base in ["gpt-oss:20b", "gpt-oss:120b"] { if !suggestions.iter().any(|s| s == base) { suggestions.push(base.to_string()); } }
        }
        suggestions.sort();
        suggestions.dedup();
        for s in suggestions {
            let button = gtk::Button::with_label(&s);
            let s_clone = s.clone();
            let entry = imp.model_entry.clone();
            button.connect_clicked(move |_| { entry.set_text(&s_clone); config::set_model_name(&s_clone); });
            imp.model_suggestions_box.append(&button);
        }
    }

    fn current_model(&self) -> String { self.imp().model_entry.text().to_string() }

    // Removed download functionality
}
