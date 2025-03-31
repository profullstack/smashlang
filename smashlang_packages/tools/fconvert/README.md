# FConvert
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


<p align="center">
  <img src="./assets/logo.svg" alt="FConvert Logo" width="200" />
</p>

A versatile file format conversion utility for SmashLang.

## Installation
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


```bash
smashpkg install fconvert
```

## Features
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


- **Multiple Format Support**: Convert between various file formats
- **Image Conversion**: Convert between PNG, JPEG, GIF, WebP, SVG, and more
- **Document Conversion**: Transform between Markdown, HTML, PDF, DOCX, and other formats
- **Audio Conversion**: Convert between MP3, WAV, FLAC, OGG, and other audio formats
- **Video Conversion**: Transform between MP4, WebM, AVI, MKV, and other video formats
- **Batch Processing**: Convert multiple files at once
- **Customizable Options**: Set quality, size, and other conversion parameters

## Basic Usage
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


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
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


FConvert can also be used from the command line:

```bash
# Basic conversion
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

smash -r fconvert input.png output.jpg

# Conversion with options
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

smash -r fconvert input.md output.pdf --quality=high --page-size=a4

# List supported formats
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

smash -r fconvert --list-formats
```

### Command Line Options
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


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
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


### Image Formats
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

- PNG, JPEG/JPG, GIF, WebP, TIFF, BMP, SVG, ICO

### Document Formats
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

- Markdown, HTML, PDF, DOCX, ODT, RTF, TXT, LaTeX

### Audio Formats
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

- MP3, WAV, FLAC, OGG, AAC, M4A, WMA

### Video Formats
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

- MP4, WebM, AVI, MKV, MOV, WMV, FLV

### Data Formats
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

- JSON, YAML, XML, CSV, TSV, Excel (XLSX)

## API Reference
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


### convert(inputPath, outputPath, options)
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


Converts a file from one format to another.

- **inputPath** (string): Path to the input file.
- **outputPath** (string): Path to the output file.
- **options** (object, optional): Conversion options.
  - Format-specific options vary based on input and output formats.

### getFormats()
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


Returns an object containing all supported input and output formats.

### class Converter
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


A class-based interface for more complex conversion operations.

#### Methods:
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


- **setInputFile(path)**: Set the input file path.
- **setOutputFile(path)**: Set the output file path.
- **setOption(key, value)**: Set a conversion option.
- **getOption(key)**: Get a conversion option value.
- **convert()**: Perform the conversion.
- **getProgress()**: Get the current conversion progress (0-100).

## Examples
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


Check the examples directory for more usage examples:

- **basic.smash**: Simple file conversion
- **image_conversion.smash**: Image format conversion with options
- **document_conversion.smash**: Document format conversion
- **audio_conversion.smash**: Audio format conversion

## Dependencies
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


FConvert requires the following system tools to be installed:

- **ImageMagick**: Required for image conversions (command: `convert`)
- **Pandoc**: Required for document conversions (command: `pandoc`)
- **FFmpeg**: Required for audio and video conversions (command: `ffmpeg`)

These dependencies are checked during installation and when using the package. If any of these tools are missing, appropriate error messages will be displayed.

### Installation of Dependencies
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


#### Debian/Ubuntu
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

```bash
sudo apt-get install imagemagick pandoc ffmpeg
```

#### Fedora
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

```bash
sudo dnf install ImageMagick pandoc ffmpeg
```

#### macOS (using Homebrew)
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

```bash
brew install imagemagick pandoc ffmpeg
```

#### Windows
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

Download and install the following:
- ImageMagick: https://imagemagick.org/script/download.php
- Pandoc: https://pandoc.org/installing.html
- FFmpeg: https://ffmpeg.org/download.html

## License
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


This project is licensed under the MIT License - see the LICENSE file for details.
