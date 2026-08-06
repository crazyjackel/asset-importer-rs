use crate::accessor::Accessors;
use crate::buffer::Buffers;
use crate::buffer::Views;
use crate::camera::Cameras;
use crate::error::Error;
use crate::error::Result;
use crate::image::Images;
use crate::light::Lights;
use crate::material::Materials;
use crate::material::Techniques;
use crate::mesh::Meshes;
use crate::node::Nodes;
use crate::scene::Scene;
use crate::scene::Scenes;
use crate::skin::Skins;
use crate::texture::Samplers;
use crate::texture::Textures;

#[derive(Clone, Debug)]
pub struct Document(gltf_v1_json::Root);

impl Document {
    pub fn from_json(mut json: json::Root) -> Result<Self> {
        json.add_default_material();
        let document = Self::from_json_without_validation(json);
        document.validate()?;
        Ok(document)
    }

    pub fn from_json_without_validation(json: json::Root) -> Self {
        Document(json)
    }

    pub fn into_json(self) -> json::Root {
        self.0
    }

    pub fn as_json(&self) -> &json::Root {
        &self.0
    }

    /// Perform validation checks on loaded glTF.
    pub(crate) fn validate(&self) -> Result<()> {
        use json::validation::Validate;
        let mut errors = Vec::new();
        self.0
            .validate(&self.0, json::Path::new, &mut |path, error| {
                errors.push((path(), error))
            });
        if errors.is_empty() {
            Ok(())
        } else {
            Err(Error::Validation(errors))
        }
    }

    pub fn default_scene(&self) -> Option<Scene<'_>> {
        self.0
            .scene
            .as_ref()
            .and_then(|index| self.scenes().find(|x| x.index() == index.value()))
    }

    pub fn accessors(&self) -> Accessors<'_> {
        Accessors {
            iter: self.0.accessors.iter(),
            document: self,
        }
    }
    pub fn buffers(&self) -> Buffers<'_> {
        Buffers {
            iter: self.0.buffers.iter(),
            document: self,
        }
    }
    pub fn views(&self) -> Views<'_> {
        Views {
            iter: self.0.buffer_views.iter(),
            document: self,
        }
    }
    pub fn images(&self) -> Images<'_> {
        Images {
            iter: self.0.images.iter(),
            document: self,
        }
    }
    pub fn textures(&self) -> Textures<'_> {
        Textures {
            iter: self.0.textures.iter(),
            document: self,
        }
    }
    pub fn samplers(&self) -> Samplers<'_> {
        Samplers {
            iter: self.0.samplers.iter(),
            document: self,
        }
    }
    pub fn materials(&self) -> Materials<'_> {
        Materials {
            iter: self.0.materials.iter(),
            document: self,
        }
    }
    pub fn techniques(&self) -> Techniques<'_> {
        Techniques {
            iter: self.0.techniques.iter(),
            document: self,
        }
    }
    pub fn meshes(&self) -> Meshes<'_> {
        Meshes {
            iter: self.0.meshes.iter(),
            document: self,
        }
    }
    pub fn cameras(&self) -> Cameras<'_> {
        Cameras {
            iter: self.0.cameras.iter(),
            document: self,
        }
    }
    pub fn nodes(&self) -> Nodes<'_> {
        Nodes {
            iter: self.0.nodes.iter(),
            document: self,
        }
    }
    pub fn skins(&self) -> Skins<'_> {
        Skins {
            iter: self.0.skins.iter(),
            document: self,
        }
    }
    pub fn scenes(&self) -> Scenes<'_> {
        Scenes {
            iter: self.0.scenes.iter(),
            document: self,
        }
    }
    pub fn lights(&self) -> Option<Lights<'_>> {
        //Model only has lights if KHR_materials common is enabled, as it is an extension
        #[cfg(feature = "KHR_materials_common")]
        {
            self.0
                .extensions
                .as_ref()
                .and_then(|x| x.ktr_materials_common.as_ref())
                .map(|x| Lights {
                    iter: x.lights.iter(),
                    document: self,
                })
        }
        #[cfg(not(feature = "KHR_materials_common"))]
        {
            None
        }
    }
}
