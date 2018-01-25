How to build
============

For every platform you'll need [.NET Core SDK 2.0](https://www.microsoft.com/net/learn/get-started).

Instruction for Windows
=======================

* Install [VC++ Compiler](https://aka.ms/vs/15/release/vs_BuildTools.exe)
* Open `x64 Native Tools Command Prompt`
* Run `$ dotnet publish -c Release -r win-x64`

Instructions for Linux
======================
* Install `clang`
* Install dev packages for `curl` and `uuid`
    * e.g. for Ubuntu: `# apt-get install uuid-dev libcurl4-{gnutls|nss|openssl}-dev`
* Run `$ dotnet publish -c Release -r linux-x64 -p:CppCompilerAndLinker=clang`
    * or `clang-6.0`, or whatever you have installed