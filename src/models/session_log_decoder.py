""" session log json decoder """

from typing import Generic, List, TypeVar

import msgspec
from msgspec import Raw
from msgspec.json import Decoder

from models.session_log_structs import (
    ChatGptEntry,
    ChatMessage,
    SessionLogEntry,
    TimeSummaryItem,
)

SLE = TypeVar("SLE", bound=SessionLogEntry)


class SessionLogDecoder(Generic[SLE]):
    """
    decoders for session logs - compatible with current individual
    session logs, where elements are untagged. This means that parsing
    is based on data structure rather than on the tag name or key name.
    This is less performant option.

    The better option to add tag with type:
    untagged: {"role": "user", "content": "Hello"} - untagged version
    tagged: {"role": "user", "content": "Hello", "type":"MessageItem"}

    the element with key "type" is used by msgspec to parse json object
    into specified structure.

    Speed up is significant, the individual logs were created compatible with old
    session logs and requires some helper for decoding.
    This class is a helper to parse untagged json into Struct

    """

    rawDeco: Decoder[Raw] = Decoder(Raw)
    rawListDeco: Decoder[List[Raw]] = Decoder(List[Raw])
    chatMessageDeco: Decoder[ChatMessage] = Decoder(ChatMessage)
    chatGptEntryDeco: Decoder[ChatGptEntry] = Decoder(ChatGptEntry)
    timeSummaryItemDeco: Decoder[TimeSummaryItem] = Decoder(TimeSummaryItem)

    decoders: List[Decoder[SLE]] = [  # pyre-ignore[8]
        chatMessageDeco,  # type:ignore
        chatGptEntryDeco,  # type:ignore
        timeSummaryItemDeco,  # type:ignore
    ]

    @staticmethod
    def decode_raw(data: bytes) -> List[Raw]:
        """decodes to raw list"""
        return SessionLogDecoder.rawListDeco.decode(data)

    @classmethod
    def decode_session_log(
        cls,
        data: bytes,
        save_raw: bool = False,
        validation: bool = False,
    ) -> List[SessionLogEntry | Raw]:
        """decoded from raw json to structures
        params:
        data: bytes - raw json
        save_raw: Raw - most likely for tests and debug
        """
        raw_list: List[Raw] = cls.rawListDeco.decode(data)
        log: List[SessionLogEntry | Raw] = []
        for raw_item in raw_list:  # pylint:disable=E1133
            raw = True
            for deco in cls.decoders:
                try:
                    entry = deco.decode(raw_item)

                    log.append(entry)
                    raw = False
                    break
                except:  # pylint:disable=W0702
                    # not correct stuct, try another
                    pass

            if raw and save_raw:
                log.append(raw_item)

            if raw and validation:
                readable = msgspec.json.decode(raw_item)
                raise ValueError(
                    f"The session log item detection failed: {readable}",
                )

        return log

    @classmethod
    def decode_session_log_only(
        cls,
        data: bytes,
    ) -> List[SessionLogEntry]:
        """decoded from raw json to log structures, no raw entries
        params:
        data: bytes - raw json
        """
        items = SessionLogDecoder.decode_session_log(
            data,
            save_raw=False,
            validation=False,
        )
        result = []
        for item in items:
            if isinstance(item, SessionLogEntry):
                result.append(item)

        return result
