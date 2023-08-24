# Auto-sensitivity

## Goal
On linx with X11 it's hard to use gaming mouse without having to change the DPI each time compared to an office mouse.
This utility allows you to have sensitivity saved per xinput device

## Installation
1. Ensure that you're using x11 and have xinput installed
2. Run `cargo install autosensitivity`

## Usage
Modify the sensivity for a device as you would normaly do:
```bash
xinput list

xinput --set-prop <ID> 'libinput Accel Speed' -0.25
```

Then you can save the config like that:
```bash
autosensitivity --save <INPUT_NAME>
```

And load all your saved configs like that:
```bash
autosensitivity --load
```
