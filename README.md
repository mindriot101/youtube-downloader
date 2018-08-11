# Youtube donwloader

This program supports two modes:

- a file containing all of the playlists which should be automatically
  checked and tracked, and
- a CLI accepting new downloads and adding them to the queue.

## Implementation

The download queue is implemented using a Python Queue. New download
entries should be added to the queue and the download thread will
download them. This serialises any downloads that need to happen.

A separate CLI allows for one-off downloads to be made. This still
interacts with the queue and talks over zeromq.
