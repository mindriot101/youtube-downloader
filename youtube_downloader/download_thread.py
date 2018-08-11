import threading


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
            self.job_queue.task_done()


    def perform_work(self, job):
        '''
        Perform the actual download
        '''
        print(f'Performing work on {job}')
