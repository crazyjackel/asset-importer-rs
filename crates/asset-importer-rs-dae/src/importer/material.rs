use std::collections::HashMap;

use asset_importer_rs_scene::{
    AiColor4D, AiMaterial, AiShadingMode,
    matkey::{
        AI_MATKEY_COLOR_AMBIENT, AI_MATKEY_COLOR_DIFFUSE, AI_MATKEY_COLOR_EMISSIVE,
        AI_MATKEY_COLOR_REFLECTIVE, AI_MATKEY_COLOR_SPECULAR, AI_MATKEY_COLOR_TRANSPARENT,
        AI_MATKEY_ENABLE_WIREFRAME, AI_MATKEY_NAME, AI_MATKEY_OPACITY, AI_MATKEY_REFLECTIVITY,
        AI_MATKEY_REFRACTI, AI_MATKEY_SHADING_MODEL, AI_MATKEY_SHININESS, AI_MATKEY_TWOSIDED,
    },
};
use dae_parser::{
    ColorParam, Document, Effect as DocumentEffect, Extra, FloatParam, Material, ProfileCommon,
    Shader,
};

use crate::DaeImportError;

use super::DaeImporter;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum ShadeType {
    Constant,
    Lambert,
    Blinn,
    #[default]
    Phong,
}

/// A collada effect. Can contain about anything according to the Collada spec,
/// but we limit our version to a reasonable subset.
#[derive(Clone, Debug)]
struct Effect {
    shade_type: ShadeType,
    emissive: AiColor4D,
    ambient: AiColor4D,
    diffuse: AiColor4D,
    specular: AiColor4D,
    transparent: AiColor4D,
    reflective: AiColor4D,
    shininess: f32,
    refract_index: f32,
    reflectivity: f32,
    transparency: f32,
    has_transparency: bool,
    rgb_transparency: bool,
    invert_transparency: bool,
    double_sided: bool,
    wireframe: bool,
    faceted: bool,
}

impl Default for Effect {
    fn default() -> Self {
        Self {
            shade_type: ShadeType::Phong,
            emissive: AiColor4D::new(0.0, 0.0, 0.0, 1.0),
            ambient: AiColor4D::new(0.1, 0.1, 0.1, 1.0),
            diffuse: AiColor4D::new(0.6, 0.6, 0.6, 1.0),
            specular: AiColor4D::new(0.4, 0.4, 0.4, 1.0),
            transparent: AiColor4D::new(0.0, 0.0, 0.0, 1.0),
            reflective: AiColor4D::new(0.0, 0.0, 0.0, 1.0),
            shininess: 10.0,
            refract_index: 1.0,
            reflectivity: 0.0,
            transparency: 1.0,
            has_transparency: false,
            rgb_transparency: false,
            invert_transparency: false,
            double_sided: false,
            wireframe: false,
            faceted: false,
        }
    }
}

impl From<&ProfileCommon> for Effect {
    fn from(profile: &ProfileCommon) -> Self {
        let mut effect = Effect::default();
        effect.apply_extras(profile.extra.iter().chain(profile.technique.extra.iter()));
        if let Some(shader) = profile.technique.data.shaders.first() {
            effect.apply_shader(shader);
        }
        effect
    }
}

impl Effect {
    fn apply_extras<'a>(&mut self, extras: impl IntoIterator<Item = &'a Extra>) {
        for extra in extras {
            for technique in &extra.technique {
                for child in technique.element.children() {
                    let flag = {
                        let text = child.text();
                        let text = text.trim();
                        text == "1" || text.eq_ignore_ascii_case("true")
                    };
                    match child.name().to_ascii_lowercase().as_str() {
                        "faceted" => self.faceted = flag,
                        "double_sided" | "double-sided" | "twosided" => self.double_sided = flag,
                        "wireframe" => self.wireframe = flag,
                        "invert_transparency" => self.invert_transparency = flag,
                        _ => {}
                    }
                }
            }
        }
    }

    fn apply_color(color_out: &mut AiColor4D, param: Option<&ColorParam>) {
        let Some(param) = param else {
            return;
        };
        if let Some(color) = param.as_color() {
            *color_out = AiColor4D::from(*color);
        }
    }

    fn apply_float(out: &mut f32, param: Option<&FloatParam>) {
        if let Some(FloatParam::Float(value)) = param {
            *out = *value;
        }
    }

    fn apply_transparent(&mut self, param: Option<&ColorParam>) {
        if param.is_some() {
            self.has_transparency = true;
        }
        Self::apply_color(&mut self.transparent, param);
    }

    fn apply_transparency(&mut self, param: Option<&FloatParam>) {
        if param.is_some() {
            self.has_transparency = true;
        }
        Self::apply_float(&mut self.transparency, param);
    }

    fn apply_shader(&mut self, shader: &Shader) {
        match shader {
            Shader::Constant(shader) => {
                self.shade_type = ShadeType::Constant;
                Self::apply_color(&mut self.emissive, shader.emission.as_deref());
                Self::apply_color(&mut self.reflective, shader.reflective.as_deref());
                Self::apply_float(&mut self.reflectivity, shader.reflectivity.as_deref());
                self.apply_transparent(shader.transparent.as_deref());
                self.apply_transparency(shader.transparency.as_deref());
                Self::apply_float(
                    &mut self.refract_index,
                    shader.index_of_refraction.as_deref(),
                );
            }
            Shader::Lambert(shader) => {
                self.shade_type = ShadeType::Lambert;
                Self::apply_color(&mut self.emissive, shader.emission.as_deref());
                Self::apply_color(&mut self.ambient, shader.ambient.as_deref());
                Self::apply_color(&mut self.diffuse, shader.diffuse.as_deref());
                Self::apply_color(&mut self.reflective, shader.reflective.as_deref());
                Self::apply_float(&mut self.reflectivity, shader.reflectivity.as_deref());
                self.apply_transparent(shader.transparent.as_deref());
                self.apply_transparency(shader.transparency.as_deref());
                Self::apply_float(
                    &mut self.refract_index,
                    shader.index_of_refraction.as_deref(),
                );
            }
            Shader::Blinn(shader) => {
                self.shade_type = ShadeType::Blinn;
                Self::apply_color(&mut self.emissive, shader.emission.as_deref());
                Self::apply_color(&mut self.ambient, shader.ambient.as_deref());
                Self::apply_color(&mut self.diffuse, shader.diffuse.as_deref());
                Self::apply_color(&mut self.specular, shader.specular.as_deref());
                Self::apply_float(&mut self.shininess, shader.shininess.as_deref());
                Self::apply_color(&mut self.reflective, shader.reflective.as_deref());
                Self::apply_float(&mut self.reflectivity, shader.reflectivity.as_deref());
                self.apply_transparent(shader.transparent.as_deref());
                self.apply_transparency(shader.transparency.as_deref());
                Self::apply_float(
                    &mut self.refract_index,
                    shader.index_of_refraction.as_deref(),
                );
            }
            Shader::Phong(shader) => {
                self.shade_type = ShadeType::Phong;
                Self::apply_color(&mut self.emissive, shader.emission.as_deref());
                Self::apply_color(&mut self.ambient, shader.ambient.as_deref());
                Self::apply_color(&mut self.diffuse, shader.diffuse.as_deref());
                Self::apply_color(&mut self.specular, shader.specular.as_deref());
                Self::apply_float(&mut self.shininess, shader.shininess.as_deref());
                Self::apply_color(&mut self.reflective, shader.reflective.as_deref());
                Self::apply_float(&mut self.reflectivity, shader.reflectivity.as_deref());
                self.apply_transparent(shader.transparent.as_deref());
                self.apply_transparency(shader.transparency.as_deref());
                Self::apply_float(
                    &mut self.refract_index,
                    shader.index_of_refraction.as_deref(),
                );
            }
        }
    }
}

impl DaeImporter {
    pub(crate) fn import_materials(
        &self,
        document: &Document,
    ) -> Result<(Vec<AiMaterial>, HashMap<String, usize>), DaeImportError> {
        let mut materials = Vec::new();
        let mut material_index_map: HashMap<String, usize> = HashMap::new();
        let document_local_map = document
            .local_map::<DocumentEffect>()
            .map_err(DaeImportError::FileFormatError)?;
        let library_materials = document.library_iter::<Material>();
        for library in library_materials {
            materials.reserve(library.items.len());
            for (index, material) in library.items.iter().enumerate() {
                let mut ai_material = AiMaterial::new();

                // Handle Name
                let name: String = material
                    .name
                    .as_ref()
                    .or(material.id.as_ref())
                    .cloned()
                    .unwrap_or(index.to_string());
                ai_material.add_binary_property(AI_MATKEY_NAME, name.bytes().collect());

                // Handle Instance Effect
                let instance_effect = document_local_map
                    .get(&material.instance_effect.url)
                    .ok_or(DaeImportError::MissingLocalMapEntry(
                        material.instance_effect.url.to_string(),
                    ))?;

                if let Some(profile) = instance_effect.get_common_profile() {
                    let mut effect = Effect::from(profile);
                    effect.apply_extras(&instance_effect.extra);

                    let shade_mode = if effect.faceted {
                        AiShadingMode::Flat
                    } else {
                        match effect.shade_type {
                            ShadeType::Constant => AiShadingMode::Unlit,
                            ShadeType::Lambert => AiShadingMode::Gouraud,
                            ShadeType::Blinn => AiShadingMode::Blinn,
                            ShadeType::Phong => AiShadingMode::Phong,
                        }
                    };
                    ai_material
                        .add_binary_property(AI_MATKEY_SHADING_MODEL, vec![shade_mode as u8]);

                    // Material Flags
                    ai_material
                        .add_binary_property(AI_MATKEY_TWOSIDED, vec![effect.double_sided as u8]);
                    ai_material.add_binary_property(
                        AI_MATKEY_ENABLE_WIREFRAME,
                        vec![effect.wireframe as u8],
                    );

                    // Material Colors
                    ai_material.add_binary_property(
                        AI_MATKEY_COLOR_AMBIENT,
                        bytemuck::bytes_of(&effect.ambient).to_vec(),
                    );
                    ai_material.add_binary_property(
                        AI_MATKEY_COLOR_DIFFUSE,
                        bytemuck::bytes_of(&effect.diffuse).to_vec(),
                    );
                    ai_material.add_binary_property(
                        AI_MATKEY_COLOR_SPECULAR,
                        bytemuck::bytes_of(&effect.specular).to_vec(),
                    );
                    ai_material.add_binary_property(
                        AI_MATKEY_COLOR_EMISSIVE,
                        bytemuck::bytes_of(&effect.emissive).to_vec(),
                    );
                    ai_material.add_binary_property(
                        AI_MATKEY_COLOR_REFLECTIVE,
                        bytemuck::bytes_of(&effect.reflective).to_vec(),
                    );

                    // Scalar Properties
                    ai_material.add_binary_property(
                        AI_MATKEY_SHININESS,
                        effect.shininess.to_le_bytes().to_vec(),
                    );
                    ai_material.add_binary_property(
                        AI_MATKEY_REFLECTIVITY,
                        effect.reflectivity.to_le_bytes().to_vec(),
                    );
                    ai_material.add_binary_property(
                        AI_MATKEY_REFRACTI,
                        effect.refract_index.to_le_bytes().to_vec(),
                    );

                    let mut transparency = effect.transparency;
                    let mut transparent = effect.transparent;
                    if (0.0..=1.0).contains(&transparency) {
                        // Handle RGB Transparency
                        if effect.rgb_transparency {
                            transparency *= 0.212671 * transparent.r
                                + 0.715160 * transparent.g
                                + 0.072169 * transparent.b;
                            transparent.a = 1.0;

                            // Add Transparency Color
                            ai_material.add_binary_property(
                                AI_MATKEY_COLOR_TRANSPARENT,
                                bytemuck::bytes_of(&transparent).to_vec(),
                            );
                        } else {
                            transparency *= transparent.a;

                            // Add Transparency Color
                            ai_material.add_binary_property(
                                AI_MATKEY_COLOR_TRANSPARENT,
                                bytemuck::bytes_of(&transparent).to_vec(),
                            );
                        }

                        // Handle Inverted Transparency
                        if effect.invert_transparency {
                            transparency = 1.0 - transparency;
                        }
                        if effect.has_transparency || transparency < 1.0 {
                            ai_material.add_binary_property(
                                AI_MATKEY_OPACITY,
                                transparency.to_le_bytes().to_vec(),
                            );
                        }
                    }
                }

                // Add Material to Index Map
                let index = materials.len();
                material_index_map.insert(name, index);
                materials.push(ai_material);
            }
        }
        Ok((materials, material_index_map))
    }
}
