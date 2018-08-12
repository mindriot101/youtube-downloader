import json


class Job(object):

    '''
    Descriptor type for a download job
    '''

    def __init__(self, url, dest):
        self.url = url
        self.dest = dest

    def serialize(self):
        return json.dumps(self.__dict__)

    def serialize_binary(self):
        return self.serialize().encode()

    @classmethod
    def deserialize(cls, s):
        return cls(**json.loads(s))

    @classmethod
    def deserialize_binary(cls, b):
        return cls.deserialize(b.decode())

    def __repr__(self):
        return '<Download job {self.url} => {self.dest}>'.format(
                self=self)
