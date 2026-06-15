use std::fs::File;
use std::io::BufReader;

use rodio::{
    Decoder,
    OutputStream,
    OutputStreamBuilder,
    Sink,
    Source,
};

pub struct AudioManager {

    stream:
        OutputStream,

    sink:
        Option<Sink>,
}

impl AudioManager {

    pub fn new() -> Self {

        let stream =
            OutputStreamBuilder::open_default_stream()
                .unwrap();

        Self {

            stream,

            sink: None,
        }
    }

    pub fn play_music(
        &mut self,
        path: &str,
    ) {

        self.stop_music();

        println!(
            "Playing music: {}",
            path
        );

        let file =
            File::open(path)
                .unwrap();

        let source =
            Decoder::try_from(
                BufReader::new(file)
            )
            .unwrap();

        let sink =
            Sink::connect_new(
                self.stream.mixer()
            );

        sink.append(
            source.repeat_infinite()
        );

        self.sink =
            Some(sink);
    }

    pub fn stop_music(
        &mut self,
    ) {

        if let Some(
            sink
        ) =
            &self.sink
        {

            sink.stop();
        }

        self.sink = None;
    }

    pub fn play_sound(
        &self,
        path: &str,
    ) {

        let file =
            File::open(path)
                .unwrap();

        let source =
            Decoder::try_from(
                BufReader::new(file)
            )
            .unwrap();

        let sink =
            Sink::connect_new(
                self.stream.mixer()
            );

        sink.append(
            source
        );

        sink.detach();
    }

}