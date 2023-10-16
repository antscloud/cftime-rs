import pytest
import cftime_rs
import datetime as dt


def test_num2date():
    arr = [1, 2, 3]
    units = "days since 1970-01-01"
    calendar = "standard"

    result = cftime_rs.num2date(arr, units, calendar)
    cf_calendar = cftime_rs.PyCFCalendar.from_str("standard")
    expected = [
        cftime_rs.PyCFDatetime.from_ymd(1970, 1, 2, cf_calendar),
        cftime_rs.PyCFDatetime.from_ymd(1970, 1, 3, cf_calendar),
        cftime_rs.PyCFDatetime.from_ymd(1970, 1, 4, cf_calendar),
    ]
    assert [i.ymd_hms() for i in result] == [i.ymd_hms() for i in expected]


def test_date2num():
    units = "days since 1970-01-01"
    calendar = "standard"

    cf_calendar = cftime_rs.PyCFCalendar.from_str("standard")
    dates = [
        cftime_rs.PyCFDatetime.from_ymd(1970, 1, 2, cf_calendar),
        cftime_rs.PyCFDatetime.from_ymd(1970, 1, 3, cf_calendar),
        cftime_rs.PyCFDatetime.from_ymd(1970, 1, 4, cf_calendar),
    ]
    expected = [1, 2, 3]
    result = cftime_rs.date2num(dates, units, calendar, dtype="i64")
    assert result == expected


def test_pydate2num():
    units = "days since 1970-01-01"
    calendar = "standard"
    dates = [
        dt.datetime(1970, 1, 2),
        dt.datetime(1970, 1, 3),
        dt.datetime(1970, 1, 4),
    ]
    expected = [1, 2, 3]
    result = cftime_rs.pydate2num(dates, units, calendar, dtype="i64")
    assert result == expected


def test_num2pydate():
    arr = [1, 2, 3]
    units = "days since 1970-01-01"
    calendar = "standard"

    result = cftime_rs.num2pydate(arr, units, calendar)
    result = [i.replace(tzinfo=None) for i in result]
    expected = [
        dt.datetime(1970, 1, 2),
        dt.datetime(1970, 1, 3),
        dt.datetime(1970, 1, 4),
    ]
    assert result == expected


def test_float():
    arr = [95795.0]
    units = "days since 1970-01-01"
    calendar = "standard"
    cf_calendar = cftime_rs.PyCFCalendar.from_str("standard")
    expected = [
        cftime_rs.PyCFDatetime.from_ymd(2232, 4, 12, cf_calendar),
    ]
    result = cftime_rs.num2date(arr, units, calendar)
    assert [i.ymd_hms() for i in result] == [i.ymd_hms() for i in expected]
