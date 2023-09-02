# Skeleton UI Theme Converter

Convert your theme data for Skeleton UI

## Getting Started

### Prerequisites

You need to have rust installed on your system, you can use rustup.sh for this.

### Building from Source

1. Clone this repository:
   ```bash
   git clone https://github.com/venual/theme-convert.git
   ```

2. Build the project:
   ```bash
   cd theme-convert
   cargo build
   ```

3. The built binary will be available at `target/debug/theme-convert`.

**Note**: You can rewrite this in any other lang pretty easily if you dont have rust installed

## Usage

Easier way is to add the --script flag at the end, this outputs a script, go to theme generator, make sure preview is off, refresh the page before or after just to make sure, then paste the outputted script in console, run it then turn on preview and should work, else you can do it manually without the --script flag like below.

1. Prepare your theme JSON data in a file. Ensure it's structured correctly; it should contain only the theme data wrapped in braces `{...}`.
  
2. Convert your theme data:
   ```bash
   ./theme-convert --file your_filename.json
   ```

3. The converted JSON will be displayed in the terminal.

4. Head over to the skeleton theme generator page.

5. Activate the "preview" option and modify a setting to enable theme adjustments.

6. Open your browser's developer tools, navigate to the "Application" tab, and then to "LocalStorage".

7. Locate the key named `storeThemGenForm`.

8. Replace its contents with the JSON data from your terminal.

9. Your custom theme should now be loaded.
