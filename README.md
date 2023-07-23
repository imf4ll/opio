<div align="center">
  <img src="https://dinopixel.com/preload/0620/minecraft-poppy.png" width="128" />
</div>

<br/>

- [Features](#features) <br/>
- [Install](#how-to-install) <br/>
- [Arguments](#arguments) <br/>
- [Usage](#how-to-use) <br/><br/>

### Features:
- AUR Helper with search and choose
- AUR's package downgrader
- Pacman's package downgrader

<br/>

### How to install:
```
$ git clone https://github.com/imf4ll/opio.git

$ cd opio/

$ make install
```

<br/>

### Arguments:
| Name | Description | Usage |
|------|-------------|-------|
| -p, --package | Package name | PACKAGE |
| -f, --file-path | Final package download path | PATH |
| -i, --ignore-cache | Ignores packages from cache while downgrading 'pacman' packages | - |
| -d, --downgrade | Turns on downgrade mode | - |
| -a, --aur | Turns on AUR mode | - |
| -s, --search | Search for a package in AUR | - |
| --status | Check Archive and AUR status | - |
| -k, --keep | Keep AUR package after installing | - |
| -h, --help | Print help | - |
| -V, --version | Print version | - |

<br/>

### How to use:
- Install first valid package:
```bash
$ opio -p allacrity
```

<br/>

- Search and choose through AUR:
```bash
$ opio -s -p allacrity
```

<br/>

- Downgrade 'pacman' package: (REQUIRES ROOT PRIVILEGES)
```bash
$ sudo opio -d -p xsel
```

<br/>

- Downgrade 'AUR' package:
```sh
$ opio -a -d -p brave-bin
```

<br/>

- Install AUR package and keep files:
```sh
$ opio -p brave-bin -k
```

<br/>

- Install AUR package and save files in specific directory:
```sh
$ opio -p brave-bin -f path/to/directory
```

<br/>
