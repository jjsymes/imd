# imd
CLI application that finds the best matching song on iTunes based on an audio files metadata tags

## Build dependencies

- libssl-dev (sudo apt-get install libssl-dev)

## Usefull documentation

- itunes API https://performance-partners.apple.com/search-api

## TODO

- Use iTunes API (maybe for albums) to find is_compilation and album_artist
- Add option to write metadata to the audio file (Select from top 5 matches)
- Add option to run with no prompt when writing metadata (selects first match)
- Improve top match by prioritizing the earliest release date that isn't a single or compilation (unless Single version is explicit in the name)
- Handle album artwork
- Tests
- Improve output formatting