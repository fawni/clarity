use lofty::{read_from_path, Accessor, AudioFile};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let path = args.get(1).expect("file path not provided");

    let tagged_file = read_from_path(path, true)?;
    let tag = tagged_file.primary_tag().unwrap();
    let properties = tagged_file.properties();

    let duration = properties.duration().as_secs();
    let h = (duration / 60) / 60;
    let m = (duration / 60) % 60;
    let s = duration % 60;
    let time = match h {
        0 => format!("{:02}:{:02}", m, s),
        _ => format!("{:02}:{:02}:{:02}", h, m, s),
    };

    println!(
        "{} ({} of {})",
        tag.title().ok_or("...")?,
        tag.track().ok_or("0")?,
        tag.track_total().ok_or("0")?,
    );
    println!("{}", tag.artist().ok_or("...")?);
    println!(
        "{} ({})",
        tag.album().ok_or("...")?,
        tag.year().ok_or("...")?
    );
    println!(
        "{:?} {} kHz, {}k, {}",
        tagged_file.file_type(),
        properties.sample_rate().unwrap() as f32 / 1000.0,
        properties.audio_bitrate().unwrap(),
        time
    );

    Ok(())
}
