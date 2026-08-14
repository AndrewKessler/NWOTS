use glam::Vec3;

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

    pub fn sample_direction(
        &self,
        direction: Vec3,
    ) -> [u8; 4] {

        let x =
            direction.x;

        let y =
            direction.y;

        let z =
            direction.z;

        let abs_x =
            x.abs();

        let abs_y =
            y.abs();

        let abs_z =
            z.abs();

        let (
            texture,
            u,
            v,
        ) =
            if abs_y >= abs_x
                &&
                abs_y >= abs_z
            {

                if y >= 0.0 {

                    let m =
                        abs_y;

                    let u =
                        0.5
                        *
                        (
                            x / m
                            + 1.0
                        );

                    let v =
                        0.5
                        *
                        (
                            1.0
                            - z / m
                        );

                    (
                        &self.north,
                        u,
                        v,
                    )

                } else {

                    let m =
                        abs_y;

                    let u =
                        0.5
                        *
                        (
                            -x / m
                            + 1.0
                        );

                    let v =
                        0.5
                        *
                        (
                            1.0
                            - z / m
                        );

                    (
                        &self.south,
                        u,
                        v,
                    )
                }

            } else if abs_x >= abs_z {

                if x >= 0.0 {

                    let m =
                        abs_x;

                    let u =
                        0.5
                        *
                        (
                            -y / m
                            + 1.0
                        );

                    let v =
                        0.5
                        *
                        (
                            1.0
                            - z / m
                        );

                    (
                        &self.east,
                        u,
                        v,
                    )

                } else {

                    let m =
                        abs_x;

                    let u =
                        0.5
                        *
                        (
                            y / m
                            + 1.0
                        );

                    let v =
                        0.5
                        *
                        (
                            1.0
                            - z / m
                        );

                    (
                        &self.west,
                        u,
                        v,
                    )
                }

            } else {

                if z >= 0.0 {

                    let m =
                        abs_z;

                    let u =
                        0.5
                        *
                        (
                            x / m
                            + 1.0
                        );

                    let v =
                        0.5
                        *
                        (
                            y / m
                            + 1.0
                        );

                    (
                        &self.top,
                        u,
                        v,
                    )

                } else {

                    let m =
                        abs_z;

                    let u =
                        0.5
                        *
                        (
                            x / m
                            + 1.0
                        );

                    let v =
                        0.5
                        *
                        (
                            -y / m
                            + 1.0
                        );

                    (
                        &self.bottom,
                        u,
                        v,
                    )
                }
            };

        let u =
            u.clamp(
                0.0,
                1.0,
            );

        let v =
            v.clamp(
                0.0,
                1.0,
            );

        let texture_x =
            (
                u
                *
                (texture.width - 1)
                    as f32
            ) as usize;

        let texture_y =
            (
                v
                *
                (texture.height - 1)
                    as f32
            ) as usize;

        texture.sample(
            texture_x,
            texture_y,
        )
    }
}