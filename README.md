# Mythic Telegram 📫

A simple steganography tool written in Rust.  
The project created as a playground to get familiar with Rust language.  


## Functionality

The tool is capable of hiding files in PNG images and restoring them.  
The following coding algoritms/modes are supported:
* <i>alpha</i> - encodes data on each pixel alpha channel
* <i>rgb</i> - encodes data on RGB channels using 1/2 or bits per channel

## Usage examples

### Encode
To encode data using <b>alpha</b> mode:
```lua
mythic-telegram encode --image-file <IMAGE_FILE> --secret-file <SECRET_FILE> alpha
```

To encode data using <b>rgb</b> mode:
```lua
mythic-telegram encode --image-file <IMAGE_FILE> --secret-file <SECRET_FILE> rgb --bits-per-channel <1/2/3/4>
```

where:
* <i>image-file</i> - path to image used to hide data in
* <i>secret-file</i> - path to secret file to be hidden inside image
* <i>bits-per-channel</i> - number of bits per channel to be used to encode data in RGB mode

### Decode
To decode data:
```lua
mythic-telegram decode --image-file <IMAGE_FILE>
```