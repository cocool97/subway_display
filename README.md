# subway_display

> Project built in Rust aiming to mimic the Paris subway displays using a Raspberry Pi and an ESP-32 microcontroller.

## Server

It listens on **TCP** for incoming connections from the microcontroller and regularly sends subway arrivals data as well as the current hour over the network.

Once the microcontroller is connected, the server will keep the connection open and send data as long as the it is connected, every `POLL_INTERVAL_SECS` seconds.

In current version the server only supports one client at a time.

It uses the [PRIM API](https://prim.iledefrance-mobilites.fr/fr) to fetch next subway arrivals.
An API key is required and can be obtained [here](https://prim.iledefrance-mobilites.fr/fr/mes-jetons-authentification) (registration needed). This key has to be passed as an environment variable `PRIM_API_KEY`.

The API endpoint used is based on two values:

- MonitoringRef: Representing the station to monitor (1 station per direction)
- LineRef: The line to monitor

Known values for MonitoringRef and LineRef can be found [here](https://prim.iledefrance-mobilites.fr/fr/jeux-de-donnees/arrets-lignes) (`stop_id` for stations/directions and `route_id` for lines).
For example, for Ligne 13 Station 'Gabriel-Péri' direction Chatillon-Montrouge, the values are respectively `STIF:StopPoint:Q:462953:` and `STIF:Line::C01383:`.

> [!warning]
> By default the API only allows for 1000 requests per day but can be increased if asked for.
> Current usage can be checked [here](https://prim.iledefrance-mobilites.fr/fr/ma-consommation-api)

To build the server container, run:

```bash
podman build -t subway_server:latest -f Dockerfile .
```

A `docker-compose.yml` is also provided to easily setup the server:

```bash
podman-compose -f podman-compose.yml up -d
```

## Microcontroller a.k.a. "Client"

TODO

## Communication protocol

The protocol used to communicate between the server and the microcontroller is a basic custom binary protocol. It has been designed to be very light to keep the microcontroller power consumption as low as possible.

The protocol's code is available in `proto/` directory, with a `no_std` contrainst to be usable on the microcontroller. 

It contains a message type over **1 byte** and a variable-length payload depending on the message type.

| Message Type |                                                       Meaning                                                        | Payload Length |
| :----------: | :------------------------------------------------------------------------------------------------------------------: | :------------: |
|    `0x00`    |                                       Provides the current hour (from 0 to 23)                                       |     1 byte     |
|    `0x01`    | Provides the **two** next subway arrivals in minutes (1 byte each). A value of 255 indicates an error or no arrival. |    2 bytes     |

## Final display

TODO
