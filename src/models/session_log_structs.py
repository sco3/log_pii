""" model for session log, for serialization and parsing """

from typing import List, Optional, Union

import msgspec
from msgspec import Struct


class SessionLogEntry(
    Struct, omit_defaults=True
):  # pylint:disable=(too-few-public-methods)
    """base class for elements"""


class TotalFunctionTimes(SessionLogEntry):  # pylint:disable=(too-few-public-methods)
    """timings structure"""

    timestamp: float
    retrieve_state: float
    get_rules: float
    get_groups: float
    get_devices: float
    get_response: float


class TimeSummary(Struct):  # pylint:disable=(too-few-public-methods)
    """timing structure"""

    total_function_times: TotalFunctionTimes
    total_request_time: float


class Property(Struct):  # pylint:disable=(too-few-public-methods)
    """gpt function property"""

    type: str
    description: str


class Parameters(
    Struct,
    omit_defaults=True,
):  # pylint:disable=(too-few-public-methods)
    """input schema"""

    type: Optional[str] = None
    properties: Optional[dict[str, Property]] = None
    required: Optional[List[str]] = None


class Function(
    Struct,
    omit_defaults=True,
):  # pylint:disable=(too-few-public-methods)
    """function struct"""

    name: str
    description: Optional[str] = None
    parameters: Optional[Parameters] = None
    arguments: Optional[str] = None


class ToolCall(Struct):  # pylint:disable=(too-few-public-methods)
    """tool call struct"""

    index: int
    function: Function
    id: str
    type: str


class ChatMessage(SessionLogEntry):  # pylint:disable=(too-few-public-methods)
    """chat message element"""

    role: str
    content: str
    tool_calls: Optional[list[ToolCall]] = None


class TimeSummaryItem(SessionLogEntry):  # pylint:disable=(too-few-public-methods)
    """time summary element"""

    time_summary_s: TimeSummary

    def __repr__(self) -> str:
        """render to string"""
        json_bytes = msgspec.json.encode(self)
        formatted = msgspec.json.format(json_bytes, indent=2)
        return formatted.decode("utf-8")

    def __str__(self) -> str:
        """render to string"""
        total = self.time_summary_s.total_request_time
        return f"Total: {total} s ..."


class Request(SessionLogEntry):  # pylint:disable=(too-few-public-methods)
    """gpt request"""

    chat_history: List[ChatMessage]
    functions: List[Function]
    model: str
    gpt_client: str


class Response(Struct):  # pylint:disable=(too-few-public-methods)
    """gpt response"""

    agent: Optional[str] = None
    message: Optional[ChatMessage] = None


class ContentItem(Struct):  # pylint:disable=(too-few-public-methods)
    """content item with type"""

    text: str
    type: str


class ChatHistoryItem(Struct):  # pylint:disable=(too-few-public-methods)
    """chat history item"""

    role: str
    content: Union[str, List[ContentItem]]
    tool_calls: Optional[List[ToolCall]] = None
    tool_call_id: Optional[str] = None


class ChatGpt(Struct):  # pylint:disable=(too-few-public-methods)
    """chat gpt entry"""

    request: Request
    response: Response
    time: float


class ChatGptEntry(SessionLogEntry):  # pylint:disable=(too-few-public-methods)
    """chat gpt entry"""

    chat_gpt: ChatGpt


# root element for session log
SessionLogType = List[Union[ChatMessage, TimeSummaryItem]]
