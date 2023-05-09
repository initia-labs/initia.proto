import com.google.protobuf.gradle.*
import org.gradle.kotlin.dsl.proto

plugins {
    id("java-library")
    id("maven-publish")
    kotlin("jvm") version "1.8.21" apply false
    id("com.google.protobuf") version "0.9.3"
}

allprojects {
    apply {
        plugin("java-library")
        plugin("com.google.protobuf")
        plugin("signing")
        plugin("maven-publish")
    }

    group = "initia"
    version = "0.1.0"

    repositories {
        mavenCentral()
    }

    protobuf {
        generatedFilesBaseDir = "${projectDir.absolutePath}/src"
    }

    sourceSets {
        main {
            proto {
                srcDir("${rootProject.rootDir.parent}/initia/third_party/proto")
                srcDir("${rootProject.rootDir.parent}/initia/proto")
            }
        }
    }

    java {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8

        withSourcesJar()
    }

    tasks.getByName<Test>("test") {
        useJUnitPlatform()
    }

    gradle.taskGraph.whenReady {
        allTasks.filter { it.name.contains("proto", true) }
            .forEach { it.outputs.upToDateWhen { false } }
    }

    
    publishing {
        publications {
            create<MavenPublication>("initiaProto") {
                from(components["java"])
        }
    }

    repositories {
        maven {
            name = "initiaLabs"
            url = uri(layout.buildDirectory.dir("repo"))
        }
    }
}

}

dependencies {
    val protobufVersion: String by project

    api("com.google.protobuf:protobuf-java:$protobufVersion")
    api("com.google.protobuf:protobuf-java-util:$protobufVersion")

    compileOnly("org.apache.tomcat:annotations-api:6.0.53")
}

tasks.create("cleanProto") {
    group = "other"

    doLast {
        delete("${project.projectDir.absolutePath}/src/main/java")
    }

    finalizedBy(tasks.getByName("clean"))
}
