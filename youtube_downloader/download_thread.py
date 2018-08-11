import threading
import youtube_dl
import subprocess as sp
import os
from contextlib import contextmanager


class DownloadThread(threading.Thread):
    '''
    Worker to perform the actual downloading.

    This takes the job queue as an argument. The `run` method spins in a
    loop, blocking on getting a new task to perform.
    '''

    ARCHIVE_FILE_PATH = os.path.expanduser(
            os.path.join(
                '~', '.ytdl-archive'
                )
            )

    def __init__(self, job_queue):
        threading.Thread.__init__(self)
        self.daemon = True
        self.job_queue = job_queue

    def run(self):
        print(f'Starting download worker {self}')
        while True:
            next_job = self.job_queue.get(block=True)
            self.perform_work(next_job)
            self.job_queue.task_done()


    def perform_work(self, job):
        '''
        Perform the actual download
        '''
        output_template = self.compute_output_template(job.dest)
        cmd = [
                'youtube-dl', job.url,
                '--format', 'best',
                '--output', output_template,
                # TODO: make archive optional
                # '--download-archive', self.ARCHIVE_FILE_PATH,
                '--continue']
        sp.run(cmd)

    @staticmethod
    def compute_output_template(destination):
        return os.path.join(destination, '%(playlist_index)d-%(title)s.%(ext)s')
