# Compass Survey Format

The Compass survey data files contain all the original measurements and associated information that make up the original survey data.
They are ordinarily created by the Compass Survey Editor;
but under special circumstances,
they can be edited with an ordinary text editor as long as the file format is maintained..

**Warning!**
Unless you have a thorough understanding of your editor and ASCII character codes,
it is not advisable to directly edit Compass survey files.

Most word processor will change the format of the survey data and this will corrupt the data.
If you need to directly edit survey data, you should use
[XEDIT](https://www.fountainware.com/Products/xedit/index.htm),
a text editor that was designed to not corrupt ASCII data.
It is available on the
[Fountain home page](https://www.fountainware.com/)
or on the Compass CD-ROM.

This section describes in detail the current raw survey data format.
Listed below you will see a sample survey file.
The layout has been compressed slightly so the file will fit on the page, but all the fields are correct:

```Compass
SECRET CAVE

SURVEY NAME: A

SURVEY DATE: 7 10 79  COMMENT:Entrance Passage

SURVEY TEAM:

D.SMITH,R.BROWN,S.MURRAY

DECLINATION: 1.00  FORMAT: DDDDLUDRADLNF  CORRECTIONS: 2.00 3.00 4.00 CORRECTIONS2: 5.0 6.0

FROM TO  LENGTH BEARING  DIP    LEFT    UP  DOWN RIGHT


A2  A1   12.00  135.00   5.00  0.00  4.00  0.50  0.00  Big Room

A2  A3   41.17   46.00   2.00  0.00  0.00  0.00  0.00  #|PC# Room

A3  A4    4.25   15.00 -85.00  5.00  3.50  0.75  0.50

A4  A5   22.50  129.00 -21.00  0.00  0.00  0.00  0.00  #|PX#

<form feed>

SECRET CAVE

SURVEY NAME: B

SURVEY DATE: 7 10 79  COMMENT:Big Room Survey

SURVEY TEAM:

D.SMITH,R.BROWN,S.MURRAY

DECLINATION: 1.00  FORMAT: DDDDLUDRADLNT  CORRECTIONS: 2.00 3.00 4.00 CORRECTIONS2: 5.0 6.0


FROM TO   LEN  BEAR   INC LEFT   UP DOWN RIGHT AZM2 INC2 FLAGS COMMENTS

B2  B1  13.0  35.0  15.0 -9.9  2.0  1.5  1.0 215.0 -15.0      Side Passage

B2  B3  22.1  16.0  22.0  6.0  1.0  0.0  2.0 196.0 -22.0 #|PC#

B3  B4   3.2  11.0 -82.0  2.0  2.5  2.7  3.5 191.0  82.0

B4  B5  23.5 111.0  11.0  0.0  0.0  1.0  1.0 291.0 -11.0 #|PX#

<form feed>
```

## Individual Surveys

You will notice that the file contains data for two individual surveys.
Compass survey files can have any number of surveys within a single file.
All surveys end with a Form Feed character on a separate line.
Thus, if there are multiple surveys in a file, each survey is separated by a form feed.
A Form Feed character is the equivalent of a Control-L in some editors or 0C hex.
If you choose to edit a survey file with a text editor, make sure you understand how to enter a Form Feed and how it is displayed.
Otherwise, it is easy to accidentally delete the Forms Feeds, which will corrupt the file.

### The Header

Each survey begins with a header that gives information about the survey that will follow.
The following list describes each header item:

#### a. Cave Name (Line 1)

The first line contains the cave name. It can be any alphanumeric characters up to 80 characters in length. It is terminated by a carriage return and line feed. For the most part, the software ignores the cave name.

#### b. Survey Name (Line 2)

The survey name is usually the alphabetic prefix that is attached to each survey station.
For example, if the survey name is AB, then the individual survey stations will be AB1, AB2, AB3, etc.
The survey name field begins with the string: "SURVEY NAME: ", which is followed by the actual survey name.
The name can be any printable ASCII character and is terminated by the first white space character, usually end-of-line characters.
It can be up to 12 characters in length.

#### c. Survey Date (Line 3)

The date field begins with the string "SURVEY DATE: " and is followed by three numerical date fields: month, day and year.
The year can be a two digit or four digit number, i.e. 1992 or 92.

#### d. Survey Comment (Line 3)

For backward compatibility, this item is optional.
It is used to describe the survey in more detail than the Survey Name.
The survey comment begins with the string "COMMENT:" and is terminated by the Carriage Return at the end of the line.
The actual comment begins immediately after the colon ":" character.

#### e. Survey Team (Line 4)

The survey team fields consists of two lines.
The first line contains the string: "SURVEY TEAM:" No other information appears on this line.
The next line holds the actual survey team information.
The survey team information can be up to 100 characters in length.
There is no specific format to this information. It is up to the user.

#### f. Declination (Line 5)

The declination field gives the magnetic declination for a particular region.
It is used to compensate for local magnetic anomalies and differences between compasses.
The declination field begins with the string "DECLINATION: " followed by a floating point number.
This number is added to the azimuth of each shot in the survey.

#### g. File Format (Line 5)

For backward compatibility, this item is optional.
This field specifies the format of the original survey notebook.
Since Compass converts the file to fixed format, this information is used by programs like the editor to display and edit the data in original form.
The field begins with the string: "FORMAT: " followed by 11, 12 or 13 upper case alphabetic characters.
Each character specifies a particular part of the format. Here is list of the format items:

- *I. Bearing Units:*
  D = Degrees, Q = quads, R = Grads
- *II. Length Units:*
  D = Decimal Feet, I = Feet and Inches M = Meters
- *III. Passage Units:*
  Same as length
- *IV. Inclination Units:*
  D = Degrees, G = Percent Grade M = Degrees and Minutes, R = Grads W = Depth Gauge
- *V. Passage Dimension Order:*
  U = Up, D = Down, R = Right L = Left
- *VI. Passage Dimension Order:*
  U = Up, D = Down, R = Right L = Left
- *VII. Passage Dimension Order:*
  U = Up, D = Down, R = Right L = Left
- *VIII. Passage Dimension Order:*
  U = Up, D = Down, R = Right L = Left
- *IX. Shot Item Order:*
  L = Length, A = Azimuth, D = Inclination, a = Back Azimuth, d = Back Inclination
- *X. Shot Item Order:*
  L = Length, A = Azimuth, D = Inclination, a = Back Azimuth, d = Back Inclination
- *XI. Shot Item Order:*
  L = Length, A = Azimuth, D = Inclination, a = Back Azimuth, d = Back Inclination
- *XII. Shot Item Order:*
  L = Length, A = Azimuth, D = Inclination, a = Back Azimuth, d = Back Inclination
- *XIII. Shot Item Order:*
  L = Length, A = Azimuth, D = Inclination, a = Back Azimuth, d = Back Inclination
- *XIV. Backsight:*
  B=Redundant, N or empty=No Redundant Backsights.
- *XV. LRUD Association:*
  F=From Station, T=To Station

##### Compatibility Issues

Over time, the Compass Format string has changed to accommodate more format information.
For backward compatibility, Compass can read all previous versions of the format.
Here is detailed information about different versions of the Format strings:

- 11-Character Format
    The earliest version of the string had 11 characters like this: UUUUDDDDSSS
- 12-Character Format
    The next version had 12 characters, adding Backsight information: UUUUDDDDSSSB
- 13-Character Format
    The next version had 13 characters, adding information about the LRUD associations: UUUUDDDDSSSBL
- 15-Character Format
    Finally, the current version has 15 characters, adding backsights to order information: UUUUDDDDSSSSSBL

U = Units, D = Dimension Order, S = Shot Order, B = Backsight Info, L = LRUD association

#### h. Instrument Correction Factors (Line 5)

For backward compatibility, this item is optional.
The item begins with the string "CORRECTIONS:"
The Instrument Correction Factors are used to correct defective instrument readings.
There are three numbers that are used to correct the compass, inclinometer and tape readings respectively.
These values are added to the azimuth, inclination and length values for the survey.
The azimuth and inclination readings are in degrees and the length value is in feet.

#### i. Back Sight Instrument Correction Factors (Line 5)

For backward compatibility, this item is optional.
The item begins with the string "CORRECTIONS2:"
The Instrument Correction Factors are used to correct defective instrument readings for Back Sights.
There are two numbers that are used to correct the compass and inclinometer readings respectively.
These values are added to the back sight azimuth and inclination values for the survey.
The azimuth and inclination readings are in degrees.

1. Survey Shots. Following the header are three lines which serve to separate the header from the shots. The middle line identifies each field in the shot. Their purpose is only to make the file more readable. They are ignored by all software.

Following the separating lines is a series of shots. Each shot is contained on a single line. There are eleven possible items on the line. Some items are optional.

##### a. From Station

The first item on the line is the "from" survey station name.
It consists of up to 12 printable ASCII characters.
It is terminated by the first white space character.
It is case sensitive.
In the normal situation, the "from" station is defined as the station whose location has already been established, whereas the "to" station is the station whose location is about to be established.
In the case of a backsight, the reverse is true.

##### b. To Station

The second item on the line is the "to" survey station name.
It consists of up to 12 printable ASCII characters.
It is terminated by the first white space character.
It is case sensitive.

##### c. Length

This is the length of the shot between the from and to stations.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies length in decimal feet.

##### d. Bearing

This item specifies the compass angle of the shot.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies bearing in decimal degrees.
In the case where back sights are being used, a missing value will be specified by a value of -999.

##### e. Inclination

This is the angle of inclination of the shot.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies inclination in decimal degrees.
In the case where back sights are being used, a missing value will be specified by a value of -999.

##### f. Left

This is the distance between the station and the left wall.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies distance in decimal feet.
A negative value specifies a missing value or cave passage in that direction where the nearest wall is too far away to measure.

##### g. Up

This is the distance between the station and the ceiling.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies distance in decimal feet.
A negative value specifies a missing value or cave passage in that direction where the nearest wall is too far away to measure.

##### h. Down

This is the distance between the station and the floor.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies distance in decimal feet.
A negative value specifies a missing value or cave passage in that direction where the nearest wall is too far away to measure.

##### i. Right

This is the distance between the station and the right wall.
It is a single floating point number of virtually any format.
It is terminated by the first white space character.
It specifies distance in decimal feet.
A negative value specifies a missing value or cave passage in that direction where the nearest wall is too far away to measure.

##### j. Azm2

For backward compatibility, this is an optional item.
It is turned on or off with the File Format item in the header.
If redundant backsights are enabled, this is the backsighted azimuth value.
The second survey in the listing above has backsights enabled.
This value is always stored uncorrected, so it should be 180 degrees from the bearing.
An editor may choose to display it as a corrected backsight, in which case, it should equal the bearing.

*Note:* redundant backsights are different from ordinary backsights.
A redundant backsight consists of an extra compass and inclination reading.
This is normally done to increase the accuracy of a survey.
An ordinary backsight occurs where it is more convenient to measure a shot in reverse order.
For example, you could do an ordinary backsight when there is a rock that interferes with the "from" station.
In Compass, ordinary backsights are simply entered in reverse.
Compass is expected to notice that the shot is reversed and handle it.

##### k. Inc2

For backward compatibility, this is an optional item.
It is turned on or off with the File Format item in the header.
If redundant backsights are enabled, this is the backsighted inclination value.
The second survey in the listing above has backsights enabled.
This value is always stored uncorrected, so it should be the same value as the inclination with sign changed.
An editor may choose to display it as a corrected backsight, in which case, it should equal the inclination.

##### l. Flags

For backward compatibility, this is an optional item.
It specifies a set of flags that modify the way in which this shot is processed.
To distinguish the flag field from the comment field that follows, flags must be preceded by two characters, a pound sign and a vertical bar: "#|".
This is followed by up to three printable characters.
The flag field is terminated by a single pound sign "#" character.
At this time there are four flags that are recognized:

- L - Exclude this shot from length calculations.
- P - Exclude this shot from plotting.
- X - Exclude this shot from all processing.
- C - Do not adjust this shot when closing loops.

##### m. Comments

For backward compatibility, this field is optional.
It contains a comment pertaining to this shot.
It can be up to 80 characters in length, and it terminates at the end-of-line.

Line Length Lines in survey files may be longer than the normal computer screen width. When working with them in non-Compass editors, be sure that the editor does not wrap the lines around, or the file may be corrupted when saved.
