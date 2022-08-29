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
        let document_element = web_sys::window()
                                .map(|w| w.document())
                                .map(|d| d.document_element());
        if let Some(document_element) = document_element {
            window.set_resolution(
                document_element.client_width() as f32,
                document_element.client_height() as f32,
            );
        }
    }
}

