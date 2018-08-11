import argparse
import zmq
from .job import Job

context = zmq.Context()


def add_job(job):
    msg = job.serialize_binary()
    socket = context.socket(zmq.REQ)
    socket.connect('tcp://127.0.0.1:5505')
    socket.send(msg)
    assert socket.recv() == b'ok'


def main():
    # TODO: host connection arguments
    parser = argparse.ArgumentParser()
    parser.add_argument('-u', '--url', required=True)
    parser.add_argument('-d', '--dest', required=True)
    args = parser.parse_args()

    job = Job(url=args.url, dest=args.dest)
    add_job(job)
