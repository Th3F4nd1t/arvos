import serial
import threading

def send_data(port, baudrate):
    with serial.Serial(port, baudrate, timeout=1) as ser:
        while True:
            message = input("Enter message to send: ")
            ser.write(message.encode('utf-8'))
            print(f"Sent on {port}: {message}")

def receive_data(port, baudrate):
    with serial.Serial(port, baudrate, timeout=1) as ser:
        while True:
            if ser.in_waiting > 0:
                data = ser.readline().decode('utf-8').strip()
                print(f"Received on {port}: {data}")

def main():
    send_port = "COM4"
    receive_port = "COM4"
    baudrate = 57600
    
    receiver_thread = threading.Thread(target=receive_data, args=(receive_port, baudrate), daemon=True)
    receiver_thread.start()
    
    # send_data(send_port, baudrate)
    
    input("Press Enter to exit...\n")

if __name__ == "__main__":
    main()