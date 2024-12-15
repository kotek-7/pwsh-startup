# pwsh-startup
This tool measures the startup time of PowerShell 10 times and calculates the average.

## Usage
Just run the released .exe file. Make sure a powershell is installed.

```
pwsh-startup
```

It tries to start powershell 10 times, and shows the results.

```
Measuring PowerShell startup time with loading profile...
(Run with -noprofile argument to measure startup time without loading profile)

(1) Startup time: 1039 ms
(2) Startup time: 875 ms
(3) Startup time: 880 ms
(4) Startup time: 864 ms
(5) Startup time: 843 ms
(6) Startup time: 835 ms
(7) Startup time: 839 ms
(8) Startup time: 932 ms
(9) Startup time: 845 ms
(10) Startup time: 886 ms

Average startup time: 883 ms
```

To measure startup time without loading a profile, run with -NoProfile argument .
```
pwsh-startup -NoProfile
```

Then the result will look like this.
```
Measuring PowerShell startup time without loading profile...
(1) Startup time: 349 ms
(2) Startup time: 303 ms
(3) Startup time: 326 ms
(4) Startup time: 323 ms
(5) Startup time: 328 ms
(6) Startup time: 317 ms
(7) Startup time: 310 ms
(8) Startup time: 307 ms
(9) Startup time: 314 ms
(10) Startup time: 330 ms

Average startup time (No Profile): 320 ms
```
