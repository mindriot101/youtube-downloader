from threading import Thread
import zmq
from .job import Job


context = zmq.Context()


class HandlerThread(Thread):
    def __init__(self, server, port=5505):
        Thread.__init__(self)
        self.daemon = True
        self.server = server
        self.socket = context.socket(zmq.REP)
        self.socket.bind('tcp://*:{port}'.format(port=port))
        print('Socket listening on port {port}'.format(port=port))

    def run(self):
        while True:
            msg = self.socket.recv()
            print('Got message: {msg}'.format(msg=msg))
            self.handle_msg(msg)

    def handle_msg(self, msg):
        job = Job.deserialize_binary(msg)
        self.server.add_job(job)
        self.socket.send(b'ok')
