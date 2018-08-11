import pytest
from youtube_downloader.server import Server
from youtube_downloader.job import Job
import queue


@pytest.fixture
def server():
    return Server()


class TestServer(object):
    def test_instance(self, server):
        assert isinstance(server.queue, queue.Queue)


    def test_perform_downloads(self, server):
        job = Job(url='https://example.com', dest='/tmp')
        server.add_job(job)

