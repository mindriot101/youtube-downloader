from dataclasses import dataclass


@dataclass
class Job:

    '''
    Descriptor type for a download job
    '''

    url: str
    dest: str
