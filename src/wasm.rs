use bevy::prelude::*;

pub struct WasmPlugin;
impl Plugin for WasmPlugin {
    fn build(&self, _app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        _app.add_system(fullscreen);
    }
}

#[cfg(target_arch = "wasm32")]
fn fullscreen(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if let Some(web_window) = web_sys::window() {
            if let Some(document) = web_window.document() {
                if let Some(document_element) = document.document_element() {
                    window.set_resolution(
                        document_element.client_width() as f32,
                        document_element.client_height() as f32,
                    );
                }
            }
        }
    }
}

