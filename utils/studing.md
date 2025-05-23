Identificando mouse

```sh
sudo udevadm info --query=all --name=/dev/hidraw1
```

```bash
E: USEC_INITIALIZED=31581031127
E: ID_VENDOR_FROM_DATABASE=Xenta
```

P: /devices/pci0000:00/0000:00:02.1/0000:02:00.0/usb1/1-4/1-4:1.1/0003:1D57:FA65.0017/hidraw/hidraw1
M: hidraw1
R: 1
U: hidraw
D: c 244:1
N: hidraw1
L: 0
E: DEVPATH=/devices/pci0000:00/0000:00:02.1/0000:02:00.0/usb1/1-4/1-4:1.1/0003:1D57:FA65.0017/hidraw/hidraw1
E: DEVNAME=/dev/hidraw1
E: MAJOR=244
E: MINOR=1
E: SUBSYSTEM=hidraw
E: USEC_INITIALIZED=31581031127
E: ID_VENDOR_FROM_DATABASE=Xenta

```sh
cat /sys/class/hidraw/hidraw2/device/uevent
```

```bash
DRIVER=hid-generic
HID_ID=0003:00001D57:0000FA65
HID_NAME=LXDDZ 2.4G 8K HS Receiver 
HID_PHYS=usb-0000:06:00.3-2/input1
HID_UNIQ=
MODALIAS=hid:b0003g0001v00001D57p0000FA65
```

Permissoes
```sh
sudo usermod -aG plugdev $USER
echo 'KERNEL=="hidraw*", SUBSYSTEM=="hidraw", MODE="0660", GROUP="plugdev"' | sudo tee /etc/udev/rules.d/99-hidraw-permissions.rules
sudo udevadm control --reload-rules
sudo udevadm trigger
``` 

Device que recebe a bateria
```sh
cat /sys/class/hidraw/hidraw2/device/uevent 

DRIVER=hid-generic
HID_ID=0003:00001D57:0000FA65
HID_NAME=LXDDZ 2.4G 8K HS Receiver 
HID_PHYS=usb-0000:06:00.3-2/input2
HID_UNIQ=
MODALIAS=hid:b0003g0001v00001D57p0000FA65
```