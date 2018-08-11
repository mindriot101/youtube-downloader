import argparse
from .server import Server


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('-c', '--config', required=True)
    parser.add_argument('-s', '--sleep-time', required=False,
            type=int, default=86400)
    args = parser.parse_args()


    server = Server(args.config, sleep_time=args.sleep_time)
    server.run()
