- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/bindings/java/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/bindings/" class="breadcrumbs__link">Bindings</a>
- Java

On this page

# Java

## Apache OpenDALâ„¢ Java Bindings

[<img src="out_opendal/bindings/java/index_media/745e4981547972eb827f306a82346e6d4bd99dd0.svg" class="img_KBPg" decoding="async" loading="lazy" />](https://central.sonatype.com/search?q=opendal&smo=true) [<img src="out_opendal/bindings/java/index_media/d8b62425599badf249c8fa69bdcdeb3c6ff79f99.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Maven Central" />](https://central.sonatype.com/search?q=opendal&smo=true) [<img src="out_opendal/bindings/java/index_media/b8908a5bd08b3de6e77ea4c1c7aa4f9e733185d5.svg" class="img_KBPg" decoding="async" loading="lazy" alt="Website" />](https://opendal.apache.org/docs/java/)

<img src="out_opendal/bindings/java/index_media/f6db506ffa2fa9b49bb043591266c8aa10c32f59.jpg" class="img_KBPg" decoding="async" loading="lazy" />

## Useful Links<a href="https://opendal.apache.org/bindings/java/#useful-links" class="hash-link" aria-label="Direct link to Useful Links" translate="no" title="Direct link to Useful Links">â€‹</a>

- [Documentation](https://opendal.apache.org/docs/java/)
- [Upgrade Guide](https://github.com/apache/opendal/blob/main/bindings/java/upgrade.md)

## Example<a href="https://opendal.apache.org/bindings/java/#example" class="hash-link" aria-label="Direct link to Example" translate="no" title="Direct link to Example">â€‹</a>

``` prism-code
import java.util.HashMap;
import java.util.Map;
import org.apache.opendal.AsyncOperator;
import org.apache.opendal.Operator;

public class Main {
  public static void main(String[] args) {
    final Map<String, String> conf = new HashMap<>();
    conf.put("root", "/tmp");

    try (AsyncOperator op = AsyncOperator.of("fs", conf)) {
      op.write("/path/to/data", "Hello world").join();
      System.out.println(new String(op.read("/path/to/data").join()));
    }
  }
}
```

## Getting Started<a href="https://opendal.apache.org/bindings/java/#getting-started" class="hash-link" aria-label="Direct link to Getting Started" translate="no" title="Direct link to Getting Started">â€‹</a>

This project is built upon the native OpenDAL lib. And it is released for multiple platforms that you can use a classifier to specify the platform you are building the application on.

### Maven<a href="https://opendal.apache.org/bindings/java/#maven" class="hash-link" aria-label="Direct link to Maven" translate="no" title="Direct link to Maven">â€‹</a>

Generally, you can first add the `os-maven-plugin` for automatically detect the classifier based on your platform:

``` prism-code
<build>
<extensions>
  <extension>
    <groupId>kr.motd.maven</groupId>
    <artifactId>os-maven-plugin</artifactId>
    <version>1.7.0</version>
  </extension>
</extensions>
</build>
```

Then add the dependency to `opendal` as following:

``` prism-code
<dependencies>
  <dependency>
    <groupId>org.apache.opendal</groupId>
    <artifactId>opendal</artifactId>
    <version>${opendal.version}</version>
  </dependency>
  <dependency>
    <groupId>org.apache.opendal</groupId>
    <artifactId>opendal</artifactId>
    <version>${opendal.version}</version>
    <classifier>${os.detected.classifier}</classifier>
  </dependency>
</dependencies>
```

### Gradle<a href="https://opendal.apache.org/bindings/java/#gradle" class="hash-link" aria-label="Direct link to Gradle" translate="no" title="Direct link to Gradle">â€‹</a>

For Gradle, you can first add the `com.google.osdetector` for automatically detect the classifier based on your platform:

``` prism-code
plugins {
    id "com.google.osdetector" version "1.7.3"
}
```

Then add the dependency to \`opendal as following:

``` prism-code
dependencies {
    implementation "org.apache.opendal:opendal:$opendalVersion"
    implementation "org.apache.opendal:opendal:$opendalVersion:$osdetector.classifier"
}
```

### Classified library<a href="https://opendal.apache.org/bindings/java/#classified-library" class="hash-link" aria-label="Direct link to Classified library" translate="no" title="Direct link to Classified library">â€‹</a>

Note that the dependency without classifier ships all classes and resources except the "opendal_java" shared library. And those with classifier bundle only the shared library.

For downstream usage, it's recommended:

- Depend on the one without classifier to write code;
- Depend on the classified ones with "test" for testing.

To load the shared library correctly, you can choose one of the following approaches:

- Append the classified JARs to the classpath at the runtime;
- Depend on the classified JARs and build a fat JAR (You may need to depend on all the provided classified JARs for running on multiple platforms);
- Build your own "opendal_java" shared library and specify "-Djava.library.path" to the folder containing that shared library.

## Build<a href="https://opendal.apache.org/bindings/java/#build" class="hash-link" aria-label="Direct link to Build" translate="no" title="Direct link to Build">â€‹</a>

This project provides OpenDAL Java bindings with artifact name `opendal`. It depends on JDK 8 or later.

You can use Maven to build both Rust dynamic lib and JAR files with one command now:

``` prism-code
./mvnw clean package -DskipTests=true
```

## Run tests<a href="https://opendal.apache.org/bindings/java/#run-tests" class="hash-link" aria-label="Direct link to Run tests" translate="no" title="Direct link to Run tests">â€‹</a>

Currently, all tests are written in Java.

You can run the base tests with the following command:

``` prism-code
./mvnw clean verify
```

## Code style<a href="https://opendal.apache.org/bindings/java/#code-style" class="hash-link" aria-label="Direct link to Code style" translate="no" title="Direct link to Code style">â€‹</a>

This project uses [spotless](https://github.com/diffplug/spotless) for code formatting so that all developers share a consistent code style without bikeshedding on it.

You can apply the code style with the following command::

``` prism-code
./mvnw spotless:apply
```

## Run behavior tests<a href="https://opendal.apache.org/bindings/java/#run-behavior-tests" class="hash-link" aria-label="Direct link to Run behavior tests" translate="no" title="Direct link to Run behavior tests">â€‹</a>

Services behavior tests read necessary configs from env vars or the `.env` file.

You can copy [.env.example](https://opendal.apache.org/.env.example) to `${project.rootdir}/.env` and change the values on need, or directly set env vars with `export KEY=VALUE`.

Take `fs` for example, we need to enable bench on `fs` on `/tmp/`:

``` prism-code
OPENDAL_TEST=fs
OPENDAL_FS_ROOT=/tmp/
```

You can run service behavior tests of enabled with the following command:

``` prism-code
./mvnw test -Dtest="behavior.*Test"
```

Remember to enable the necessary features via `-Dcargo-build.features=services-xxx` when running specific service test:

``` prism-code
export OPENDAL_TEST=redis
export OPENDAL_REDIS_ENDPOINT=tcp://127.0.0.1:6379
export OPENDAL_REDIS_ROOT=/
export OPENDAL_REDIS_DB=0
./mvnw test -Dtest="behavior.*Test" -Dcargo-build.features=services-redis
```

## Used by<a href="https://opendal.apache.org/bindings/java/#used-by" class="hash-link" aria-label="Direct link to Used by" translate="no" title="Direct link to Used by">â€‹</a>

Check out the [users](https://github.com/apache/opendal/blob/main/bindings/java/users.md) list for more details on who is using OpenDAL.

## License and Trademarks<a href="https://opendal.apache.org/bindings/java/#license-and-trademarks" class="hash-link" aria-label="Direct link to License and Trademarks" translate="no" title="Direct link to License and Trademarks">â€‹</a>

Licensed under the Apache License, Version 2.0: <http://www.apache.org/licenses/LICENSE-2.0>

Apache OpenDAL, OpenDAL, and Apache are either registered trademarks or trademarks of the Apache Software Foundation.

<a href="https://github.com/apache/opendal/tree/main/website/docs/20-bindings/java.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/bindings/java/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/bindings/haskell/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Haskell ðŸš§

<a href="https://opendal.apache.org/bindings/lua/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Lua ðŸš§

- <a href="https://opendal.apache.org/bindings/java/#useful-links" class="table-of-contents__link toc-highlight">Useful Links</a>
- <a href="https://opendal.apache.org/bindings/java/#example" class="table-of-contents__link toc-highlight">Example</a>
- <a href="https://opendal.apache.org/bindings/java/#getting-started" class="table-of-contents__link toc-highlight">Getting Started</a>
  - <a href="https://opendal.apache.org/bindings/java/#maven" class="table-of-contents__link toc-highlight">Maven</a>
  - <a href="https://opendal.apache.org/bindings/java/#gradle" class="table-of-contents__link toc-highlight">Gradle</a>
  - <a href="https://opendal.apache.org/bindings/java/#classified-library" class="table-of-contents__link toc-highlight">Classified library</a>
- <a href="https://opendal.apache.org/bindings/java/#build" class="table-of-contents__link toc-highlight">Build</a>
- <a href="https://opendal.apache.org/bindings/java/#run-tests" class="table-of-contents__link toc-highlight">Run tests</a>
- <a href="https://opendal.apache.org/bindings/java/#code-style" class="table-of-contents__link toc-highlight">Code style</a>
- <a href="https://opendal.apache.org/bindings/java/#run-behavior-tests" class="table-of-contents__link toc-highlight">Run behavior tests</a>
- <a href="https://opendal.apache.org/bindings/java/#used-by" class="table-of-contents__link toc-highlight">Used by</a>
- <a href="https://opendal.apache.org/bindings/java/#license-and-trademarks" class="table-of-contents__link toc-highlight">License and Trademarks</a>
