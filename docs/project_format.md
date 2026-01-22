# Compass Project File Format

Normally, you would create Project or "MAK" files using the Project Manager.
Occasionally, someone might like to create a Project File in some other way.
For example, you might want to create a Project File using a database.
For this reason, this section of the document covers the actual internal format of the Project File.
If you are not a programmer, you can skip this section.
 
Generally speaking, a Project File is just an ASCII text file.
Thus, it is very easy to create Project Files using a text editor, word processor, database or spreadsheet.
Here is a detailed explanation of the different parts

## Project File Elements

### Base Location

Compass allows you to enter a base location for a cave or cave system.
The Base Location is used for calculating Magnetic Declination.
It allows you make Magnetic Declination calculations even when you don't have any fixed stations associated with any of the surveys.
 
Here is an example of a Base Location in a Compass MAK file:

~~~
@398315.500,4483735.300,3048.000,13,0.780;
&North American 1983;
~~~
 
#### Base Location Details
 
##### The '@' Parameter

This parameter indicates the base location of the cave.
Since the value is only used to calculate Magnetic Declination, it does not have to be the entrance or specific location in the cave.
 
Five values are specified in the command. Here is a description of each value in the order:
 
1. UTM East. The first value specifies the standard UTM-East distance in meters.
 
2. UTM North. This value specified the standard UTM-North distance in meters.
 
3. Elevation. This value specifies the elevation in meters.
 
4. Zone. This value specified the UTM zone number.
 
5. Convergence Angle. This value is the UTM convergence angle between the UTM grid and the line longitude.
 
##### The '&' Parameter

This parameter specifies the Datum used in geographic conversion operations.
The Datum is specified as a string which must match exactly one of the Datums displayed in the Compass Geographic Calculator.

### File List

Basically, a Project File just consists of a list of files that will be combined.
As an example, here is a simple project file:

~~~
#DEEP1.DAT;
#DEEP2.DAT;
#DEEP3.DAT;
~~~

As you can see, it is a list of filenames.
Each line begins with a pound sign "#" and ends with a semicolon ";".
All other lines are ignored by the program and can be used for notes or comments.
After the pound sign, a file name must appear.
This can be any file name and can include a path specification.
Spaces, tabs, carriage returns, line feeds and all "white space" characters are ignored.
Comments begin with a forward slash “/” and are terminated either by another forward slash or the end of the line.
The "make" format allows a great deal of freedom in the way you can lay out the file.
The format allows the files and linking information to be set up virtually free form.

#### Path

A path is the information that a computer needs in order to find a file on a hard drive.
Since one folder can be nested inside another, a path is a list of folders you have to go through to get to the file.
When a path is written out, it usually begins with drive letter, followed by a list of folder names, each one separated by a slash “\”. 
For example: 

~~~
C:\Cave\Compass\Lechuguilla
~~~

#### OS Folders and Directories

In addition to Compass Folders, Windows has its own system of directories or Folders.
Windows Folders are a way in which Windows subdivides disk space into separate spaces.
They are used to keep files separate from one and other and they are also used to help organize information and programs.
Folders can be nested inside each other and this allows you to create hierarchical organizations.
The term Folder and Directory are interchangeable.
The word Directory originated in the DOS environment and the word Folder originates in Windows.

### Compass Folders

Compass allows for the creation of folders within a Project File.
Folders allow you to organize cave data into separate logical sections.
For example, if you a large cave, you could put different areas of a cave into separate folders.
Likewise, if you have project that contains multiple caves, you could group caves from specific a specific area together.
Each folder can be processed and viewed  separately.
 
In a MAK file, folders are specified by angle brackets.
The beginning of a folder is specified by a left angle bracket followed by the name of the folder and ending with a Semicolon.
For example:

 ```
[Mouse Palace;
``` 

A folders ends with a right angle bracket and a semicolon:

``` 
];
```

Folder can be placed anywhere in the MAK file, however, every left angle bracket must eventually be matched with a right angle bracket.
Folders can be nested arbitrarily deep.
 
Here is an example MAK file with folders nested three levels deep.

```
[Folder-1;
#cave1.dat;
[Folder-2;
  #cave2.dat;
  [Folder-3;
     #cave3.dat;
     #cave4.dat;
  ];
  #cave5.dat;
];
#cave6.dat;
];
```

### Fixed Stations

It is sometimes useful to set the location of a survey station to a fixed set of coordinates.
For example, this is useful when you have the coordinates for the entrances of several caves and you want to tie them together into a single survey.
 
You can set the location of a station by placing the station name in the list of links for a survey.
The station name is then followed by the coordinates for the fixed location you want to use enclosed in “square brackets” “[]”.
The coordinates item begins with a “Units” command that specifies the unit that should be used for the location coordinates.
The letter F means that feet are used, the letter M means that meters are used.
This is followed by the east, north and vertical coordinates of the fixed location.
Each item in the measurement units and the coordinates can be separated by commas, spaces or any other character that cannot be interpreted as part of the Units Command or the coordinate numbers.
 
#TEST1.DAT,A1[F,10.1,20.2,30.3];
#TEST2.DAT,C1[M,1.2,2.3,3.4];
 
The example entry links together two survey files, TEST1 and TEST2.
In the first file, the station A1 is fixed to the location 10.1 east, 20.2 north and 30.3 vertical and the units are in feet.
The second file has one fixed station.
The coordinates for this fixed station are specified in meters.

### Linking


"Linking" is a technique that was necessary under DOS where memory was limited.
Under Windows, which gives programs access to large blocks of memory, linking is seldom necessary.
 
Basically, "links" are a method of connecting the two files together.
Links specify the stations in the old file that connect to the new file.
For example, if the shot B22 to CD1 connects the old file to the new file, then B22 is the link and you would use it as a link.
You can have up to 500 links between files.
The value of linking is that after the compiler has been given all the linking stations, it can forget all other stations in the old file.
This frees up a large block of memory.
 
Even though Windows generally gives you enough memory so that linking is unnecessary, there are still situations where linking is useful.
First of all, since it frees memory, it could be used to combine several large caves into a huge cave system.
Second, linked files compile slightly faster.
Finally, with linking, you can combine two caves that have duplicate survey names.
Normally, you would have to rename all conflicting stations;
but with linking, the program "forgets" all the stations in the old file so there is no conflict.
 
Link stations should be placed in the make file after the filename.
If there is more than one link station they should be separated by commas.
Here is a simple example:
 
 ```
#OLDCAVE.DAT;
#NEWCAVE.DAT,B22,C17;
```
 
In this example, B22 and C17 are linking stations between OLDCAVE and NEWCAVE. You will notice that OLDCAVE has no links. This is because it is the first file to processed, and it does not need to be connected to a previous file. You can combine links and fixed stations like this:
 
```
#TEST2.DAT,AB4,C1[M,1.2,2.3,3.4],C12;
```

If you are working with three or more files, you have to plan ahead.
This is because you may have links between the first file and the third file.
Since the program erases everything but the link stations between files, you must be sure to carry all the links from the first to the third file.
Look at the following example:

``` 
   FILE1     -    (No links)
   FILE2     -    Needs: A22 (From FILE1)
   FILE3     -    Needs: A16 (From FILE1), B14 (From FILE2)
```

FILE2 needs A22 as a link from FILE1.
FILE3 needs two links, A16 from FILE1 and B14 from FILE2.
Since FILE2 is processed before FILE3, and FILE3 needs A16 from FILE1,
you must carry A16 into FILE2 even though FILE2 doesn't need it for its own processing.
This is the way the Make file would look:

``` 
   #FILE1.DAT;                /no links
   #FILE2.DAT,A22,A16;
   #FILE3.DAT,A16,B14;
```

The following Make file for Wind Cave illustrates a complex Make file.
(Under Windows, this kind of complex Make file is no longer necessary unless your have duplicate station names.)

``` 
#WIND1.DAT;
 
#WIND2.DAT,
C41,F12,P9,C41,C40,UG30,NFP1,C39,SA'12,PP3,  /from Wind1 to Wind2
JF65,JF109,L*6,JF10,                         /from Wind1 to Wind3
KX37R,KY258R,JW1R,KY357,                     /from Wind1 to Wind5
CR1,KK32,SA9R,BX21,KK32;                     /from Wind1 to Wind4
 
#WIND3.DAT,
JF65,JF109,L*6,JF10,                    /from Wind1 to Wind3
KX37R,KY258R,JW1R,KY357,                /from Wind1 to Wind5
CR1,KK32,SA9R,BX21,KK32;                /from Wind1 to Wind4
 
#WIND4.DAT,
KX37R,KY258R,JW1R,KY357,           /from Wind1 to Wind5
CR1,KK32,SA9R,BX21,KK32,           /from Wind1 to Wind4
MP74,MP28,PC2,KY349,KY326,         /from Wind3 to Wind5
AA29,AA30,AA32,CR4,PC7,ZB1;        /from Wind3 to Wind4
 
#WIND5.DAT,
SE202,                             /from Wind4 to Wind6
KX37R,KY258R,JW1R,KY357,           /from Wind1 to Wind5
MP74,MP28,PC2,KY349,KY326,              /from Wind3 to Wind5
KK20,K29,KK26,KF14,BB35,BB37,KQ45,      /from Wind4 to Wind5
KA1,KO4,KI24,KK33,KK37,KK41,KK53,BB33,SD15;
 
#WIND6.DAT,
SE202;                             /from Wind4 to Wind6
```
