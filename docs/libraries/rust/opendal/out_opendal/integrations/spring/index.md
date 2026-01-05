- <a href="https://opendal.apache.org/" class="breadcrumbs__link" aria-label="Home page"><img src="out_opendal/integrations/spring/index_media/edf54a765e27bedcc87f5708545b58efaaa38a1a.svg" class="breadcrumbHomeIcon_q3uS" /></a>
- <a href="https://opendal.apache.org/category/integrations/" class="breadcrumbs__link">Integrations</a>
- Spring

On this page

# Spring

## Apache OpenDALâ„¢ Spring Integrations

Apache OpenDALâ„¢ Spring Integrations provide seamless integration between the Apache OpenDAL library and Spring applications. This project offers both synchronous and asynchronous configurations tailored to different Spring environments.

## Overview<a href="https://opendal.apache.org/integrations/spring/#overview" class="hash-link" aria-label="Direct link to Overview" translate="no" title="Direct link to Overview">â€‹</a>

This project includes three primary modules:

- opendal-spring: Core integration module for [Spring](https://spring.io/) applications.
- opendal-spring-boot-starter: Synchronous starter for [Spring WebMVC](https://docs.spring.io/spring-framework/reference/web/webmvc.html).
- opendal-spring-boot-starter-reactive: Asynchronous starter for [Spring WebFlux](https://docs.spring.io/spring-framework/reference/web/webflux.html).

## Features<a href="https://opendal.apache.org/integrations/spring/#features" class="hash-link" aria-label="Direct link to Features" translate="no" title="Direct link to Features">â€‹</a>

- SpringBoot autoconfiguration support for an OpenDALTemplate/ReactiveOpenDALTemplate instance.

## Prerequisites<a href="https://opendal.apache.org/integrations/spring/#prerequisites" class="hash-link" aria-label="Direct link to Prerequisites" translate="no" title="Direct link to Prerequisites">â€‹</a>

This project requires JDK 17 or later and supports Spring 6 and Spring Boot 3.

## Getting Started With Spring Boot Starter<a href="https://opendal.apache.org/integrations/spring/#getting-started-with-spring-boot-starter" class="hash-link" aria-label="Direct link to Getting Started With Spring Boot Starter" translate="no" title="Direct link to Getting Started With Spring Boot Starter">â€‹</a>

Below is a brief example demonstrating how to use the OpenGemini Spring Boot Starter in a Java application.

### Maven Configuration<a href="https://opendal.apache.org/integrations/spring/#maven-configuration" class="hash-link" aria-label="Direct link to Maven Configuration" translate="no" title="Direct link to Maven Configuration">â€‹</a>

Add the following dependency to your project's `pom.xml`:

``` prism-code
<dependency>
    <groupId>org.apache.opendal</groupId>
    <artifactId>opendal-spring-boot-starter</artifactId>
    <version>${version}</version>
</dependency>
```

### SpringBoot Application Configuration<a href="https://opendal.apache.org/integrations/spring/#springboot-application-configuration" class="hash-link" aria-label="Direct link to SpringBoot Application Configuration" translate="no" title="Direct link to SpringBoot Application Configuration">â€‹</a>

Following properties can be used in your `application.yaml`:

``` prism-code
spring:
  opendal:
    schema: "fs"
    conf:
      root: "/tmp"
```

## Getting Started With Spring Boot Reactive Starter<a href="https://opendal.apache.org/integrations/spring/#getting-started-with-spring-boot-reactive-starter" class="hash-link" aria-label="Direct link to Getting Started With Spring Boot Reactive Starter" translate="no" title="Direct link to Getting Started With Spring Boot Reactive Starter">â€‹</a>

Below is a brief example demonstrating how to use the OpenGemini Spring Boot Starter in a Java application.

### Maven Configuration<a href="https://opendal.apache.org/integrations/spring/#maven-configuration-1" class="hash-link" aria-label="Direct link to Maven Configuration" translate="no" title="Direct link to Maven Configuration">â€‹</a>

Add the following dependency to your project's `pom.xml`:

``` prism-code
<dependency>
    <groupId>org.apache.opendal</groupId>
    <artifactId>opendal-spring-boot-starter-reactive</artifactId>
    <version>${version}</version>
</dependency>
```

### SpringBoot Reactive Application Configuration<a href="https://opendal.apache.org/integrations/spring/#springboot-reactive-application-configuration" class="hash-link" aria-label="Direct link to SpringBoot Reactive Application Configuration" translate="no" title="Direct link to SpringBoot Reactive Application Configuration">â€‹</a>

Following properties can be used in your `application.yaml`:

``` prism-code
spring:
  opendal:
    schema: "fs"
    conf:
      root: "/tmp"
```

<a href="https://github.com/apache/opendal/tree/main/website/docs/30-integrations/spring.mdx" class="theme-edit-this-page" rel="noopener noreferrer" target="_blank"><img src="out_opendal/integrations/spring/index_media/82254bca835e54e35c885c6543f8416b5aff021e.svg" class="iconEdit_evxu" />Edit this page</a>

Last updated on **Mar 10, 2025** by **Xuanwo**

<a href="https://opendal.apache.org/integrations/parquet/" class="pagination-nav__link pagination-nav__link--prev"></a>

Previous

Parquet

<a href="https://opendal.apache.org/integrations/unftp_sbe/" class="pagination-nav__link pagination-nav__link--next"></a>

Next

Unftp Sbe

- <a href="https://opendal.apache.org/integrations/spring/#overview" class="table-of-contents__link toc-highlight">Overview</a>
- <a href="https://opendal.apache.org/integrations/spring/#features" class="table-of-contents__link toc-highlight">Features</a>
- <a href="https://opendal.apache.org/integrations/spring/#prerequisites" class="table-of-contents__link toc-highlight">Prerequisites</a>
- <a href="https://opendal.apache.org/integrations/spring/#getting-started-with-spring-boot-starter" class="table-of-contents__link toc-highlight">Getting Started With Spring Boot Starter</a>
  - <a href="https://opendal.apache.org/integrations/spring/#maven-configuration" class="table-of-contents__link toc-highlight">Maven Configuration</a>
  - <a href="https://opendal.apache.org/integrations/spring/#springboot-application-configuration" class="table-of-contents__link toc-highlight">SpringBoot Application Configuration</a>
- <a href="https://opendal.apache.org/integrations/spring/#getting-started-with-spring-boot-reactive-starter" class="table-of-contents__link toc-highlight">Getting Started With Spring Boot Reactive Starter</a>
  - <a href="https://opendal.apache.org/integrations/spring/#maven-configuration-1" class="table-of-contents__link toc-highlight">Maven Configuration</a>
  - <a href="https://opendal.apache.org/integrations/spring/#springboot-reactive-application-configuration" class="table-of-contents__link toc-highlight">SpringBoot Reactive Application Configuration</a>
