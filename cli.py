import argparse
from interpreter import interpreter

def main():
    parser = argparse.ArgumentParser(description="opens le file")
    parser.add_argument("filename", help="dir of file to open")
    args = parser.parse_args()

    interpreter(args.filename)



if __name__ == '__main__':
    main()