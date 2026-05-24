use serde::Deserialize;
use yew::prelude::*;

// The data contract matching Rocket's structure
#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct MessageResponse {
    pub message: String,
}

#[function_component(App)]
pub fn app() -> Html {
    // State to store the message string we get from Rocket
    let api_data = use_state(|| "Not connected to backend yet.".to_string());

    let on_click = {
        let api_data = api_data.clone();
        Callback::from(move |_| {
            let api_data = api_data.clone();

            // Spawn an isolated async task browser thread
            wasm_bindgen_futures::spawn_local(async move {
                match gloo::net::http::Request::get("/api/hello").send().await {
                    Ok(response) => {
                        if let Ok(data) = response.json::<MessageResponse>().await {
                            api_data.set(data.message);
                        } else {
                            api_data.set("Connected, but failed to parse JSON data.".to_string());
                        }
                    }
                    Err(e) => {
                        let full_err = format!("JSON Parsing Error: {:?}", e);
                        gloo::console::error!(full_err.clone());
                        api_data.set("Network error: Is Rocket running on port 8000?".to_string());
                    }
                }
            });
        })
    };

    html! {
        <main style="padding: 40px; font-family: sans-serif; text-align: center;">
            <h1>{"Yew Front -> Rocket Back"}</h1>

            <button onclick={on_click} style="padding: 10px 20px; font-size: 16px; cursor: pointer;">
                {"Fetch Data From Port 8000"}
            </button>


            <p style="margin-top: 25px; font-size: 18px; color: #333; font-weight: bold;">
                {"Response: "}{(*api_data).clone()}
            </p>
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
