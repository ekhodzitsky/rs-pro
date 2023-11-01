# Умная розетка по TCP

Запуск сервера (имитатор умной розетки):

```bash
cargo run --bin socket_server
```

Запуск клиента:

```bash
cargo run --bin socket_client
```

## Как работает

Сервер понимает команды **INFO**, **ON**, **OFF**. Клиент может отправить их из консоли и получить ответ сервера.

```
INFO
Received response: Name: Smart Socket; Status: false; Power Consumption: 0

ON
Received response: Name: Smart Socket; Status: true; Power Consumption: 10

OFF
Received response: Name: Smart Socket; Status: false; Power Consumption: 0
```
