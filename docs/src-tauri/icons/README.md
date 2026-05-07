# Icons

This folder should contain your app icons. 

When you're ready to customize your app, replace these icons with your own:

- `32x32.png` - 32x32 PNG icon
- `128x128.png` - 128x128 PNG icon  
- `128x128@2x.png` - 256x256 PNG icon (2x retina)
- `icon.icns` - macOS icon file
- `icon.ico` - Windows icon file

You can use the Tauri CLI to generate icons from a single source image:

```bash
npm run tauri icon path/to/your-icon.png
```

This will automatically generate all required icon sizes and formats.

For now, the starter uses Tauri's default icons which will be generated automatically when you first run the app.


