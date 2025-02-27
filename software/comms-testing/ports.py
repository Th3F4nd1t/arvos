import serial.tools.list_ports

def scan_com_ports():
    ports = serial.tools.list_ports.comports()
    available_ports = [port.device for port in ports]
    
    if available_ports:
        print("Available COM Ports:")
        for port in available_ports:
            print(f"  - {port}")
    else:
        print("No COM ports found.")

if __name__ == "__main__":
    scan_com_ports()
