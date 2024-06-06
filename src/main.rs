mod app_config;
mod metadata;
use std::path::Path;
use app_config::AppConfig;
use metadata::song_metadata::SongMetadata;
use metadata::metadata_fixer;

fn main() {
    let command_options = AppConfig::from_command_args();
    print_title();
    print_command_options(&command_options);

    let song_metadata: SongMetadata = SongMetadata::read_metadata_from_audio_file(&command_options.path);
    let fixed_metadata: SongMetadata = metadata_fixer::get_fixed_metadata(&song_metadata);
    println!("Fixed metadata:");
    fixed_metadata.pretty_print();

    let path = Path::new(&command_options.path);
    if !path.is_file() {
		panic!("ERROR: Provided path is not a file!");
	}
    println!("Done");
}

fn print_command_options(command_options: &AppConfig) {
    println!("File name: {:?}", command_options.path);
    println!("Debug: {:?}", command_options.debug);
}

fn print_title() {
    let version: &str = env!("CARGO_PKG_VERSION");
    println!("imd version: {}", version);
}

#[cfg(test)]
mod tests {
    // use super::*;
    #[test]
    fn tests_work() {
        assert_eq!(2 + 2, 4);
    }
}