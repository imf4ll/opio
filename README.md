```ls
                                                   _________
                                      _____________ ______  /
                                      __  ___/  __ `/  __  / 
                                      _  /   / /_/ // /_/ /  
                                      /_/    \__,_/ \__,_/

                                  AUR helper & package downgrader
```

<br/>

[Features](#features) <br/>
[Install](#how-to-install) <br/>
[Arguments](#arguments) <br/>
[Usage](#how-to-use) <br/><br/>

### Features:
- AUR Helper with search and choose
- Pacman's package downgrader
- AUR's package downgrader

<br/>

### How to install:
```
$ git clone https://github.com/imf4ll/rad.git

$ cd rad/

$ make install
```

### Arguments:
| Name | Description | Usage |
|------|-------------|-------|
| -p, --package | Package name | PACKAGE |
| -f, --file-path | Final package download path | PATH |
| -i, --ignore-cache | Ignores packages from cache | - |
| -d, --downgrade | Turns on downgrade mode | - |
| -a, --aur | Turns on AUR mode | - |
| -s, --search | Runs AUR in search mode | - |
| -h, --help | Print help | - |
| -V, --version | Print version | - |

<br/>

### How to use:
- Install first valid package:
```bash
$ rad -p allacrity
```

<br/>

- Search and choose through AUR:
```bash
$ rad -s -p allacrity
```

<br/>

- Downgrade 'pacman' package: (REQUIRES ROOT PRIVILEGES)
```bash
$ sudo rad -d -p xsel
```

<br/>

- Downgrade 'AUR' package:
```sh
$ sudo rad -a -d -p brave-bin
```

<br/>
