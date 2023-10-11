# flake8: noqa

from typing import Iterable, Union, List

class PyCFDuration:
    """PyCFDuration is a wrapper around Rust CFDuration.

    All the methods depend on the Calendar definitions found in
    [udunits package](https://github.com/nco/nco/blob/master/data/udunits.dat).

    This duration can be added to a PyCFDatetime.
    The result of the subtraction between two PyCFDatetime objects gives a PyCFDuration.
    """

    @classmethod
    def from_years(cls, years: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of years and specific calendar.

        Args:
            years (int): Number of years.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_month(cls, month: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of month and specific calendar.

        Args:
            month (int): Number of month.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_weeks(cls, weeks: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of weeks and specific calendar.

        Args:
            weeks (int): Number of weeks.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_days(cls, days: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of days and specific calendar.

        Args:
            days (int): Number of days.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_hours(cls, hours: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of hours and specific calendar.

        Args:
            hours (int): Number of hours.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_minutes(cls, minutes: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of minutes and specific calendar.

        Args:
            minutes (int): Number of minutes.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_seconds(cls, seconds: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of seconds and specific calendar.

        Args:
            seconds (int): Number of seconds.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_milliseconds(cls, milliseconds: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of milliseconds and specific calendar.

        Args:
            milliseconds (int): Number of milliseconds.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_microseconds(cls, microseconds: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of microseconds and specific calendar.

        Args:
            microseconds (int): Number of microseconds.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    @classmethod
    def from_nanoseconds(cls, nanoseconds: int, calendar: PyCFCalendar) -> PyCFDuration:
        """
        Makes a new PyCFDuration with the given number of nanoseconds and specific calendar.

        Args:
            nanoseconds (int): Number of nanoseconds.
            calendar (PyCFCalendar): The calendar for the duration.

        Returns:
            PyCFDuration: A new PyCFDuration object.
        """
        ...
    def num_years(self) -> float:
        """
        Returns the total number of years in the duration.

        Returns:
            float: Number of years.
        """
        ...
    def num_months(self) -> float:
        """
        Returns the total number of months in the duration.

        Returns:
            float: Number of months.
        """
        ...
    def num_days(self) -> float:
        """
        Returns the total number of days in the duration.

        Returns:
            float: Number of days.
        """
        ...
    def num_weeks(self) -> float:
        """
        Returns the total number of weeks in the duration.

        Returns:
            float: Number of weeks.
        """
        ...
    def num_days(self) -> float:
        """
        Returns the total number of days in the duration.

        Returns:
            float: Number of days.
        """
        ...
    def num_hours(self) -> float:
        """
        Returns the total number of hours in the duration.

        Returns:
            float: Number of hours.
        """
        ...
    def num_minutes(self) -> float:
        """
        Returns the total number of minutes in the duration.

        Returns:
            float: Number of minutes.
        """
        ...
    def num_seconds(self) -> float:
        """
        Returns the total number of seconds in the duration.

        Returns:
            float: Number of seconds.
        """
        ...
    def num_milliseconds(self) -> float:
        """
        Returns the total number of milliseconds in the duration.

        Returns:
            float: Number of milliseconds.
        """
        ...
    def num_microseconds(self) -> float:
        """
        Returns the total number of microseconds in the duration.

        Returns:
            float: Number of microseconds.
        """
        ...
    def num_nanoseconds(self) -> float:
        """
        Returns the total number of nanoseconds in the duration.

        Returns:
            float: Number of nanoseconds.
        """
        ...

class PyCFCalendar:
    """PyCFCalendar represents a calendar object."""

    @staticmethod
    def from_str(s: str) -> PyCFCalendar:
        """Create a PyCFCalendar from a string representation.

        Args:
            s (str): The string representation of the calendar.

        Returns:
            PyCFCalendar: A PyCFCalendar object.

        Raises:
            PyValueError: If the calendar string cannot be parsed.
        """
        ...

class PyCFDatetime:
    """
    PyCFDatetime is a wrapper around Rust CFDatetime.

    It represents a date in a specific calendar.

    All the methods depend on the Calendar definitions found in
    [udunits package](https://github.com/nco/nco/blob/master/data/udunits.dat).
    """

    @classmethod
    def new(
        cls,
        year: int,
        month: int,
        day: int,
        hour: int,
        minute: int,
        second: float,
        calendar: PyCFCalendar,
    ) -> "PyCFDatetime":
        """
        Makes a new PyCFDatetime with given year, month, day, hour, minute, second, and specific calendar.

        Args:
            year (int): The year.
            month (int): The month.
            day (int): The day.
            hour (int): The hour.
            minute (int): The minute.
            second (float): The second.
            calendar (PyCFCalendar): The calendar for the datetime.

        Returns:
            PyCFDatetime: A new PyCFDatetime object.
        """
        ...
    def ymd(self) -> Tuple[int, int, int]:
        """
        Returns the year, month, and day of the date.

        Returns:
            Tuple[int, int, int]: A tuple of (year, month, day).
        """
        ...
    def hms(self) -> Tuple[int, int, int]:
        """
        Returns the hour, minute, and second of the time.

        Returns:
            Tuple[int, int, int]: A tuple of (hour, minute, second).
        """
        ...
    def ymd_hms(self) -> Tuple[int, int, int, int, int, int]:
        """
        Returns the year, month, day, hour, minute, and second of the datetime.

        Returns:
            Tuple[int, int, int, int, int, int]: A tuple of (year, month, day, hour, minute, second).
        """
        ...
    @classmethod
    def from_ymd_hms(
        cls,
        year: int,
        month: int,
        day: int,
        hour: int,
        minute: int,
        second: float,
        calendar: PyCFCalendar,
    ) -> "PyCFDatetime":
        """
        Makes a new PyCFDatetime with given year, month, day, hour, minute, second, and specific calendar.

        Args:
            year (int): The year.
            month (int): The month.
            day (int): The day.
            hour (int): The hour.
            minute (int): The minute.
            second (float): The second.
            calendar (PyCFCalendar): The calendar for the datetime.

        Returns:
            PyCFDatetime: A new PyCFDatetime object.
        """
        ...
    @classmethod
    def from_hms(
        cls,
        hour: int,
        minute: int,
        second: float,
        calendar: PyCFCalendar,
    ) -> "PyCFDatetime":
        """
        Makes a new PyCFDatetime with given hour, minute, second, and specific calendar.
        The year, month, and day are set to 1970-01-01.

        Args:
            hour (int): The hour.
            minute (int): The minute.
            second (float): The second.
            calendar (PyCFCalendar): The calendar for the datetime.

        Returns:
            PyCFDatetime: A new PyCFDatetime object.
        """
        ...
    @classmethod
    def from_ymd(
        cls,
        year: int,
        month: int,
        day: int,
        calendar: PyCFCalendar,
    ) -> "PyCFDatetime":
        """
        Makes a new PyCFDatetime with given year, month, day, and specific calendar.
        The hour, minute, and second are set to 0.

        Args:
            year (int): The year.
            month (int): The month.
            day (int): The day.
            calendar (PyCFCalendar): The calendar for the datetime.

        Returns:
            PyCFDatetime: A new PyCFDatetime object.
        """
        ...
    @classmethod
    def from_timestamp(
        cls,
        timestamp: int,
        nanoseconds: int,
        calendar: PyCFCalendar,
    ) -> "PyCFDatetime":
        """
        Makes a new PyCFDatetime with given timestamp, nanoseconds, and specific calendar.

        Args:
            timestamp (int): The timestamp.
            nanoseconds (int): The nanoseconds.
            calendar (PyCFCalendar): The calendar for the datetime.

        Returns:
            PyCFDatetime: A new PyCFDatetime object.
        """
        ...

def num2date(
    arr: Iterable[Union[int, float]],
    units: str,
    calendar: str,
) -> List[PyCFDatetime]:
    """Convert a list of numbers to PyCFDatetime objects based on the specified calendar.

    Args:
        arr : Iterable[Union[int, float]]
            Array of numbers to convert to PyCFDatetime
        units : str
            Valid CF units
        calendar : str
            CF calendar name. Should be one of "standard", "gregorian",
            "proleptic_gregorian", "julian", "all_leap", "no_leap", "360_day", "365_day", "366_day".
            If the calendar is not recognized, "standard" will be used

    Raises:
        ValueError
            If the date is not valid in the calendar

    Returns:
        List[PyCFDatetime]
            List of PyCFDatetime objects

    """
    ...

def date2num(
    datetimes: List[PyCFDatetime],
    units: str,
    calendar: str,
    dtype: str,
) -> Union[int, float]:
    """Convert a list of PyCFDatetime objects to a list of numbers based on calendar, units, and dtype.


    Args:
        datetimes : List[PyCFDatetime]
            List of PyCFDatetime objects
        units : str
            Valid CF units
        calendar : str
            CF calendar name. Should be one of "standard", "gregorian",
        dtype : str
            32 bit integer : "i32"
            64 bit integer : "i64", "i", "integer", "int"
            32 bit float   : "f32"
            64 bit float   :  "f64", "f", "float"

    Raises:
        ValueError
            If the date is not valid in the calendar
        ValueError
            If the dtype is not recognized

    Returns:
        Union[int, float]
            List of numbers based on calendar, units, and dtype
    """
    ...