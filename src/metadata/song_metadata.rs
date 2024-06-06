use std::path::PathBuf;
use std::time::Duration;
use lofty::config::WriteOptions;
use lofty::probe::Probe;
use lofty::prelude::*;
use lofty::tag::{ItemValue, Tag, TagItem};

#[derive(Clone, Debug)]
pub struct SongMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub composer: Option<String>,
    pub genre: Option<String>,
    pub track_number: Option<u16>,
    pub disc_number: Option<u16>,
    pub year: Option<u16>,
    pub comment: Option<String>,
    pub duration: Option<Duration>,
    pub total_tracks: Option<u16>,
    pub total_discs: Option<u16>,
    pub is_compilation: Option<bool>
}

impl SongMetadata {
    pub fn read_metadata_from_audio_file(file_path: &PathBuf) -> SongMetadata {

        if !file_path.is_file() {
            panic!("ERROR: Path is not a file!");
        }

        let tagged_file = Probe::open(file_path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag() {
            Some(primary_tag) => primary_tag,
            None => tagged_file.first_tag().expect("ERROR: No tags found!"),
        };

        let properties = tagged_file.properties();
        let duration = properties.duration();

        return SongMetadata {
            title: tag.title().map(|s| s.to_string()),
            artist: tag.artist().map(|s| s.to_string()),
            album: tag.album().map(|s| s.to_string()),
            album_artist: tag.get_string(&ItemKey::AlbumArtist).map(|s| s.to_string()),
            composer: tag.get_string(&ItemKey::Composer).map(|s| s.to_string()),
            genre: tag.genre().map(|s| s.to_string()),
            track_number: tag.track().map(|s| s as u16),
            disc_number: tag.disk().map(|s| s as u16),
            year: tag.year().map(|s| s as u16),
            comment: tag.comment().map(|s| s.to_string()),
            duration: Some(duration),
            total_tracks: tag.track_total().map(|s| s as u16),
            total_discs: tag.disk_total().map(|s| s as u16),
            is_compilation: match tag.get_string(&ItemKey::FlagCompilation) {
                Some(s) => Some(s == "1"),
                None => None,
            }
        };
    }

    pub fn write_metadata_to_audio_file(&self, file_path: &PathBuf) {
        let mut tagged_file = Probe::open(file_path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let tag = match tagged_file.primary_tag_mut() {
            Some(primary_tag) => primary_tag,
            None => {
                if let Some(first_tag) = tagged_file.first_tag_mut() {
                    first_tag
                } else {
                    let tag_type = tagged_file.primary_tag_type();
    
                    eprintln!("WARN: No tags found, creating a new tag of type `{tag_type:?}`");
                    tagged_file.insert_tag(Tag::new(tag_type));
    
                    tagged_file.primary_tag_mut().unwrap()
                }
            },
        };

        if self.title.is_some() {
            tag.set_title(self.title.clone().unwrap());
        }
        if self.artist.is_some() {
            tag.set_artist(self.artist.clone().unwrap());
        }
        if self.album.is_some() {
            tag.set_album(self.album.clone().clone().unwrap());
        }
        if self.album_artist.is_some() {
            tag.insert(TagItem::new(ItemKey::AlbumArtist, ItemValue::Text(self.album_artist.clone().unwrap())));
        }
        if self.composer.is_some() {
            tag.insert(TagItem::new(ItemKey::Composer, ItemValue::Text(self.composer.clone().unwrap())));
        }
        if self.genre.is_some() {
            tag.set_genre(self.genre.clone().unwrap());
        }
        if self.track_number.is_some() {
            tag.set_track(self.track_number.clone().unwrap() as u32);
        }
        if self.disc_number.is_some() {
            tag.set_disk(self.disc_number.clone().unwrap() as u32);
        }
        if self.year.is_some() {
            tag.set_year(self.year.clone().unwrap() as u32);
        }
        if self.comment.is_some() {
            tag.set_comment(self.comment.clone().unwrap());
        }
        if self.total_tracks.is_some() {
            tag.set_track_total(self.total_tracks.clone().unwrap() as u32);
        }
        if self.total_discs.is_some() {
            tag.set_disk_total(self.total_discs.clone().unwrap() as u32);
        }
        if self.is_compilation.is_some() {
            tag.insert(TagItem::new(ItemKey::FlagCompilation, ItemValue::Text(if self.is_compilation.unwrap() { "1" } else { "0" }.to_string())));
        }

        match tag.save_to_path(file_path, WriteOptions::default()) {
            Ok(_) => println!("Metadata saved successfully!"),
            Err(e) => panic!("ERROR: Failed to save metadata to file: {:?}", e),
        }
    }

    pub fn pretty_print(&self) {
        let track_number: String = match self.track_number.clone() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let disc_number: String = match self.disc_number.clone() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let year: String = match self.year.clone() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let duration: String = match self.duration.clone() {
            Some(d) => format!("{:?}", d),
            None => "".to_string(),
        };
        let total_tracks: String = match self.total_tracks.clone() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let total_discs: String = match self.total_discs.clone() {
            Some(n) => n.to_string(),
            None => "".to_string(),
        };
        let is_compilation: String = match self.is_compilation.clone() {
            Some(b) => b.to_string(),
            None => "".to_string(),
        };
        println!("Title:          {:?}", self.title.clone().unwrap_or("".to_string()));
        println!("Artist:         {:?}", self.artist.clone().unwrap_or("".to_string()));
        println!("Album:          {:?}", self.album.clone().unwrap_or("".to_string()));
        println!("Album Artist:   {:?}", self.album_artist.clone().unwrap_or("".to_string()));
        println!("Composer:       {:?}", self.composer.clone().unwrap_or("".to_string()));
        println!("Genre:          {:?}", self.genre.clone().unwrap_or("".to_string()));
        println!("Track Number:   {:?}", track_number);
        println!("Disc Number:    {:?}", disc_number);
        println!("Year:           {:?}", year);
        println!("Comment:        {:?}", self.comment.clone().unwrap_or("".to_string()));
        println!("Duration:       {:?}", duration);
        println!("Total Tracks:   {:?}", total_tracks);
        println!("Total Discs:    {:?}", total_discs);
        println!("Is Compilation: {:?}", is_compilation);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_work() {
        assert_eq!(2 + 2, 4);
    }
}