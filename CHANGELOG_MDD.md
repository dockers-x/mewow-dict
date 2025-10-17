# Changelog - MDD Support Implementation

## Overview
Added comprehensive support for parsing and serving MDD (Mdict Data) resource files. MDD files are companion files to MDX dictionaries that contain media resources such as images, audio files, CSS, JavaScript, and other assets referenced in dictionary entries.

## Changes Made

### New Files

1. **src/mdict/mdd.rs**
   - Complete MDD file parser following the same structure as MDX
   - `Mdd` struct for managing parsed MDD files
   - `ResourceOffsetInfo` struct for tracking resource locations in compressed data
   - `Resource` struct for representing extracted resources
   - `get_resource_by_path()` method with case-insensitive path matching
   - Support for normalized paths (handles leading slashes and backslashes)

2. **src/mdd_manager.rs**
   - Global MDD resource manager using `once_cell::Lazy` for thread-safe singleton
   - `MddManager` struct for caching loaded MDD files
   - Automatic loading of all MDD files from configured directories
   - Resource lookup across all loaded MDD files
   - Support for dictionary-specific resource queries

3. **examples/mdd_usage.rs**
   - Example code demonstrating MDD API usage
   - Shows how to load, parse, and extract resources from MDD files

4. **test_mdd.sh**
   - Simple test script for verifying the build
   - Usage instructions for testing MDD functionality

### Modified Files

1. **src/mdict/mod.rs**
   - Added `pub mod mdd;` to expose the MDD module

2. **src/main.rs**
   - Added `mod mdd_manager;` module declaration
   - Imported `init_mdd_manager` and `handle_mdd_resource`
   - Added MDD resource route: `/mdd/{tail:.*}`
   - Initialize MDD manager on startup with logging
   - Route must come before the catch-all static handler

3. **src/handlers/mod.rs**
   - Added `handle_mdd_resource()` handler for serving MDD resources
   - Implemented `get_content_type()` helper function for proper MIME type detection
   - Supports 20+ file types including:
     - Images: jpg, png, gif, svg, webp, ico
     - Audio: mp3, wav, ogg
     - Video: mp4, webm
     - Fonts: woff, woff2, ttf, otf
     - Web: css, js, html, json, xml

4. **src/mdict/mdx.rs**
   - Fixed lifetime warning by specifying `Record<'_>` in `items()` return type
   - Added `#[allow(dead_code)]` for unused struct fields

5. **README.md**
   - Added "Features" section highlighting MDX and MDD support
   - Updated usage instructions to mention MDD files
   - Added comprehensive "MDD Resource Support" section with:
     - Explanation of what MDD files are
     - How the MDD system works
     - Supported resource types
     - Example usage

## Technical Details

### Architecture

The MDD implementation follows the same parsing strategy as MDX:
- Header parsing (version, encoding, encryption)
- Key block parsing (resource paths/names)
- Record block parsing (actual resource data)
- Lazy decompression (resources are decompressed only when requested)

### Resource Serving Flow

1. **Startup**: All `.mdd` files are discovered and loaded into memory
2. **Request**: Client requests `/mdd/path/to/resource.png`
3. **Lookup**: MDD manager searches all loaded MDD files for the resource
4. **Decompression**: If found, the specific block containing the resource is decompressed
5. **Response**: Resource is served with appropriate MIME type

### Memory Management

- MDD file structures are cached in memory (paths and offsets)
- Actual resource data remains compressed until requested
- Global singleton pattern ensures thread-safe access
- Resources are decompressed on-demand to minimize memory usage

## Testing

To test the implementation:

1. Place `.mdd` files in the same directories as your `.mdx` files
2. Run: `cargo build --release`
3. Run: `cargo run --bin mewow-dict`
4. Check console output for "Successfully loaded MDD: {name}"
5. Access resources via: `http://localhost:8181/mdd/{resource_path}`

## Compatibility

- Works with MDD files that use the same format as MDX v2.0
- Supports encrypted (2) and unencrypted (0) MDD files
- Compatible with various compression methods (zlib, lzo)

## Future Enhancements

Potential improvements:
- LRU cache for frequently accessed resources
- Compression of responses (gzip, brotli)
- Range request support for large media files
- Resource pre-loading for improved performance
- Separate MDD indexing similar to MDX SQLite indexing
