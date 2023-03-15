# Desktop linker

Make any binary a desktop application with a single command.

## Installation

```bash
git clone https://github.com/vclmenzi/desktop-linker.git && cd desktop-linker
```

Run the `install.sh` script:

```bash
sh ./install.sh
```

## Usage

```bash
desktop-linker <binary> [--name <name>] [--icon <icon>] [--type <type>] [--assets "file1, file2, ..."]
```

`<binary>` is the path to the binary you want to link.

`<name>` is the name of the desktop application.

`<icon>` is the path to the icon of the desktop application.

`<type>` is the type of the desktop application.

`<assets>` is a list of assets to be copied to the desktop application directory.