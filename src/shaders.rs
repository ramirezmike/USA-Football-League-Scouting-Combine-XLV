use bevy::{
    asset::AssetServerSettings,
    prelude::*,
    reflect::TypeUuid,
    render::{
        camera::{Camera, RenderTarget},
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

pub struct ShadersPlugin;
impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<FresnelMaterial>::default())
            .add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
            .add_plugin(MaterialPlugin::<TextureMaterial>::default())
            .add_system(pass_time_to_shader);
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "717f64fe-6844-4822-8926-e0ed374294c8"]
pub struct FresnelMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub env_texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub color: Color,
    #[uniform(3)]
    pub time: f32,
}

impl Material for FresnelMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/fresnel.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "817f64fe-6844-4822-8926-e0ed374294c8"]
pub struct TextureMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub env_texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub color: Color,
    #[uniform(3)]
    pub time: f32,
    #[uniform(4)]
    pub x_scroll_speed: f32,
    #[uniform(5)]
    pub y_scroll_speed: f32,
    #[uniform(6)]
    pub scale: f32,
}

impl Material for TextureMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/texture.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
pub struct PostProcessingMaterial {
    /// In this example, this image will be the result of the main camera.
    #[texture(0)]
    #[sampler(1)]
    pub source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/post_processing.wgsl".into()
    }
}

fn pass_time_to_shader(
    time: Res<Time>,
    fresnels : Query<&Handle<FresnelMaterial>>,
    mut fresnel_materials: ResMut<Assets<FresnelMaterial>>,
    textures: Query<&Handle<TextureMaterial>>,
    mut texture_materials: ResMut<Assets<TextureMaterial>>,
) {
    for fresnel_handle in &fresnels {
        if let Some(material) = fresnel_materials.get_mut(fresnel_handle) {
            material.time = time.time_since_startup().as_secs_f32();
        }
    }

    for texture_handle in &textures {
        if let Some(material) = texture_materials.get_mut(texture_handle) {
            material.time = time.time_since_startup().as_secs_f32();
        }
    }
}
