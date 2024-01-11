[Setup]
AppName=Clipman
AppVersion=0.1.4
DefaultDirName={autopf}\Clipman
DefaultGroupName=Clipman
OutputBaseFilename=Clipman_Installer
PrivilegesRequired=lowest


[Files]
Source: "..\target\release\clipman.exe"; DestDir: "{app}"
Source: "readme.txt"; DestDir: "{app}"; Flags: "isreadme"

; run app automatically after install
[Run]
Filename: "{app}\clipman.exe"; Flags: "nowait"

[Dirs]
Name: "{app}\clipboard"

; kill the task so that the uninstaller can remove the software
[UninstallRun]
Filename: "{cmd}"; Parameters: "/C ""taskkill /im clipman.exe /f /t"; Flags: "runminimized"; RunOnceId: "a"

; placing shortcut in the auto start menu.
[Icons]
Name: "{userstartup}\clipman"; Filename: "{app}\clipman.exe"; WorkingDir: "{app}"