use crate::assets::Texture;

pub struct Skybox {

    pub north: Texture,

    pub east: Texture,

    pub south: Texture,

    pub west: Texture,

    pub top: Texture,

    pub bottom: Texture,
}

impl Skybox {

    pub fn load(
        folder: &str,
    ) -> Self {

        Self {

            north:
                Texture::load(
                    &format!(
                        "{}/DUSK_N.png",
                        folder
                    )
                ),

            east:
                Texture::load(
                    &format!(
                        "{}/DUSK_E.png",
                        folder
                    )
                ),

            south:
                Texture::load(
                    &format!(
                        "{}/DUSK_S.png",
                        folder
                    )
                ),

            west:
                Texture::load(
                    &format!(
                        "{}/DUSK_W.png",
                        folder
                    )
                ),

            top:
                Texture::load(
                    &format!(
                        "{}/DUSK_T.png",
                        folder
                    )
                ),

            bottom:
                Texture::load(
                    &format!(
                        "{}/DUSK_B.png",
                        folder
                    )
                ),
        }
    }
}