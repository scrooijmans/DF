Title: Compile Time Commands

Previous | Contents | Next

*   **Chapter 5:** Compile Time Commands

*   Compiler Utility Commands

*   !include
*   !addincludedir
*   !addplugindir
*   !appendfile
*   !cd
*   !delfile
*   !echo
*   !error
*   !assert
*   !execute
*   !makensis
*   !packhdr
*   !finalize
*   !uninstfinalize
*   !system
*   !tempfile
*   !getdllversion
*   !gettlbversion
*   !warning
*   !pragma
*   !verbose

*   Predefines

*   ${\_\_COUNTER\_\_}
*   ${\_\_FILE\_\_}
*   ${\_\_FILEDIR\_\_}
*   ${\_\_LINE\_\_}
*   ${\_\_DATE\_\_}
*   ${\_\_TIME\_\_}
*   ${\_\_TIMESTAMP\_\_}
*   ${NSIS\_VERSION}
*   ${NSIS\_PACKEDVERSION}
*   ${NSIS\_CHAR\_SIZE}
*   ${NSIS\_PTR\_SIZE}
*   ${U+1}...${U+10FFFF}
*   Scope Predefines

*   Read environment variables

*   $%envVarName%

*   Conditional Compilation

*   !define
*   !undef
*   !ifdef
*   !ifndef
*   !if
*   !ifmacrodef
*   !ifmacrondef
*   !else
*   !endif
*   !insertmacro
*   !macro
*   !macroend
*   !macroundef
*   !searchparse
*   !searchreplace

Chapter 5: Compile Time Commands
================================

5.1 Compiler Utility Commands
-----------------------------

These commands are similar to the C preprocessor in terms of purpose and functionality. They allow file inclusion, conditional compilation, executable header packing and process execution during the build process. Note: None of these commands allow the use of variables.

Number literals support the `0b`, `0o`, `0n` and `0x` radix prefixes (base 2, 8, 10 and 16 respectively). Note: The deprecated plain `0` octal prefix is also supported in some places but its usage is discouraged.

### 5.1.1 !include

\[/NONFATAL\] \[/CHARSET=ACP|OEM|CP#|UTF8|UTF16LE|UTF16BE\] file

This command will include 'file' as if it was part of the original script. Note that if a file is included in another directory, the current directory is still where the script was compiled from (not where the included file resides). If the compiler can't find the file it will look for it in every include directory. See !addincludedir for more information. If the /nonfatal switch is used and no files are found, a warning will be issued instead of an error. /charset can be used to specify a codepage for plain text files without a BOM.

!include WinMessages.nsh
!include Library.nsh
!include /CHARSET=CP1252 C:\\MyConfig.nsi
!include ..\\MyConfig.nsh
!include /NONFATAL file\_that\_may\_exist\_or\_not.nsh

### 5.1.2 !addincludedir

directory

Adds another include directory to the include directories list. This list is searched when !include is used. This list's initial value is ${NSISDIR}\\Include.

!addincludedir ..\\include
!include something.nsh

### 5.1.3 !addplugindir

\[/x86-ansi | /x86-unicode\] directory

Causes the NSIS compiler to scan the given directory for plug-in DLLs. If you don't specify the plug-in architecture it is assumed to match the current target architecture. If the architecture does not match the installer will probably crash!

!addplugindir ..\\myplugin
MyPlugin::SomeFunction

### 5.1.4 !appendfile

\[/CHARSET=ACP|OEM|CP#|UTF8\[SIG\]|UTF16<LE|BE>\[BOM\]\] \[/RawNL\] file text

Appends _text_ to _file_. The text is written as ANSI (ACP) unless the file already has a BOM. Using /CHARSET will force a specific character encoding. `$\n` will be translated to `$\r$\n` on Windows unless you specify /RawNL.

!tempfile FILE
!appendfile "${FILE}" "XPStyle on$\\n"
!appendfile "${FILE}" "Name 'test'$\\n"
!include "${FILE}"
!delfile "${FILE}"
!undef FILE

### 5.1.5 !cd

new\_path

This command will change the compiler to the new directory, new\_path. new\_path can be relative or absolute.

!cd ..\\more-scripts\\new

### 5.1.6 !delfile

\[/nonfatal\] file

This command deletes a file.

!tempfile FILE
!delfile "${FILE}"
!undef FILE

### 5.1.7 !echo

message

This command will echo a message to the user compiling the script.

!echo "hello world"

### 5.1.8 !error

\[message\]

This command will issue an error to the script compiler and will stop execution of the script. You can also add a message to this error.

!ifdef VERSION & NOVERSION
!error "both VERSION and NOVERSION are defined"
!endif

### 5.1.9 !assert

value \[op value2\] message

This command will stop the compiler if the expression is not true. The expression is evaluated in a similar fashion to !if.

!assert ${NSIS\_CHAR\_SIZE} = 2 "Unicode required"

### 5.1.10 !execute

command \[compare comparevalue | symbol\]

This command will execute 'command' using a call to CreateProcess(). Unlike !system, it does not use the command line processor, so input/output redirection and commands like 'cd', 'dir' and 'type' can not be used. Currently, the only known advantage of !execute over !system is that it does not give trouble when the current working directory is specified using UNC.

On POSIX platforms, !execute will use system() just like !system.

!execute '"$%WINDIR%\\notepad.exe" /P "${NSISDIR}\\COPYING"'

### 5.1.11 !makensis

parameters \[compare comparevalue | symbol\]

This command will !execute a new instance of MakeNSIS with the parameters you specify.

!makensis '-DGENERATEUNINST "${\_\_FILE\_\_}"' = 0
!system '"signtool" sign ...' = 0

### 5.1.12 !packhdr

tempfile command

This option makes the compiler use an external EXE packer (such as Petite or UPX) to compress the executable header. Specify a temporary file name (such as "temp.dat") and a command line (such as "C:\\program files\\upx\\upx -9 temp.dat") to compress the header.

!packhdr "$%TEMP%\\exehead.tmp" '"C:\\Program Files\\UPX\\upx.exe" "$%TEMP%\\exehead.tmp"'

### 5.1.13 !finalize

command \[compare comparevalue\]

This option will execute 'command' using a call to system() after the installer EXE has been generated. You can typically use it to sign (Authenticode) your installer. If 'command' contains a '%1' it will be replaced by the executables filename.

!finalize 'sign.bat "%1" "MyProduct Installer" http://example.com'

### 5.1.14 !uninstfinalize

command \[compare comparevalue\]

This option will execute 'command' using a call to system() after the uninstaller EXE has been generated. You can typically use it to sign (Authenticode) your uninstaller. If 'command' contains a '%1' it will be replaced by the executables filename.

!uninstfinalize 'sign.bat "%1" "MyProduct Installer" http://example.com'

### 5.1.15 !system

command \[compare comparevalue | symbol\]

This command will execute 'command' using a call to system(). You can store the return value in a define ('symbol') or halt execution if the return value compared (using 'compare') to 'comparevalue' is false. 'compare' can be '<' or '>' or '<>' or '='.

!system '"%WINDIR%\\notepad.exe" "${NSISDIR}\\COPYING"'
!system 'echo !define something > newinclude.nsh'
!include newinclude.nsh
!ifdef something
!echo "something is defined"
!endif
!system 'attrib +H Secret.txt' = 0
!system 'ping localhost' ERRLVL
!echo "Ping returned ${ERRLVL}"

### 5.1.16 !tempfile

symbol

This command creates a temporary file. It puts its path into a define, named _symbol_.

!tempfile PACKHDRTEMP
!packhdr "${PACKHDRTEMP}" '"C:\\Program Files\\UPX\\upx.exe" "${PACKHDRTEMP}"'

!tempfile FILE
!define /date DATE "%H:%M:%S %d %b, %Y"
!system 'echo built on ${DATE} > "${FILE}"'
!undef DATE
File /oname=build.txt "${FILE}"
!delfile "${FILE}"
!undef FILE

### 5.1.17 !getdllversion

\[/noerrors\] \[/packed\] \[/productversion\] localfilename define\_basename

This is similar to GetDLLVersionLocal, only it stores the version number in defines and can therefore be used anywhere, not just inside functions and sections. /packed returns the information in two DWORDs.

!getdllversion "$%WINDIR%\\Explorer.exe" Expv\_
!echo "Explorer.exe version is ${Expv\_1}.${Expv\_2}.${Expv\_3}.${Expv\_4}"

### 5.1.18 !gettlbversion

\[/noerrors\] \[/packed\] localfilename define\_basename

Get the version information from a .TLB file.

!gettlbversion /packed "$%WINDIR%\\System32\\stdole32.tlb" TLBVER\_
!echo "${TLBVER\_HIGH}.${TLBVER\_LOW}"

### 5.1.19 !warning

\[message\]

This command will issue a warning to the script compiler. You can also add a message to this warning.

!ifdef USE\_DANGEROUS\_STUFF
!warning "using dangerous stuff"
!endif

### 5.1.20 !pragma

warning <enable|disable|default|error|warning> <code|all>
warning <push|pop>

The pragma commands allows you to change compiler features and behavior.

!pragma warning disable 9000 ; Disable warning about using "Setup.exe" as the name
OutFile "Setup.exe"

### 5.1.21 !verbose

level | push | pop

This command will set the level of verbosity: 4=all, 3=no script, 2=no info, 1=no warnings, 0=none.

Passing push will cause !verbose to push the current verbosity level on a special stack. Passing pop will cause !verbose to pop the current verbosity level from the same stack and use it.

!verbose push
!verbose 1
!include WinMessages.nsh
!verbose pop

5.2 Predefines
--------------

You can use these standard predefines to automatically add the build time to the title of development versions, add the date to the version number, etc.

### 5.2.1 ${\_\_COUNTER\_\_}

Expands to a number (Starting at 0 and incrementing by 1 every time it is used)

### 5.2.2 ${\_\_FILE\_\_}

Current script name.

### 5.2.3 ${\_\_FILEDIR\_\_}

Current script directory.

### 5.2.4 ${\_\_LINE\_\_}

Current line number.

### 5.2.5 ${\_\_DATE\_\_}

Date when the script started compiling according to the current locale.

### 5.2.6 ${\_\_TIME\_\_}

Time when the script started compiling according to the current locale.

### 5.2.7 ${\_\_TIMESTAMP\_\_}

Date & time of the last modification to the script file according to the current locale.

### 5.2.8 ${NSIS\_VERSION}

NSIS version used to build the script.

### 5.2.9 ${NSIS\_PACKEDVERSION}

NSIS version as a 32-bit number.

!if 0x3014000 >= "${NSIS\_PACKEDVERSION}"
!error "NSIS 3.15 or higher is required to build this installer!"
!endif

### 5.2.10 ${NSIS\_CHAR\_SIZE}

The size of a character code unit (in bytes). 1 in ANSI installers and 2 in Unicode installers.

A grapheme cluster consists of a base character plus optional combining characters and diacritics and is defined as one or more code points. One or more code units is required to encode a single code point.

### 5.2.11 ${NSIS\_PTR\_SIZE}

The size of a pointer (in bytes) in the generated installer.

### 5.2.12 ${U+1}...${U+10FFFF}

A Unicode (UCS-4) character.

DetailPrint "${U+2115}SIS" # DOUBLE-STRUCK CAPITAL N + "SIS"

### 5.2.13 Scope Predefines

Standard predefines that contain information about the current code scope.

#### 5.2.13.1 ${\_\_GLOBAL\_\_}

Defined in the global scope.

Section test
!ifdef \_\_GLOBAL\_\_
!error "this shouldn't be here!"
!endif
SectionEnd

PageEx instfiles
!ifdef \_\_GLOBAL\_\_
!error "this shouldn't be here!"
!endif
PageExEnd

#### 5.2.13.2 ${\_\_SECTION\_\_}

Defined as the section name, without any prefixes, in section scope.

!ifdef \_\_SECTION\_\_
!error "this shouldn't be here!"
!endif

Section test
!ifndef \_\_SECTION\_\_
!error "missing predefine!"
!endif

!if ${\_\_SECTION\_\_} != test
!error "wrong predefine value!"
!endif
SectionEnd

Section !test
!if ${\_\_SECTION\_\_} != test
!error "wrong predefine value!"
!endif
SectionEnd

Section un.test
!if ${\_\_SECTION\_\_} != test
!error "wrong predefine value!"
!endif
SectionEnd

#### 5.2.13.3 ${\_\_FUNCTION\_\_}

Defined as the function name, without any prefixes, in function scope.

!ifdef \_\_FUNCTION\_\_
!error "this shouldn't be here!"
!endif

Function test
!ifndef \_\_FUNCTION\_\_
!error "missing predefine!"
!endif

!if ${\_\_FUNCTION\_\_} != test
!error "wrong predefine value!"
!endif
FunctionEnd

Function un.test
!if ${\_\_FUNCTION\_\_} != test
!error "wrong predefine value!"
!endif
FunctionEnd

#### 5.2.13.4 ${\_\_PAGEEX\_\_}

Defined as the page type in PageEx scope.

!ifdef \_\_PAGEEX\_\_
!error "this shouldn't be here!"
!endif

PageEx instfiles
!ifndef \_\_PAGEEX\_\_
!error "missing predefine!"
!endif

!if ${\_\_PAGEEX\_\_} != instfiles
!error "wrong page type"
!endif
PageExEnd

#### 5.2.13.5 ${\_\_UNINSTALL\_\_}

Defined in section, function or PageEx scopes of the uninstaller.

!ifdef \_\_UNINSTALL\_\_
!error "this shouldn't be here!"
!endif

Function test
!ifdef \_\_UNINSTALL\_\_
!error "this shouldn't be here!"
!endif
FunctionEnd

Function un.test
!ifndef \_\_UNINSTALL\_\_
!error "missing predefine!"
!endif
FunctionEnd

#### 5.2.13.6 ${\_\_MACRO\_\_}

Defined as the name of the current macro.

5.3 Read environment variables
------------------------------

### 5.3.1 $%envVarName%

$%envVarName% will be replaced at compile time by the environment variable envVarName.

5.4 Conditional Compilation
---------------------------

The compiler maintains a list of defined symbols, which can be defined using !define or the /D command line switch. These defined symbols can be used for conditional compilation (using !ifdef) or for symbol replacement (a simple form of macros). To replace a symbol with its value, use ${SYMBOL} (if SYMBOL is not defined, no translation will occur). The translation is first-come-first-served, meaning if you do:

!define symbol\_one ${symbol\_two}

If symbol\_two is defined when that line occurs, it will be replaced. Otherwise, any replacing will occur when ${symbol\_one} is referenced.

Define/conditional compilation related commands:

### 5.4.1 !define

\[/ifndef | /redef\](\[/date|/utcdate\]gflag\[value\]) | (/file gflag filename.txt) | (/intfmt gflag fmtstr value) | (/math gflag val1 OP val2)

This command will add _gflag_ to the global define list. This will have a similar effect as using the /D switch on the command line (the define only becomes effective after the !define command).

If _/date_ or _/utcdate_ are used, _value_ will be passed to strftime() and the result will be used as the value of _gflag_. strftime converts special symbols into certain parts of the current time or date. For example, %H will be converted into the current hour in 24-hour format. For a complete list of available symbols, search for strftime on MSDN. On POSIX, you can get the list by using `man strftime`.

If _/file_ is used, the entire text file specified (including whitespace and newlines) will be read and placed into _gflag_.

If _/intfmt_ is used, _value_ is interpreted as a integer and formatted using the same syntax as IntFmt.

If _/math_ is used, the result of 'val1 OP val2', where OP may be +,-,\*,/,%,<<,>>,>>>,&,|,^,~,!,&& or ||, will be used as the value of _gflag_. Note that val1 AND val2 MUST be integer values!

!define USE\_SOMETHING
!define VERSION 1.2
!define /date NOW "%H:%M:%S %d %b, %Y"
!define /math RESULT 3 + 10
!define /math REST 15 % ${RESULT}
!define /file BunchaStuff somesourcefile.cpp
!define /redef USE\_SOMETHING ${RESULT} ;redefine USE\_SOMETHING
!define /intfmt HEX "0x%X" 3133078222

### 5.4.2 !undef

\[/noerrors\] gflag \[...\]

Removes an item from the global define list. Note that ${SYMBOL} where SYMBOL is undefined will be translated to "${SYMBOL}".

!define SOMETHING
!undef SOMETHING

### 5.4.3 !ifdef

gflag \[bcheck gflag \[...\]\]

This command, when paired with an !endif command, will tell the compiler whether or not to compile the lines in between the two lines. If gflag is globally defined (using !define or the /D switch), then the contained lines will be compiled. Otherwise, they will be skipped. 'bcheck' can be specified as & (boolean and) or | (boolean or) along with more gflags -- precedence is simple, left to right.

!define SOMETHING
!ifdef SOMETHING
!echo "SOMETHING is defined"
!endif
!undef SOMETHING
!ifdef SOMETHING
!echo "SOMETHING is defined" # will never be printed
!endif

### 5.4.4 !ifndef

gflag \[bcheck gflag \[...\]\]\]

The opposite of !ifdef. The lines will be compiled when the gflag has not been defined.

### 5.4.5 !if

\[!\] value \[op value2\]
\[!\] /FileExists "c:\\path\\file.exe"

This command, when paired with an !endif command, will tell the compiler whether or not to compile the lines in between the two lines. If value is non-zero, or the comparison of value and value2 depending on the operator results in true, the contained lines will be compiled. Otherwise, they will be skipped. op can be either == or != (case-insensitive string comparison), S== or S!= (case-sensitive string comparison), =, <>, <=, <, > or >= (int/hex/float comparison), & (bitwise AND comparison), && or || (boolean comparison). If \[!\] is set, the result will be flipped from true to false and vice versa.

!if 1 < 0x2
!echo "1 is smaller than 2!!"
!else if ! 3.1 > 1.99
!error "this line should never appear"
!else
!error "neither should this"
!endif

!if /FileExists ".\\cert.pfx"
!finalize '".\\sign.bat" "%1"'
!endif

### 5.4.6 !ifmacrodef

gflag \[bcheck gflag \[...\]\]\]

This command, when paired with an !endif command, will tell the compiler whether or not to compile the lines in between the two lines. If the macro gflag exists, then the contained lines will be compiled. Otherwise, they will be skipped. 'bcheck' can be specified as & (boolean and) or | (boolean or) along with more gflags -- precedence is simple, left to right.

!macro SomeMacro
!macroend
!ifmacrodef SomeMacro
!echo "SomeMacro is defined"
!endif

### 5.4.7 !ifmacrondef

gflag \[bcheck gflag \[...\]\]\]

The opposite of !ifmacrodef. The lines will be compiled when the macro gflag does not exist.

### 5.4.8 !else

\[if|ifdef|ifndef|ifmacrodef|ifmacrondef \[...\]\]

This command allows to easily insert different code when different defines or macros are set. You can create blocks like !ifdef/!else/!endif, !ifdef/!else ifdef/!else/!endif etc.

!ifdef VERSION
OutFile installer-${VERSION}.exe
!else
OutFile installer.exe
!endif

### 5.4.9 !endif

This command closes a block started with !if, !ifdef, !ifndef, !ifmacrodef or !ifmacrondef.

### 5.4.10 !insertmacro

macro\_name \[parameter\] \[...\]

Inserts the contents of a macro that was created with !macro. If the macro was created with parameters, then you must pass as many parameters to the macro as it requires.

!macro Print text
DetailPrint "${text}"
!macroend
!insertmacro Print "some text"
!insertmacro Print "some more text"

### 5.4.11 !macro

macro\_name \[parameter\]\[...\]

Creates a macro named 'macro\_name'. All lines between the !macro and the !macroend will be saved. To insert the macro later on, use !insertmacro. !macro definitions can have one or more parameters defined. The parameters may be accessed the same way a !define would (e.g. ${PARMNAME}) from inside the macro.

!macro SomeMacro parm1 parm2 parm3
DetailPrint "${parm1}"
MessageBox MB\_OK "${parm2}"
File "${parm3}"
!macroend

### 5.4.12 !macroend

Ends a macro that was started with !macro.

### 5.4.13 !macroundef

macro\_name

Deletes a macro.

### 5.4.14 !searchparse

\[/ignorecase\] \[/noerrors\] \[/file\] source\_string\_or\_file substring\_start OUTPUTSYMBOL1 \[substring \[OUTPUTSYMBOL2 \[substring ...\]\]\]

Parses _source\_string\_or\_file_ (which is treated as a string, or as a filename if _/file_ is set), looking for _substring\_start_. If _substring\_start_ is found, then _OUTPUTSYMBOL1_ is defined to the rest of the string (minus any other _substring_ that may be found). Any number of _OUTPUTSYMBOLx_ may be specified, and the final _substring_ is optional.

If _/noerrors_ is specified, matching less than the full number of strings is allowed (all _OUTPUTSYMBOLx_ after the not-found substring will be ignored).

If _/file_ is specified, the file is treated as a series of lines. The file is searched until all substrings are matched. If _/noerrors_ is specified and not all strings are matched, the first line with the most symbols matched is used.

\# search filename.cpp for a line '#define APP\_VERSION "2.5"' and set ${VER\_MAJOR} to 2, ${VER\_MINOR} to 5.
!searchparse /file filename.cpp \`#define APP\_VERSION "\` VER\_MAJOR \`.\` VER\_MINOR \`"\`

### 5.4.15 !searchreplace

\[/ignorecase\] symbol\_out source\_string searchfor replacewith

Searches _source\_string_, looking for _searchfor_ and replacing all instances of it with _replacewith_. Unlike !define, !searchreplace allows you to redefine _symbol\_out_ without warning or error.

\# defines ${blah} to "i like ponies"
!searchreplace blah "i love ponies" "love" "like"

Previous | Contents | Next

Description: Detect script path at compile time

Title: Detect script path at compile time - NSIS Forums

2.  NSIS Discussion
3.  Detect script path at compile time

Archive: Detect script path at compile time

e-novative  
29th June 2004 22:18 UTC

**Detect script path at compile time**  
Hi folks,  

I was having some trouble excluding .svn version control subdirectories from getting packages with my application, but thanks to rsegal I could solve the problem.  

One question remains: I need the full absolute path to the NSIS script (resp. to the directories I choose to wrap up into the installer). Since my source is tucked away in a version control database, the actual path where the NSIS script is compiled from is not fixed, but differs from time to time (and PC to PC).  

Is there a way of detecting the script directory at \_compile\_ time? I am thinking of something like ${\_\_DIR\_\_}, similar to ${\_\_FILE\_\_}? I was not able to find any solution in the documentation or by googling around.  

Kind regards,  

Steve

iceman\_k  
1st July 2004 19:24 UTC

There is a macro in the archive which you can use to extract the directory from a file name. Use that in conjunction with ${\_\_FILE\_\_} to get what you need.

deguix  
2nd July 2004 03:04 UTC

The only way to change Compile Time strings is to create another program and make this second program run the first program with the information you want to change in the command line as defines.

iceman\_k  
2nd July 2004 15:12 UTC

Oops, you're right- I missed the compile time reference.  
To the original poster- why do you need absolute paths. If you write your NSI with relative paths, you should be OK.  
I am doing the same thing as you, i.e., my NSI file is in a version control system and the absolute path may vary from PC to PC.  
However, since my scripts use relative paths and the version control relative directory structure is the same on every PC, I experience no problems compiling on different PCs.  
If you really want the ${\_\_DIR\_\_} macro, a couple of options are to:  
a) Modify the makensis source code and recompile, or,  
b) Create a batch file in the same folder as the NSI file. In the batch file generate an NSH file which defines the ${\_\_DIR\_\_} macro for you and then include the NSH file in your main NSI. You can invoke the batch file using the !system command.

e-novative  
3rd July 2004 11:14 UTC

Thanks for your help, folks. The relative paths I required because I use rsegal's FolderList to exclude .svn version control files from being packaged into the installer and being deployed to the target system. Originally, rsegal told me that I must use absolute paths to specify the exclude directories, but (lucky my) I tried with relative paths and they - unexpectedly - worked, too.  

It's good to know a way of doing at, though, in case I should need it in the future. Thanks again for your help.  

Regards,  

Steve

Fork me on GitHub

Description: Variable Source Path

Title: Variable Source Path - NSIS Forums

2.  NSIS Discussion
3.  Variable Source Path

Archive: Variable Source Path

langdon  
15th November 2011 21:16 UTC

**Variable Source Path**  
  I'm trying to get my NSIS script able to be compiled on two different machines with different directory structures. I thought I could simply:

```

Path
>Function .OnInit
StrCpy $Path "c:\test.txt"
>FunctionEnd
Section
File "${Path}"
>SectionEnd 
>
```

...but obviously variables are for run-time use only. What I need is some sort of compile-time constant. I was hoping to be able to create an NSH file on my machine that sets the source path to "c:\\source\_files", and the build machine could have the same NSH file, but point it do d:\\project\\latest". Or maybe even something simpler?  

Any ideas?  

TIA!

Anders  
15th November 2011 22:12 UTC

mypath.nsh:  
!define thepath "c:\\foo\\bar"  

install.nsi:  
!include "mypath.nsh"  
...  
File "${thepath}\\somefile.exe"

MSG  
16th November 2011 08:25 UTC

No real need to use an nsh for that, of course. Just put the define at the top of the script.

langdon  
16th November 2011 15:24 UTC

Thanks Anders!  

@MSG, putting the !define at the top of the script won't allow that same script to run on multiple environments.

demiller9  
16th November 2011 17:03 UTC

You can use relative paths in the define, or directly in the FILE commands. I do this to work at home and at the office, where my NSIS folders are on different drives

```
!define Addons "\NSIS Installers\Addons"!AddPluginDir "${Addons}\Plugins"!AddIncludeDir "${Addons}\Include"...!define SRCDIR ".\Ipls.3177"File "${SRCDIR}\ArchHelpUtil*.ocx"
```

Fork me on GitHub
Description: !system call during compile time

Title: !system call during compile time - NSIS Forums

2.  NSIS Discussion
3.  !system call during compile time

Archive: !system call during compile time

PatrickFraley  
4th April 2005 10:09 UTC

**!system call during compile time**  
Hi All,  

I am making a !system call during compile time to start a batch file. In this batch file I have 2 commands, the first synchronises the source directory agains a given release using a tool called directory compare. this part works perfect. the sekond step is to copy in any customer specific files using xcopy and this command gets ignored. if i open a command shell in windows and call the batch file all works great. if i call it from nsis the xcopy command does not seem to execute (even though the interpreter says it is executing it).  

please help  

tia  
patrick

flizebogen  
4th April 2005 11:58 UTC

how do you call it?  

As xcopy is an internal command of cmd you need to start cmd and give the xcopy as a parameter  

Maybe you can share your code

PatrickFraley  
4th April 2005 12:54 UTC

**code**  
Hi,  

actually I do not call xcopy directly, I call the following batch file, which in turn calls xcopy. xcopy is cmd internal? I have it laying under: C:\\WINDOWS\\SYSTEM32\\xcopy.exe  

NSIS call:  

!system '..\\common\\scripte\\copyRelease.bat'  

oder:  

!system 'cmd /c ..\\common\\scripte\\copyRelease.bat'  

beides lässt den xcopy befehl aus :(  

tia  
patrick  

copyRelease.bat:  

rem @echo off  

"M:\\mM4Setups\\current\\NSIS Source\\common\\programme\\DIRCMP" /s M:\\mediMACH\\Releases\\mediMACH4-%1 /t M:\\mM4Setups\\current\\mM4Source\\%2\\mediMACH4 /m /q /w  

xcopy.exe "M:\\mediMACH\\Kundenversionen\\mediMACH4\\\*.\*" "M:\\mM4Setups\\current\\mM4Source\\mediMACH4\\" /E /Y

flizebogen  
4th April 2005 13:41 UTC

oops... i always thought xcopy is an internal command  

i tested the following and i worked:  

!system 'xcopy /s /e c:\\test\_ordner d:\\3'

PatrickFraley  
4th April 2005 14:08 UTC

I tried calling it the way you did, but no luck.  

do the files realy get copied? the compiler output say's !system returned 0, but actually nothing happened.  

this is really odd. like I said when I call the batchfile from hand everything works fine, but when I call it from nsis during compiletime it never executes the xcopy command.

flizebogen  
4th April 2005 14:16 UTC

yes i see a list of files being copied. Is M:\\ a drive mapping? If yes test a xcopy with local paths

PatrickFraley  
4th April 2005 14:31 UTC

yes m: is a drive mapping. i tried using just c: and it did not work either. it must have something to do with the way nsis executes shell commands. maybe it sees the xcopy command and then does not execute it, since it is a special command for nsis (doesn't it use xcopy to copy files onto a target system?)

Comm@nder21  
4th April 2005 15:38 UTC

try to run your batch file manually, i don't think, your xcopy-syntax is correct.  

as first argument, u used a file specification:  
"M:\\mediMACH\\Kundenversionen\\mediMACH4\\\*.\*"  
but as second one just a directory:  
"M:\\mM4Setups\\current\\mM4Source\\mediMACH4\\"  

use a directory OR a file specification in BOTH arguments, like this:  
xcopy.exe "M:\\mediMACH\\Kundenversionen\\mediMACH4" "M:\\mM4Setups\\current\\mM4Source\\mediMACH4" /E /Y

PatrickFraley  
4th April 2005 15:46 UTC

Hi,  

thanks for the reply, but the syntax is correct and the batchFile when I start it from a command (cmd) shell works fine (as I stated in my first post ;), just calling it from NSIS it does not work. I also tried your syntax, with no difference.  

thanks  
patrick

Gabriel Sieben  
4th May 2005 14:29 UTC

**Solution...**  
Use the robust copy command out of the windows resource kit.  

!system '${InstallScriptInput}\\robocopy /NFL /E "${ProductInput}\\cd-Root" "${Output}" '

kichik  
5th May 2005 17:44 UTC

NSIS simply calls system() which calls %COMSPEC% /c <your command here>. Your problem might be that the COMSPEC environment variable is not defined correctly.

mynab  
19th October 2005 09:53 UTC

Hello,  

I am facing the exact same problem. If you write a simple .bat file that does:  

echo %COMSPEC%  
xcopy /?  

and execute it from your script with a !system call you'll notice that it does output the right COMSPEC but that the call to xcopy is simply bypassed and does not output anthing. I replaced xcopy by "c:\\windows\\system32\\xcopy.exe" and got the same result. Of course, running this from anywhere displays the help message of xcopy...  

Thanks,  
mynab  

PS: copy /? does work (but does not perform recursive file copy) so this seems to be xcopy related...

kichik  
20th October 2005 20:33 UTC

xcopy requires stdin which MakeNSISw doesn't provide. This problem was already fixed for nsExec, so fixing it again for MakeNSISw wasn't hard. Until 2.11, or the next nightly build, you can use makensis.exe directly.

mynab  
21st October 2005 16:31 UTC

Thanks you very much kichik!  

Another almost related question: when I run makensis.exe using the /O flag all my output is nicely redirected to a file except the output of commands run with !system. Is there a may to make this output go to the /O specified log file the same way it merges in the makensisw.exe window?  

Thanks  
mynab

kichik  
21st October 2005 16:35 UTC

Use output redirection:

```
makensis.exe script.nsi > output.txt
```

To redirect errors to the file as well:

```
makensis.exe script.nsi > output.txt 2>&1
```

For more information:  
http://www.robvanderwoude.com/redirection.html

mynab  
21st October 2005 16:37 UTC

Thanks I know redirections but in that case why did you put a /O switch to makensis? :)

kichik  
21st October 2005 16:39 UTC

I don't know, I didn't add it. It might be for easy redirection outside of a command prompt.

Fork me on GitHub