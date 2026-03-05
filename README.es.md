## 💻 Tecnologías

[![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)](https://www.linux.org)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)](https://www.microsoft.com/windows)

---

<div align="center">
<h1>Kipple - Organizador de Archivos por Terminal</h1>
  <img src="assets/kipple-logo.png" alt="Kipple Logo" width="300" height="300">

### 🌍 Elige Idioma
[**English**](README.md) • [**Español**](#español)
</div>

---


## 🚀 Descripción

Kipple es una herramienta de línea de comandos escrita en Rust que organiza automáticamente tus archivos por extensión en carpetas categorizadas. Un comando y el caos de tus descargas desaparece.


## ✨ Características

- 📁 **Organización Automática** - Clasifica archivos por extensión en carpetas (Imágenes, Documentos, Música, etc.)
- 🔍 **Modo Simulación** - Vista previa de lo que se hará sin mover nada (`--dry-run`)
- 🎯 **Directorio Personalizable** - Organiza cualquier carpeta, no solo la actual (`-d`)
- 📂 **Procesamiento Recursivo** - Incluye subcarpetas con `--include-dirs`
- 🗑️ **Limpieza Automática** - Elimina carpetas vacías después de organizar
- 🔄 **Manejo de Duplicados** - Renombra archivos con el mismo nombre (`archivo(1).txt`)
- ⚡ **Modo Forzado** - Sobrescribe archivos existentes con `--force`
- 🎭 **Modo Verbose** - Mira exactamente qué pasa bajo el capó con `-v`
- 🎨 **Salida Atractiva** - Mensajes con colores para errores, advertencias y éxitos

## 📦 Instalación

### Desde el Código Fuente
```bash
git clone https://github.com/Jorge-Guedes/kipple.git
cd kipple
cargo build --release
cp target/release/kipple ~/.local/bin/  # o /usr/local/bin/ con sudo
```

### Instalación del Binario en Windows

```bash
# Windows (después de compilar)
copy target\release\kipple.exe %USERPROFILE%\bin\
```

### Usando Cargo
```bash
cargo install --git https://github.com/Jorge-Guedes/kipple.git
```

## 🚀 Uso

```bash
# Organizar directorio actual
kipple

# Organizar una carpeta específica
kipple -d ~/Descargas

# Ver qué se haría sin mover nada
kipple --dry-run

# Con detalles del proceso
kipple -v

# Incluir subcarpetas
kipple --include-dirs

# Sobrescribir archivos duplicados
kipple --force

# Combinar opciones
kipple -d ~/Descargas --include-dirs -v
```

## ⚙️ Opciones

| Opción | Descripción | Por Defecto |
|--------|-------------|-------------|
| `-d, --directory` | Directorio a organizar | Directorio actual |
| `--dry-run` | Muestra lo que se haría sin mover nada | `false` |
| `--include-dirs` | Procesa subcarpetas recursivamente y elimina las vacías | `false` |
| `--force` | Sobrescribe archivos en lugar de renombrar | `false` |
| `-v, --verbose` | Muestra mensajes detallados del proceso | `false` |
| `-h, --help` | Muestra la ayuda | - |
| `-V, --version` | Muestra la versión | - |

## 📁 Cómo Funciona

Los archivos se organizan por extensión en estas carpetas:

| Carpeta | Extensiones |
|---------|-------------|
| **📁 Pictures** | `.jpg`, `.jpeg`, `.png`, `.gif`, `.bmp` |
| **📁 Documents** | `.pdf`, `.txt`, `.doc`, `.docx`, `.odt` |
| **📁 Music** | `.mp3`, `.wav`, `.flac`, `.aac` |
| **📁 Videos** | `.mp4`, `.avi`, `.mkv`, `.mov` |
| **📁 Archives** | `.zip`, `.rar`, `.7z`, `.tar`, `.gz` |
| **📁 Code** | `.rs`, `.py`, `.js`, `.ts`, `.html`, `.css`, `.jsx`, `.tsx`, `.json`, `.sql`, `.yml` |
| **📁 Others** | Cualquier otra extensión o archivos sin extensión |

## 🔧 Ejemplo

```bash
$ kipple -d ~/Descargas --include-dirs -v

PATH SELECTED: /home/user/Descargas
--------------------------------------------------------------------------------------------------------------
DIR: 📁 /home/user/Descargas/proyecto
--------------------------------------------------------------------------------------------------------------
EVALUATING: foto.jpg (.jpg)
CLASSIFIED: foto.jpg → 📁 pictures
--------------------------------------------------------------------------------------------------------------
MOVED: foto.jpg → /home/user/Descargas/Pictures
--------------------------------------------------------------------------------------------------------------
EVALUATING: documento.pdf (.pdf)
CLASSIFIED: documento.pdf → 📁 documents
--------------------------------------------------------------------------------------------------------------
MOVED: documento.pdf → /home/user/Descargas/Documents
--------------------------------------------------------------------------------------------------------------
WARNING: File with no extension: "/home/user/Descargas/README"
--------------------------------------------------------------------------------------------------------------
DUPLICATE: cancion.mp3 already exists, looking for alternative...
--------------------------------------------------------------------------------------------------------------
RENAMED: cancion.mp3 → cancion(1).mp3
--------------------------------------------------------------------------------------------------------------
MOVED: cancion(1).mp3 → /home/user/Descargas/Music
--------------------------------------------------------------------------------------------------------------
REMOVED: /home/user/Descargas/proyecto
--------------------------------------------------------------------------------------------------------------
SUCCESS: Files organized in /home/user/Descargas
```


## 📄 Licencia

Este proyecto está bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para más detalles.