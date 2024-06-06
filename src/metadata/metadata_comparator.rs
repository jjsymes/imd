use strsim::jaro_winkler;
use super::song_metadata::SongMetadata;

pub struct MetadataComparator {
    song_metadata: SongMetadata,
    potential_metadata_match: SongMetadata,
}

impl MetadataComparator {
    pub fn new(song_metadata: SongMetadata, potential_metadata_match: SongMetadata) -> MetadataComparator {
        MetadataComparator {
            song_metadata,
            potential_metadata_match,
        }
    }

    pub fn get_overall_score(&self) -> f64 {
        let title_artist_score = self.get_title_artist_score();
        let duration_score = self.get_duration_score();
        return average([title_artist_score, duration_score]);

    }

    fn get_title_artist_score(&self) -> f64 {
        let title_score = self.get_title_score();
        let artist_score = self.get_artist_score();
        return average([title_score, artist_score]);
    }

    fn get_title_score(&self) -> f64 {
        return match (&self.song_metadata.title, &self.potential_metadata_match.title) {
            (Some(song_title), Some(itunes_title)) => {
                jaro_winkler_distance(song_title, itunes_title)
            },
            _ => 0.0,
        };
    }

    fn get_artist_score(&self) -> f64 {
        return match (&self.song_metadata.artist, &self.potential_metadata_match.artist) {
            (Some(song_artist), Some(itunes_artist)) => {
                jaro_winkler_distance(song_artist, itunes_artist)
            },
            _ => 0.0,
        };
    }

    fn get_duration_score(&self) -> f64 {
        return if song_time_within_tolerance(&self.song_metadata, &self.potential_metadata_match) {
            1.0
        } else {
            0.0
        };
    }
}

fn song_time_within_tolerance(song_metadata: &SongMetadata, itunes_metadata: &SongMetadata) -> bool {
    const TOLERANCE_S: u64 = 10;
    let song_duration = song_metadata.duration.unwrap();
    let itunes_duration = itunes_metadata.duration.unwrap();
    let duration_diff = song_duration.checked_sub(itunes_duration).unwrap_or_else(|| itunes_duration.checked_sub(song_duration).unwrap());
    return duration_diff.as_secs() <= TOLERANCE_S;
}

fn jaro_winkler_distance(s1: &str, s2: &str) -> f64 {
    return jaro_winkler(s1, s2);
}

fn average<T>(values: T) -> f64
where
    T: IntoIterator,
    T::Item: Into<f64>,
{
    let mut sum = 0.0;
    let mut count = 0;
    for value in values {
        sum += value.into();
        count += 1;
    }
    return sum / count as f64;
}
