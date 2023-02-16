# pr0wser4

Pr0wser4 is a video scene browser that can be controlled using only one hand.

It is primarily good for very large collections of videos.

The video collection (consisting of a set of scenes) must be supplied in a text file, using a custom text-based format. Each entry describes a video file or a portion of a video file along with various attributes, including user-defined tags. Screenshots for all scenes can be stored in subdirectories called `.pr0wser` of the directories where the respective video files are.

In the program, you can filter those scenes by logical expressions over their attributes and play them using an external video player (currently mpv, hardcoded in the source code).

This version is very much WIP.

## Technology

This version of pr0wser is build on Tauri and SvelteKit. Previous versions where made with Qt.

As mentioned, the data is kept in a text-based format. It is inspired by JSON, but optimized primarily for brevity. You can find a description

## Building / development

You will need to have rust and Tauri installed to build the software.

```bash
# build development version and execute it
cargo tauri dev
```

or build the production version (not really on my radar right know, but works)

```bash
# build production version
cargo tauri build
```
