# Sumas de verificación (SHA-256)

El SHA-256 de cada archivo de cada versión, para comprobar que lo que descargaste es exactamente lo que se publicó. La misma lista está en cada [release](https://github.com/PSGM-Romhacking/v-bear/releases) como `SHA256SUMS`; esta copia vive en el repositorio, así que se pueden comparar.

## Cómo comprobar

Deja el archivo junto a un `SHA256SUMS` hecho con el bloque de su versión (o descarga el de la release) y ejecuta:

```
sha256sum -c --ignore-missing SHA256SUMS
```

En Windows (PowerShell), compara el resultado con la línea del archivo:

```
Get-FileHash .\v-bear-0.5.2-windows-x86_64.exe -Algorithm SHA256
```

## Versión 0.5.2

```
cb5c487a4aa4167c451151c7a5d8b69e967afbdef3b84e4cae6ed26c6016ffb3  v-bear-0.5.2-freebsd-x86_64.tar.gz
056a8cb811166756b8f89e99d6421304cdebb597e9c551c10c92254b046eb645  v-bear-0.5.2-linux-x86_64.AppImage
37a5e3c5b0889b7d86a2d5dfadadc737f6c20c48ce235f01f4cefce42cb85378  v-bear-0.5.2-linux-x86_64.AppImage.zsync
34985f08a33d7d4edd44d0514d1d87dc7f7ebb47d85958770717522862cc4385  v-bear-0.5.2-windows-x86_64.exe
87ac71c7218a2ecaf5f04a6b87ef7ce196175829ee0bc1fa018093e84136f4c1  v_bear_libretro-0.5.2-freebsd-x86_64.zip
14399df984e90b2820315513a334be1442409953c8f1d04c865f3ea968ff858d  v_bear_libretro-0.5.2-linux-x86_64.zip
272ae78cce454d68c6d80b970d1264e120c641ec17804e58b0aa1820c63a7f47  v_bear_libretro-0.5.2-windows-x86_64.zip
```

## Versión 0.5.1

```
880c20996b5117ee86263173460dff1ba789ba062b674afe217a8f17f828330b  v-bear-0.5.1-freebsd-x86_64.tar.gz
4aa12d7f54773f6f8d5a37809ec30c3467bb4d40b87de47456278ec95eb6c519  v-bear-0.5.1-linux-x86_64.AppImage
c228d505153efe3a928a38a98bf4add72d622ed7ccfb60aa92d1ac4fc854e897  v-bear-0.5.1-linux-x86_64.AppImage.zsync
4f451472bc22bd55100af291e38064a57d7d4cf596a5374636997c3b8aec21c2  v-bear-0.5.1-windows-x86_64.exe
dd0f2d23ff43f42b0c1ef3f8842f9f87c642f8b42a9feaae7b1f11f8a220464b  v_bear_libretro-0.5.1-freebsd-x86_64.zip
94713fdaa33152f8618644741ebacb0966e3d9e261579e1ebb52fcc3fc70c8d2  v_bear_libretro-0.5.1-linux-x86_64.zip
932b3af9ee4efbeb2d0b9381126a1e93c8618256862f3cb6f4ea09a2321d17da  v_bear_libretro-0.5.1-windows-x86_64.zip
```

## Versión 0.5.0

```
bd9dd4649ff8d537e4ec2fdbda9e7187cd6e774da7bf2819343414d8f3cd0d73  v-bear-0.5.0-freebsd-x86_64.tar.gz
0fdcd95a1567b9f41ccfabba80024d4e678d43cdc6a5b4c0ec54459fa49c596e  v-bear-0.5.0-linux-x86_64.AppImage
747026c42a9deedf24b6f97b7b57eb038e892685e102ac5704366a6e4d45aec4  v-bear-0.5.0-linux-x86_64.AppImage.zsync
b7612e5ef41587e525510872e01bcdbbfa11ea511e3a493ecb3204fd20ea12ed  v-bear-0.5.0-windows-x86_64.exe
f7e2eebc4d8df81cdf2efc8b9fe1dba14fb944a49b8d546e01f1eaa7039dcb89  v_bear_libretro-0.5.0-freebsd-x86_64.zip
801ebe459a8efd8cc1be533a03387cc56f136b95c6985b91743f7021c60a0d5a  v_bear_libretro-0.5.0-linux-x86_64.zip
e3bf082283cb8da1cf580cb9aa3550a85300727977e342a0a6971dc3fd754b24  v_bear_libretro-0.5.0-windows-x86_64.zip
```
