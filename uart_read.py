import serial
import time

ser = serial.Serial('/dev/ttyACM0', timeout=1)
while True:
    ser.write(b"h")
    print(x:=ser.read(1))
    time.sleep(0.1)
