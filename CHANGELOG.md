# `message` changelog

## Unreleased

## 0.3.1 (21.01.2022)

- Improved error message for the send error. [#9]

[#9]: https://github.com/popzxc/messages-rs/pull/9

## 0.3.0 (16.06.2021)

- Big performance boost:
  - Actor spawning performance: +3%.
  - Sending message with response: +5% (now on par with `futures` channels).
  - Sending notification: **+593%**.
  - Ring benchmark report: 4 252 826 msg/second (was 3 121 844 msg/second; `actix`'s
    async version of ring benchmark: 3 608 196 msg/second).

- Some internal improvements.

## 0.2.4 (06.06.2021)

- Actor support was implemented.
