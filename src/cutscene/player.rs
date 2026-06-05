use std::{
    fs,
    path::PathBuf,
    time::Instant,
};

pub struct CutscenePlayer {

    pub frame_paths:
        Vec<PathBuf>,

    pub current_frame:
        usize,

    pub fps:
        u32,

    pub last_frame_time:
        Instant,
}

impl CutscenePlayer {

    pub fn new(
        path: &str,
        fps: u32,
    ) -> Self {

        println!(
            "Loading cutscene: {}",
            path
        );

        let mut frame_paths =
            Vec::new();

        for entry in
            fs::read_dir(path)
                .unwrap()
        {

            let entry =
                entry.unwrap();

            let path =
                entry.path();

            if let Some(ext) =
                path.extension()
            {

                let ext =
                    ext.to_string_lossy()
                        .to_lowercase();

                if ext == "png" {

                    frame_paths.push(
                        path
                    );
                }
            }
        }

        frame_paths.sort();

        println!(
            "Found {} cutscene frames",
            frame_paths.len()
        );

        Self {

            frame_paths,

            current_frame:
                0,

            fps,

            last_frame_time:
                Instant::now(),
        }
    }

    pub fn update(
        &mut self,
    ) {

        let frame_time =
            1.0
                / self.fps as f32;

        if self
            .last_frame_time
            .elapsed()
            .as_secs_f32()
            >= frame_time
        {

            self.current_frame += 1;

            self.last_frame_time =
                Instant::now();
        }
    }

    pub fn finished(
        &self,
    ) -> bool {

        self.current_frame
            >=
            self.frame_paths.len()
    }

    pub fn current_path(
        &self,
    ) -> Option<&PathBuf> {

        self.frame_paths.get(
            self.current_frame
        )
    }
}