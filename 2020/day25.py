#! /usr/bin/env python

def transform(subject, loop_size):
    """
    >>> transform(7, 8)
    5764801
    >>> transform(7, 11)
    17807724
    >>> transform(17807724, 8) == transform(5764801, 11)
    True
    """
    value = 1
    for i in range(loop_size):
        value = value * subject
        value = value % 20201227
    return value

card_pub = 9717666
door_pub = 20089533

if __name__ == "__main__":
    door = None
    card = None
    lpsize = 1
    value = 1
    while True:
        value = value * 7
        value = value % 20201227
        print(lpsize, value)
        if door_pub == value:
            print("door_lpsize", lpsize)
            door = lpsize
            break
        if card_pub == value:
            print("card_lpsize", lpsize)
            card = lpsize
            break
        lpsize += 1
        if card and door:
            break
    print(transform(card_pub, door))
