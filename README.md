# Litematica JSON Parser
Application for viewing and managing material lists exported in JSON format from the [Litematica](https://github.com/maruohon/litematica/) minecraft mod.

## Features

* View material lists with icons
* Keep track of gathered items
* Auto-hide unobtainable items and custom user-specified ones

## Folders

_This app uses [platform_dirs](https://github.com/cjbassi/platform-dirs-rs) to maybe hopefully work on windows as well, although it isn't tested_

Directory  | Windows                                                | Linux/*BSD                           | macOS
-----------|--------------------------------------------------------|--------------------------------------|------------------------------------
Config | `%APPDATA%\LitematicaJSONParser` (`C:\Users\%USERNAME%\AppData\Roaming\LitematicaJSONParser`)    | `$XDG_CONFIG_HOME/LitematicaJSONParser` (`~/.config/LitematicaJSONParser`)     | `~/Library/Application Support/LitematicaJSONParser`
Data   | `%LOCALAPPDATA%\LitematicaJSONParser` (`C:\Users\%USERNAME%\AppData\Local\LitematicaJSONParser`) | `$XDG_DATA_HOME/LitematicaJSONParser` (`~/.local/share/LitematicaJSONParser`)  | `~/Library/Application Support/LitematicaJSONParser`

## Contributing / Feedback

If you would like to contribute, please keep in mind that this is an app written by someone who has no experience in rust, and that the codebase may be very messy. Do also keep in mind that it might take a while for a PR to get approved, as this app mainly has to work for myself for my own uses. Always make an issue before implementing a feature, and feel free to ask for feedback in a discussion first.

If you have understood that, then please feel free to clean up my code or add new features.

