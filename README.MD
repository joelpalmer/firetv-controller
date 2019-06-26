## Control any Fire TV or Fire TV Stick on your network via your terminal.

**Prerequisites:** 

* [ADB](https://developer.android.com/studio/command-line/adb) installed & on your path
* Your Fire TV device has ADB enabled
* You know your Fire TV's IP Address

**Todo:**

- [x] Connect 
```console 
joel@Joels-iMac firetv-controller (master) $ cargo run <your-firetv-ip>
* daemon not running; starting now at tcp:5037
* daemon started successfully
connected to <your-firetv-ip>
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

