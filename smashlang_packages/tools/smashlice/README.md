# SmashLice
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


<p align="center">
  <img src="./assets/logo.svg" alt="SmashLice Logo" width="200" />
</p>

A license generator for SmashLang projects, inspired by [lice](https://github.com/superkhau/lice).

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
smashpkg install smashlice
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


- **Interactive Mode**: Generate licenses through an interactive prompt
- **Non-interactive Mode**: Generate licenses with command-line arguments
- **Multiple License Types**: Support for 30+ open source licenses
- **License Viewer**: View the contents of any supported license
- **Customization**: Set custom author name, year, and output path

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
import "smashlice";

// Generate a license interactively
smashlice.generateLicense();

// Generate a specific license non-interactively
smashlice.generateLicense("mit", {
  user: "Your Name",
  year: "2025",
  path: "./",
  name: "LICENSE"
});

// Show the contents of a license
const licenseContent = smashlice.showLicense("mit");
console.log(licenseContent);

// List all available licenses
const licenses = smashlice.listLicenses();
console.log(licenses);
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


SmashLice can also be used from the command line:

```bash
# Interactive mode
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

smash -r smashlice

# Non-interactive mode
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files

smash -r smashlice -l mit -u "Your Name" -y 2025
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
  -h, --help      Display the help menu
  -l, --license   The type of license to generate, [default: mit]
  -n, --name      The name of the generated license, [default: LICENSE]
  -p, --path      License generation file path, [default: current working dir]
  -s, --show      Show the contents of a license
  -u, --user      The name to use in the generated license
  -y, --year      Year placeholder [default: current year]
```

## Available Licenses
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


SmashLice supports the following licenses:

- **aal**: Attribution Assurance License (AAL)
- **afl-3.0**: Academic Free License 3.0 (AFL-3.0)
- **apl-1.0**: Adaptive Public License (APL-1.0)
- **agpl-3.0**: Affero GNU Public License 3.0 (AGPL-3.0)
- **apache-2.0**: Apache License, Version 2.0 (Apache-2.0)
- **artistic-2.0**: Artistic License (Artistic-2.0)
- **bsd-2-clause**: BSD 2-Clause "Simplified" License (BSD-2-Clause)
- **bsd-3-clause**: BSD 3-Clause "New" or "Revised" License (BSD-3-Clause)
- **bsl-1.0**: Boost Software License (BSL-1.0)
- **cpal-1.0**: Common Public Attribution License 1.0 (CPAL-1.0)
- **eupl**: European Union Public License v. 1.2
- **gpl-3.0**: GNU General Public License 3.0 (GPL-3.0)
- **isc**: ISC License (ISC)
- **mit**: The MIT License (MIT) [default]
- **mpl-2.0**: Mozilla Public License Version 2.0 (MPL-2.0)
- **ms-pl**: Microsoft Public License (MS-PL)
- **w3c**: The W3C SOFTWARE NOTICE AND LICENSE (W3C)
- **zlib-libpng**: The zlib/libpng License (Zlib)
- And many more...

## API Reference
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


### generateLicense(licenseType, options)
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


Generates a license file.

- **licenseType** (string, optional): The type of license to generate. Defaults to "mit".
- **options** (object, optional): Configuration options.
  - **user** (string): The name to use in the license.
  - **year** (string): The year to use in the license. Defaults to current year.
  - **path** (string): The path where the license will be generated. Defaults to current directory.
  - **name** (string): The filename of the license. Defaults to "LICENSE".

### showLicense(licenseType)
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


Returns the content of a license without generating a file.

- **licenseType** (string): The type of license to show.

### listLicenses()
## Package Structure

This package follows the standard SmashLang package structure:

- `package.smash`: Build and installation configuration
- `package_config.json`: Theme and presentation configuration
- `assets/`: Package assets (logos, icons, etc.)
- `src/`: Source code
- `examples/`: Example code
- `tests/`: Test files


Returns an array of all available license types.

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

- **basic.smash**: Simple license generation
- **interactive.smash**: Interactive license generation
- **non_interactive.smash**: Non-interactive license generation with options

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
