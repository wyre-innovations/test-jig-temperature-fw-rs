# Test Jig - Temperature (using Rust)

This project implements a temperature test jig using an STM32G0 microcontroller, 16x2 LCD display, and 6 thermistor sensors connected to ADC channels.

## Features

- **Boot Logo**: Displays "Wyre Innovations" on startup for 2 seconds
- **Temperature Sensing**: Reads 6 NTC thermistor sensors using ADC polling
- **Steinhart-Hart Calculation**: Converts ADC readings to temperature in Celsius using Steinhart-Hart equation
- **LCD Display**: Shows all 6 temperatures on a 16x2 LCD (3 per line, format: T1:XX.X T2:XX.X T3:XX.X)
- **Real-time Updates**: Refreshes temperature display every second

## Hardware Configuration

- **Microcontroller**: STM32G030C6Tx
- **ADC Channels**: 6 channels (ADC0-ADC5 on GPIOA pins 0-5)
- **LCD Interface**: 4-bit mode on GPIOB (RS, EN, D4-D7 pins)
- **Thermistors**: 10k NTC thermistors in voltage divider with 10k fixed resistors

## License

Copyright (c) 2025 Wyre Innovations. All rights reserved.