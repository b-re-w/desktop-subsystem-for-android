# Modo de desenvolvedor e adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · [Français](../fr/developer-mode.md) · [Deutsch](../de/developer-mode.md) · **Português (Brasil)** · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

O modo de desenvolvedor permite que o Android Debug Bridge (adb) do seu PC se conecte ao seu
Android no Desktop Subsystem for Android™ (DSA). Ele vem desativado por padrão.

O adb não faz parte do DSA. Obtenha-o pelo Android SDK Platform-Tools.

## Conectar o adb

1. Abra a página **Network** (rede) das DSA Settings e ative **Developer mode** (modo de
   desenvolvedor). Ou execute `dsa settings set developer-mode on`.
2. Inicie o Android, se ele não estiver rodando: `dsa start --wait`.
3. Conecte:

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   O Android não pede que você permita a conexão. A porta só responde neste PC, então outros
   computadores não conseguem se conectar.
4. Quando terminar, desative **Developer mode** de novo, ou execute
   `dsa settings set developer-mode off`.

**Connect adb** (conectar o adb) na página **Network** mostra o comando exato, e
`dsa settings` o exibe na linha `Developer mode`.

## Usar outra porta

A porta padrão é 58526. Para alterá-la, edite **Port** (porta) em **Connect adb**, ou execute:

```
dsa settings set adb-port 5555
```

## O que o modo de desenvolvedor não altera

O DSA nunca usa o adb para o próprio trabalho. As janelas, a entrada e as instalações usam o
canal de controle do próprio DSA, então tudo funciona da mesma forma com o modo de desenvolvedor
desativado.

Para ler o log do Android, você não precisa do adb. Veja
[Solução de problemas](troubleshooting.md).

---

Android é uma marca registrada da Google LLC. O DSA não é afiliado ao Google, nem endossado ou
patrocinado por ele.
