# Kipple

Organizador de archivos por terminal. Un comando y el caos de tus descargas desaparece.

## Uso

kipple                # ordena la carpeta actual
kipple --dry-run      # muestra lo que haría sin mover nada
kipple --carpeta DIR  # ordena una carpeta específica
kipple --help         # muestra ayuda

## Instalación

git clone https://github.com/Jorge-Guedes/kipple
cd kipple
cargo build --release
cp target/release/kipple ~/.local/bin/

## Cómo funciona

Los archivos se ordenan por extensión en carpetas:

- Imagenes/ → .jpg, .png, .gif, .bmp
- Documentos/ → .pdf, .txt, .doc, .odt
- Musica/ → .mp3, .wav, .flac
- Videos/ → .mp4, .avi, .mkv
- Codigo/ → .rs, .py, .js, .html
- Comprimidos/ → .zip, .rar, .7z
- Otros/ → todo lo demás

## Por qué Kipple

Porque la basura digital se reproduce cuando no miras.

## Licencia

MIT