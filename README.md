#### MATEI Sebastian-Alexandru

#### 1221EA – PM 2025 Project

# RO10_Strigoi

# 1. Description

#### Flight controller written in Rust with the Embassy HAL. It controls several systems on my

# drone, such as data acquisition and peripheral control (servo, ESC).

## 2. Initial Bill of Materials

no. Item Name Qty Already
have it

```
Description
```
```
1 STM32H7L8S3 1 STM Family Microcontroller
```
##### DRONE COMPONENTS

```
2 NEO 8M GPS 1 GPS module
3 MPU-9250 10DOF Gyro 1 Gyro to get aircraft orientation
4 precision MS5611 pressure sensor^1 senzor to determine altitude
```
```
5 EEPROM AT24C256^1
```
```
EEPROM to store FHSS/DHSS
seed and keys
6 70MM EDF^2
6 3.7 LIION CELL 16 Just 10
Battery
7 5000MAH 100C 6S LIPO^1
8 MAX485 RS485 to TTL 1
9 80A ESC ZMR Brushles 2 With BEC/UBEC
10 2W RF Power Wideband Amplifier 2 Range AMP
11 TS832 and RC832 1 Video reciever/ sender
```
```
12 12dBi High Gain RP SMA Male Omni Foldable 2 40cm Antenna
```
```
Transmitter
COMPONENTS
```
```
13 EBYTE E32 900 30D 3 900mhz RC transmitter/recv
```
```
14 ARDUINO UNO 1 Middleman between transmitter
and GUI
15 MP1584EN 3A 3 Stepdown Module
16 JX Servo PDI-1181MG 18g 3.5KG^6 just 4Just 4
```

## 3. Initial Software Bill of Materials

### 1.Embassy HAL ( stm32h7xx-hal) 7. heapless

### 2.Nalgebra 8. embedded-hal-bus

### 3.mpu9250 9. eeprom24x-rs

### 4.dshot-rs 10.

### 5.pid 11.

### 6. madgwick 12.

## 4. Diagram


