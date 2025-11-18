# kotlin-klyx

Kotlin language support for [Klyx](https://github.com/klyx-dev/klyx).

> [!IMPORTANT]
> Java must be installed on your system. JAVA_HOME must be set.

## Installing Java (if not already installed)

If Java is not installed on your system, install the default JDK using:

```bash
sudo apt update
sudo apt install default-jdk
```

After installation, verify it with:

```bash
java -version
```

And ensure `JAVA_HOME` is set:

```bash
echo "export JAVA_HOME=/usr/lib/jvm/default-java" >> ~/.bashrc
source ~/.bashrc
```

## Installing Gradle (if not already installed)

Download the Gradle distribution zip:

```bash
curl -LO https://services.gradle.org/distributions/gradle-9.2.1-bin.zip
```

Create a directory and unzip it:

```bash
sudo mkdir /opt/gradle
sudo unzip -d /opt/gradle gradle-9.2.1-bin.zip
```

Add Gradle to your PATH:

```bash
echo "export PATH=\$PATH:/opt/gradle/gradle-9.2.1/bin" >> ~/.bashrc
source ~/.bashrc
```

Verify installation:

```bash
gradle -v
```

## Language Servers

### Kotlin Language Server

The [Kotlin Language Server](https://github.com/fwcd/kotlin-language-server) is an unofficial LSP for Kotlin, it is currently the most stable and popular language server for Kotlin. It is enabled by default by this extension.

#### Configuration

Workspace configuration options can be passed to the language server via lsp settings in `settings.json`.

The following example changes the JVM target from `default` (which is 1.8) to `17`:

```json
{
  "lsp": {
    "kotlin-language-server": {
      "settings": {
        "compiler": {
          "jvm": {
            "target": "17"
          }
        }
      }
    }
  }
}
```

The full list of workspace configuration options can be found [here](https://github.com/fwcd/kotlin-language-server/blob/main/server/src/main/kotlin/org/javacs/kt/Configuration.kt).

### Kotlin LSP

[Kotlin LSP](https://github.com/kotlin/kotlin-lsp) is an official LSP implementation for Kotlin, built by JetBrains. It is currently pre-alpha.

#### Configuration

To use Kotlin LSP instead of the Kotlin Language Server, you must explicitly enable it in your `settings.json`:

```json
{
  "languages": {
    "Kotlin": {
      "language_servers": ["kotlin-lsp"]
    }
  }
}
```

It will be downloaded and updated automatically when enabled, however, you can use a manually installed version by setting the path to the `kotlin-lsp.sh` script in the release assets:

```json
{
  "lsp": {
    "kotlin-lsp": {
      "binary": {
        "path": "path/to/kotlin-lsp.sh",
        "arguments": [ "--stdio" ]
      }
    }
  }
}
```

Note that the `kotlin-lsp.sh` script expects to be run from within the unzipped release zip file, and should not be moved elsewhere.
