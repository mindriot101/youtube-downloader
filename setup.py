from setuptools import setup, find_packages


setup(
        name='youtube-downloader',
        author='Simon Walker',
        author_email='s.r.walker101@googlemail.com',
        packages=find_packages(),
        entry_points = {
            'console_scripts': [
                'yt-server=youtube_downloader.servercli:main',
                'yt-add=youtube_downloader.addjob:main',
                ],
            },
        )
