mod app;
mod chapters;
mod components;
mod state;
mod styles;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Cetak Biru Berdarah — Memulai...");
    yew::Renderer::<app::App>::new().render();
}
