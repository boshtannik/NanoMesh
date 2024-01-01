# Mesh Network Protocol for embedded devices

## Goal

The goal of this project is to create easy to use, mesh like, data transferring protocol using cheap and simple components (Arduino with atmega 328p chip, but not only, and any radio module with USART interface, which allows duplex data transfer). This protocol allows you to build a reliable and easy-to-use mesh-like network for various applications, such as:
- Home automation
- Remote control
- Remote monitoring (telemetry)
- Decentralized messaging

While initially designed to be able to run on Atmega328p chips,
so as well as it runs on this platform - it can be ported to
many other platforms, which are supported by embedded-hal.

Potentially this protocol can be run on other platforms. (Not tested yet, instead of Atmega328p)
It is done by generic behaviour being moved out of implementation
to make it interchangable with implementations of PlatformTime and PlatformSerial traits for
each other platform.

## Status
The code is designed to utilize UART rx/tx pins of your MCU and has been tested with popular radio modules JDY-40.
The code potentially can use radio modules with similar UART interface, that devices, such as:
- JDY-41
- SV-610
- HC-11
- HC-12
- LC12S
- GT-38
- LoRa modules
  
The following functionalities of protocol have been tested and verified:
- Send data.
- Receive data.
- Send data with ignorance of duplicated packets.
- Send data with limited of number of hops.
- Multicast.
- Message transition by the intermediate nodes.
- Send message with ping flag, and receive message with pong flag set.
- Transaction send and receive packet about transaction being finished.

### Note: The more nodes in the network leads to the more network stability. In the stable networks - there is less need to use `transaction` or `ping_pong` sending, unless, you send something very important.

## Warning

This protocol does not provide data encryption. To secure your data from being stolen, you should implement (de/en)cryption mechanisms independently.

## Note

It is recommended to set `listen_period` value on multiple devices different from each other,
like: device 1 - 150 ms, device 2 - 200 ms, device 3 - 250 ms - in order to reduce chance of
the network to sychronize, which will lead to packet collisions.

Number of hops, or (`lifetime`),    - sets ammount - for how many devices the packet will be able to jump trough.
`MULTICAST_RESERVED_IDENTIFIER`     - is the identifier, that is reserved by the protocol
                                    for every device in the network to be treated as it's own.

## Main Components

The central component of this protocol is the `Node` structure, which offers a
user-friendly interface for actions like sending, receiving, multicast, ping-pong,
and handling message transactions. The `Node` should be constantly updated by
calling its `update` method.

To initialize a `Node`, you need to provide two values:

1. `AddressType`: Represents the device's identification address in the node pool.
2. `listen_period`: A value in milliseconds that determines how long the device
will wait before transmitting on the network to prevent network congestion.

You can regulate the number of hops that the packet will be able to
make - by configuring the `lifetime` parameter. For example,
setting `lifetime` to 1 will limit the message's reach to
the nearest devices in the network.

The term "echoed message" refers to a duplicated message that has
been re-transmitted into the ether by an intermediate device.

### Send Method

The `send` method requires the following arguments:

- `data`: A `PacketDataBytes` instance to hold the message bytes.
- `destination_device_identifier`: A `AddressType` instance indicating the target device.
- `lifetime`: A `LifeTimeType` instance to control the message's travel distance.
- `filter_out_duplication`: A boolean flag to filter out echoed messages from the network.

### Receive Method

The `receive` method optionally returns received data in a `PacketDataBytes` instance.

### Send Ping-Pong Method

The `send_ping_pong` method sends a message with a "ping" flag to the destination node and
waits for the same message with a "pong" flag. It returns an error if the ping-pong exchange fails.
The following arguments are required:

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `AddressType` instance.
- `lifetime`: A `LifeTimeType` instance.
- `filter_out_duplication`: A boolean flag.
- `timeout`: An `ms` instance specifying how long to wait for a response.

### Send with Transaction Method

The `send_with_transaction` method sends a message and handles all further work to
ensure the target device responds. It returns an error if the transaction failed.
The required arguments are:

- `data`: A `PacketDataBytes` instance.
- `destination_device_identifier`: A `AddressType` instance.
- `lifetime`: A `LifeTimeType` instance.
- `filter_out_duplication`: A boolean flag.
- `timeout`: An `ms` instance to specify the response wait time.

## Note

Under the hood, data is packed into a `Packet` instance. 
If you need customized packets - you can configure the `Packet`
fields in `src/Node/packet/config.rs` and `src/Node/packet/types.rs`.

Important!!!
All nodes must have the same version of the protocol installed to
communicate effectively. Different device implementations of the
`Packet` structure may lead to communication issues.

## Getting Started

You can use this either as a library or by modification of code of example
in the exmaples directory.

## License

This project is licensed under:

- GPL License, Version 3.0 ([LICENSE-GNU](LICENSE-GNU) or [General Public License 3.0](http://www.gnu.org/licenses/gpl-3.0.txt))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [Apache License 2.0](http://www.apache.org/licenses/LICENSE-2.0))
- MIT License ([LICENSE-MIT](LICENSE-MIT) or [MIT License](http://opensource.org/licenses/MIT))

You can choose the license that best suits your preferences.

## Contribution

Unless you specify otherwise, any contributions submitted for inclusion in this project, as defined in the Apache-2.0 license, will be dual-licensed under both licenses without additional terms or conditions.

## Donation
Also you can support the project by donating few
bucks on the bitcoin address: bc1qc50tm0ppj3hh7fecd6d0rv8tdygy8uhe2cemzt
