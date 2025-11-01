import serial

ser = serial.Serial('/dev/ttyACM0', timeout=100)
while True:
    print(x:=ser.read(12))
