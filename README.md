# RGB Utils

This was just a project I made that would load my mouse profile whenever I connected my mouse to my laptop so I didn't have to have something on startup besides this

Run the following command to find the id of your mouse

    lsusb

Use the product id for configuring the program

    Bus 001 Device 062: ID 1532:0067 Razer USA, Ltd Naga Trinity

In this case I would run with the -c option and set the mouseID to 0067
