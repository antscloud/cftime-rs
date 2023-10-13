# Installation

```
pip install cftime-rs
```

# Examples 

## Decoding to PyCfDatetimes

```python
import cftime_rs

to_decode = [0, 1, 2, 3, 4, 5]
units = "days since 2000-01-01 00:00:00"
calendar = "standard"
datetimes = cftime_rs.num2date(arr, units, calendar)
for datetime in datetimes:
    print(datetime)
```

will print :

```
2000-01-01 00:00:00.000
2000-01-02 00:00:00.000
2000-01-03 00:00:00.000
2000-01-04 00:00:00.000
2000-01-05 00:00:00.000
2000-01-06 00:00:00.000
```

## Encoding PyCFDatetimes

```python
calendar = cftime_rs.PyCFCalendar.from_str("standard")
to_encode = [
    cftime_rs.PyCFDatetime.from_ymd(2000, 1, 1, calendar),
    cftime_rs.PyCFDatetime.from_ymd(2000, 1, 2, calendar),
    cftime_rs.PyCFDatetime.from_ymd(2000, 1, 3, calendar),
    cftime_rs.PyCFDatetime.from_ymd(2000, 1, 4, calendar),
    cftime_rs.PyCFDatetime.from_ymd(2000, 1, 5, calendar),
    cftime_rs.PyCFDatetime.from_ymd(2000, 1, 6, calendar),
]
units = "days since 2000-01-01 00:00:00"
calendar = "standard"
numbers = cftime_rs.date2num(to_encode, units, calendar, dtype="int")
for number in numbers:
    print(number)
```

will print :

```
0
1
2
3
4
5
```

## Decoding to Python datetimes

```python
to_decode = [0, 1, 2, 3, 4, 5]
units = "days since 2000-01-01 00:00:00"
calendar = "standard"
datetimes = cftime_rs.num2pydate(to_decode, units, calendar)
for datetime in datetimes:
    print(datetime)
```
will print 

```
2000-01-01 01:00:00
2000-01-02 01:00:00
2000-01-03 01:00:00
2000-01-04 01:00:00
2000-01-05 01:00:00
2000-01-06 01:00:00
```

## Decoding Python datetimes

```python
to_encode = [
    dt.datetime(2000, 1, 1),
    dt.datetime(2000, 1, 2),
    dt.datetime(2000, 1, 3),
    dt.datetime(2000, 1, 4),
    dt.datetime(2000, 1, 5),
    dt.datetime(2000, 1, 6),
]
units = "days since 2000-01-01 00:00:00"
calendar = "standard"
numbers = cftime_rs.pydate2num(to_encode, units, calendar, dtype="int")
for number in numbers:
    print(number)
```

will print 

```
0
1
2
3
4
5
```