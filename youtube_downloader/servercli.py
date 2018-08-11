import argparse
from .server import Server


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--config', required=True)
    args = parser.parse_args()


    server = Server(args.config)
    server.run()
