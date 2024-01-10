# Win95 Key Tool
[![Build](https://github.com/greenlonk/Win95-KeyTool/actions/workflows/build.yml/badge.svg)](https://github.com/greenlonk/Win95-KeyTool/actions/workflows/build.yml)

This software is a cross-plattform generator and validator for Windows 95 keys written in Rust. It is only sporadically developed as it is solely a learning project for me and does not have particularly significant practical use.
>⚠️ **Note**: Win95 Key Tool and I are in no way associated with Microsoft or their products. Only publicly available information from the internet has been used. It neither bypasses effective copy measures nor constitutes a "crack." The purpose of this repository, from my perspective, is solely to gain experience in programming and enhance my skills and knowledge.
# Usage
To use the software, it needs to be launched via a terminal. There are two options:
## Generate
Launch the software in a terminal window with _generate_ as additional argument. In a fraction of a second, a key will be generated that meets all the requirements for activating Windows 95. The command in the Windows command prompt would look something like this:
```
.\<PATH_TO_EXECUTABLE\win95-keytool.exe generate
```
## Validate
To check the validity of a key, you can use it as an argument. This can be done in the command prompt like this:
```
.\<PATH_TO_EXECUTABLE\win95-keytool.exe 012-3456789
```
# Limits
- Currently, only standard Windows 95 keys are supported. OEM keys, for example, are not supported.
- If a key is not properly formatted _(XXX-XXXXXXX)_ during validation, the software considers it invalid, even though that might not necessarily be the case.
- Only a single key can be validated per command.
