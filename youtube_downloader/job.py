from dataclasses import dataclass
import json


@dataclass
class Job:

    '''
    Descriptor type for a download job
    '''

    url: str
    dest: str

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
