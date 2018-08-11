import threading
import queue
import time


class DownloadThread(threading.Thread):
    '''
    Worker to perform the actual downloading.

    This takes the job queue as an argument. The `run` method spins in a
    loop, blocking on getting a new task to perform.
    '''
    def __init__(self, job_queue):
        threading.Thread.__init__(self)
        self.daemon = True
        self.job_queue = job_queue

    def run(self):
        print(f'Starting download worker {self}')
        while True:
            next_job = self.job_queue.get(block=True)
            self.perform_work(next_job)


    def perform_work(self, job):
        '''
        Perform the actual download
        '''
        print(f'Performing work on {job}')


class Server(object):

    '''
    Server object.

    This class has the following responsibilities:

    - perform the update loop for checking the config file
    - spawn the listener thread waiting for zeromq connections
    - start the worker thread waiting for downloads
    '''

    def __init__(self, sleep_time=86400, download_configs=None):
        '''
        Construct a server object.
        '''
        self.queue = queue.Queue()
        self.sleep_time = sleep_time
        if download_configs is None:
            self.download_configs = []
        else:
            self.download_configs = download_configs
        self.download_thread = self.start_download_thread()


    def run(self):
        '''
        Sleep for the alotted time, then check all of the download
        configs.
        '''

    def add_job(self, job):
        self.queue.put(job)

    def start_download_thread(self):
        download_thread = DownloadThread(self.queue)
        download_thread.start()
        return download_thread

