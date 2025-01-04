# Hardware Abstraction layer FOR ATMEGA328p

- We established distinct branches dedicated to the specific features of GPIO, USART, and SPI. Each feature has been allocated its own separate branch for better organization and clarity. For anyone interested in exploring these features more thoroughly, you can find detailed information regarding each one within their corresponding branches. These separate branches serve to enhance the clarity and manageability of our project.


[CORRECTION GPIO] (Don't hesitate to remove this part)
I couldn't compile ! When you build your project for the first time, I recommand you to use the ```cargo new your_project``` command.
It is good that you use a generic function like "modify_reg()", but you could have try to subdivise your project into different module.

[CORRECTION USART] (Don't hesitate to remove this part)
Your code cannot work with an empty Cargo.toml (or an empty code). I invite you to follow the advice I gave you in the GPIO correction (about cargo new).
Don't hesitate to send me an email.

[CORRECTION SPI] (Don't hesitate to remove this part)
You didn't implement the SPI feature.
