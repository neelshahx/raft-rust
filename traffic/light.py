# light.py
#
# An internet connected light capable of displaying three colors (R,
# G, Y and responding to button presses. You pick a port number and
# send it a message via UDP to change the color.  When the button is
# pressed, it sends a message to an output port of your choosing.  Here's an
# example of communication assuming the light is listening for color
# changes on port 10000 and sending button presses to port 11000.
#
# 
#    >>> from socket import socket, AF_INET, SOCK_DGRAM
#    >>> sock = socket(AF_INET, SOCK_DGRAM)
#    >>> sock.bind(('localhost', 11000))
#    >>> # Send a message to change the light color 
#    >>> sock.sendto(b'NS-G', ('localhost', 10000))
#
# Try sending messages such as b"EW-G" or b"NS-Y". You should see the
# output change.   Try receiving a message to get the button press
#
#    >>> sock.recvfrom(100)
#
# Type 'ns' or 'ew' at the prompt to send a button press.

from socket import socket, AF_INET, SOCK_DGRAM
import sys
import time
import os

codes = {
    'G': '\x1b[32mG\x1b[0m',
    'R': '\x1b[31mR\x1b[0m',
    'Y': '\x1b[33mY\x1b[0m',
}

class TrafficSignal:
    def __init__(self, ew='R', ns='R'):
        self.ew = ew
        self.ns = ns

    def __str__(self):
        return (
            f'    {codes[self.ns]}\n'
            f'  +---+\n'
            f'{codes[self.ew]} |   | {codes[self.ew]}\n'
            f'  +---+\n'
            f'    {codes[self.ns]}\n')

def main(inport, outport):
    sock = socket(AF_INET, SOCK_DGRAM)
    sock.bind(('', inport))
    sock.setblocking(False)
    os.set_blocking(0, False)
    signal = TrafficSignal()
    print(f"I'm a traffic light. Send me colors on port {inport}.")
    print(f"Receive button presses by listening on port {outport}")
    
    while True:
        sys.stdout.write(f'\n{signal}')
        sys.stdout.write(f"\n[Type 'ns' or 'ew' for button]")
        sys.stdout.flush()
        while True:
            try:
                msg, addr = sock.recvfrom(8192)
                break
            except BlockingIOError:
                time.sleep(0.05)

            line = sys.stdin.readline().lower()
            if line.startswith('ns'):
                sock.sendto(b'NS-BUTTON', ('localhost', outport))
            if line.startswith('ew'):
                sock.sendto(b'EW-BUTTON', ('localhost', outport))
        
        msg = msg.decode('ascii')
        if msg.startswith('EW'):
            signal.ew = msg[3]
        elif msg.startswith('NS'):
            signal.ns = msg[3]

if __name__ == '__main__':
    import os
    if len(sys.argv) != 3:
        raise SystemExit(f'Usage: {sys.argv[0]} inport outport')
    os.system('')  # Windows hack. Don't ask.
    main(int(sys.argv[1]), int(sys.argv[2]))
