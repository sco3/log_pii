""" simple de-serialization"""

from models.session_log_decoder import SessionLogDecoder
import pathlib
import msgspec
import time


def main() -> None:
    """reads json file and decodes it"""

    file = pathlib.Path(__file__).parent / "../test/log.json"

    with open(file, "rb") as f:
        buf = f.read()
        start = time.monotonic()
        for _ in range(0, 1_000_000):
            data = SessionLogDecoder.decode_session_log(buf)
            data_str = msgspec.json.encode(data)
        took = 1_000 * (time.monotonic() - start)
        print(took, "ms")


if __name__ == "__main__":

    main()
