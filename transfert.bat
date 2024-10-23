@echo off
REM Clean up any previous build files
echo Cleaning previous build files...
cargo clean
del output.elf
del output.hex

REM Build the project for AVR target
echo Building the project for AVR target...
cargo +nightly build -Z build-std=core --target avr-unknown-gnu-atmega328 --release

REM Link the output using avr-gcc
echo Linking with avr-gcc...
C:\WinAVR-20100110\bin\avr-gcc.exe -mmcu=atmega328 -o output.elf .\target\avr-unknown-gnu-atmega328\release\deps\*.o .\target\avr-unknown-gnu-atmega328\release\deps\*.rlib

REM Convert ELF to HEX format
echo Converting ELF to HEX...
C:\WinAVR-20100110\bin\avr-objcopy.exe -O ihex .\output.elf output.hex

REM Upload the HEX file to Arduino using avrdude
echo Uploading HEX to Arduino...
C:\WinAVR-20100110\bin\avrdude.exe -C C:\WinAVR-20100110\bin\avrdude.conf -v -p atmega328p -c arduino -P COM4 -b 115200 -U flash:w:output.hex:i

echo Done!
pause
