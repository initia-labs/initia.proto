import com.google.protobuf.gradle.protobuf

plugins {
    id("java-library")
    id("signing")
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
        //generatedFilesBaseDir = "${projectDir.absolutePath}/src"
    }

    sourceSets {
        main {
            proto {
                srcDir("${rootProject.rootDir.parent}/initia/proto")
                srcDir("${rootProject.rootDir.parent}/cosmos-sdk/proto")
                srcDir("${rootProject.rootDir.parent}/cosmos-proto/proto")
                srcDir("${rootProject.rootDir.parent}/ibc-go/proto")
                srcDir("${rootProject.rootDir.parent}/ics23/proto")
                srcDir("${rootProject.rootDir.parent}/pob/proto")
                srcDir("${rootProject.rootDir.parent}/third_party")
            }
        }
    }

    java {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11

        withSourcesJar()
        withJavadocJar()
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
            create<MavenPublication>("lib") {
                groupId = project.group.toString()
                artifactId = project.name
                version = project.version.toString()

                from(components["java"])

                pom {
                    name.set(project.name)
                    description.set("Initia Protobuf Builds")
                    url.set("https://github.com/initia-labs/initia.proto")
                    licenses {
                        license {
                            name.set("The Apache License, Version 2.0")
                            url.set("http://www.apache.org/licenses/LICENSE-2.0.txt")
                        }
                    }
                    developers {
                        developer {
                            id.set("initia-labs")
                            name.set("Initia-Labs")
                        }
                    }
                    scm {
                        connection.set("scm:git:git://github.com/initia-labs/initia.proto.git")
                        developerConnection.set("scm:git:git://github.com/initia-labs/initia.proto.git")
                        url.set("https://github.com/initia-labs/initia.proto")
                    }
                }
            }
        }

    }

    signing {
        sign(publishing.publications["lib"])
    }
}

dependencies {
    val protobufVersion: String by project

    api("com.google.protobuf:protobuf-java:$protobufVersion")
    api("com.google.protobuf:protobuf-java-util:$protobufVersion")

    compileOnly("javax.annotation:javax.annotation-api:1.3.2")
}

tasks.create("cleanProto") {
    group = "other"

    doLast {
        delete("${project.projectDir.absolutePath}/src/main/java")
    }

    finalizedBy(tasks.getByName("clean"))
}
