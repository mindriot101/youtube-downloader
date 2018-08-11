import pytest
import os
from youtube_downloader.server import Server
from youtube_downloader.job import Job
import queue


@pytest.fixture
def config_filename():
    return os.path.join(
            os.path.basename(__file__),
            'testconfig.toml')


@pytest.fixture
def server(config_filename):
    return Server(config_file=config_filename)


class TestServer(object):
    def test_instance(self, server):
        assert isinstance(server.queue, queue.Queue)


    def test_perform_downloads(self, server):
        job = Job(url='https://example.com', dest='/tmp')
        server.add_job(job)
