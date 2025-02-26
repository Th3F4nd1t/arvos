import serial
import time

# Replace 'COM3' with your port name (e.g., 'COM3' on Windows or '/dev/ttyUSB0' on Linux)
port_name = 'COM4'
baud_rate = 57600  # Default baud rate for LR24-F

try:
    # Open serial port
    ser = serial.Serial(port_name, baud_rate, timeout=1)
    print(f"Opened port {port_name} successfully.")

    # Send data
    test_message = "Hello, LR24-F!"
    ser.write(test_message.encode())
    print(f"Sent: {test_message}")

    # Receive data
    while True:
        if ser.in_waiting > 0:
            incoming_data = ser.readline().decode().strip()
            print(f"Received: {incoming_data}")

        # For demonstration, exit after 10 seconds
        time.sleep(10)
        break

except serial.SerialException as e:
    print(f"Error opening or using serial port: {e}")

finally:
    if 'ser' in locals() and ser.is_open:
        ser.close()
        print(f"Closed port {port_name}.")