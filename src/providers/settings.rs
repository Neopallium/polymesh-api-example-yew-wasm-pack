use leptos::*;

use serde::{Deserialize, Serialize};

use gloo_storage::{LocalStorage, Storage};

use crate::providers::backend::*;

const APP_KEY: &str = "example.app.polymesh.network";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub url: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            url: "ws://localhost:9944".into(),
        }
    }
}

impl AppSettings {
    pub fn update_settings(&mut self, settings: Self) {
        *self = settings;
        log::info!("app settings = {:#?}", self);
    }
}

#[component]
pub fn SettingsProvider(children: Children) -> impl IntoView {
    let settings: AppSettings = LocalStorage::get(APP_KEY).unwrap_or_else(|_| {
        let settings = AppSettings::default();
        // Save settings.
        if let Err(err) = LocalStorage::set(APP_KEY, &settings) {
            log::error!("Failed to save settings: {err:?}");
        }
        settings
    });

    let (backend, set_backend) = use_backend();
    let (_, set_state) = use_backend_state();

    let url = settings.url.clone();

    // Set backend URL on mount
    create_effect(move |_| {
        let mut b = backend.get();
        b.connect_to(url.clone(), set_state);
        set_backend.set(b);
    });

    let (settings_signal, _) = create_signal(settings);
    provide_context(settings_signal);

    children()
}

pub fn use_settings() -> ReadSignal<AppSettings> {
    use_context::<ReadSignal<AppSettings>>().expect("Settings context")
}
