Description: Relative path question (Resolved)

Title: Relative path question (Resolved) - NSIS Forums

2.  NSIS Discussion
3.  Relative path question (Resolved)

Archive: Relative path question (Resolved)

siegeon  
3rd August 2011 17:00 UTC

**Relative path question (Resolved)**  
I recently joined a company who is using NSIS for their installer scripts, so I do not know much about them. I have been trying to set up the pathing of the files to add to the installer to be relative, but all my attempts are failing. As an example here they are adding the program files that will be included in the install.  

; program files  
SetOutPath "$INSTDIR"  
SetOverwrite ifnewer  
File "C:\\Users\\me\\Documents\\VisualStudio\\2010\\Projects\\project\\subfolder\\bin\\Release\\company.Windows.exe"  

I would like the c:\\Users\\me portion to be relative so that we could all use the same script without changing the user all of the time.  

I have tried a few things like  
$DOCUMENTS\\visualStudio\\2010...  
But I think I'm constructing the script wrong, or not using the Constants right, any suggestions?

MSG  
3rd August 2011 17:02 UTC

$DOCUMENTS is a runtime variable, while the compiler needs a compiletime path. You can read an environment variable during compile IIRC... Have you tried searching the forums for a bit? I can't remember where I saw it.

siegeon  
3rd August 2011 17:10 UTC

I have been dredging through the forums, it does not have to be $DOCUMENTS, that was just an example. I just figured there was a simple way that I was overlooking.  

Oh I have tried to use ${\_\_FILEDIR\_\_} as well, but with the same result of file not found.

siegeon  
3rd August 2011 17:20 UTC

Got it, never mind =)

MSG  
3rd August 2011 20:58 UTC

Feel free to mention the solution here, for posterity's sake. :)

siegeon  
4th August 2011 15:30 UTC

**Solution**  
Like a tard I did not realize that the installer already knew where it was, and I could use ./ syntax to move the folder I needed. Since the .nsi script is stored in the project I was able to replace the whole path  

File "C:\\Users\\Me\\Documents\\Visual Studio 2010\\Projects\\Project\\CFO\\bin\\Release\\Gyrum.Data.SqlServerCe35.dll"  

with  

FILE ".\\CFO\\bin\\Release\\Gyrum.Data.SqlServerCe35.dll"  

making the installer work on everyone's machine.

Fork me on GitHub