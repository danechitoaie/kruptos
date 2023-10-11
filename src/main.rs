use leptos::*;
use leptos_workers::worker;
use serde::{Deserialize, Serialize};
use web_sys::console;

#[derive(Clone, Serialize, Deserialize)]
struct EncryptionRequest {
    data: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct EncryptionResponse {
    data: String,
}

#[worker(EncryptionWorker)]
async fn encrypt(req: EncryptionRequest) -> EncryptionResponse {
    EncryptionResponse {
        data: req.data.to_uppercase(),
    }
}

#[component]
pub fn MyComponent() -> impl IntoView {
    let (value, set_value) = create_signal(String::from(""));

    let do_encrypt = move |_| {
        console::log_1(&"Here1...".into());
        spawn_local(async move {
            console::log_1(&"Here2...".into());
            let r = encrypt(EncryptionRequest { data: String::from("hello") }).await;
            console::log_1(&"Here3...".into());
            match r {
                Ok(r) => set_value(r.data),
                Err(_) => set_value(String::from("ERROR")),
            }
        });
    };

    view! {
        <div>
            <button on:click=do_encrypt>
                "Encrypt"
            </button>
            <span>"Value: " {value} "!"</span>
        </div>
    }
}

pub fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    mount_to_body(|| {
        view! {
            <MyComponent />
        }
    })
}
