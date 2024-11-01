# HAL
Create an hardware abstraction layer for Arduino Uno and ESP8266.

[CORRECTION GPIO] (Don't hesitate to remove this part)
Consider subdividing your project into separate modules. 
It would be nice to have something to prevent modifying the register in an incoherent way. For example, I could do ``` let pin2 = DigitalPin::new(Port::D, 50);```, it won't bug during the compilation, but it may generate a problem on your hardware.