use rusoto_core::Region;
use rusoto_polly::{Polly, PollyClient, SynthesizeSpeechInput};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<(), std::io::Error> {

    let client = PollyClient::new(Region::EuCentral1);

    let speech_future = client.synthesize_speech(SynthesizeSpeechInput {
        language_code: None,
        lexicon_names: None,
        output_format: "mp3".into(),
        sample_rate: None,
        speech_mark_types: None,
        text: r#"""Holy shit! It's me Miroslav spawned from Rust code.""#.into(),
        text_type: None,
        voice_id: "Miroslav".into(),
    });

    match speech_future.sync() {
        Ok(output) => {
            println!("Output content type is: {:?}", output.content_type);
            if let Some(stream) = output.audio_stream {
                let mut file = File::create("response.mp3")?;
                file.write_all(&stream)?;
            }
        }
        Err(error) => println!("Failed with {:?}", error),
    };

    Ok(())
}
