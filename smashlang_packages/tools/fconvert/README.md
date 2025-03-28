# FConvert

<p align="center">
  <img src="./assets/logo.svg" alt="FConvert Logo" width="200" />
</p>

A versatile file format conversion utility for SmashLang.

## Installation

```bash
smashpkg install fconvert
```

## Features

- **Multiple Format Support**: Convert between various file formats
- **Image Conversion**: Convert between PNG, JPEG, GIF, WebP, SVG, and more
- **Document Conversion**: Transform between Markdown, HTML, PDF, DOCX, and other formats
- **Audio Conversion**: Convert between MP3, WAV, FLAC, OGG, and other audio formats
- **Video Conversion**: Transform between MP4, WebM, AVI, MKV, and other video formats
- **Batch Processing**: Convert multiple files at once
- **Customizable Options**: Set quality, size, and other conversion parameters

## Basic Usage

```js
import "fconvert";

// Simple conversion with automatic format detection
fconvert.convert("input.png", "output.jpg");

// Conversion with options
fconvert.convert("input.md", "output.pdf", {
  quality: "high",
  pageSize: "a4",
  margins: "1in"
});

// Get list of supported formats
const formats = fconvert.getFormats();
console.log(formats);

// Using the Converter class for more control
const converter = new fconvert.Converter();
converter.setInputFile("input.wav");
converter.setOutputFile("output.mp3");
converter.setOption("bitrate", "320k");
converter.setOption("sampleRate", "44100");
converter.convert();
```

## Command Line Usage

FConvert can also be used from the command line:

```bash
# Basic conversion
smash -r fconvert input.png output.jpg

# Conversion with options
smash -r fconvert input.md output.pdf --quality=high --page-size=a4

# List supported formats
smash -r fconvert --list-formats
```

### Command Line Options

```
Options:
  -h, --help           Display the help menu
  -q, --quality        Set the quality level (low, medium, high)
  -s, --size           Set the output size (e.g., 800x600)
  -f, --format         Explicitly specify the output format
  -b, --batch          Process multiple files (glob pattern)
  -o, --output-dir     Specify output directory for batch processing
  --list-formats       Show all supported formats
```

## Supported Formats

### Image Formats
- PNG, JPEG/JPG, GIF, WebP, TIFF, BMP, SVG, ICO

### Document Formats
- Markdown, HTML, PDF, DOCX, ODT, RTF, TXT, LaTeX

### Audio Formats
- MP3, WAV, FLAC, OGG, AAC, M4A, WMA

### Video Formats
- MP4, WebM, AVI, MKV, MOV, WMV, FLV

### Data Formats
- JSON, YAML, XML, CSV, TSV, Excel (XLSX)

## API Reference

### convert(inputPath, outputPath, options)

Converts a file from one format to another.

- **inputPath** (string): Path to the input file.
- **outputPath** (string): Path to the output file.
- **options** (object, optional): Conversion options.
  - Format-specific options vary based on input and output formats.

### getFormats()

Returns an object containing all supported input and output formats.

### class Converter

A class-based interface for more complex conversion operations.

#### Methods:

- **setInputFile(path)**: Set the input file path.
- **setOutputFile(path)**: Set the output file path.
- **setOption(key, value)**: Set a conversion option.
- **getOption(key)**: Get a conversion option value.
- **convert()**: Perform the conversion.
- **getProgress()**: Get the current conversion progress (0-100).

## Examples

Check the examples directory for more usage examples:

- **basic.smash**: Simple file conversion
- **image_conversion.smash**: Image format conversion with options
- **document_conversion.smash**: Document format conversion
- **audio_conversion.smash**: Audio format conversion

## Dependencies

FConvert requires the following system tools to be installed:

- **ImageMagick**: Required for image conversions (command: `convert`)
- **Pandoc**: Required for document conversions (command: `pandoc`)
- **FFmpeg**: Required for audio and video conversions (command: `ffmpeg`)

These dependencies are checked during installation and when using the package. If any of these tools are missing, appropriate error messages will be displayed.

### Installation of Dependencies

#### Debian/Ubuntu
```bash
sudo apt-get install imagemagick pandoc ffmpeg
```

#### Fedora
```bash
sudo dnf install ImageMagick pandoc ffmpeg
```

#### macOS (using Homebrew)
```bash
brew install imagemagick pandoc ffmpeg
```

#### Windows
Download and install the following:
- ImageMagick: https://imagemagick.org/script/download.php
- Pandoc: https://pandoc.org/installing.html
- FFmpeg: https://ffmpeg.org/download.html

## License

This project is licensed under the MIT License - see the LICENSE file for details.
