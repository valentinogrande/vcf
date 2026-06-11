# Wordlists

`rockyou.txt` y `rockyou_sorted.txt` superan el límite de 100 MB de GitHub, así
que están subidos divididos en partes de 90 MB.

## Reensamblar

```sh
cat rockyou.txt.part* > rockyou.txt
cat rockyou_sorted.txt.part* > rockyou_sorted.txt
```
