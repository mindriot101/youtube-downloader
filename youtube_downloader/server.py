import threading
import queue
import time
import toml
from .download_thread import DownloadThread
from .handler_thread import HandlerThread
from .job import Job


class Server(object):

    '''
    Server object.

    This class has the following responsibilities:

    - perform the update loop for checking the config file
    - spawn the listener thread waiting for zeromq connections
    - start the worker thread waiting for downloads
    '''

    def __init__(self, config_file, sleep_time=86400):
        '''
        Construct a server object.
        '''
        self.queue = queue.Queue()
        self.config_file = config_file
        self.sleep_time = sleep_time
        self.download_thread = self.start_download_thread()
        self.handler_thread = self.start_handler_thread()
        self.download_jobs_to_enqueue = []


    def run(self):
        '''
        Sleep for the alotted time, then check all of the download
        configs.
        '''
        while True:
            self.update_configs()
            self.queue_configs()
            print('Sleeping for {self.sleep_time} seconds'.format(self=self))
            time.sleep(self.sleep_time)

    def update_configs(self):
        with open(self.config_file) as infile:
            config = toml.load(infile)

        jobs = []
        for job in config['jobs']:
            jobs.append(Job(**job))
        self.download_jobs_to_enqueue = jobs

    def queue_configs(self):
        for job in self.download_jobs_to_enqueue:
            print('Adding job {job} to the queue'.format(job=job))
            self.add_job(job)

    def add_job(self, job):
        self.queue.put(job)

    def start_download_thread(self):
        download_thread = DownloadThread(self.queue)
        download_thread.start()
        return download_thread
    
    def start_handler_thread(self):
        # TODO: pass port down
        handler_thread = HandlerThread(self)
        handler_thread.start()
        return handler_thread

