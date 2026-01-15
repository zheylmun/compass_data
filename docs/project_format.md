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

#### Folders and Directories

In addition to Compass Folders, Windows has its own system of directories or Folders.
Windows Folders are a way in which Windows subdivides disk space into separate spaces.
They are used to keep files separate from one and other and they are also used to help organize information and programs.
Folders can be nested inside each other and this allows you to create hierarchical organizations.
The term Folder and Directory are interchangeable.
The word Directory originated in the DOS environment and the word Folder originates in Windows.
