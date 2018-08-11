import pytest
from youtube_downloader.job import Job


@pytest.fixture
def url():
    return 'https://example.com'


@pytest.fixture
def dest():
    return '/tmp'


@pytest.fixture
def job(url, dest):
    return Job(url=url, dest=dest)


class TestJob(object):
    def test_serialize(self, job):
        assert job.serialize() == '{"url": "https://example.com", "dest": "/tmp"}'

    def test_serialize_binary(self, job):
        assert job.serialize_binary() == b'{"url": "https://example.com", "dest": "/tmp"}'

    def test_deserialize(self, job):
        msg = '{"url": "https://example.com", "dest": "/tmp"}'
        assert Job.deserialize(msg) == job

    def test_serialize_binary(self, job):
        msg = b'{"url": "https://example.com", "dest": "/tmp"}'
        assert job.deserialize_binary(msg) == job

