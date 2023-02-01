# Сборка
Проект зависит от tonic, который зависит от protoc.

protoc нужно установить отдельно:

## Установка protoc на Ubuntu
```
sudo apt update && sudo apt upgrade -y
sudo apt install -y protobuf-compiler libprotobuf-dev
```

## Установка protoc на Alpine
```
sudo apk add protoc protobuf-dev
```

## Установка protoc на MacOS
```
brew install protobuf
```

## Установка protoc на Windows
### Вручную
- Скачать последнюю версию protoc-xx.y-win64.zip из [релизов protocolbuffers/protobuf](https://github.com/protocolbuffers/protobuf/releases)
- Разархивировать bin\protoc.exe куда-нибудь в PATH
- Вызвать `protoc --version`, чтобы убедиться в правильности установки
### Scoop
```
scoop bucket add extras
scoop install protobuf
```