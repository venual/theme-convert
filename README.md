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

### With Console Script

1. **Enable the Script Flag**: Append the `--script` flag at the end of your command. 
    ```bash
    ./theme-convert --file your_filename.json --script
    ```

2. **Copy the Output**: The script will be displayed in your terminal.

3. **Navigate to Theme Generator**: Open the skeleton theme generator page in your browser.

4. **Refresh the Page**: Just to ensure a clean slate, refresh the page.

5. **Turn Off Preview**: Make sure the preview option is deactivated.

6. **Run the Script**: Open your browser's dev tools and navigate to the "Console" tab. Paste the outputted script from your terminal into the console and run it.

7. **Activate Preview**: Turn the preview option back on.

8. **Test Your Theme**: Your custom theme should now be loaded. If it doesn't work, you can follow the manual mode steps below.

---

### Manual Mode

1. **Prepare JSON File**: Make sure your theme JSON data is in a file, wrapped in braces `{...}`.

2. **Convert Data**: Run the following command to convert your theme data.
    ```bash
    ./theme-convert --file your_filename.json
    ```

3. **Copy Output**: The converted JSON will be displayed in the terminal.

4. **Navigate to Theme Generator**: Open the skeleton theme generator page.

5. **Enable Preview and Modify**: Activate the "preview" option and modify any setting to enable theme adjustments.

6. **Access LocalStorage**: Open your browser's developer tools, navigate to the "Application" tab, and then to "LocalStorage".

7. **Replace Key**: Locate the key named `storeThemGenForm` and replace its contents with the JSON data from your terminal.

8. **Test Your Theme**: Your custom theme should now be loaded.
