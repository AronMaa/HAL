# HAL
Create an hardware abstraction layer for Arduino Uno and ESP8266.

[CORRECTION GPIO] (Don't hesitate to remove this part)
Consider subdividing your project into separate modules. 
It would be nice to have something to prevent modifying the register in an incoherent way. For example, I could do ``` let pin2 = DigitalPin::new(Port::D, 50);```, it won't bug during the compilation, but it may generate a problem on your hardware.


[CORRECTION USART] (Don't hesitate to remove this part)
You didn't really implement the USART for your CORTEX M7 target.
You could abstract your register adresses, by putting them outside your function (as constant for example).


[CORRECTION SPI] (Don't hesitate to remove this part)
For your CORTEX target, you didn't really implement the SPI feature. Also, you should abstract yourself the hardware specification, and not rely on any external HAL (cf ```let peripherals = Peripherals::take().unwrap();```).
You should implement the peripheral/slave mode as well (not only the controler/master mode).