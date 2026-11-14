# Versões do Android

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · [Français](../fr/android-versions.md) · [Deutsch](../de/android-versions.md) · **Português (Brasil)** · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

O Desktop Subsystem for Android™ (DSA) oferece suporte às duas versões principais mais
recentes do Android: hoje, Android 16 e Android 17. Por padrão, ele executa a mais nova. Alguns
apps rodam em só uma delas, então você pode escolher.

## Ver qual versão está rodando

Abra a página **Updates** (atualizações) das DSA Settings. Em **Android**, ela mostra a versão,
o build, o nível do patch de segurança, a versão que ela acompanha e a situação da atualização,
com o progresso do download. **Check now** (verificar agora) pede ao serviço que procure uma
atualização na hora. Ou execute:

```
dsa android version   # a versão que roda e as que você pode escolher
```

## Mudar a versão

Todos os usuários do PC executam a mesma versão do Android, então só um administrador pode
mudá-la.

1. Abra a página **Updates** das DSA Settings.
2. Em **Version to use** (versão a usar), escolha uma versão, ou **Always the newest** (sempre a
   mais nova).
3. Selecione **Change** (mudar) e aprove a solicitação de administrador.

A versão escolhida é baixada e usada a partir da próxima inicialização do Android. Em um
terminal de administrador:

```
dsa android use 16
dsa android use latest
```

### Mudar para uma versão mais antiga apaga os dados

O Android não consegue ler dados gravados por uma versão mais nova. Por isso, mudar para uma
versão principal mais antiga apaga os dados do Android de todos os usuários do PC: os apps que
eles instalaram, as contas deles e todos os dados dos apps. As configurações do DSA permanecem.
Isso não pode ser desfeito.

As DSA Settings pedem que você confirme com **Erase data and change** (apagar os dados e mudar).
Na linha de comando, acrescente `--erase-data`:

```
dsa android use 16 --erase-data
```

Mudar para uma versão mais nova mantém seus apps e dados.

## Atualizações

- A versão escolhida continua recebendo atualizações de segurança.
- Quando uma versão sai do suporte, o DSA passa para a seguinte e mantém seus apps.
- O DSA nunca passa por conta própria para uma versão principal mais antiga.
- Cada imagem é verificada em uma lista assinada antes de ser usada. Uma imagem que não consegue
  inicializar é substituída pela anterior.

Um administrador pode desativar as atualizações de imagem com **Update the Android image
automatically** na página **Updates** das DSA Settings, ou com
`dsa settings set image-updates off`.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
