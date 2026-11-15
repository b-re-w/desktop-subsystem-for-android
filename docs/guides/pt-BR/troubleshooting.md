# Solução de problemas e logs

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · [Français](../fr/troubleshooting.md) · [Deutsch](../de/troubleshooting.md) · **Português (Brasil)** · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Esta página ajuda você a descobrir por que o Desktop Subsystem for Android™ (DSA) não funciona,
a ler o log do Android e a enviar um relato de problema. Ela cobre o Windows e, onde indicado, o
Linux.

## Verifique o estado primeiro

```
dsa status          # estado do Android, problemas e atualização pendente, se houver
dsa start --wait    # inicia o Android e espera até ele inicializar
```

A página **General** (geral) das DSA Settings mostra o mesmo estado.

| `dsa status` | DSA Settings | Significado |
|---|---|---|
| `Active` | **Running** | O Android inicializou, e o agente do DSA dentro dele está conectado |
| `Dozing` | **Dozing (idle)** | O Android está ocioso. Veja [Memória](memory.md) |
| `Frozen` | **Frozen** | O Android está congelado até você abrir uma das janelas dele |
| `Stalled` | **Not ready** | O Android está rodando, mas demorou demais. A linha seguinte diz por quê e o que fazer |

## Problemas comuns

| Problema | O que fazer |
|---|---|
| A instalação diz que a Plataforma de Máquina Virtual está desativada | Ative o recurso do Windows, reinicie e execute a instalação de novo. Veja [Primeiros passos](getting-started.md) |
| Nenhuma janela de app abre | Saia da sua conta e entre de novo. O gerenciador de janelas inicia quando você entra |
| Um app abre em um tamanho ruim | `dsa app-window <package> reset` |
| O Android não inicializa depois de uma atualização de imagem | Nada. O DSA volta sozinho para a imagem anterior, e `dsa status` informa isso |
| O Android não tem rede | Abra a página **Network** das DSA Settings, ou execute `dsa network`. Ela mostra o modo, o endereço, os servidores DNS e qualquer problema |
| O Android está com defeito e nada mais ajuda | Redefina os dados do Android (abaixo) |

### Redefinir os dados do Android

A redefinição apaga todos os apps que você instalou, as suas contas e todos os dados dos apps.
Depois, o Android inicia como na primeira instalação. A imagem do Android e as configurações do
DSA permanecem.

- Nas DSA Settings: página **Advanced** (avançado) > **Reset Android's data** > **Reset**.
- Na linha de comando: `dsa reset-data` informa o que seria perdido. `dsa reset-data --yes`
  apaga os dados.

## Ler o log do Android

O log do Android fica na memória, dentro do Android, e nunca é gravado em um arquivo. Ele só
chega ao seu PC enquanto algo o lê.

- Nas DSA Settings: a página **Logs**. Você pode filtrar por tag ou texto, e pausar.
- Na linha de comando:

  ```
  dsa logcat                        # acompanha o log; Ctrl+C encerra
  dsa logcat -d -b all *:W          # o que o log contém agora, avisos e acima
  dsa logcat ActivityManager:I *:S  # só uma tag
  ```

## Enviar um relato de problema

1. Salve os diagnósticos. Na página **Logs**, selecione **Save diagnostics** (salvar
   diagnósticos), ou execute:

   ```
   dsa logs collect -o diag.zip
   ```

   Isso leva um minuto ou mais.
2. Anexe o arquivo zip a uma
   [issue](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

O arquivo contém todo o log do Android, os registros de falhas dele (tombstones, rastreamentos
de ANR, dropbox, pstore), um resumo dos serviços de sistema do Android e os logs do DSA neste PC
que dizem respeito a você, sem as linhas de outros usuários. O `summary.txt` no arquivo informa
o que não pôde ser coletado.

## Onde ficam os logs no Windows

| Pasta | Arquivos |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (o gerenciador de janelas), `dsa-app-window.log` (as janelas dos apps) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (o serviço), `android-<SID>.log` (o console do Android de cada usuário, no máximo 2 MiB por inicialização), `setup-<version>.log` (as autoatualizações) |

Para abri-los, use **Open my log folder** e **Open the service's log folder (administrators)**
na página **Advanced** das DSA Settings.

## Onde ficam os logs no Linux (ainda não lançado)

Os serviços gravam no journal do systemd:

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Todos os comandos

`dsa` sem argumentos mostra todos os comandos. Acrescente `--json` para ter uma saída que
programas consigam ler.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
