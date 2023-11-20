use auth_w_yew::App;
use gloo::console::log;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}