[![](https://tokei.rs/b1/github/joelpalmer/firetv-controller)](https://github.com/joelpalmer/firetv-controller) 
[![Build Status](https://travis-ci.org/joelpalmer/firetv-controller.svg?branch=master)](https://travis-ci.org/joelpalmer/firetv-controller)
## Control any Fire TV or Fire TV Stick on your network via your terminal.

**Note:** _This should more or less work w/ any Android-based device. I haven't tested any others but you should be able to hijack your kid's tablet or your spouse's phone. Control that screen time!_ 😉
 

**Prerequisites:** 

* [ADB](https://developer.android.com/studio/command-line/adb) installed & on your path
* Your Fire TV device has ADB enabled (this opens port 5555)
* You know your Fire TV's IP Address

**Todo:**

- [x] Connect 
```console 
joel@Joels-iMac firetv-controller (master) $ cargo run <your-firetv-ip:5555>
* daemon not running; starting now at tcp:5037
* daemon started successfully
connected to <your-firetv-ip:5555>
What key would you like to press? (q to quit)
```
- [x] Control (up, down, left, right, enter, back, home, menu, play, pause)
    - [x] up
    - [x] down
    - [x] left
    - [x] right
    - [x] enter
    - [x] back
    - [x] home
    - [x] menu
    - [x] play
    - [x] pause
- [ ] Documentation

