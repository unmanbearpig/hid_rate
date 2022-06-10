# hid_rate

Roughly measures mouse or some other device poll rate by reading data in a
loop and measuring how long the read took us.

#### Build

```sh
cargo build --release
```

The built binary is at `target/release/hid_rate`

You can "install" it (which is completely optional) into you system by just
copying the binary into any directory in your path, for example into
`/usr/local/bin/`

```sh
sudo cp target/release/hid_rate /usr/local/bin/hid_rate
```

#### Run

It needs permissions to read the file, so if it's an input device you're
likely need to run it as root.

```sh
sudo hid_rate /dev/input/by-id/usb-Logitech_USB-PS_2_Optical_Mouse-event-mouse
```

I'll keep running until you `Ctrl-C` it.

#### Output

Shows time each read took in nanoseconds, microseconds, converted to hz,
and how many bytes we read.

```
       1995992 ns,       1995.992 us,  501.00401 hz,   72 bytes read
       3980757 ns,       3980.757 us,  251.20850 hz,   72 bytes read
       3988748 ns,       3988.748 us,  250.70523 hz,   72 bytes read
       3977186 ns,       3977.186 us,  251.43405 hz,   72 bytes read
       3991401 ns,       3991.401 us,  250.53860 hz,   72 bytes read
       3987779 ns,       3987.779 us,  250.76615 hz,   72 bytes read
       3982110 ns,       3982.110 us,  251.12315 hz,   48 bytes read
       3979834 ns,       3979.834 us,  251.26676 hz,   48 bytes read
       1988891 ns,       1988.891 us,  502.79276 hz,   48 bytes read
       1984657 ns,       1984.657 us,  503.86540 hz,   48 bytes read
       1985271 ns,       1985.271 us,  503.70957 hz,   48 bytes read
       7988877 ns,       7988.877 us,  125.17404 hz,   48 bytes read
```
